use bcrypt::{BcryptError, hash_with_salt, Version};
use scrypt::{Scrypt, password_hash::{PasswordHasher, Salt}};

use std::error::Error;

pub fn bcrypt(data: Vec<u8>, cost: u32, salt: Vec<u8>) -> Result<String, BcryptError> {
    let hashed = hash_with_salt(data, cost, &salt)?;
    Ok(hashed.format_for_version(Version::TwoA))
}

pub fn scrypt(data: Vec<u8>, log_n: u8, r: u32, p: u32, len: usize, salt: &str) -> Result<String, Box<dyn Error>> {
    let params = scrypt::Params::new(log_n, r, p, len)?;
    Ok(Scrypt.hash_password_customized(
        &data,
        None,
        None,
        params,
        Salt::new(&salt)?
    )?
    .to_string())
}