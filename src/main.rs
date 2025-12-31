mod command_add;
mod timers;
use crate::command_add::add_timer;

use clap::{Command, arg, command, value_parser};

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("add")
                .about("Add a timer")
                .args([
                    arg!(-e --exec <EXECUTABLE> "The executable the timer will run")
                        .value_parser(value_parser!(String)),
                    arg!(-m --"exec-if-missed" "Execute immediately if missed")
                        .action(clap::ArgAction::SetTrue)
                        .default_value("false"),
                    arg!(-d --desc <DESCRIPTION> "A description of the timer")
                        .value_parser(value_parser!(String)),
                    arg!(-s --schedule <SCHEDULE> "The schedule for the timer")
                        .value_parser(value_parser!(String)),
                    arg!(-n --name <NAME> "Optional: The name for the timer")
                        .value_parser(value_parser!(String)),
                    // recurring (official)
                    arg!(--"recurring" "Whether the timer is recurring")
                        .action(clap::ArgAction::SetTrue)
                        .default_value("false"),
                    // legacy alias kept as a separate, hidden flag with explicit action to avoid clap validation panics
                    arg!(--"repeating" "Deprecated: use --recurring")
                        .action(clap::ArgAction::SetTrue)
                        .hide(true)
                        .default_value("false"),
                    arg!(--"on-calendar" "Use OnCalendar= (systemd calendar schedule) instead of OnActiveSec/OnUnitActiveSec")
                        .action(clap::ArgAction::SetTrue)
                        .default_value("false"),
                    arg!(--"from-boot" "Make schedule relative to system boot (OnBootSec=)")
                        .action(clap::ArgAction::SetTrue)
                        .default_value("false"),
                    arg!(--"single-use" "Whether the timer is single-use")
                        .action(clap::ArgAction::SetTrue)
                        .default_value("false"),
                    arg!(--"enable-at-login" "Enable the timer for the user at login")
                        .action(clap::ArgAction::SetTrue)
                        .default_value("false"),
                    arg!(--"start-after-create" "Start the timer immediately after creating it")
                        .action(clap::ArgAction::SetTrue)
                        .default_value("false"),
                    arg!(--service <SERVICE> "Specify service unit name to create/use")
                        .value_parser(value_parser!(String)),
                    arg!(--"already-made-service" "Assume the service already exists; do not write a service file")
                        .action(clap::ArgAction::SetTrue)
                        .default_value("false"),
                    arg!(--"normal-service" "Whether the timer activates a normal service instead of a one-shot")
                        .action(clap::ArgAction::SetTrue)
                        .default_value("false"),
                ])
        ).subcommand(
            Command::new("remove")
                .about("Remove a timer")
                .args([
                    arg!(<name> "The name of the timer to remove")
                        .value_parser(value_parser!(String)),
                    arg!(--"remove-service" "Also remove the associated service unit file if it exists")
                        .action(clap::ArgAction::SetTrue)
                        .default_value("false"),
                ])
        ).subcommand(
            Command::new("list")
                .about("List all timers")
        ).subcommand(
            Command::new("start")
                .about("Start a timer")
                .args([
                    arg!(<name> "The name of the timer to start")
                        .value_parser(value_parser!(String)),
                ])
        ).subcommand(
            Command::new("stop")
                .about("Stop a timer")
                .args([
                    arg!(<name> "The name of the timer to stop")
                        .value_parser(value_parser!(String)),
                ])
        ).subcommand(
            Command::new("status")
                .about("Show status of a timer")
                .args([
                    arg!(<name> "The name of the timer to show status for")
                        .value_parser(value_parser!(String)),
                ])
        ).subcommand(
            Command::new("reload")
                .about("Reload systemd user daemon")
        ).subcommand(
            Command::new("enable")
                .about("Enable a timer")
                .args([
                    arg!(<name> "The name of the timer to enable")
                        .value_parser(value_parser!(String)),
                ])
        ).subcommand(
            Command::new("disable")
                .about("Disable a timer")
                .args([
                    arg!(<name> "The name of the timer to disable")
                        .value_parser(value_parser!(String)),
                ])
        ).subcommand(
            Command::new("help")
                .about("Show help information")
        ).get_matches();

    if let Some(add_matches) = matches.subcommand_matches("add") {
        add_timer(add_matches);
    } else if let Some(remove_matches) = matches.subcommand_matches("remove") {
        let name = remove_matches
            .get_one::<String>("name")
            .expect("required argument");
        println!("Removing timer: {}", name);
            // reload using the user systemd instance
        let timer_unit = format!("{}.timer", name);
        let _ = std::process::Command::new("systemctl").args(&["--user", "stop", &timer_unit]).status();
        let _ = std::process::Command::new("systemctl").args(&["--user", "disable", &timer_unit]).status();

        // Use HOME to build the user units path and remove files directly
        let home = std::env::var("HOME").unwrap_or_else(|_| "~".to_string());
        let timer_path = format!("{}/.config/systemd/user/{}.timer", home, name);
        let _ = std::fs::remove_file(&timer_path);

        let _ = std::process::Command::new("systemctl").args(&["--user", "daemon-reload"]).status();
        if remove_matches.get_flag("remove-service") {
            // Try to read the timer file and extract Unit= value (if present).
            // Fallback to "<name>.service" if not found or file unreadable.
            let resolved_service = match std::fs::read_to_string(&timer_path) {
                Ok(contents) => contents.lines()
                    .map(|l| l.trim())
                    .filter(|l| !l.starts_with('#') && !l.starts_with(';'))
                    .find_map(|l| {
                        if let Some(rest) = l.strip_prefix("Unit=") {
                            // Remove inline comments after the value and trim
                            let val = rest.split(|c| c == '#' || c == ';').next().unwrap_or("").trim();
                            if val.is_empty() { None } else { Some(val.to_string()) }
                        } else {
                            None
                        }
                    })
                    .unwrap_or_else(|| format!("{}.service", name)),
                Err(_) => format!("{}.service", name),
            };

            let _ = std::process::Command::new("systemctl").args(&["--user", "stop", &resolved_service]).status();
            let _ = std::process::Command::new("systemctl").args(&["--user", "disable", &resolved_service]).status();

            let service_path = format!("{}/.config/systemd/user/{}", home, resolved_service);
            let _ = std::fs::remove_file(&service_path);
        }
    } else if let Some(_list_matches) = matches.subcommand_matches("list") {
        println!("Listing timers:");
        let _ = std::process::Command::new("systemctl").args(&["--user", "list-unit-files", "--type=timer"]).status();
    } else if let Some(_start_matches) = matches.subcommand_matches("start") {
        let name = _start_matches
            .get_one::<String>("name")
            .expect("required argument");
        println!("Starting timer: {}", name);
        let timer_unit = format!("{}.timer", name);
        let _ = std::process::Command::new("systemctl").args(&["--user", "start", &timer_unit]).status();
    } else if let Some(_stop_matches) = matches.subcommand_matches("stop") {
        let name = _stop_matches
            .get_one::<String>("name")
            .expect("required argument");
        println!("Stopping timer: {}", name);
        let timer_unit = format!("{}.timer", name);
        let _ = std::process::Command::new("systemctl").args(&["--user", "stop", &timer_unit]).status();
    } else if let Some(_status_matches) = matches.subcommand_matches("status") {
        let name = _status_matches
            .get_one::<String>("name")
            .expect("required argument");
        println!("Showing status of timer: {}", name);
        let timer_unit = format!("{}.timer", name);
        let _ = std::process::Command::new("systemctl").args(&["--user", "show", &timer_unit]).status();
    } else if let Some(_reload_matches) = matches.subcommand_matches("reload") {
        println!("Reloading systemd user daemon");
        let _ = std::process::Command::new("systemctl").args(&["--user", "daemon-reload"]).status();
    } else if let Some(_enable_matches) = matches.subcommand_matches("enable") {
        let name = _enable_matches
            .get_one::<String>("name")
            .expect("required argument");
        println!("Enabling timer: {}", name);
        let timer_unit = format!("{}.timer", name);
        let _ = std::process::Command::new("systemctl").args(&["--user", "enable", &timer_unit]).status();
    } else if let Some(_disable_matches) = matches.subcommand_matches("disable") {
        let name = _disable_matches
            .get_one::<String>("name")
            .expect("required argument");
        println!("Disabling timer: {}", name);
        let timer_unit = format!("{}.timer", name);
        let _ = std::process::Command::new("systemctl").args(&["--user", "disable", &timer_unit]).status();
    } else if let Some(_help_matches) = matches.subcommand_matches("help") {
        println!("Help information:");
        println!("Help hasn't been implemented yet. Check README or use --help with specific commands.");
    } else {
        println!("No valid subcommand was used. Use --help for more information.");
    }
}