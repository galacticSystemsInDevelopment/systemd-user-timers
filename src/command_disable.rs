
pub fn command_disable(_disable_matches: &clap::ArgMatches) {
    let name = _disable_matches
        .get_one::<String>("name")
        .expect("required argument");
    
    match systemd_user_timers_lib::disable::disable(name) {
        Ok(_) => println!("Timer '{}' disabled successfully.", name),
        Err(e) => eprintln!("Error disabling timer '{}': {}", name, e),
    }
}
