use std::io::{Read, BufReader};
use crypto;
use crypto::digest::Digest;

pub trait HexHasher {
    fn hex_str(&self, read: &mut Read) -> String;
}

pub enum HashAlgorithm {
    SipHash,
    Sha1,
    MD5,
}

impl HashAlgorithm {
    pub fn of(algorithm: &str) -> HashAlgorithm {
        match algorithm {
            "siphash" => HashAlgorithm::SipHash,
            "sha1" => HashAlgorithm::Sha1,
            "md5" => HashAlgorithm::MD5,
            _ => panic!("Invalid algorithm name is specified.")
        }
    }
}

impl HexHasher for HashAlgorithm {
    fn hex_str(&self, read: &mut Read) -> String {
        match *self {
            HashAlgorithm::SipHash => create_hexstr_siphash(read),
            HashAlgorithm::Sha1 => create_hexstr_with_digest(read, &mut crypto::sha1::Sha1::new()),
            HashAlgorithm::MD5 => create_hexstr_with_digest(read, &mut crypto::md5::Md5::new()),
        }
    }
}

fn create_hexstr_siphash(read: &mut Read) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn siphash() {
        let hasher = HashAlgorithm::of("siphash");
        let digest = hasher.hex_str(&mut "test".as_bytes());

        assert_eq!(digest, "3d5124c4cd58914e");
    }

    #[test]
    fn sha1() {
        let hasher = HashAlgorithm::of("sha1");
        let digest = hasher.hex_str(&mut "test".as_bytes());

        assert_eq!(digest, "a94a8fe5ccb19ba61c4c0873d391e987982fbbd3");
    }

    #[test]
    fn md5() {
        let hasher = HashAlgorithm::of("md5");
        let digest = hasher.hex_str(&mut "test".as_bytes());

        assert_eq!(digest, "098f6bcd4621d373cade4e832627b4f6");
    }
}
