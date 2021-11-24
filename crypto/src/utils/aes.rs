use std::error::Error;

use aes::{Aes128, Aes192, Aes256};
use block_modes::{BlockMode, Ecb, Cbc, block_padding::*};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AesError {
    #[error("padding unsupported yet: `{0}`")]
    InvalidPadding(String),
    #[error("mode unsupported yet: `{0}`")]
    InvalidMode(String),
    #[error("unsupported key len: `{0}`. Only 16, 24, 32 bytes")]
    InvalidKeyLen(usize),
}

pub enum Pad {
    AnsiX923,
    Iso7816,
    NoPadding,
    Pkcs7,
    ZeroPadding,
}

impl Pad {
    pub fn new_from_str(padding: String) -> Result<Pad, AesError> {
        let padding = match padding.as_str() {
            "pkcs7" => Pad::Pkcs7,
            "zero" => Pad::ZeroPadding,
            "iso7816" => Pad::Iso7816,
            "ansi_x923" => Pad::AnsiX923,
            _ => return Err(AesError::InvalidPadding(padding))
        };
        Ok(padding)
    }
}

pub enum Mode {
    Ecb,
    Cbc,
}

impl Mode {
    pub fn new_from_str(mode: String) -> Result<Mode, AesError> {
        let mode = match mode.as_str() {
            "ecb" => Mode::Ecb,
            "cbc" => Mode::Cbc,
            _ => return Err(AesError::InvalidMode(mode))
        };
        Ok(mode)
    }
}

pub enum Len {
    Aes128,
    Aes192,
    Aes256,
}

impl Len {
    pub fn new_from_key_length(key_length: usize) -> Result<Len, AesError> {
        let aes_type = match key_length {
            16 => Len::Aes128,
            24 => Len::Aes192,
            32 => Len::Aes256,
            _ => return Err(AesError::InvalidKeyLen(key_length))
        };
        Ok(aes_type)
    }
}

pub fn aes_encrypt(data: Vec<u8>, key: Vec<u8>, iv: Vec<u8>, mode: String, padding: String) -> Result<String, Box<dyn Error>> {
    let aes_type = Len::new_from_key_length(key.len())?;
    let mode = Mode::new_from_str(mode)?;
    let padding = Pad::new_from_str(padding)?;

    let cipher = Aes::new(aes_type, mode, padding);
    let encrypted = cipher.encrypt(key, iv, data)?;

    Ok(base64::encode(encrypted))
}

pub fn aes_decrypt(data: Vec<u8>, key: Vec<u8>, iv: Vec<u8>, mode: String, padding: String) -> Result<Vec<u8>, Box<dyn Error>> {
    let aes_type = Len::new_from_key_length(key.len())?;
    let mode = Mode::new_from_str(mode)?;
    let padding = Pad::new_from_str(padding)?;

    let cipher = Aes::new(aes_type, mode, padding);
    let decrypted = cipher.decrypt(key, iv, data)?;

    Ok(decrypted)
}

pub struct Aes {
    len: Len,
    mode: Mode,
    pad: Pad,
}

