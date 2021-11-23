use digest::Digest;
use sha2::*;
use sha3::*;
use md5::Md5;
use md4::Md4;
use sha1::Sha1;
use ripemd160::Ripemd160;
use ripemd256::Ripemd256;
use ripemd320::Ripemd320;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum HashError {
    #[error("hash type unsupported yet: `{0}`")]
    InvalidHashType(String),
}

fn _hash<D: Digest>(data: Vec<u8>) -> Result<Vec<u8>, HashError>{
    let mut hasher = <D>::new();
    hasher.update(&data);
    Ok(hasher.finalize().to_vec())
}

pub fn hash_base64(data: Vec<u8>, hash_type: String) -> Result<String, HashError> {
    let hashed = match hash_type.as_str() {
        "md5" => _hash::<Md5>(data),
        "md4" => _hash::<Md4>(data),
        "sha1" =>_hash::<Sha1>(data),
        "sha224" =>_hash::<Sha224>(data),
        "sha256" =>_hash::<Sha256>(data),
        "sha384" =>_hash::<Sha384>(data),
        "sha512" =>_hash::<Sha512>(data),
        "sha3-224" => _hash::<Sha3_224>(data),
        "sha3-256" => _hash::<Sha3_256>(data),
        "sha3-384" => _hash::<Sha3_384>(data),
        "sha3-512" => _hash::<Sha3_512>(data),
        "keccak224" => _hash::<Keccak224>(data),
        "keccak256" => _hash::<Keccak256>(data),
        "keccak384" => _hash::<Keccak384>(data),
        "keccak512" => _hash::<Keccak512>(data),
        "ripemd160" => _hash::<Ripemd160>(data),
        "ripemd256" => _hash::<Ripemd256>(data),
        "ripemd320" => _hash::<Ripemd320>(data),
        _ => return Err(HashError::InvalidHashType(hash_type))
    }?;

    Ok(base64::encode(hashed))
}