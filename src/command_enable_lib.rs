pub fn enable(name: &str) -> Result<String, std::io::Error> {
    let output_message = format!("Enabling timer: {}", name);
    let timer_unit = format!("{}.timer", name);
    
    let output = std::process::Command::new("systemctl")
        .args(&["--user", "enable", &timer_unit])
        .output()?;

    if output.status.success() {
        Ok(format!("{} - Success: {}", output_message, String::from_utf8_lossy(&output.stdout)))
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        Ok(format!("{} - Failed: {}", output_message, error_message))
    }
}
