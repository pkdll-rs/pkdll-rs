use base64::{engine::general_purpose::STANDARD_NO_PAD as BASE64, Engine};
use hmac::Hmac;

use bcrypt::{hash_with_salt, Version};
use pbkdf2::pbkdf2 as _pbkdf2;
use scrypt as _scrypt;

use md4::Md4;
use md5::Md5;
use ripemd::*;
use sha1::Sha1;
use sha2::*;
use sha3::*;

use super::error::KdfError;
use crate::imp::hashing::error::HashError;

pub fn bcrypt(data: Vec<u8>, cost: u32, salt: [u8; 16]) -> Result<String, KdfError> {
    let hashed = hash_with_salt(data, cost, salt)?;
    Ok(hashed.format_for_version(Version::TwoA))
}

pub fn scrypt(
    data: impl AsRef<[u8]>,
    log_n: u8,
    r: u32,
    p: u32,
    len: usize,
    salt: impl AsRef<[u8]>,
) -> Result<String, KdfError> {
    let params = _scrypt::Params::new(log_n, r, p)?;
    let mut output = vec![0u8; len];
    _scrypt::scrypt(data.as_ref(), salt.as_ref(), &params, &mut output)?;

    Ok(BASE64.encode(output))
}

pub fn pbkdf2(
    data: impl AsRef<[u8]>,
    salt: impl AsRef<[u8]>,
    rounds: u32,
    key_length: usize,
    hash_type: &str,
) -> Result<String, HashError> {
    let data = data.as_ref();
    let salt = salt.as_ref();
    let mut output = vec![0u8; key_length];
    match hash_type {
        "md5" => _pbkdf2::<Hmac<Md5>>(data, salt, rounds, &mut output),
        "md4" => _pbkdf2::<Hmac<Md4>>(data, salt, rounds, &mut output),
        "sha1" => _pbkdf2::<Hmac<Sha1>>(data, salt, rounds, &mut output),
        "sha224" => _pbkdf2::<Hmac<Sha224>>(data, salt, rounds, &mut output),
        "sha256" => _pbkdf2::<Hmac<Sha256>>(data, salt, rounds, &mut output),
        "sha384" => _pbkdf2::<Hmac<Sha384>>(data, salt, rounds, &mut output),
        "sha512" => _pbkdf2::<Hmac<Sha512>>(data, salt, rounds, &mut output),
        "sha3-224" => _pbkdf2::<Hmac<Sha3_224>>(data, salt, rounds, &mut output),
        "sha3-256" => _pbkdf2::<Hmac<Sha3_256>>(data, salt, rounds, &mut output),
        "sha3-384" => _pbkdf2::<Hmac<Sha3_384>>(data, salt, rounds, &mut output),
        "sha3-512" => _pbkdf2::<Hmac<Sha3_512>>(data, salt, rounds, &mut output),
        "keccak224" => _pbkdf2::<Hmac<Keccak256>>(data, salt, rounds, &mut output),
        "keccak256" => _pbkdf2::<Hmac<Keccak256>>(data, salt, rounds, &mut output),
        "keccak384" => _pbkdf2::<Hmac<Keccak384>>(data, salt, rounds, &mut output),
        "keccak512" => _pbkdf2::<Hmac<Keccak512>>(data, salt, rounds, &mut output),
        "ripemd128" => _pbkdf2::<Hmac<Ripemd128>>(data, salt, rounds, &mut output),
        "ripemd160" => _pbkdf2::<Hmac<Ripemd160>>(data, salt, rounds, &mut output),
        "ripemd256" => _pbkdf2::<Hmac<Ripemd256>>(data, salt, rounds, &mut output),
        "ripemd320" => _pbkdf2::<Hmac<Ripemd320>>(data, salt, rounds, &mut output),
        _ => return Err(HashError::InvalidHashType(hash_type.to_owned())),
    };

    Ok(BASE64.encode(output))
}
