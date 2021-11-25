use std::error::Error;

use sha2::*;
use sha3::*;
use md5::Md5;
use md4::Md4;
use sha1::Sha1;
use ripemd160::Ripemd160;
use ripemd256::Ripemd256;
use ripemd320::Ripemd320;

use hmac::{Hmac, Mac, NewMac, crypto_mac::InvalidKeyLength};

use super::hash::HashError;
use crate::switch_hmac_trait;

fn hmac<F: Mac + NewMac + Sync>(data: Vec<u8>, key: Vec<u8>) -> Result<Vec<u8>, InvalidKeyLength>{
    let mut hasher = F::new_from_slice(&key)?;
    hasher.update(&data);
    Ok(hasher.finalize().into_bytes().as_slice().to_vec())
}

pub fn hmac_base64(data: Vec<u8>, key: Vec<u8>, hash_type: String) -> Result<String, Box<dyn Error>> {
    let hashed = switch_hmac_trait!(hmac, hash_type.as_str(), , data, key)?;

    Ok(base64::encode(hashed))
}