use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

pub fn parse_cli<'a>() -> ArgMatches<'a> {
    App::new("rconc")
        .version(crate_version!())
        .setting(AppSettings::SubcommandsNegateReqs)
        .setting(AppSettings::VersionlessSubcommands)
        .set_term_width(80)
        .usage("rconc <server> <command>\n    \
                rconc server add <name> <address> <password>\n    \
                rconc server remove <name>\n    \
                rconc server list")
        .arg(Arg::with_name("server")
                 .required(true))
        .arg(Arg::with_name("command")
                 .required(true)
                 .multiple(true))
        .subcommand(SubCommand::with_name("server")
            .about("Manage the list of servers")
            .setting(AppSettings::SubcommandRequired)
            .setting(AppSettings::VersionlessSubcommands)
            .subcommand(SubCommand::with_name("add")
                .about("Add a new server")
                .arg(Arg::with_name("name")
                    .required(true))
                .arg(Arg::with_name("address")
                    .required(true))
                .arg(Arg::with_name("password")
                    .required(true)))
            .subcommand(SubCommand::with_name("remove")
                .about("Remove a server")
                .arg(Arg::with_name("name")
                    .required(true)))
            .subcommand(SubCommand::with_name("list")
                .about("List all configured servers")
                .arg(Arg::with_name("show-passwords")
                    .long("show-passwords")
                    .help("Include passwords in the listing"))))
        .get_matches()
}
