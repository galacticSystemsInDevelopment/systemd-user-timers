pub fn list_timers() {
    let output = crate::command_list_lib::list_timers().unwrap_or_else(|e| {
        eprintln!("Error listing timers: {}", e);
        String::new()  // Returning an empty string on error
    });

    if !output.is_empty() {
        println!("{}", output);  // Print the output
    }

}
