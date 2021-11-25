use std::error::Error;

use aes::{Aes128, Aes192, Aes256};
use block_modes::{BlockMode, Cbc, Ecb, block_padding::*};

use blowfish::Blowfish;
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

pub enum Cipher {
    Aes128,
    Aes192,
    Aes256,
    Blowfish,
}

impl Cipher {
    pub fn new_from_key_length_for_aes(key_length: usize) -> Result<Cipher, AesError> {
        let aes_type = match key_length {
            16 => Cipher::Aes128,
            24 => Cipher::Aes192,
            32 => Cipher::Aes256,
            _ => return Err(AesError::InvalidKeyLen(key_length))
        };
        Ok(aes_type)
    }
}

pub struct BlockCipher {
    cipher: Cipher,
    mode: Mode,
    pad: Pad,
}

impl BlockCipher {
    pub fn new(cipher: Cipher, mode: Mode, pad: Pad) -> Self {
        Self { cipher, mode, pad }
    }
    pub fn encrypt(
        &self,
        key: Vec<u8>,
        iv: Vec<u8>,
        plaintext: Vec<u8>,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        match (&self.cipher, &self.mode, &self.pad) {
            // Aes
            (Cipher::Aes128, Mode::Cbc, Pad::Pkcs7) => {
                Ok(Cbc::<Aes128, Pkcs7>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Aes128, Mode::Cbc, Pad::ZeroPadding) => {
                Ok(Cbc::<Aes128, ZeroPadding>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Aes128, Mode::Cbc, Pad::NoPadding) => {
                Ok(Cbc::<Aes128, NoPadding>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Aes128, Mode::Cbc, Pad::Iso7816) => {
                Ok(Cbc::<Aes128, Iso7816>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Aes128, Mode::Cbc, Pad::AnsiX923) => {
                Ok(Cbc::<Aes128, AnsiX923>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Aes128, Mode::Ecb, Pad::Pkcs7) => {
                Ok(Ecb::<Aes128, Pkcs7>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Aes128, Mode::Ecb, Pad::ZeroPadding) => {
                Ok(Ecb::<Aes128, ZeroPadding>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Aes128, Mode::Ecb, Pad::NoPadding) => {
                Ok(Ecb::<Aes128, NoPadding>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Aes128, Mode::Ecb, Pad::Iso7816) => {
                Ok(Ecb::<Aes128, Iso7816>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Aes128, Mode::Ecb, Pad::AnsiX923) => {
                Ok(Ecb::<Aes128, AnsiX923>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Aes192, Mode::Cbc, Pad::Pkcs7) => {
                Ok(Cbc::<Aes192, Pkcs7>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Aes192, Mode::Cbc, Pad::ZeroPadding) => {
                Ok(Cbc::<Aes192, ZeroPadding>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Aes192, Mode::Cbc, Pad::NoPadding) => {
                Ok(Cbc::<Aes192, NoPadding>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Aes192, Mode::Cbc, Pad::Iso7816) => {
                Ok(Cbc::<Aes192, Iso7816>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Aes192, Mode::Cbc, Pad::AnsiX923) => {
                Ok(Cbc::<Aes192, AnsiX923>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Aes192, Mode::Ecb, Pad::Pkcs7) => {
                Ok(Ecb::<Aes192, Pkcs7>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Aes192, Mode::Ecb, Pad::ZeroPadding) => {
                Ok(Ecb::<Aes192, ZeroPadding>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Aes192, Mode::Ecb, Pad::NoPadding) => {
                Ok(Ecb::<Aes192, NoPadding>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Aes192, Mode::Ecb, Pad::Iso7816) => {
                Ok(Ecb::<Aes192, Iso7816>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Aes192, Mode::Ecb, Pad::AnsiX923) => {
                Ok(Ecb::<Aes192, AnsiX923>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Aes256, Mode::Cbc, Pad::Pkcs7) => {
                Ok(Cbc::<Aes256, Pkcs7>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Aes256, Mode::Cbc, Pad::ZeroPadding) => {
                Ok(Cbc::<Aes256, ZeroPadding>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Aes256, Mode::Cbc, Pad::NoPadding) => {
                Ok(Cbc::<Aes256, NoPadding>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Aes256, Mode::Cbc, Pad::Iso7816) => {
                Ok(Cbc::<Aes256, Iso7816>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Aes256, Mode::Cbc, Pad::AnsiX923) => {
                Ok(Cbc::<Aes256, AnsiX923>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Aes256, Mode::Ecb, Pad::Pkcs7) => {
                Ok(Ecb::<Aes256, Pkcs7>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Aes256, Mode::Ecb, Pad::ZeroPadding) => {
                Ok(Ecb::<Aes256, ZeroPadding>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Aes256, Mode::Ecb, Pad::NoPadding) => {
                Ok(Ecb::<Aes256, NoPadding>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Aes256, Mode::Ecb, Pad::Iso7816) => {
                Ok(Ecb::<Aes256, Iso7816>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Aes256, Mode::Ecb, Pad::AnsiX923) => {
                Ok(Ecb::<Aes256, AnsiX923>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }

            // Blowfish
            (Cipher::Blowfish, Mode::Cbc, Pad::Pkcs7) => {
                Ok(Cbc::<Blowfish, Pkcs7>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Blowfish, Mode::Cbc, Pad::ZeroPadding) => {
                Ok(Cbc::<Blowfish, ZeroPadding>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Blowfish, Mode::Cbc, Pad::NoPadding) => {
                Ok(Cbc::<Blowfish, NoPadding>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Blowfish, Mode::Cbc, Pad::Iso7816) => {
                Ok(Cbc::<Blowfish, Iso7816>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Blowfish, Mode::Cbc, Pad::AnsiX923) => {
                Ok(Cbc::<Blowfish, AnsiX923>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Blowfish, Mode::Ecb, Pad::Pkcs7) => {
                Ok(Ecb::<Blowfish, Pkcs7>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Blowfish, Mode::Ecb, Pad::ZeroPadding) => {
                Ok(Ecb::<Blowfish, ZeroPadding>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Blowfish, Mode::Ecb, Pad::NoPadding) => {
                Ok(Ecb::<Blowfish, NoPadding>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Blowfish, Mode::Ecb, Pad::Iso7816) => {
                Ok(Ecb::<Blowfish, Iso7816>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
            (Cipher::Blowfish, Mode::Ecb, Pad::AnsiX923) => {
                Ok(Ecb::<Blowfish, AnsiX923>::new_from_slices(&key, &iv)?.encrypt_vec(&plaintext))
            }
        }
    }

    pub fn decrypt(
        &self,
        key: Vec<u8>,
        iv: Vec<u8>,
        ciphertext: Vec<u8>,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        match (&self.cipher, &self.mode, &self.pad) {
            (Cipher::Aes128, Mode::Cbc, Pad::Pkcs7) => {
                Ok(Cbc::<Aes128, Pkcs7>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Aes128, Mode::Cbc, Pad::ZeroPadding) => {
                Ok(Cbc::<Aes128, ZeroPadding>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Aes128, Mode::Cbc, Pad::NoPadding) => {
                Ok(Cbc::<Aes128, NoPadding>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Aes128, Mode::Cbc, Pad::Iso7816) => {
                Ok(Cbc::<Aes128, Iso7816>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Aes128, Mode::Cbc, Pad::AnsiX923) => {
                Ok(Cbc::<Aes128, AnsiX923>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Aes128, Mode::Ecb, Pad::Pkcs7) => {
                Ok(Ecb::<Aes128, Pkcs7>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Aes128, Mode::Ecb, Pad::ZeroPadding) => {
                Ok(Ecb::<Aes128, ZeroPadding>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Aes128, Mode::Ecb, Pad::NoPadding) => {
                Ok(Ecb::<Aes128, NoPadding>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Aes128, Mode::Ecb, Pad::Iso7816) => {
                Ok(Ecb::<Aes128, Iso7816>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Aes128, Mode::Ecb, Pad::AnsiX923) => {
                Ok(Ecb::<Aes128, AnsiX923>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Aes192, Mode::Ecb, Pad::Pkcs7) => {
                Ok(Ecb::<Aes192, Pkcs7>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Aes192, Mode::Ecb, Pad::ZeroPadding) => {
                Ok(Ecb::<Aes192, ZeroPadding>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Aes192, Mode::Ecb, Pad::NoPadding) => {
                Ok(Ecb::<Aes192, NoPadding>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Aes192, Mode::Ecb, Pad::Iso7816) => {
                Ok(Ecb::<Aes192, Iso7816>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Aes192, Mode::Ecb, Pad::AnsiX923) => {
                Ok(Ecb::<Aes192, AnsiX923>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Aes192, Mode::Cbc, Pad::Pkcs7) => {
                Ok(Cbc::<Aes192, Pkcs7>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Aes192, Mode::Cbc, Pad::ZeroPadding) => {
                Ok(Cbc::<Aes192, ZeroPadding>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Aes192, Mode::Cbc, Pad::NoPadding) => {
                Ok(Cbc::<Aes192, NoPadding>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Aes192, Mode::Cbc, Pad::Iso7816) => {
                Ok(Cbc::<Aes192, Iso7816>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Aes192, Mode::Cbc, Pad::AnsiX923) => {
                Ok(Cbc::<Aes192, AnsiX923>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Aes256, Mode::Cbc, Pad::Pkcs7) => {
                Ok(Cbc::<Aes256, Pkcs7>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Aes256, Mode::Cbc, Pad::ZeroPadding) => {
                Ok(Cbc::<Aes256, ZeroPadding>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Aes256, Mode::Cbc, Pad::NoPadding) => {
                Ok(Cbc::<Aes256, NoPadding>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Aes256, Mode::Cbc, Pad::Iso7816) => {
                Ok(Cbc::<Aes256, Iso7816>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Aes256, Mode::Cbc, Pad::AnsiX923) => {
                Ok(Cbc::<Aes256, AnsiX923>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Aes256, Mode::Ecb, Pad::Pkcs7) => {
                Ok(Ecb::<Aes256, Pkcs7>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Aes256, Mode::Ecb, Pad::ZeroPadding) => {
                Ok(Ecb::<Aes256, ZeroPadding>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Aes256, Mode::Ecb, Pad::NoPadding) => {
                Ok(Ecb::<Aes256, NoPadding>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Aes256, Mode::Ecb, Pad::Iso7816) => {
                Ok(Ecb::<Aes256, Iso7816>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Aes256, Mode::Ecb, Pad::AnsiX923) => {
                Ok(Ecb::<Aes256, AnsiX923>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }

            // Blowfish
            (Cipher::Blowfish, Mode::Cbc, Pad::Pkcs7) => {
                Ok(Cbc::<Blowfish, Pkcs7>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Blowfish, Mode::Cbc, Pad::ZeroPadding) => {
                Ok(Cbc::<Blowfish, ZeroPadding>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Blowfish, Mode::Cbc, Pad::NoPadding) => {
                Ok(Cbc::<Blowfish, NoPadding>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Blowfish, Mode::Cbc, Pad::Iso7816) => {
                Ok(Cbc::<Blowfish, Iso7816>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Blowfish, Mode::Cbc, Pad::AnsiX923) => {
                Ok(Cbc::<Blowfish, AnsiX923>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Blowfish, Mode::Ecb, Pad::Pkcs7) => {
                Ok(Ecb::<Blowfish, Pkcs7>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Blowfish, Mode::Ecb, Pad::ZeroPadding) => {
                Ok(Ecb::<Blowfish, ZeroPadding>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Blowfish, Mode::Ecb, Pad::NoPadding) => {
                Ok(Ecb::<Blowfish, NoPadding>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Blowfish, Mode::Ecb, Pad::Iso7816) => {
                Ok(Ecb::<Blowfish, Iso7816>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            (Cipher::Blowfish, Mode::Ecb, Pad::AnsiX923) => {
                Ok(Ecb::<Blowfish, AnsiX923>::new_from_slices(&key, &iv)?.decrypt_vec(&ciphertext)?)
            }
            
        }
    }
}