use std::process::Command;
use std::fs;
use std::env;

pub struct DeletionInfo {
    pub name: String,
    pub remove_service: bool,
}

pub fn remove_timer(deletion_info: DeletionInfo) -> Result<String, String> {
    let name = deletion_info.name;

    let timer_unit = format!("{}.timer", name);
    let stop_status = Command::new("systemctl")
        .args(&["--user", "stop", &timer_unit])
        .status()
        .map_err(|e| format!("Failed to stop timer {}: {}", name, e))?;

    if !stop_status.success() {
        return Err(format!("Failed to stop timer: {}", name));
    }

    let disable_status = Command::new("systemctl")
        .args(&["--user", "disable", &timer_unit])
        .status()
        .map_err(|e| format!("Failed to disable timer {}: {}", name, e))?;

    if !disable_status.success() {
        return Err(format!("Failed to disable timer: {}", name));
    }

    let home = env::var("HOME").unwrap_or_else(|_| "~".to_string());
    let timer_path = format!("{}/.config/systemd/user/{}.timer", home, name);
    fs::remove_file(&timer_path).map_err(|e| format!("Failed to remove timer file: {}", e))?;

    let reload_status = Command::new("systemctl")
        .args(&["--user", "daemon-reload"])
        .status()
        .map_err(|e| format!("Failed to reload daemon: {}", e))?;

    if !reload_status.success() {
        return Err("Failed to reload daemon".to_string());
    }

    if deletion_info.remove_service {
        let resolved_service = format!("{}.service", name);
        let stop_service_status = Command::new("systemctl")
            .args(&["--user", "stop", &resolved_service])
            .status()
            .map_err(|e| format!("Failed to stop service {}: {}", resolved_service, e))?;

        if !stop_service_status.success() {
            return Err(format!("Failed to stop service: {}", resolved_service));
        }

        let disable_service_status = Command::new("systemctl")
            .args(&["--user", "disable", &resolved_service])
            .status()
            .map_err(|e| format!("Failed to disable service {}: {}", resolved_service, e))?;

        if !disable_service_status.success() {
            return Err(format!("Failed to disable service: {}", resolved_service));
        }

        let service_path = format!("{}/.config/systemd/user/{}", home, resolved_service);
        fs::remove_file(&service_path).map_err(|e| format!("Failed to remove service file: {}", e))?;
    }
    
    Ok(format!("Successfully removed timer: {}", name))
}
