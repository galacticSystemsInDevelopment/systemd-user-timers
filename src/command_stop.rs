pub fn command_stop(_stop_matches: &clap::ArgMatches) {    
    let name = _stop_matches
            .get_one::<String>("name")
            .expect("required argument");
    println!("Stopping timer: {}", name);
    println!("{}", systemd_user_timers_lib::stop::stop(name).unwrap());
}