use std::io::{Read, BufReader};
use crypto;
use crypto::digest::Digest;

pub trait HexHasher {
    fn hex_str(&self, read: &mut Read) -> String;
}

pub enum HashAlgorithm {
    ShipHash,
    Sha1,
    MD5,
}

impl HashAlgorithm {
    pub fn of(algorithm: &str) -> HashAlgorithm {
        match algorithm {
            "shiphash" => HashAlgorithm::ShipHash,
            "sha1" => HashAlgorithm::Sha1,
            "md5" => HashAlgorithm::MD5,
            _ => panic!("Invalid algorithm name is specified.")
        }
    }
}

impl HexHasher for HashAlgorithm {
    fn hex_str(&self, read: &mut Read) -> String {
        match *self {
            HashAlgorithm::ShipHash => create_hexstr_shiphash(read),
            HashAlgorithm::Sha1 => create_hexstr_with_digest(read, &mut crypto::sha1::Sha1::new()),
            HashAlgorithm::MD5 => create_hexstr_with_digest(read, &mut crypto::md5::Md5::new()),
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

fn create_hexstr_with_digest(read: &mut Read, hasher: &mut Digest) -> String {
    let reader = BufReader::new(read);

    for byte in reader.bytes() {
        hasher.input(&[byte.unwrap()]);
    }

    hasher.result_str()
}
