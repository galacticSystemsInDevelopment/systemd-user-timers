use std::process::Command;
use std::fs;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Debug)]
pub struct Timer {
    pub name: String,
    pub description: Option<String>,
    pub schedule: String,
    pub executable: Option<String>,
    pub exec_if_missed: bool,
    pub single_use: bool, // retained as the "promise"
    pub repeating: bool,
    pub normal_service: bool,
    pub service: Option<String>,
    pub already_made_service: bool,
    pub enable_at_login: bool,
    pub start_after_create: bool,
}

pub fn add_timer(timer: Timer) {
    // Implementation to add the timer to the system

    println!("Adding timer: {:?}", timer);

    let description_line = if let Some(ref desc) = timer.description {
        format!("Description={}", desc)
    } else {
        String::new()
    };

    // If we need to create a service, build its ExecStart line from executable
    let (service_contents_opt, service_unit_name) = if timer.already_made_service {
        (None, timer.service.clone().unwrap_or_else(|| timer.name.clone()))
    } else {
        // executable must be present unless already_made_service was true
        let exe = timer.executable.as_ref().expect("executable required to create service");
        let esc = exe.replace('\'', "'\\''");
        let exec_start_line = format!("ExecStart=/bin/sh -c '{}'", esc);

        // Service Type depends on repeating: simple if repeating, oneshot otherwise
        let service_type_line = if timer.repeating {
            "Type=simple"
        } else {
            "Type=oneshot"
        };
        let restart_line = if timer.repeating { "Restart=on-failure" } else { "Restart=no" };

        let mut svc = String::new();
        svc.push_str("[Unit]\n");
        if !description_line.is_empty() {
            svc.push_str(&format!("{}\n", description_line));
        }
        svc.push_str("\n[Service]\n");
        svc.push_str(&format!("{}\n{}\n{}\n", service_type_line, exec_start_line, restart_line));
        svc.push_str("\n[Install]\nWantedBy=default.target\n");

        let unit_name = timer.service.clone().unwrap_or_else(|| timer.name.clone());
        (Some(svc), unit_name)
    };

    let persistent_line = if timer.exec_if_missed { "Persistent=yes" } else { "Persistent=no" };
    let timer_trigger_line = if timer.repeating {
        format!("OnUnitActiveSec={}", timer.schedule)
    } else {
        format!("OnActiveSec={}", timer.schedule)
    };

    // timer references the chosen service unit name
    let timer_contents = format!(
        "[Unit]\nDescription=Timer for {}\n\n[Timer]\nUnit={}.service\n{}\n{}\n\n[Install]\nWantedBy=timers.target\n",
        timer.name, service_unit_name, timer_trigger_line, persistent_line
    );

    // determine user systemd unit directory:
    // prefer XDG_CONFIG_HOME when set and non-empty,
    // otherwise fall back to $HOME/.config,
    // otherwise use "~/.config/systemd/user" as a final fallback.
    let unit_dir = match env::var("XDG_CONFIG_HOME").ok().filter(|s| !s.is_empty()) {
        Some(xdg) => format!("{}/systemd/user", xdg),
        None => match env::var("HOME").ok().filter(|s| !s.is_empty()) {
            Some(home) => format!("{}/.config/systemd/user", home),
            None => "~/.config/systemd/user".to_string(),
        },
    };

    if let Err(e) = fs::create_dir_all(&unit_dir) {
        println!("Failed to create user systemd unit dir {}: {}", unit_dir, e);
        return;
    }

    // service unit filename is based on service_unit_name
    let service_path = format!("{}/{}.service", unit_dir, service_unit_name);
    let timer_path = format!("{}/{}.timer", unit_dir, timer.name);

    // only write service file when we created it (not already-made)
    if let Some(svc) = service_contents_opt {
        if let Err(e) = fs::write(&service_path, svc) {
            println!("Failed to write {}: {}", service_path, e);
            return;
        }
    }
    if let Err(e) = fs::write(&timer_path, timer_contents) {
        println!("Failed to write {}: {}", timer_path, e);
        return;
    }

    // record single-use promise by appending to .single_use.txt if requested
    if timer.single_use {
        let su_path = format!("{}/.single_use.txt", unit_dir);
        let mut already = false;
        if let Ok(content) = fs::read_to_string(&su_path) {
            for line in content.lines() {
                if line.trim() == timer.name {
                    already = true;
                    break;
                }
            }
        }
        if !already {
            if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(&su_path) {
                if let Err(e) = writeln!(f, "{}", timer.name) {
                    println!("Failed to write single-use promise {}: {}", su_path, e);
                }
            } else {
                println!("Failed to open {}", su_path);
            }
        }
    }

    // reload using the user systemd instance
    let _ = Command::new("systemctl").args(&["--user", "daemon-reload"]).status();

    // enable/start logic controlled by flags:
    if timer.enable_at_login && timer.start_after_create {
        match Command::new("systemctl")
            .args(&["--user", "enable", "--now", &format!("{}.timer", timer.name)])
            .status()
        {
            Ok(s) if s.success() => println!("Enabled and started {}.timer (user)", timer.name),
            Ok(s) => println!("systemctl returned status {:?}", s.code()),
            Err(e) => println!("Failed to enable/start timer: {}", e),
        }
    } else if timer.enable_at_login {
        match Command::new("systemctl")
            .args(&["--user", "enable", &format!("{}.timer", timer.name)])
            .status()
        {
            Ok(s) if s.success() => println!("Enabled {}.timer (user)", timer.name),
            Ok(s) => println!("systemctl returned status {:?}", s.code()),
            Err(e) => println!("Failed to enable timer: {}", e),
        }
    } else if timer.start_after_create {
        match Command::new("systemctl")
            .args(&["--user", "start", &format!("{}.timer", timer.name)])
            .status()
        {
            Ok(s) if s.success() => println!("Started {}.timer (user)", timer.name),
            Ok(s) => println!("systemctl returned status {:?}", s.code()),
            Err(e) => println!("Failed to start timer: {}", e),
        }
    } else {
        println!("Timer created but not enabled or started (flags not set).");
    }
}
