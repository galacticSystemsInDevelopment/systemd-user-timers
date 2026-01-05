// src/cli_parser.rs
use crate::command_rm_lib::DeletionInfo;

pub fn remove_timer(matches: &clap::ArgMatches){
    

    let deletion_info = DeletionInfo {
        name: matches.get_one::<String>("name").unwrap().to_string(),
        remove_service: matches.get_flag("remove-service"),
    };
    crate::command_rm_lib::remove_timer(deletion_info);
}
