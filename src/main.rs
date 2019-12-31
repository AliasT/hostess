extern crate clap;

use clap::*;
use hostess::hostfile;

// @TODO: cli app
fn main() {
    let matches = App::new("Hostess")
        .subcommand(SubCommand::with_name("list").about("List all hosts item"))
        .subcommand(
            SubCommand::with_name("on")
                .arg(Arg::with_name("domain").index(1))
                .about("Enable a host item"),
        )
        .subcommand(
            SubCommand::with_name("off")
                .arg(Arg::with_name("domain").index(1))
                .about("Disable host item"),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .arg(Arg::with_name("domain").index(1))
                .about("Remove host item"),
        )
        .subcommand(
            SubCommand::with_name("add")
                .arg(Arg::with_name("domain").index(1))
                .arg(Arg::with_name("ip").index(2))
                .about("Add a new host item")
                .usage("Add [domain] [ip]"),
        )
        .get_matches();

    let mut hosts = hostfile::Hostfile::default();

    // match matches.subcommand_matches(name: S)
    // list all hosts
    if let Some(_) = matches.subcommand_matches("list") {
        hosts.format();
    }

    // add
    if let Some(matches) = matches.subcommand_matches("add") {
        let domain = matches.value_of("domain").unwrap_or_default();
        // 默认使用127.0.0.1
        let ip = matches.value_of("ip").unwrap_or("127.0.0.1");

        hosts.append(String::from(domain), ip).unwrap();
    }

    // enable
    if let Some(matches) = matches.subcommand_matches("on") {
        let domain = matches.value_of("domain").unwrap();
        hosts.on(String::from(domain));
    }

    // disable
    if let Some(matches) = matches.subcommand_matches("off") {
        let domain = matches.value_of("domain").unwrap();
        hosts.off(String::from(domain));
    }

    // remove
    if let Some(matches) = matches.subcommand_matches("rm") {
        let domain = matches.value_of("domain").unwrap();
        hosts.remove(String::from(domain));
    }
}
