use clap::ArgMatches;
use crate::timers::Timer;

pub fn add_timer(add_matches: &ArgMatches) {
	// existing/new flags
	let already_made_service = add_matches.get_flag("already-made-service");
	let service_name = add_matches.get_one::<String>("service").map(|s| s.to_string());

	// exec is required unless user specified --already-made-service
	let exec_opt = if already_made_service {
		add_matches.get_one::<String>("exec").map(|s| s.to_string())
	} else {
		Some(add_matches
			.get_one::<String>("exec")
			.map(|s| s.to_string())
			.unwrap_or_else(|| {
				eprintln!("--exec is required unless --already-made-service is set");
				std::process::exit(1)
			}))
	};

	let schedule = add_matches
		.get_one::<String>("schedule")
		.map(|s| s.to_string())
		.unwrap_or_else(|| {
			eprintln!("--schedule is required");
			std::process::exit(1)
		});

	// optional
	let description = add_matches.get_one::<String>("desc").map(|s| s.to_string());
	let name = add_matches
		.get_one::<String>("name")
		.map(|s| s.to_string())
		.unwrap_or_else(|| {
			std::path::Path::new(&exec_opt.as_ref().unwrap_or(&"".to_string()))
				.file_stem()
				.and_then(|s| s.to_str())
				.map(|s| s.to_string())
				.unwrap_or_else(|| schedule.replace(|c: char| !c.is_ascii_alphanumeric(), "_"))
		});

	let exec_if_missed = add_matches.get_flag("exec-if-missed");
	let repeating = add_matches.get_flag("repeating");
	let single_use = add_matches.get_flag("single-use");
	let normal_service = add_matches.get_flag("normal-service");
	let enable_at_login = add_matches.get_flag("enable-at-login");
	let start_after_create = add_matches.get_flag("start-after-create");

	let timer = Timer {
		name,
		description,
		schedule,
		executable: exec_opt,
		exec_if_missed,
		single_use,
		repeating,
		normal_service,
		service: service_name,
		already_made_service,
		enable_at_login,
		start_after_create,
	};

	crate::timers::add_timer(timer);
}