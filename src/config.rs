use clap::{Arg, ArgAction, ArgMatches, Command};

pub fn read_flags() -> ArgMatches {
    Command::new("rtree")
        .version("1.0")
        .arg(
            Arg::new("Level")
                .short('L')
                .long("level")
                .default_value("5")
                .value_parser(clap::value_parser!(u8)),
        )
        .arg(
            Arg::new("All")
                .short('a')
                .long("All")
                .help("if present, hidden files and directories will be showed")
                .action(ArgAction::SetTrue)
                .required(false),
        )
        .get_matches()
}
