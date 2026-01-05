
pub fn command_stop(name: &str) {
    let timer_unit = format!("{}.timer", name);
    let _ = std::process::Command::new("systemctl").args(&["--user", "stop", &timer_unit]).status();
}
  