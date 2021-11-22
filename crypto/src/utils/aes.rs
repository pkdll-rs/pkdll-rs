use std::error::Error;

use aes::{Aes128, Aes192, Aes256};
use block_modes::{BlockMode, Ecb, Cbc, block_padding::*};

pub enum Pad {
    AnsiX923,
    Iso7816,
    NoPadding,
    Pkcs7,
    ZeroPadding,
}

pub enum Mode {
    Ecb,
    Cbc,
}
pub enum Len {
    Aes128,
    Aes192,
    Aes256,
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