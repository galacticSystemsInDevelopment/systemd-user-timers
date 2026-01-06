use systemd_user_timers_lib::remove_timer::{remove_timer as remove_timer_fn, DeletionInfo};
use clap::ArgMatches;

pub fn remove_timer(matches: &ArgMatches) {
    let deletion_info = DeletionInfo {
        name: matches.get_one::<String>("name").unwrap().to_string(),
        remove_service: matches.get_flag("remove-service"),
    };

    match remove_timer_fn(deletion_info) {
        Ok(success_message) => {
            println!("{}", success_message);  // Print success message
        }
        Err(error_message) => {
            eprintln!("Error: {}", error_message);  // Print error message
        }
    }
}
