
// src/deletion.rs

use std::process::Command;
use std::fs;
use std::env;

pub struct DeletionInfo {
    pub name: String,
    pub remove_service: bool,
}

pub fn remove_timer(deletion_info: DeletionInfo) {
    let name = deletion_info.name;
    println!("Removing timer: {}", name);

    // Stop and disable the timer
    let timer_unit = format!("{}.timer", name);
    let _ = Command::new("systemctl").args(&["--user", "stop", &timer_unit]).status();
    let _ = Command::new("systemctl").args(&["--user", "disable", &timer_unit]).status();

    let home = env::var("HOME").unwrap_or_else(|_| "~".to_string());
    let timer_path = format!("{}/.config/systemd/user/{}.timer", home, name);
    let _ = fs::remove_file(&timer_path);

    let _ = Command::new("systemctl").args(&["--user", "daemon-reload"]).status();
    
    if deletion_info.remove_service {
        let resolved_service = format!("{}.service", name);
        let _ = Command::new("systemctl").args(&["--user", "stop", &resolved_service]).status();
        let _ = Command::new("systemctl").args(&["--user", "disable", &resolved_service]).status();
        
        let service_path = format!("{}/.config/systemd/user/{}", home, resolved_service);
        let _ = fs::remove_file(&service_path);
    }
}
