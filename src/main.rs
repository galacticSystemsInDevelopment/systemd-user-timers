mod extract_arg_contents_add;
mod write_to_file;
mod timers;
mod command_rm;
mod command_list;
mod command_list_lib;
use crate::extract_arg_contents_add::add_timer;
use crate::command_rm::remove_timer;
mod command_rm_lib;
use clap::{Command, arg, command, value_parser};
mod command_start;
mod command_start_lib;
mod command_stop;
mod command_stop_lib;
mod command_status;
mod command_status_lib;
mod command_enable;
mod command_enable_lib;
mod command_disable;
mod command_disable_lib;

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("add")
                .about("Add a timer")
                .args([
                    arg!(-e --exec <EXECUTABLE> "The executable the timer will run")
                        .value_parser(value_parser!(String)),
                    arg!(-m --"exec-if-missed" "Execute immediately if missed")
                        .action(clap::ArgAction::SetTrue)
                        .default_value("false"),
                    arg!(-d --desc <DESCRIPTION> "A description of the timer")
                        .value_parser(value_parser!(String)),
                    arg!(-s --schedule <SCHEDULE> "The schedule for the timer")
                        .value_parser(value_parser!(String)),
                    arg!(-n --name <NAME> "Optional: The name for the timer")
                        .value_parser(value_parser!(String)),
                    // recurring (official)
                    arg!(--"recurring" "Whether the timer is recurring")
                        .action(clap::ArgAction::SetTrue)
                        .default_value("false"),
                    // legacy alias kept as a separate, hidden flag with explicit action to avoid clap validation panics
                    arg!(--"repeating" "Deprecated: use --recurring")
                        .action(clap::ArgAction::SetTrue)
                        .hide(true)
                        .default_value("false"),
                    arg!(--"on-calendar" "Use OnCalendar= (systemd calendar schedule) instead of OnActiveSec/OnUnitActiveSec")
                        .action(clap::ArgAction::SetTrue)
                        .default_value("false"),
                    arg!(--"from-boot" "Make schedule relative to system boot (OnBootSec=)")
                        .action(clap::ArgAction::SetTrue)
                        .default_value("false"),
                    arg!(--"single-use" "Whether the timer is single-use")
                        .action(clap::ArgAction::SetTrue)
                        .default_value("false"),
                    arg!(--"enable-at-login" "Enable the timer for the user at login")
                        .action(clap::ArgAction::SetTrue)
                        .default_value("false"),
                    arg!(--"start-after-create" "Start the timer immediately after creating it")
                        .action(clap::ArgAction::SetTrue)
                        .default_value("false"),
                    arg!(--service <SERVICE> "Specify service unit name to create/use")
                        .value_parser(value_parser!(String)),
                    arg!(--"already-made-service" "Assume the service already exists; do not write a service file")
                        .action(clap::ArgAction::SetTrue)
                        .default_value("false"),
                    arg!(--"normal-service" "Whether the timer activates a normal service instead of a one-shot")
                        .action(clap::ArgAction::SetTrue)
                        .default_value("false"),
                ])
        ).subcommand(
            Command::new("remove")
                .about("Remove a timer")
                .args([
                    arg!(<name> "The name of the timer to remove")
                        .value_parser(value_parser!(String)),
                    arg!(--"remove-service" "Also remove the associated service unit file if it exists")
                        .action(clap::ArgAction::SetTrue)
                        .default_value("false"),
                ])
        ).subcommand(
            Command::new("list")
                .about("List all timers")
        ).subcommand(
            Command::new("start")
                .about("Start a timer")
                .args([
                    arg!(<name> "The name of the timer to start")
                        .value_parser(value_parser!(String)),
                ])
        ).subcommand(
            Command::new("stop")
                .about("Stop a timer")
                .args([
                    arg!(<name> "The name of the timer to stop")
                        .value_parser(value_parser!(String)),
                ])
        ).subcommand(
            Command::new("status")
                .about("Show status of a timer")
                .args([
                    arg!(<name> "The name of the timer to show status for")
                        .value_parser(value_parser!(String)),
                ])
        ).subcommand(
            Command::new("reload")
                .about("Reload systemd user daemon")
        ).subcommand(
            Command::new("enable")
                .about("Enable a timer")
                .args([
                    arg!(<name> "The name of the timer to enable")
                        .value_parser(value_parser!(String)),
                ])
        ).subcommand(
            Command::new("disable")
                .about("Disable a timer")
                .args([
                    arg!(<name> "The name of the timer to disable")
                        .value_parser(value_parser!(String)),
                ])
        ).get_matches();

    if let Some(add_matches) = matches.subcommand_matches("add") {
        add_timer(add_matches);
    } else if let Some(remove_matches) = matches.subcommand_matches("remove") {
        remove_timer(remove_matches);
    } else if let Some(_list_matches) = matches.subcommand_matches("list") {
        crate::command_list::list_timers();
    } else if let Some(_start_matches) = matches.subcommand_matches("start") {
        crate::command_start::start(_start_matches);
    } else if let Some(_stop_matches) = matches.subcommand_matches("stop") {
        crate::command_stop::command_stop(_stop_matches);
    } else if let Some(_status_matches) = matches.subcommand_matches("status") {
        crate::command_status::command_status(_status_matches);
    } else if let Some(_reload_matches) = matches.subcommand_matches("reload") {
        println!("Reloading systemd user daemon");
        let _ = std::process::Command::new("systemctl").args(&["--user", "daemon-reload"]).status();
    } else if let Some(_enable_matches) = matches.subcommand_matches("enable") {
        crate::command_enable::command_enable(_enable_matches);
    } else if let Some(_disable_matches) = matches.subcommand_matches("disable") {
        crate::command_disable::command_disable(_disable_matches);
    } else {
        println!("No valid subcommand was used. Use --help for more information.");
    }
} 