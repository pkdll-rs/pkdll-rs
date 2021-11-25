use std::error::Error;

use super::cipher::*;

pub fn blowfish_encrypt(data: Vec<u8>, key: Vec<u8>, iv: Vec<u8>, mode: String, padding: String) -> Result<String, Box<dyn Error>> {
    let cipher_type = Cipher::Blowfish;
    let mode = Mode::new_from_str(mode)?;
    let padding = Pad::new_from_str(padding)?;

    let cipher = BlockCipher::new(cipher_type, mode, padding);
    let encrypted = cipher.encrypt(key, iv, data)?;

    Ok(base64::encode(encrypted))
}

pub fn blowfish_decrypt(data: Vec<u8>, key: Vec<u8>, iv: Vec<u8>, mode: String, padding: String) -> Result<Vec<u8>, Box<dyn Error>> {
    let cipher_type = Cipher::Blowfish;
    let mode = Mode::new_from_str(mode)?;
    let padding = Pad::new_from_str(padding)?;

    let cipher = BlockCipher::new(cipher_type, mode, padding);
    let decrypted = cipher.decrypt(key, iv, data)?;

    Ok(decrypted)
}