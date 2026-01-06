
pub fn start(name: &str) -> Result<String, std::io::Error> {
    println!("Starting timer: {}", name);
    let timer_unit = format!("{}.timer", name);
    let output = std::process::Command::new("systemctl").args(&["--user", "start", &timer_unit]).output()?;
    // Convert output to a String
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}