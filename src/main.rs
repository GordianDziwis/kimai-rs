use clap::{
    crate_authors, crate_description, crate_name, crate_version, values_t, App, Arg, SubCommand,
};

fn main() {
    fn usize_validator(s: String) -> Result<(), String> {
        match s.parse::<usize>() {
            Ok(_) => Ok(()),
            Err(_) => Err("Input must be integer!".to_string()),
        }
    }

    let config_path_arg = Arg::with_name("config_path")
        .long("config_path")
        .help("Path to a config file")
        .takes_value(true);

    let term_arg = Arg::with_name("term")
        .long("term")
        .short("t")
        .takes_value(true)
        .help("A free search term");

    let user_arg = Arg::with_name("user")
        .short("u")
        .long("user")
        .takes_value(true)
        .validator(usize_validator)
        .help("ID of the user on who's behalf to act");

    let projects_arg = Arg::with_name("projects")
        .short("p")
        .long("projects")
        .help("Limit the returned projects")
        .validator(usize_validator)
        .takes_value(true)
        .multiple(true);

    let customers_arg = Arg::with_name("customers")
        .short("c")
        .long("customers")
        .help("Limit the returned customers")
        .validator(usize_validator)
        .takes_value(true)
        .multiple(true);

    let activities_arg = Arg::with_name("activities")
        .short("a")
        .long("activities")
        .help("Limit the returned customers")
        .validator(usize_validator)
        .takes_value(true)
        .multiple(true);

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("customers")
                .author(crate_authors!())
                .version(crate_version!())
                .about("Get a list of all customers")
                .arg(&config_path_arg)
                .arg(&term_arg),
        )
        .subcommand(
            SubCommand::with_name("projects")
                .author(crate_authors!())
                .version(crate_version!())
                .about("Get a list of all projects")
                .arg(&config_path_arg)
                .arg(&term_arg)
                .arg(&customers_arg),
        )
        .subcommand(
            SubCommand::with_name("activities")
                .author(crate_authors!())
                .version(crate_version!())
                .about("Get a list of all activities")
                .arg(&config_path_arg)
                .arg(&term_arg)
                .arg(&projects_arg),
        )
        .subcommand(
            SubCommand::with_name("timesheet")
                .author(crate_authors!())
                .version(crate_version!())
                .about("Interact with the time sheet.")
                .arg(&config_path_arg)
                .arg(&user_arg)
                .arg(&projects_arg)
                .arg(&customers_arg)
                .arg(&activities_arg)
                .arg(&term_arg),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("customers") {
        kimai::print_customers(
            matches.value_of("config_path").map(|p| p.to_string()),
            matches.value_of("term").map(|t| t.to_string()),
        )
        .unwrap();
    }

    if let Some(matches) = matches.subcommand_matches("projects") {
        kimai::print_projects(
            matches.value_of("config_path").map(|p| p.to_string()),
            match matches.is_present("customers") {
                true => Some(values_t!(matches, "customers", usize).unwrap_or_else(|e| e.exit())),
                false => None,
            },
            matches.value_of("term").map(|t| t.to_string()),
        )
        .unwrap();
    }

    if let Some(matches) = matches.subcommand_matches("activities") {
        kimai::print_activities(
            matches.value_of("config_path").map(|p| p.to_string()),
            match matches.is_present("projects") {
                true => Some(values_t!(matches, "projects", usize).unwrap_or_else(|e| e.exit())),
                false => None,
            },
            matches.value_of("term").map(|t| t.to_string()),
        )
        .unwrap();
    }

    if let Some(matches) = matches.subcommand_matches("timesheet") {
        kimai::print_timesheet(
            matches.value_of("config_path").map(|p| p.to_string()),
            matches
                .value_of("user")
                .map(|u| u.parse::<usize>().unwrap()),
            match matches.is_present("customers") {
                true => Some(values_t!(matches, "customers", usize).unwrap_or_else(|e| e.exit())),
                false => None,
            },
            match matches.is_present("projects") {
                true => Some(values_t!(matches, "projects", usize).unwrap_or_else(|e| e.exit())),
                false => None,
            },
            match matches.is_present("activities") {
                true => Some(values_t!(matches, "activities", usize).unwrap_or_else(|e| e.exit())),
                false => None,
            },
        )
        .unwrap();
    }
}
