use crate::command_rm_lib::DeletionInfo;

pub fn remove_timer(matches: &clap::ArgMatches) {
    let deletion_info = DeletionInfo {
        name: matches.get_one::<String>("name").unwrap().to_string(),
        remove_service: matches.get_flag("remove-service"),
    };

    match crate::command_rm_lib::remove_timer(deletion_info) {
        Ok(success_message) => {
            println!("{}", success_message);  // Print success message
        }
        Err(error_message) => {
            eprintln!("Error: {}", error_message);  // Print error message
        }
    }
}
