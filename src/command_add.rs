use::clap::ArgMatches;

pub fn add_timer(add_matches: &ArgMatches) {
    println!(
        "Add subcommand exec-if-missed value: {}",
        add_matches.get_flag("exec-if-missed")
    );

    // Retrieving the other arguments safely, using .value_of
    if let Some(executable) = add_matches.get_one::<String>("exec") {
        println!("Executable: {}", executable);
    }
    
    if let Some(description) = add_matches.get_one::<String>("desc") {
        println!("Description: {}", description);
    }
    
    if let Some(schedule) = add_matches.get_one::<String>("schedule") {
        println!("Schedule: {}", schedule);
    }
    
    if let Some(name) = add_matches.get_one::<String>("name") {
        println!("Name: {}", name);
    }
}