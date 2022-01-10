use std::error::Error;

use super::cipher::*;

pub fn aes_encrypt(
    data: Vec<u8>,
    key: Vec<u8>,
    iv: Vec<u8>,
    mode: String,
    padding: String,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let aes_type = Cipher::new_from_key_length_for_aes(key.len())?;
    let mode = Mode::new_from_str(mode)?;
    let padding = Pad::new_from_str(padding)?;

    let cipher = BlockCipher::new(aes_type, mode, padding);
    let encrypted = cipher.encrypt(key, iv, data)?;

    Ok(encrypted)
}

pub fn aes_decrypt(
    data: Vec<u8>,
    key: Vec<u8>,
    iv: Vec<u8>,
    mode: String,
    padding: String,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let aes_type = Cipher::new_from_key_length_for_aes(key.len())?;
    let mode = Mode::new_from_str(mode)?;
    let padding = Pad::new_from_str(padding)?;

    let cipher = BlockCipher::new(aes_type, mode, padding);
    let decrypted = cipher.decrypt(key, iv, data)?;

    Ok(decrypted)
}
