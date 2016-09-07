use std::io::{Read, BufReader};

pub trait HexHasher {
    fn hex_str(&self, read: &mut Read) -> String;
}

pub enum HashAlgorithm {
    ShipHash,
    Sha1,
}

impl HashAlgorithm {
    pub fn of(algorithm: &str) -> HashAlgorithm {
        match algorithm {
            "shiphash" => HashAlgorithm::ShipHash,
            "sha1" => HashAlgorithm::Sha1,
            _ => panic!("Invalid algorithm name is specified.")
        }
    }
}

impl HexHasher for HashAlgorithm {
    fn hex_str(&self, read: &mut Read) -> String {
        match *self {
            HashAlgorithm::ShipHash => create_hexstr_shiphash(read),
            HashAlgorithm::Sha1 => create_hexstr_sha1(read),
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
