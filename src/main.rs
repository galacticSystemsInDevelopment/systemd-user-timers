mod command_add;
mod timers;

use crate::command_add::add_timer;

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
        ).get_matches();

    if let Some(add_matches) = matches.subcommand_matches("add") {
        add_timer(add_matches);
    } else {
        println!("No subcommand");
    }
}
