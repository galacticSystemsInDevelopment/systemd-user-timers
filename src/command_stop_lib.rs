
pub fn command_stop(name: &str) -> Result<String, std::io::Error> {
    let timer_unit = format!("{}.timer", name);
    let output = std::process::Command::new("systemctl").args(&["--user", "stop", &timer_unit]).output()?;
    Ok(format!("Stopped timer: {}\n{}", name, String::from_utf8_lossy(&output.stdout)))
}
  