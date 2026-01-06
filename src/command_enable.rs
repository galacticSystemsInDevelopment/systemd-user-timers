pub fn command_enable(_enable_matches: &clap::ArgMatches) {
    let name = _enable_matches
        .get_one::<String>("name")
        .expect("required argument");
    println!("{}", crate::command_enable_lib::enable(name).unwrap_or_else(|e| {
        eprintln!("Error enabling timer: {}", e);
        String::new()
    }));
}