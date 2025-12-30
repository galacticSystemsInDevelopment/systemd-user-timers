use clap::{Command, arg, command, value_parser};

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("add")
                .about("Add a timer")
                .args([
                    arg!(-e --exec <EXECUTABLE> "The executable the timer will run")
                        .value_parser(value_parser!(String)),
                    arg!(-m --"exec-if-missed" "Execute immediately if missed")
                        .action(clap::ArgAction::SetTrue),
                    arg!(-d --desc <DESCRIPTION> "A description of the timer")
                        .value_parser(value_parser!(String)),
                    arg!(-s --schedule <SCHEDULE> "The schedule for the timer")
                        .value_parser(value_parser!(String)),
                    arg!(-n --name <NAME> "Optional: The name for the timer")
                        .value_parser(value_parser!(String)),
                ]),
        )
        .get_matches();

    if let Some(add_matches) = matches.subcommand_matches("add") {
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
        
        if let Some(schedule) = add_matches.get_one::<String>("sche
        ") {
            println!("Schedule: {}", schedule);
        }
        
        if let Some(name) = add_matches.get_one::<String>("name") {
            println!("Name: {}", name);
        }
    } else {
        println!("No subcommand");
    }
}
