use bcrypt::{hash_with_salt, Version};
use pbkdf2::pbkdf2 as _pbkdf2;
use scrypt as _scrypt;

use super::error::KdfError;
use crate::{call_with_hash_generic, imp::hashing::error::HashError, base64};

pub fn bcrypt(data: Vec<u8>, cost: u32, salt: [u8; 16]) -> Result<String, KdfError> {
    let hashed = hash_with_salt(data, cost, salt)?;
    Ok(hashed.format_for_version(Version::TwoA))
}

pub fn scrypt(
    data: &[u8],
    log_n: u8,
    r: u32,
    p: u32,
    len: usize,
    salt: &[u8],
) -> Result<String, KdfError> {
    let params = _scrypt::Params::new(log_n, r, p)?;
    let mut output = vec![0u8; len];
    _scrypt::scrypt(data.as_ref(), salt.as_ref(), &params, &mut output)?;

    Ok(base64::encode(output))
}

pub fn pbkdf2(
    data: &[u8],
    salt: &[u8],
    rounds: u32,
    key_length: usize,
    hash_type: &str,
) -> Result<Vec<u8>, HashError> {
    let data = data.as_ref();
    let salt = salt.as_ref();
    let mut output = vec![0u8; key_length];
    call_with_hash_generic!(
        _pbkdf2(data, salt, rounds, &mut output),
        hmac(hash_type),
        HashError::InvalidHashType
    );
    Ok(output)
}

pub fn evpkdf(
    data: &[u8],
    salt: &[u8],
    rounds: usize,
    output_length: usize,
    hash_type: &str,
) -> Result<Vec<u8>, HashError> {
    let mut output = vec![0u8; output_length];
    call_with_hash_generic!(
        evpkdf::evpkdf(data, salt, rounds, &mut output),
        hash_type,
        HashError::InvalidHashType
    );
    Ok(output)
}
