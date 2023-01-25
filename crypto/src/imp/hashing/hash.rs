use base64::{engine::general_purpose::STANDARD_NO_PAD as BASE64, Engine};
use digest::Digest;
use md4::Md4;
use md5::Md5;
use ripemd::{Ripemd128, Ripemd160, Ripemd256, Ripemd320};
use sha1::Sha1;
use sha2::*;
use sha3::*;

use super::error::HashError;

fn _hash<D: Digest>(data: &[u8]) -> Vec<u8> {
    let mut hasher = <D>::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

pub fn make_hash(data: impl AsRef<[u8]>, hash_type: &str) -> Result<String, HashError> {
    let data = data.as_ref();
    let hashed = match hash_type {
        "md5" => _hash::<Md5>(data),
        "md4" => _hash::<Md4>(data),
        "sha1" => _hash::<Sha1>(data),
        "sha224" => _hash::<Sha224>(data),
        "sha256" => _hash::<Sha256>(data),
        "sha384" => _hash::<Sha384>(data),
        "sha512" => _hash::<Sha512>(data),
        "sha3-224" => _hash::<Sha3_224>(data),
        "sha3-256" => _hash::<Sha3_256>(data),
        "sha3-384" => _hash::<Sha3_384>(data),
        "sha3-512" => _hash::<Sha3_512>(data),
        "keccak224" => _hash::<Keccak256>(data),
        "keccak256" => _hash::<Keccak256>(data),
        "keccak384" => _hash::<Keccak384>(data),
        "keccak512" => _hash::<Keccak512>(data),
        "ripemd128" => _hash::<Ripemd128>(data),
        "ripemd160" => _hash::<Ripemd160>(data),
        "ripemd256" => _hash::<Ripemd256>(data),
        "ripemd320" => _hash::<Ripemd320>(data),
        _ => return Err(HashError::InvalidHashType(hash_type.to_owned())),
    };
    Ok(BASE64.encode(hashed))
}
