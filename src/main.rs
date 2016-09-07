extern crate core;
extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate colored;
extern crate crypto;

mod hasher;

use clap::{App, Arg, SubCommand};
use std::result::Result;

fn main() {
    env_logger::init().unwrap();

    let Args { ref file_path, ref algorithm } = match extract_args() {
        Ok(args) => args,
        Err(ref message) => {
            error!("{}", message);
            std::process::exit(1);
        },
    };

    info!("Given file path: {}", file_path);

    if let Err(error) = hasher::runner::run(file_path, algorithm) {
        error!("Failed. error: {}", error);
        std::process::exit(error.raw_os_error().unwrap_or(1));
    };
}

struct Args {
    file_path: String,
    algorithm: String,
}

fn extract_args() -> Result<Args, String> {
    let file_path_arg =
        Arg::with_name("path")
            .help("Generate a hash of the given file")
            .required(true)
            .index(1);
    let matches = App::new("hasher")
        .version("0.1.0")
        .about("Simple hash generator.")
        .subcommand(SubCommand::with_name("shiphash")
            .about("using ShipHash 2-4 algorithms")
            .arg(&file_path_arg))
        .subcommand(SubCommand::with_name("sha1")
            .about("using SHA1 algorithms")
            .arg(&file_path_arg))
        .get_matches();

    match matches.subcommand() {
        (algorithm, Some(sub_matches)) =>
            Ok(Args {
                file_path: sub_matches.value_of("path").unwrap().to_owned(),
                algorithm: algorithm.to_owned(),
            }),
        _ => Err(matches.usage().to_owned()),
    }
}
