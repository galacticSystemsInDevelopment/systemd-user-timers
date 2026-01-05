pub fn list_timers() -> Result<String, std::io::Error> {
    let output = std::process::Command::new("systemctl")
        .args(&["--user", "list-unit-files", "--type=timer"])
        .output()?; // Capture the output

    // Convert output to a String
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
