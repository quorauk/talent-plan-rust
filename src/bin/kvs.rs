extern crate clap;
use clap::{App, Arg, SubCommand};

fn unimplemented() {
    eprintln!("unimplemented");
    std::process::exit(1);
}

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(SubCommand::with_name("get").arg(Arg::with_name("key")))
        .subcommand(
            SubCommand::with_name("set")
                .arg(Arg::with_name("key"))
                .arg(Arg::with_name("value")),
        )
        .subcommand(SubCommand::with_name("rm").arg(Arg::with_name("key")))
        .get_matches();
    match matches.subcommand() {
        ("get", Some(_sub_m)) => unimplemented(),
        ("set", Some(_sub_m)) => unimplemented(),
        ("rm", Some(_sub_m)) => unimplemented(),
        (&_, _) => {
            println!("{}", matches.usage());
        }
    }
    std::process::exit(1);
}
