use std::io;
use std::fs::File;
use std::path::Path;
use std::time::SystemTime;
use colored::*;
use super::core::HashAlgorithm;

pub fn run(file_path: &str, algorithm: &str) -> io::Result<()> {
    let start_time = SystemTime::now();

    println!("{} {}", "Calculating".green().bold(), file_path);

    let path = Path::new(file_path);
    let mut file = try!(File::open(&path));

    use super::core::HexHasher;
    let hasher = HashAlgorithm::of(algorithm);
    let ref digest = hasher.hex_str(&mut file);

    println!("{} {}", "Done".green().bold(), digest);

    let elapsed = start_time.elapsed().unwrap();
    println!("{}: {}.{:09} sec", "Elapsed time".bold(), elapsed.as_secs(), elapsed.subsec_nanos());

    Ok(())
}
