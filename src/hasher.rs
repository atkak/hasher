use std::io::{Read, BufReader};

pub trait HexHasher {
    fn hex_str(&self, read: &mut Read) -> String;
}

pub enum HashAlgorithms {
    ShipHash,
    Sha1,
}

impl HexHasher for HashAlgorithms {
    fn hex_str(&self, read: &mut Read) -> String {
        match *self {
            HashAlgorithms::ShipHash => create_hexstr_shiphash(read),
            HashAlgorithms::Sha1 => create_hexstr_sha1(read),
        }
    }
}

impl HashAlgorithms {
    pub fn of(algorithm: &str) -> HashAlgorithms {
        match algorithm {
            "shiphash" => HashAlgorithms::ShipHash,
            "sha1" => HashAlgorithms::Sha1,
            _ => panic!("Invalid algorithm name is specified.")
        }
    }
}

fn create_hexstr_shiphash(read: &mut Read) -> String {
    use core::hash::SipHasher;
    use core::hash::Hasher;

    let reader = BufReader::new(read);
    let mut hasher = SipHasher::new();

    for byte in reader.bytes() {
        hasher.write_u8(byte.unwrap());
    }

    format!("{:x}", hasher.finish())
}

fn create_hexstr_sha1(read: &mut Read) -> String {
    use crypto::sha1::Sha1;
    use crypto::digest::Digest;

    let reader = BufReader::new(read);
    let mut hasher = Sha1::new();

    for byte in reader.bytes() {
        hasher.input(&[byte.unwrap()]);
    }

    hasher.result_str()
}
