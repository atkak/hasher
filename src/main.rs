extern crate core;
extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate colored;

mod hasher;

use clap::{App, Arg};
use colored::*;
use std::io::Result;
use std::fs::File;
use std::path::Path;

fn main() {
    env_logger::init().unwrap();

    let Args { ref file_path } = extract_args();

    info!("Given file path: {}", file_path);

    if let Err(error) = hash_for(file_path) {
        error!("Failed. error: {}", error);
        std::process::exit(error.raw_os_error().unwrap_or(1));
    }
}

struct Args {
    file_path: String,
}

fn extract_args() -> Args {
    let matches = App::new("hasher")
        .version("0.1.0")
        .about("Simple hash generator with the SipHash 2-4 algorithms.")
        .arg(Arg::with_name("path")
            .help("Generate a hash of the given file")
            .required(true)
            .index(1))
        .get_matches();

    Args {
        file_path: matches.value_of("path").unwrap().to_owned(),
    }
}

fn hash_for(file_path: &str) -> Result<()> {
    use std::time::SystemTime;
    let start_time = SystemTime::now();

    println!("{} {}", "Calculating".green(), file_path);

    let path = Path::new(file_path);
    let mut file = try!(File::open(&path));

    let ref digest = hasher::calc_hash(&mut file);

    println!("{} {}", "Done".green(), digest);

    let elapsed = start_time.elapsed().unwrap();
    println!("{}.{:09} sec elapsed", elapsed.as_secs(), elapsed.subsec_nanos());

    Ok(())
}
