extern crate core;
extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate colored;
extern crate crypto;

mod hasher;

use clap::{App, Arg, SubCommand};
use colored::*;
use std::io::Result;
use std::fs::File;
use std::path::Path;
use hasher::HashAlgorithms;

fn main() {
    env_logger::init().unwrap();

    let (ref file_path, ref algorithms) = match extract_args() {
        ArgsParseResult::Args { file_path, algorithms } => (file_path, algorithms),
        ArgsParseResult::Invalid => {
            error!("Invalid commands. usage: ");
            std::process::exit(1);
        },
    };

    info!("Given file path: {}", file_path);

    if let Err(error) = hash_for(file_path, algorithms) {
        error!("Failed. error: {}", error);
        std::process::exit(error.raw_os_error().unwrap_or(1));
    }
}

enum ArgsParseResult {
    Args { file_path: String, algorithms: String },
    Invalid,
}

fn extract_args() -> ArgsParseResult {
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

    if let (algorithms, Some(sub_matches)) = matches.subcommand() {
        ArgsParseResult::Args {
            file_path: sub_matches.value_of("path").unwrap().to_owned(),
            algorithms: algorithms.to_owned(),
        }
    } else {
        ArgsParseResult::Invalid
    }
}

fn hash_for(file_path: &str, algorithms: &str) -> Result<()> {
    use std::time::SystemTime;
    let start_time = SystemTime::now();

    println!("{} {}", "Calculating".green(), file_path);

    let path = Path::new(file_path);
    let mut file = try!(File::open(&path));

    use hasher::HexHasher;
    let hasher = match algorithms {
        "shiphash" => HashAlgorithms::ShipHash,
        "sha1" => HashAlgorithms::Sha1,
        _ => HashAlgorithms::ShipHash,
    };
    let ref digest = hasher.hex_str(&mut file);

    println!("{} {}", "Done".green(), digest);

    let elapsed = start_time.elapsed().unwrap();
    println!("{}.{:09} sec elapsed", elapsed.as_secs(), elapsed.subsec_nanos());

    Ok(())
}
