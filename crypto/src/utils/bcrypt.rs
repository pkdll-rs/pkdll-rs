use bcrypt::{BcryptError, hash_with_salt, Version};

pub fn make_hash(data: Vec<u8>, cost: u32, salt: Vec<u8>) -> Result<String, BcryptError> {
    let hashed = hash_with_salt(data, cost, &salt)?;
    Ok(hashed.format_for_version(Version::TwoA))
}