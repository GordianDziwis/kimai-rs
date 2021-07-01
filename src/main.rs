use clap::{
    crate_authors, crate_description, crate_name, crate_version, values_t, App, Arg, SubCommand,
};

fn main() {
    let config_path_arg = Arg::with_name("config_path")
        .long("config_path")
        .help("Path to a config file")
        .takes_value(true);

    let term_arg = Arg::with_name("term")
        .long("term")
        .short("t")
        .takes_value(true)
        .help("A free search term");

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
                .arg(
                    Arg::with_name("customers")
                        .short("c")
                        .long("customers")
                        .help("Limit the returned projects to those connected with the given customer IDs")
                        .takes_value(true)
                        .multiple(true)
                ),
        )
        .subcommand(
            SubCommand::with_name("activities")
                .author(crate_authors!())
                .version(crate_version!())
                .about("Get a list of all activities")
                .arg(&config_path_arg)
                .arg(&term_arg)
                .arg(
                    Arg::with_name("projects")
                        .short("p")
                        .long("projects")
                        .help("Limit the returned activities to those applicable to the the given project IDs")
                        .takes_value(true)
                        .multiple(true)
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
}
