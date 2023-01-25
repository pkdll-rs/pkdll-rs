use std::{borrow::Cow, str::FromStr};

use super::{cipher::*, error::CipherError};

pub fn blowfish_encrypt<'a>(
    data: &'a mut [u8],
    key: &[u8],
    iv: &[u8],
    mode: &str,
    padding: &str,
) -> Result<Cow<'a, [u8]>, CipherError> {
    let mode = Mode::from_str(mode)?;
    let padding = Padding::from_str(padding)?;

    let cipher = Cipher::new(BlockCipher::Blowfish, mode, padding);
    let encrypted = cipher.encrypt(key, iv, data)?;

    Ok(encrypted)
}

pub fn blowfish_decrypt<'a>(
    data: &'a mut [u8],
    key: &[u8],
    iv: &[u8],
    mode: &str,
    padding: &str,
) -> Result<Cow<'a, [u8]>, CipherError> {
    let mode = Mode::from_str(mode)?;
    let padding = Padding::from_str(padding)?;

    let cipher = Cipher::new(BlockCipher::Blowfish, mode, padding);
    let decrypted = cipher.decrypt(key, iv, data)?;

    Ok(decrypted)
}
