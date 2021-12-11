use std::error::Error;

use bcrypt::{BcryptError, hash_with_salt, Version};
use hmac::Hmac;
use scrypt as _scrypt;
use pbkdf2::pbkdf2 as _pbkdf2;

use super::hash::*;
use crate::switch_hmac_trait;

pub fn bcrypt(data: Vec<u8>, cost: u32, salt: Vec<u8>) -> Result<String, BcryptError> {
    let hashed = hash_with_salt(data, cost, &salt)?;
    Ok(hashed.format_for_version(Version::TwoA))
}

pub fn scrypt(data: Vec<u8>, log_n: u8, r: u32, p: u32, len: usize, salt: Vec<u8>) -> Result<String, Box<dyn Error>> {
    let params = _scrypt::Params::new(log_n, r, p)?;
    let mut output = vec![0u8; len];
    _scrypt::scrypt(&data, &salt, &params, &mut output)?;

    Ok(
        base64::encode_config(output, base64::STANDARD),
    )
}

pub fn pbkdf2(data: Vec<u8>, salt: Vec<u8>, rounds: u32, key_length: usize, hash_type: String) -> Result<String, HashError> {
    let mut output = vec![0u8; key_length];
    switch_hmac_trait!(_pbkdf2, hash_type.as_str(), , &data, &salt, rounds, &mut output);

    Ok(
        base64::encode_config(output, base64::STANDARD),
    )
}