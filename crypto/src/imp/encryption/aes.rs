use std::{borrow::Cow, str::FromStr};

use super::{cipher::*, error::CipherError};

pub fn aes_encrypt<'a>(
    data: &'a mut [u8],
    key: &[u8],
    iv: &[u8],
    mode: &str,
    padding: &str,
) -> Result<Cow<'a, [u8]>, CipherError> {
    let aes_type = BlockCipher::new_from_key_length(key.len())?;
    let mode = Mode::from_str(mode)?;
    let padding = Padding::from_str(padding)?;

    let cipher = Cipher::new(aes_type, mode, padding);
    let encrypted = cipher.encrypt(key, iv, data)?;

    Ok(encrypted)
}

pub fn aes_decrypt<'a>(
    data: &'a mut [u8],
    key: &[u8],
    iv: &[u8],
    mode: &str,
    padding: &str,
) -> Result<Cow<'a, [u8]>, CipherError> {
    let aes_mode = BlockCipher::new_from_key_length(key.len())?;
    let mode = Mode::from_str(mode)?;
    let padding = Padding::from_str(padding)?;

    let cipher = Cipher::new(aes_mode, mode, padding);
    let decrypted = cipher.decrypt(key, iv, data)?;

    Ok(decrypted)
}
