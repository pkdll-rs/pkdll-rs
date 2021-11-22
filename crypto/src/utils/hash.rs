use digest::{Digest, DynDigest};
use sha2::*;
use sha3::*;
use md5::Md5;
use md4::Md4;
use sha1::Sha1;
use ripemd160::Ripemd160;
use ripemd256::Ripemd256;
use ripemd320::Ripemd320;

pub fn hash_base64(data: Vec<u8>, hash_type: &str) -> Option<String> {
    let mut hasher: Box<dyn DynDigest> = match hash_type {
        "md5" => Box::new(Md5::new()),
        "md4" => Box::new(Md4::new()),
        "sha1" => Box::new(Sha1::new()),
        "sha224" => Box::new(Sha224::new()),
        "sha256" => Box::new(Sha256::new()),
        "sha384" => Box::new(Sha384::new()),
        "sha512" => Box::new(Sha512::new()),
        "sha3-224" => Box::new(Sha3_224::new()),
        "sha3-256" => Box::new(Sha3_256::new()),
        "sha3-384" => Box::new(Sha3_384::new()),
        "sha3-512" => Box::new(Sha3_512::new()),
        "keccak224" => Box::new(Keccak224::new()),
        "keccak256" => Box::new(Keccak256::new()),
        "keccak384" => Box::new(Keccak384::new()),
        "keccak512" => Box::new(Keccak512::new()),
        "ripemd160" => Box::new(Ripemd160::new()),
        "ripemd256" => Box::new(Ripemd256::new()),
        "ripemd320" => Box::new(Ripemd320::new()),
        _ => return None
    };

    hasher.update(&data);
    Some(base64::encode(hasher.finalize()))
}