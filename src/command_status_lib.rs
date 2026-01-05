pub fn show_status(name: &str) -> Result<String, std::io::Error> {
    let timer_unit = format!("{}.timer", name);
    let output = std::process::Command::new("systemctl")
        .args(&["--user", "show", &timer_unit])
        .output()?; // Capture the output

    // Convert output to a String
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
