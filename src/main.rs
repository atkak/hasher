extern crate core;
extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate colored;

use clap::{App, Arg};
use colored::*;
use std::io::{Read, BufReader, Result};
use std::fs::File;
use std::path::Path;

fn main() {
    env_logger::init().unwrap();

    let matches = App::new("hasher")
        .version("0.1.0")
        .about("Simple hash generator with the SipHash 2-4 algorithms.")
        .arg(Arg::with_name("path")
            .help("Generate a hash of the given file")
            .required(true)
            .index(1))
        .get_matches();

    let file_path = matches.value_of("path").unwrap();

    info!("Given file path: {}", file_path);

    if let Err(error) = hash_for(file_path) {
        error!("Failed. error: {}", error);
        std::process::exit(error.raw_os_error().unwrap_or(1));
    }
}

fn hash_for(file_path: &str) -> Result<()> {
    use std::time::SystemTime;
    let start_time = SystemTime::now();

    println!("{} {}", "Calculating".green(), file_path);

    let path = Path::new(file_path);
    let mut file = try!(File::open(&path));

    let ref digest = calc_hash(&mut file);

    println!("{} {}", "Done".green(), digest);

    let elapsed = start_time.elapsed().unwrap();
    println!("{}.{:09} sec elapsed", elapsed.as_secs(), elapsed.subsec_nanos());

    Ok(())
}

fn calc_hash(read: &mut Read) -> String {
    use core::hash::SipHasher;
    use core::hash::Hasher;

    let reader = BufReader::new(read);
    let mut hasher = SipHasher::new();

    for byte in reader.bytes() {
        hasher.write_u8(byte.unwrap());
    }

    format!("{:x}", hasher.finish())
}