impl Aes {
    pub fn new(len: Len, mode: Mode, pad: Pad) -> Self {
        Self { len, mode, pad }
    }
    pub fn encrypt(
        &self,
        key: Vec<u8>,
        iv: Vec<u8>,
        plaintext: Vec<u8>,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        match (&self.len, &self.mode, &self.pad) {
            (Len::Aes128, Mode::Cbc, Pad::Pkcs7) => {
                Ok(Cbc::<Aes128, Pkcs7>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Len::Aes128, Mode::Cbc, Pad::ZeroPadding) => {
                Ok(Cbc::<Aes128, ZeroPadding>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Len::Aes128, Mode::Cbc, Pad::NoPadding) => {
                Ok(Cbc::<Aes128, NoPadding>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Len::Aes128, Mode::Cbc, Pad::Iso7816) => {
                Ok(Cbc::<Aes128, Iso7816>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Len::Aes128, Mode::Cbc, Pad::AnsiX923) => {
                Ok(Cbc::<Aes128, AnsiX923>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Len::Aes128, Mode::Ecb, Pad::Pkcs7) => {
                Ok(Ecb::<Aes128, Pkcs7>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Len::Aes128, Mode::Ecb, Pad::ZeroPadding) => {
                Ok(Ecb::<Aes128, ZeroPadding>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Len::Aes128, Mode::Ecb, Pad::NoPadding) => {
                Ok(Ecb::<Aes128, NoPadding>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Len::Aes128, Mode::Ecb, Pad::Iso7816) => {
                Ok(Ecb::<Aes128, Iso7816>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Len::Aes128, Mode::Ecb, Pad::AnsiX923) => {
                Ok(Ecb::<Aes128, AnsiX923>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Len::Aes192, Mode::Cbc, Pad::Pkcs7) => {
                Ok(Cbc::<Aes192, Pkcs7>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Len::Aes192, Mode::Cbc, Pad::ZeroPadding) => {
                Ok(Cbc::<Aes192, ZeroPadding>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Len::Aes192, Mode::Cbc, Pad::NoPadding) => {
                Ok(Cbc::<Aes192, NoPadding>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Len::Aes192, Mode::Cbc, Pad::Iso7816) => {
                Ok(Cbc::<Aes192, Iso7816>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Len::Aes192, Mode::Cbc, Pad::AnsiX923) => {
                Ok(Cbc::<Aes192, AnsiX923>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Len::Aes192, Mode::Ecb, Pad::Pkcs7) => {
                Ok(Ecb::<Aes192, Pkcs7>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Len::Aes192, Mode::Ecb, Pad::ZeroPadding) => {
                Ok(Ecb::<Aes192, ZeroPadding>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Len::Aes192, Mode::Ecb, Pad::NoPadding) => {
                Ok(Ecb::<Aes192, NoPadding>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Len::Aes192, Mode::Ecb, Pad::Iso7816) => {
                Ok(Ecb::<Aes192, Iso7816>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Len::Aes192, Mode::Ecb, Pad::AnsiX923) => {
                Ok(Ecb::<Aes192, AnsiX923>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Len::Aes256, Mode::Cbc, Pad::Pkcs7) => {
                Ok(Cbc::<Aes256, Pkcs7>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Len::Aes256, Mode::Cbc, Pad::ZeroPadding) => {
                Ok(Cbc::<Aes256, ZeroPadding>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Len::Aes256, Mode::Cbc, Pad::NoPadding) => {
                Ok(Cbc::<Aes256, NoPadding>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Len::Aes256, Mode::Cbc, Pad::Iso7816) => {
                Ok(Cbc::<Aes256, Iso7816>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Len::Aes256, Mode::Cbc, Pad::AnsiX923) => {
                Ok(Cbc::<Aes256, AnsiX923>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Len::Aes256, Mode::Ecb, Pad::Pkcs7) => {
                Ok(Ecb::<Aes256, Pkcs7>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Len::Aes256, Mode::Ecb, Pad::ZeroPadding) => {
                Ok(Ecb::<Aes256, ZeroPadding>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Len::Aes256, Mode::Ecb, Pad::NoPadding) => {
                Ok(Ecb::<Aes256, NoPadding>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Len::Aes256, Mode::Ecb, Pad::Iso7816) => {
                Ok(Ecb::<Aes256, Iso7816>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Len::Aes256, Mode::Ecb, Pad::AnsiX923) => {
                Ok(Ecb::<Aes256, AnsiX923>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
        }
    }
    pub fn decrypt(
        &self,
        key: Vec<u8>,
        iv: Vec<u8>,
        ciphertext: Vec<u8>,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        match (&self.len, &self.mode, &self.pad) {
            (Len::Aes128, Mode::Cbc, Pad::Pkcs7) => {
                Ok(Cbc::<Aes128, Pkcs7>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Len::Aes128, Mode::Cbc, Pad::ZeroPadding) => {
                Ok(Cbc::<Aes128, ZeroPadding>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Len::Aes128, Mode::Cbc, Pad::NoPadding) => {
                Ok(Cbc::<Aes128, NoPadding>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Len::Aes128, Mode::Cbc, Pad::Iso7816) => {
                Ok(Cbc::<Aes128, Iso7816>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Len::Aes128, Mode::Cbc, Pad::AnsiX923) => {
                Ok(Cbc::<Aes128, AnsiX923>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Len::Aes128, Mode::Ecb, Pad::Pkcs7) => {
                Ok(Ecb::<Aes128, Pkcs7>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Len::Aes128, Mode::Ecb, Pad::ZeroPadding) => {
                Ok(Ecb::<Aes128, ZeroPadding>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Len::Aes128, Mode::Ecb, Pad::NoPadding) => {
                Ok(Ecb::<Aes128, NoPadding>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Len::Aes128, Mode::Ecb, Pad::Iso7816) => {
                Ok(Ecb::<Aes128, Iso7816>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Len::Aes128, Mode::Ecb, Pad::AnsiX923) => {
                Ok(Ecb::<Aes128, AnsiX923>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Len::Aes192, Mode::Ecb, Pad::Pkcs7) => {
                Ok(Ecb::<Aes192, Pkcs7>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Len::Aes192, Mode::Ecb, Pad::ZeroPadding) => {
                Ok(Ecb::<Aes192, ZeroPadding>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Len::Aes192, Mode::Ecb, Pad::NoPadding) => {
                Ok(Ecb::<Aes192, NoPadding>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Len::Aes192, Mode::Ecb, Pad::Iso7816) => {
                Ok(Ecb::<Aes192, Iso7816>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Len::Aes192, Mode::Ecb, Pad::AnsiX923) => {
                Ok(Ecb::<Aes192, AnsiX923>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Len::Aes192, Mode::Cbc, Pad::Pkcs7) => {
                Ok(Cbc::<Aes192, Pkcs7>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Len::Aes192, Mode::Cbc, Pad::ZeroPadding) => {
                Ok(Cbc::<Aes192, ZeroPadding>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Len::Aes192, Mode::Cbc, Pad::NoPadding) => {
                Ok(Cbc::<Aes192, NoPadding>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Len::Aes192, Mode::Cbc, Pad::Iso7816) => {
                Ok(Cbc::<Aes192, Iso7816>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Len::Aes192, Mode::Cbc, Pad::AnsiX923) => {
                Ok(Cbc::<Aes192, AnsiX923>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Len::Aes256, Mode::Cbc, Pad::Pkcs7) => {
                Ok(Cbc::<Aes256, Pkcs7>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Len::Aes256, Mode::Cbc, Pad::ZeroPadding) => {
                Ok(Cbc::<Aes256, ZeroPadding>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Len::Aes256, Mode::Cbc, Pad::NoPadding) => {
                Ok(Cbc::<Aes256, NoPadding>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Len::Aes256, Mode::Cbc, Pad::Iso7816) => {
                Ok(Cbc::<Aes256, Iso7816>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Len::Aes256, Mode::Cbc, Pad::AnsiX923) => {
                Ok(Cbc::<Aes256, AnsiX923>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Len::Aes256, Mode::Ecb, Pad::Pkcs7) => {
                Ok(Ecb::<Aes256, Pkcs7>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Len::Aes256, Mode::Ecb, Pad::ZeroPadding) => {
                Ok(Ecb::<Aes256, ZeroPadding>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Len::Aes256, Mode::Ecb, Pad::NoPadding) => {
                Ok(Ecb::<Aes256, NoPadding>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Len::Aes256, Mode::Ecb, Pad::Iso7816) => {
                Ok(Ecb::<Aes256, Iso7816>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Len::Aes256, Mode::Ecb, Pad::AnsiX923) => {
                Ok(Ecb::<Aes256, AnsiX923>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
        }
    }
}