
pub fn start(name: &str) {
    println!("Starting timer: {}", name);
    let timer_unit = format!("{}.timer", name);
    let _ = std::process::Command::new("systemctl").args(&["--user", "start", &timer_unit]).status();   
}