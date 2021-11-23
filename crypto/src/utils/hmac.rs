use thiserror::Error;

use sha2::*;
use sha3::*;
use md5::Md5;
use md4::Md4;
use sha1::Sha1;
use ripemd160::Ripemd160;
use ripemd256::Ripemd256;
use ripemd320::Ripemd320;
use digest::*;

use hmac::{Hmac, Mac, NewMac, crypto_mac::InvalidKeyLength};

use super::hash::HashError;

#[derive(Error, Debug)]
pub enum HmacError {
    #[error(transparent)]
    InvalidKeyLength(#[from] InvalidKeyLength),
    #[error(transparent)]
    InvalidHashType(#[from] HashError),
}

fn hmac<D: Update + FixedOutput + Reset + BlockInput + Clone + Default>(data: Vec<u8>, key: Vec<u8>) -> Result<Vec<u8>, InvalidKeyLength>{
    let mut hasher = Hmac::<D>::new_from_slice(&key)?;
    hasher.update(&data);
    Ok(hasher.finalize().into_bytes().as_slice().to_vec())
}

pub fn hmac_base64(data: Vec<u8>, key: Vec<u8>, hash_type: String) -> Result<String, HmacError> {
    let hashed = match hash_type.as_str() {
        "md5" => hmac::<Md5>(data, key),
        "md4" => hmac::<Md4>(data, key),
        "sha1" => hmac::<Sha1>(data, key),
        "sha224" => hmac::<Sha224>(data, key),
        "sha256" => hmac::<Sha256>(data, key),
        "sha384" => hmac::<Sha384>(data, key),
        "sha512" => hmac::<Sha512>(data, key),
        "sha3-224" => hmac::<Sha3_224>(data, key),
        "sha3-256" => hmac::<Sha3_256>(data, key),
        "sha3-384" => hmac::<Sha3_384>(data, key),
        "sha3-512" => hmac::<Sha3_512>(data, key),
        "keccak224" => hmac::<Keccak224>(data, key),
        "keccak256" => hmac::<Keccak256>(data, key),
        "keccak384" => hmac::<Keccak384>(data, key),
        "keccak512" => hmac::<Keccak512>(data, key),
        "ripemd160" => hmac::<Ripemd160>(data, key),
        "ripemd256" => hmac::<Ripemd256>(data, key),
        "ripemd320" => hmac::<Ripemd320>(data, key),
        _ => return Err(HmacError::InvalidHashType(HashError::InvalidHashType(hash_type)))
    }?;

    Ok(base64::encode(hashed))
}