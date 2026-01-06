pub fn command_status(_status_matches: &clap::ArgMatches) {
    let name = _status_matches
            .get_one::<String>("name")
            .expect("required argument");
    println!("Showing status of timer: {}", name);
    println!("{}", systemd_user_timers_lib::show_status::show_status(name).unwrap_or_else(|e| {
        eprintln!("Error showing status: {}", e);
        String::new()
    }));
}