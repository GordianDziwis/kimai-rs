use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand};

fn main() {
    let config_path_arg = Arg::with_name("config_path")
        .short("c")
        .long("config_path")
        .help("Path to a config file")
        .takes_value(true);

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("customers")
                .author(crate_authors!())
                .version(crate_version!())
                .about("Get a list of all customers")
                .arg(&config_path_arg),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("customers") {
        kimai::print_customers(matches.value_of("config_path").map(|p| p.to_string())).unwrap();
    }
}
