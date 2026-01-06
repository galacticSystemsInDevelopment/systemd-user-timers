

pub fn start(_start_matches: &clap::ArgMatches) {
    let name = _start_matches
            .get_one::<String>("name")
            .expect("required argument");
    let output = crate::command_start_lib::start(name).unwrap_or_else(|e| {
        eprintln!("Error listing timers: {}", e);
        String::new()  // Returning an empty string on error
    });

    if !output.is_empty() {
        println!("{}", output);  // Print the output
    }
}