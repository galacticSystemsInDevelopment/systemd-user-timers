pub fn disable(name: &str) -> Result<(), std::io::Error> {
    let output_message = format!("Disabling timer: {}", name);
    let timer_unit = format!("{}.timer", name);
    
    let output = std::process::Command::new("systemctl")
        .args(&["--user", "disable", &timer_unit])
        .output()?;

    if output.status.success() {
        println!("{} - Success: {}", output_message, String::from_utf8_lossy(&output.stdout));
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        println!("{} - Failed: {}", output_message, error_message);
        return Err(std::io::Error::new(std::io::ErrorKind::Other, error_message.to_string()));
    }

    Ok(()) // Return Ok on success
}
