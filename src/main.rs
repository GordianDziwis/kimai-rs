use chrono::prelude::*;
use clap::{
    crate_authors, crate_description, crate_name, crate_version, values_t, App, Arg, SubCommand,
};

macro_rules! arg {
    ($name:expr, $help:expr) => {
        Arg::with_name($name)
            .help($help)
            .takes_value(true)
            .required(true)
    };
    ($name:expr, $help:expr, $validator:expr) => {
        Arg::with_name($name)
            .help($help)
            .takes_value(true)
            .required(true)
            .validator($validator)
    };
    ($name:expr, $short:expr, $long:expr, $help:expr) => {
        Arg::with_name($name)
            .short($short)
            .long($long)
            .help($help)
            .takes_value(true)
    };
    ($name:expr, $short:expr, $long:expr, $help:expr, $validator:expr) => {
        Arg::with_name($name)
            .short($short)
            .long($long)
            .help($help)
            .takes_value(true)
            .validator($validator)
    };
}

fn main() {
    fn usize_validator(s: String) -> Result<(), String> {
        match s.parse::<usize>() {
            Ok(_) => Ok(()),
            Err(_) => Err("Input must be integer!".to_string()),
        }
    }

    fn datetime_validator(s: String) -> Result<(), String> {
        match NaiveDateTime::parse_from_str(&s, kimai::DATETIME_FORMAT) {
            Ok(_) => Ok(()),
            Err(_) => match NaiveTime::parse_from_str(&s, kimai::TIME_FORMAT) {
                Ok(_) => Ok(()),
                Err(_) => Err(format!(
                    "DateTime must be of format \"{}\" or \"{}\"!",
                    kimai::DATETIME_FORMAT,
                    kimai::TIME_FORMAT
                )),
            },
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

    let begin_arg = arg!(
        "begin",
        "b",
        "begin",
        "A beginning time",
        datetime_validator
    );

    let end_arg = arg!("end", "e", "end", "An end time", datetime_validator);

    let project_arg = arg!(
        "project",
        "p",
        "project",
        "ID of a Project",
        usize_validator
    );

    let activity_arg = arg!(
        "activity",
        "a",
        "activity",
        "ID of an activity",
        usize_validator
    );

    let description_arg = arg!(
        "description",
        "d",
        "description",
        "Description to be added to a record"
    );

    let tags_arg = arg!("tags", "t", "tags", "Tags for a timesheet record").multiple(true);
    let id_arg = arg!("id", "ID of a timesheet record", usize_validator);

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
                .arg(&term_arg)
                .subcommand(
                    SubCommand::with_name("recent")
                        .author(crate_authors!())
                        .version(crate_version!())
                        .about("View only recent timesheet records")
                        .arg(&config_path_arg)
                        .arg(&begin_arg)
                        .arg(&user_arg),
                )
                .subcommand(
                    SubCommand::with_name("active")
                        .author(crate_authors!())
                        .version(crate_version!())
                        .about("View only currently active timesheet records")
                        .arg(&config_path_arg),
                )
                .subcommand(
                    SubCommand::with_name("begin")
                        .alias("start")
                        .author(crate_authors!())
                        .version(crate_version!())
                        .about("Begin a new timesheet record")
                        .arg(&config_path_arg)
                        .arg(&user_arg)
                        .arg(&begin_arg)
                        .arg(&project_arg.clone().required(true))
                        .arg(&activity_arg.clone().required(true))
                        .arg(&description_arg)
                        .arg(&tags_arg),
                )
                .subcommand(
                    SubCommand::with_name("end")
                        .alias("stop")
                        .author(crate_authors!())
                        .version(crate_version!())
                        .about("End a given timesheet record")
                        .arg(&config_path_arg)
                        .arg(&id_arg),
                )
                .subcommand(
                    SubCommand::with_name("log")
                        .author(crate_authors!())
                        .version(crate_version!())
                        .about("Log a new timesheet record")
                        .arg(&config_path_arg)
                        .arg(&begin_arg.required(true))
                        .arg(&end_arg)
                        .arg(&project_arg.clone().required(true))
                        .arg(&activity_arg.clone().required(true))
                        .arg(&description_arg)
                        .arg(&tags_arg),
                )
                .subcommand(
                    SubCommand::with_name("change")
                        .aliases(&["update", "patch"])
                        .author(crate_authors!())
                        .version(crate_version!())
                        .about("Change a given timesheet record")
                        .arg(&config_path_arg),
                )
                .subcommand(
                    SubCommand::with_name("restart")
                        .author(crate_authors!())
                        .version(crate_version!())
                        .about("Restart a given timesheet record")
                        .arg(&config_path_arg),
                )
                .subcommand(
                    SubCommand::with_name("delete")
                        .author(crate_authors!())
                        .version(crate_version!())
                        .about("Delete the given timesheet records")
                        .arg(&config_path_arg),
                ),
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
        if let Some(matches) = matches.subcommand_matches("recent") {
            kimai::print_recent_timesheet(
                matches.value_of("config_path").map(|p| p.to_string()),
                matches
                    .value_of("user")
                    .map(|u| u.parse::<usize>().unwrap()),
                matches.value_of("begin").map(|p| p.to_string()),
            )
            .unwrap();
        } else if let Some(matches) = matches.subcommand_matches("active") {
            kimai::print_active_timesheet(matches.value_of("config_path").map(|p| p.to_string()))
                .unwrap();
        } else if let Some(matches) = matches.subcommand_matches("begin") {
            kimai::print_begin_timesheet_record(
                matches.value_of("config_path").map(|p| p.to_string()),
                matches
                    .value_of("user")
                    .map(|u| u.parse::<usize>().unwrap()),
                matches.value_of("project").unwrap().parse().unwrap(),
                matches.value_of("activity").unwrap().parse().unwrap(),
                matches.value_of("begin").map(|p| p.to_string()),
                matches.value_of("description").map(|d| d.to_string()),
                match matches.is_present("tags") {
                    true => Some(values_t!(matches, "tags", String).unwrap_or_else(|e| e.exit())),
                    false => None,
                },
            )
            .unwrap();
        } else if let Some(matches) = matches.subcommand_matches("end") {
            kimai::print_end_timesheet_record(
                matches.value_of("config_path").map(|p| p.to_string()),
                matches.value_of("id").unwrap().parse().unwrap(),
            )
            .unwrap();
        } else if let Some(matches) = matches.subcommand_matches("restart") {
            dbg!(matches);
            todo!("The restart subcommand still needs to be implemented?");
        } else if let Some(matches) = matches.subcommand_matches("change") {
            dbg!(matches);
            todo!("The change subcommand still needs to be implemented?");
        } else if let Some(matches) = matches.subcommand_matches("delete") {
            dbg!(matches);
            todo!("The delete subcommand still needs to be implemented?");
        } else if let Some(matches) = matches.subcommand_matches("log") {
            kimai::print_log_timesheet_record(
                matches.value_of("config_path").map(|p| p.to_string()),
                matches
                    .value_of("user")
                    .map(|u| u.parse::<usize>().unwrap()),
                matches.value_of("project").unwrap().parse().unwrap(),
                matches.value_of("activity").unwrap().parse().unwrap(),
                matches.value_of("begin").unwrap().to_string(),
                matches.value_of("end").map(|p| p.to_string()),
                matches.value_of("description").map(|d| d.to_string()),
                match matches.is_present("tags") {
                    true => Some(values_t!(matches, "tags", String).unwrap_or_else(|e| e.exit())),
                    false => None,
                },
            )
            .unwrap();
        } else {
            kimai::print_timesheet(
                matches.value_of("config_path").map(|p| p.to_string()),
                matches
                    .value_of("user")
                    .map(|u| u.parse::<usize>().unwrap()),
                match matches.is_present("customers") {
                    true => {
                        Some(values_t!(matches, "customers", usize).unwrap_or_else(|e| e.exit()))
                    }
                    false => None,
                },
                match matches.is_present("projects") {
                    true => {
                        Some(values_t!(matches, "projects", usize).unwrap_or_else(|e| e.exit()))
                    }
                    false => None,
                },
                match matches.is_present("activities") {
                    true => {
                        Some(values_t!(matches, "activities", usize).unwrap_or_else(|e| e.exit()))
                    }
                    false => None,
                },
            )
            .unwrap();
        }
    }
}
