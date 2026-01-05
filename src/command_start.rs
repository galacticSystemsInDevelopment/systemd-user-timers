

pub fn start(_start_matches: &clap::ArgMatches) {
    let name = _start_matches
            .get_one::<String>("name")
            .expect("required argument");
    crate::command_start_lib::start(name);
}