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

#[derive(Error, Debug)]
pub enum HmacError {
    #[error(transparent)]
    InvalidKeyLength(#[from] InvalidKeyLength),
    #[error("hash type unsupported yet: `{0}`")]
    InvalidHashType(String),
}

fn _hmac<D: Update + FixedOutput + Reset + BlockInput + Clone + Default>(data: Vec<u8>, key: Vec<u8>) -> Result<Vec<u8>, InvalidKeyLength>{
    let mut hasher = Hmac::<D>::new_from_slice(&key)?;
    hasher.update(&data);
    Ok(hasher.finalize().into_bytes().as_slice().to_vec())
}

pub fn hmac_base64(data: Vec<u8>, key: Vec<u8>, hash_type: String) -> Result<String, HmacError> {
    let hashed = match hash_type.as_str() {
        "md5" => _hmac::<Md5>(data, key),
        "md4" => _hmac::<Md4>(data, key),
        "sha1" =>_hmac::<Sha1>(data, key),
        "sha224" =>_hmac::<Sha224>(data, key),
        "sha256" =>_hmac::<Sha256>(data, key),
        "sha384" =>_hmac::<Sha384>(data, key),
        "sha512" =>_hmac::<Sha512>(data, key),
        "sha3-224" => _hmac::<Sha3_224>(data, key),
        "sha3-256" => _hmac::<Sha3_256>(data, key),
        "sha3-384" => _hmac::<Sha3_384>(data, key),
        "sha3-512" => _hmac::<Sha3_512>(data, key),
        "keccak224" => _hmac::<Keccak224>(data, key),
        "keccak256" => _hmac::<Keccak256>(data, key),
        "keccak384" => _hmac::<Keccak384>(data, key),
        "keccak512" => _hmac::<Keccak512>(data, key),
        "ripemd160" => _hmac::<Ripemd160>(data, key),
        "ripemd256" => _hmac::<Ripemd256>(data, key),
        "ripemd320" => _hmac::<Ripemd320>(data, key),
        _ => return Err(HmacError::InvalidHashType(hash_type))
    }?;

    Ok(base64::encode(hashed))
}