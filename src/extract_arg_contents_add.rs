use clap::ArgMatches;
use systemd_user_timers_lib::timers::Timer;


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
	let description = add_matches
		.get_one::<String>("desc")
		.map(|s| s.to_string())
		.or_else(|| {
			if already_made_service {
				service_name.as_ref().map(|s| {
					let svc = if s.ends_with(".service") {
						s.trim_end_matches(".service")
					} else {
						s.as_str()
					};
					format!("Run {}", svc)
				})
			} else {
				exec_opt.as_ref().map(|s| {
					let exe = std::path::Path::new(s)
						.file_name()
						.and_then(|os| os.to_str())
						.unwrap_or(s.as_str());
					format!("Run {}", exe)
				})
			}
		});

	let name = add_matches
		.get_one::<String>("name")
		.map(|s| s.to_string())
		.unwrap_or_else(|| {
			if already_made_service {
				if let Some(svc) = service_name.as_ref() {
					if svc.ends_with(".service") {
						return svc.trim_end_matches(".service").to_string();
					} else {
						return svc.clone();
					}
				}
			}
			if let Some(exec) = exec_opt.as_ref() {
				if let Some(stem) = std::path::Path::new(exec).file_stem().and_then(|os| os.to_str()) {
					return stem.to_string();
				}
			}
			schedule.replace(|c: char| !c.is_ascii_alphanumeric(), "_")
		});

	let exec_if_missed = add_matches.get_flag("exec-if-missed");
	let single_use = add_matches.get_flag("single-use");
	let normal_service = add_matches.get_flag("normal-service");
	let enable_at_login = add_matches.get_flag("enable-at-login");
	let start_after_create = add_matches.get_flag("start-after-create");
	let recurring = add_matches.get_flag("recurring") || add_matches.get_flag("repeating");
	let on_calendar = add_matches.get_flag("on-calendar");
	let from_boot = add_matches.get_flag("from-boot");

	let timer = Timer {
		name,
		description,
		schedule,
		executable: exec_opt,
		exec_if_missed,
		single_use,
		recurring,
		on_calendar,
		from_boot,
		normal_service,
		service: service_name,
		already_made_service,
		enable_at_login,
		start_after_create,
	};

	println!("{}", systemd_user_timers_lib::add_timer::add_timer(timer));
}
