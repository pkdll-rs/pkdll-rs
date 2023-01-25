use std::{borrow::Cow, str::FromStr};

use aes::{
    cipher::{
        block_padding::{AnsiX923, Iso10126, Iso7816, NoPadding, Pkcs7, ZeroPadding},
        BlockDecryptMut, BlockEncryptMut, KeyIvInit, StreamCipher,
    },
    Aes128, Aes192, Aes256,
};
use cbc;
use ctr::Ctr128BE;
use digest::KeyInit;
use ecb;

use blowfish::Blowfish;

use super::error::CipherError;

pub enum Padding {
    NoPadding,
    Pkcs7,
    ZeroPadding,
    AnsiX923,
    Iso7816,
    Iso10126,
}

impl FromStr for Padding {
    type Err = CipherError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let padding = match s {
            "pkcs7" => Padding::Pkcs7,
            "nopadding" => Padding::NoPadding,
            "zero" => Padding::ZeroPadding,
            "iso7816" => Padding::Iso7816,
            "iso10126" => Padding::Iso10126,
            "ansi_x923" => Padding::AnsiX923,
            _ => return Err(CipherError::InvalidPadding(s.to_owned())),
        };
        Ok(padding)
    }
}

pub enum Mode {
    Ecb,
    Cbc,
    Ctr,
}

impl FromStr for Mode {
    type Err = CipherError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mode = match s {
            "ecb" => Mode::Ecb,
            "cbc" => Mode::Cbc,
            "ctr" => Mode::Ctr,
            _ => return Err(CipherError::InvalidMode(s.to_owned())),
        };
        Ok(mode)
    }
}

#[derive(Clone, Copy)]
pub enum BlockCipher {
    Aes128,
    Aes192,
    Aes256,
    Blowfish,
}

impl BlockCipher {
    pub fn new_from_key_length(key_length: usize) -> Result<BlockCipher, CipherError> {
        let aes_type = match key_length {
            16 => BlockCipher::Aes128,
            24 => BlockCipher::Aes192,
            32 => BlockCipher::Aes256,
            _ => return Err(CipherError::InvalidKeyLen(key_length)),
        };
        Ok(aes_type)
    }
}

pub struct Cipher {
    cipher: BlockCipher,
    mode: Mode,
    padding: Padding,
}

impl Cipher {
    pub fn new(cipher: BlockCipher, mode: Mode, padding: Padding) -> Self {
        Self {
            cipher,
            mode,
            padding,
        }
    }

    pub fn encrypt<'a>(
        &self,
        key: &[u8],
        iv: &[u8],
        data: &'a mut [u8],
    ) -> Result<Cow<'a, [u8]>, CipherError> {
        if !Self::validate_key(self.cipher, key) {
            return Err(CipherError::InvalidKeyLen(key.len()));
        }

        match (&self.cipher, &self.mode, &self.padding) {
            // AES128 CBC
            (BlockCipher::Aes128, Mode::Cbc, Padding::Pkcs7) => {
                Ok(cbc::Encryptor::<Aes128>::new_from_slices(key, iv)?
                    .encrypt_padded_vec_mut::<Pkcs7>(data)
                    .into())
            }
            (BlockCipher::Aes128, Mode::Cbc, Padding::ZeroPadding) => {
                Ok(cbc::Encryptor::<Aes128>::new_from_slices(key, iv)?
                    .encrypt_padded_vec_mut::<ZeroPadding>(data)
                    .into())
            }
            (BlockCipher::Aes128, Mode::Cbc, Padding::NoPadding) => {
                Ok(cbc::Encryptor::<Aes128>::new_from_slices(key, iv)?
                    .encrypt_padded_vec_mut::<NoPadding>(data)
                    .into())
            }
            (BlockCipher::Aes128, Mode::Cbc, Padding::Iso7816) => {
                Ok(cbc::Encryptor::<Aes128>::new_from_slices(key, iv)?
                    .encrypt_padded_vec_mut::<Iso7816>(data)
                    .into())
            }
            (BlockCipher::Aes128, Mode::Cbc, Padding::Iso10126) => {
                Ok(cbc::Encryptor::<Aes128>::new_from_slices(key, iv)?
                    .encrypt_padded_vec_mut::<Iso10126>(data)
                    .into())
            }
            (BlockCipher::Aes128, Mode::Cbc, Padding::AnsiX923) => {
                Ok(cbc::Encryptor::<Aes128>::new_from_slices(key, iv)?
                    .encrypt_padded_vec_mut::<AnsiX923>(data)
                    .into())
            }

            // AES192 CBC
            (BlockCipher::Aes192, Mode::Cbc, Padding::Pkcs7) => {
                Ok(cbc::Encryptor::<Aes192>::new_from_slices(key, iv)?
                    .encrypt_padded_vec_mut::<Pkcs7>(data)
                    .into())
            }
            (BlockCipher::Aes192, Mode::Cbc, Padding::ZeroPadding) => {
                Ok(cbc::Encryptor::<Aes192>::new_from_slices(key, iv)?
                    .encrypt_padded_vec_mut::<ZeroPadding>(data)
                    .into())
            }
            (BlockCipher::Aes192, Mode::Cbc, Padding::NoPadding) => {
                Ok(cbc::Encryptor::<Aes192>::new_from_slices(key, iv)?
                    .encrypt_padded_vec_mut::<NoPadding>(data)
                    .into())
            }
            (BlockCipher::Aes192, Mode::Cbc, Padding::Iso7816) => {
                Ok(cbc::Encryptor::<Aes192>::new_from_slices(key, iv)?
                    .encrypt_padded_vec_mut::<Iso7816>(data)
                    .into())
            }
            (BlockCipher::Aes192, Mode::Cbc, Padding::Iso10126) => {
                Ok(cbc::Encryptor::<Aes192>::new_from_slices(key, iv)?
                    .encrypt_padded_vec_mut::<Iso10126>(data)
                    .into())
            }
            (BlockCipher::Aes192, Mode::Cbc, Padding::AnsiX923) => {
                Ok(cbc::Encryptor::<Aes192>::new_from_slices(key, iv)?
                    .encrypt_padded_vec_mut::<AnsiX923>(data)
                    .into())
            }

            // AES256 CBC
            (BlockCipher::Aes256, Mode::Cbc, Padding::ZeroPadding) => {
                Ok(cbc::Encryptor::<Aes256>::new_from_slices(key, iv)?
                    .encrypt_padded_vec_mut::<ZeroPadding>(data)
                    .into())
            }
            (BlockCipher::Aes256, Mode::Cbc, Padding::NoPadding) => {
                Ok(cbc::Encryptor::<Aes256>::new_from_slices(key, iv)?
                    .encrypt_padded_vec_mut::<NoPadding>(data)
                    .into())
            }
            (BlockCipher::Aes256, Mode::Cbc, Padding::Iso7816) => {
                Ok(cbc::Encryptor::<Aes256>::new_from_slices(key, iv)?
                    .encrypt_padded_vec_mut::<Iso7816>(data)
                    .into())
            }
            (BlockCipher::Aes256, Mode::Cbc, Padding::Iso10126) => {
                Ok(cbc::Encryptor::<Aes256>::new_from_slices(key, iv)?
                    .encrypt_padded_vec_mut::<Iso10126>(data)
                    .into())
            }
            (BlockCipher::Aes256, Mode::Cbc, Padding::AnsiX923) => {
                Ok(cbc::Encryptor::<Aes256>::new_from_slices(key, iv)?
                    .encrypt_padded_vec_mut::<AnsiX923>(data)
                    .into())
            }
            (BlockCipher::Aes256, Mode::Cbc, Padding::Pkcs7) => {
                Ok(cbc::Encryptor::<Aes256>::new_from_slices(key, iv)?
                    .encrypt_padded_vec_mut::<Pkcs7>(data)
                    .into())
            }

            // AES128 ECB
            (BlockCipher::Aes128, Mode::Ecb, Padding::Pkcs7) => {
                Ok(ecb::Encryptor::<Aes128>::new_from_slice(key)?
                    .encrypt_padded_vec_mut::<Pkcs7>(data)
                    .into())
            }
            (BlockCipher::Aes128, Mode::Ecb, Padding::ZeroPadding) => {
                Ok(ecb::Encryptor::<Aes128>::new_from_slice(key)?
                    .encrypt_padded_vec_mut::<ZeroPadding>(data)
                    .into())
            }
            (BlockCipher::Aes128, Mode::Ecb, Padding::NoPadding) => {
                Ok(ecb::Encryptor::<Aes128>::new_from_slice(key)?
                    .encrypt_padded_vec_mut::<NoPadding>(data)
                    .into())
            }
            (BlockCipher::Aes128, Mode::Ecb, Padding::Iso7816) => {
                Ok(ecb::Encryptor::<Aes128>::new_from_slice(key)?
                    .encrypt_padded_vec_mut::<Iso7816>(data)
                    .into())
            }
            (BlockCipher::Aes128, Mode::Ecb, Padding::Iso10126) => {
                Ok(ecb::Encryptor::<Aes128>::new_from_slice(key)?
                    .encrypt_padded_vec_mut::<Iso10126>(data)
                    .into())
            }
            (BlockCipher::Aes128, Mode::Ecb, Padding::AnsiX923) => {
                Ok(ecb::Encryptor::<Aes128>::new_from_slice(key)?
                    .encrypt_padded_vec_mut::<AnsiX923>(data)
                    .into())
            }
            (BlockCipher::Aes192, Mode::Ecb, Padding::Pkcs7) => {
                Ok(ecb::Encryptor::<Aes192>::new_from_slice(key)?
                    .encrypt_padded_vec_mut::<Pkcs7>(data)
                    .into())
            }

            // AES192 ECB
            (BlockCipher::Aes192, Mode::Ecb, Padding::ZeroPadding) => {
                Ok(ecb::Encryptor::<Aes192>::new_from_slice(key)?
                    .encrypt_padded_vec_mut::<ZeroPadding>(data)
                    .into())
            }
            (BlockCipher::Aes192, Mode::Ecb, Padding::NoPadding) => {
                Ok(ecb::Encryptor::<Aes192>::new_from_slice(key)?
                    .encrypt_padded_vec_mut::<NoPadding>(data)
                    .into())
            }
            (BlockCipher::Aes192, Mode::Ecb, Padding::Iso7816) => {
                Ok(ecb::Encryptor::<Aes192>::new_from_slice(key)?
                    .encrypt_padded_vec_mut::<Iso7816>(data)
                    .into())
            }
            (BlockCipher::Aes192, Mode::Ecb, Padding::Iso10126) => {
                Ok(ecb::Encryptor::<Aes192>::new_from_slice(key)?
                    .encrypt_padded_vec_mut::<Iso7816>(data)
                    .into())
            }
            (BlockCipher::Aes192, Mode::Ecb, Padding::AnsiX923) => {
                Ok(ecb::Encryptor::<Aes192>::new_from_slice(key)?
                    .encrypt_padded_vec_mut::<AnsiX923>(data)
                    .into())
            }

            // AES256 ECB
            (BlockCipher::Aes256, Mode::Ecb, Padding::Pkcs7) => {
                Ok(ecb::Encryptor::<Aes256>::new_from_slice(key)?
                    .encrypt_padded_vec_mut::<Pkcs7>(data)
                    .into())
            }
            (BlockCipher::Aes256, Mode::Ecb, Padding::ZeroPadding) => {
                Ok(ecb::Encryptor::<Aes256>::new_from_slice(key)?
                    .encrypt_padded_vec_mut::<ZeroPadding>(data)
                    .into())
            }
            (BlockCipher::Aes256, Mode::Ecb, Padding::NoPadding) => {
                Ok(ecb::Encryptor::<Aes256>::new_from_slice(key)?
                    .encrypt_padded_vec_mut::<NoPadding>(data)
                    .into())
            }
            (BlockCipher::Aes256, Mode::Ecb, Padding::Iso7816) => {
                Ok(ecb::Encryptor::<Aes256>::new_from_slice(key)?
                    .encrypt_padded_vec_mut::<Iso7816>(data)
                    .into())
            }
            (BlockCipher::Aes256, Mode::Ecb, Padding::Iso10126) => {
                Ok(ecb::Encryptor::<Aes256>::new_from_slice(key)?
                    .encrypt_padded_vec_mut::<Iso10126>(data)
                    .into())
            }
            (BlockCipher::Aes256, Mode::Ecb, Padding::AnsiX923) => {
                Ok(ecb::Encryptor::<Aes256>::new_from_slice(key)?
                    .encrypt_padded_vec_mut::<AnsiX923>(data)
                    .into())
            }

            // AES CTR
            (BlockCipher::Aes128, Mode::Ctr, _) => {
                Ctr128BE::<Aes128>::new_from_slices(key, iv)?.apply_keystream(data);
                Ok(Cow::Borrowed(data))
            }

            (BlockCipher::Aes192, Mode::Ctr, _) => {
                Ctr128BE::<Aes192>::new_from_slices(key, iv)?.apply_keystream(data);
                Ok(std::borrow::Cow::Borrowed(data))
            }

            (BlockCipher::Aes256, Mode::Ctr, _) => {
                Ctr128BE::<Aes256>::new_from_slices(key, iv)?.apply_keystream(data);
                Ok(std::borrow::Cow::Borrowed(data))
            }

            // Blowfish CBC
            (BlockCipher::Blowfish, Mode::Cbc, Padding::Pkcs7) => {
                Ok(cbc::Encryptor::<Blowfish>::new_from_slices(key, iv)?
                    .encrypt_padded_vec_mut::<Pkcs7>(data)
                    .into())
            }
            (BlockCipher::Blowfish, Mode::Cbc, Padding::ZeroPadding) => {
                Ok(cbc::Encryptor::<Blowfish>::new_from_slices(key, iv)?
                    .encrypt_padded_vec_mut::<ZeroPadding>(data)
                    .into())
            }
            (BlockCipher::Blowfish, Mode::Cbc, Padding::NoPadding) => {
                Ok(cbc::Encryptor::<Blowfish>::new_from_slices(key, iv)?
                    .encrypt_padded_vec_mut::<NoPadding>(data)
                    .into())
            }
            (BlockCipher::Blowfish, Mode::Cbc, Padding::Iso7816) => {
                Ok(cbc::Encryptor::<Blowfish>::new_from_slices(key, iv)?
                    .encrypt_padded_vec_mut::<Iso7816>(data)
                    .into())
            }
            (BlockCipher::Blowfish, Mode::Cbc, Padding::Iso10126) => {
                Ok(cbc::Encryptor::<Blowfish>::new_from_slices(key, iv)?
                    .encrypt_padded_vec_mut::<Iso10126>(data)
                    .into())
            }
            (BlockCipher::Blowfish, Mode::Cbc, Padding::AnsiX923) => {
                Ok(cbc::Encryptor::<Blowfish>::new_from_slices(key, iv)?
                    .encrypt_padded_vec_mut::<AnsiX923>(data)
                    .into())
            }

            // Blowfish ECB
            (BlockCipher::Blowfish, Mode::Ecb, Padding::Pkcs7) => {
                Ok(ecb::Encryptor::<Blowfish>::new_from_slice(key)?
                    .encrypt_padded_vec_mut::<Pkcs7>(data)
                    .into())
            }
            (BlockCipher::Blowfish, Mode::Ecb, Padding::ZeroPadding) => {
                Ok(ecb::Encryptor::<Blowfish>::new_from_slice(key)?
                    .encrypt_padded_vec_mut::<ZeroPadding>(data)
                    .into())
            }
            (BlockCipher::Blowfish, Mode::Ecb, Padding::NoPadding) => {
                Ok(ecb::Encryptor::<Blowfish>::new_from_slice(key)?
                    .encrypt_padded_vec_mut::<NoPadding>(data)
                    .into())
            }
            (BlockCipher::Blowfish, Mode::Ecb, Padding::Iso7816) => {
                Ok(ecb::Encryptor::<Blowfish>::new_from_slice(key)?
                    .encrypt_padded_vec_mut::<Iso7816>(data)
                    .into())
            }
            (BlockCipher::Blowfish, Mode::Ecb, Padding::Iso10126) => {
                Ok(ecb::Encryptor::<Blowfish>::new_from_slice(key)?
                    .encrypt_padded_vec_mut::<Iso10126>(data)
                    .into())
            }
            (BlockCipher::Blowfish, Mode::Ecb, Padding::AnsiX923) => {
                Ok(ecb::Encryptor::<Blowfish>::new_from_slice(key)?
                    .encrypt_padded_vec_mut::<AnsiX923>(data)
                    .into())
            }

            (BlockCipher::Blowfish, Mode::Ctr, _) => Err(CipherError::CtrBlowfish),
        }
    }

    pub fn decrypt<'a>(
        &self,
        key: &[u8],
        iv: &[u8],
        encrypted_data: &'a mut [u8],
    ) -> Result<Cow<'a, [u8]>, CipherError> {
        if !Self::validate_key(self.cipher, key) {
            return Err(CipherError::InvalidKeyLen(key.len()));
        }

        match (&self.cipher, &self.mode, &self.padding) {
            // AES128 CBC
            (BlockCipher::Aes128, Mode::Cbc, Padding::Pkcs7) => {
                Ok(cbc::Decryptor::<Aes128>::new_from_slices(key, iv)?
                    .decrypt_padded_vec_mut::<Pkcs7>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Aes128, Mode::Cbc, Padding::ZeroPadding) => {
                Ok(cbc::Decryptor::<Aes128>::new_from_slices(key, iv)?
                    .decrypt_padded_vec_mut::<ZeroPadding>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Aes128, Mode::Cbc, Padding::NoPadding) => {
                Ok(cbc::Decryptor::<Aes128>::new_from_slices(key, iv)?
                    .decrypt_padded_vec_mut::<NoPadding>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Aes128, Mode::Cbc, Padding::Iso7816) => {
                Ok(cbc::Decryptor::<Aes128>::new_from_slices(key, iv)?
                    .decrypt_padded_vec_mut::<Iso7816>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Aes128, Mode::Cbc, Padding::Iso10126) => {
                Ok(cbc::Decryptor::<Aes128>::new_from_slices(key, iv)?
                    .decrypt_padded_vec_mut::<Iso10126>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Aes128, Mode::Cbc, Padding::AnsiX923) => {
                Ok(cbc::Decryptor::<Aes128>::new_from_slices(key, iv)?
                    .decrypt_padded_vec_mut::<AnsiX923>(encrypted_data)?
                    .into())
            }

            // AES192 CBC
            (BlockCipher::Aes192, Mode::Cbc, Padding::Pkcs7) => {
                Ok(cbc::Decryptor::<Aes192>::new_from_slices(key, iv)?
                    .decrypt_padded_vec_mut::<Pkcs7>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Aes192, Mode::Cbc, Padding::ZeroPadding) => {
                Ok(cbc::Decryptor::<Aes192>::new_from_slices(key, iv)?
                    .decrypt_padded_vec_mut::<ZeroPadding>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Aes192, Mode::Cbc, Padding::NoPadding) => {
                Ok(cbc::Decryptor::<Aes192>::new_from_slices(key, iv)?
                    .decrypt_padded_vec_mut::<NoPadding>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Aes192, Mode::Cbc, Padding::Iso7816) => {
                Ok(cbc::Decryptor::<Aes192>::new_from_slices(key, iv)?
                    .decrypt_padded_vec_mut::<Iso7816>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Aes192, Mode::Cbc, Padding::Iso10126) => {
                Ok(cbc::Decryptor::<Aes192>::new_from_slices(key, iv)?
                    .decrypt_padded_vec_mut::<Iso10126>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Aes192, Mode::Cbc, Padding::AnsiX923) => {
                Ok(cbc::Decryptor::<Aes192>::new_from_slices(key, iv)?
                    .decrypt_padded_vec_mut::<AnsiX923>(encrypted_data)?
                    .into())
            }

            // AES256 CBC
            (BlockCipher::Aes256, Mode::Cbc, Padding::ZeroPadding) => {
                Ok(cbc::Decryptor::<Aes256>::new_from_slices(key, iv)?
                    .decrypt_padded_vec_mut::<ZeroPadding>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Aes256, Mode::Cbc, Padding::NoPadding) => {
                Ok(cbc::Decryptor::<Aes256>::new_from_slices(key, iv)?
                    .decrypt_padded_vec_mut::<NoPadding>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Aes256, Mode::Cbc, Padding::Iso7816) => {
                Ok(cbc::Decryptor::<Aes256>::new_from_slices(key, iv)?
                    .decrypt_padded_vec_mut::<Iso7816>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Aes256, Mode::Cbc, Padding::Iso10126) => {
                Ok(cbc::Decryptor::<Aes256>::new_from_slices(key, iv)?
                    .decrypt_padded_vec_mut::<Iso10126>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Aes256, Mode::Cbc, Padding::AnsiX923) => {
                Ok(cbc::Decryptor::<Aes256>::new_from_slices(key, iv)?
                    .decrypt_padded_vec_mut::<AnsiX923>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Aes256, Mode::Cbc, Padding::Pkcs7) => {
                Ok(cbc::Decryptor::<Aes256>::new_from_slices(key, iv)?
                    .decrypt_padded_vec_mut::<Pkcs7>(encrypted_data)?
                    .into())
            }

            // AES128 ECB
            (BlockCipher::Aes128, Mode::Ecb, Padding::Pkcs7) => {
                Ok(ecb::Decryptor::<Aes128>::new_from_slice(key)?
                    .decrypt_padded_vec_mut::<Pkcs7>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Aes128, Mode::Ecb, Padding::ZeroPadding) => {
                Ok(ecb::Decryptor::<Aes128>::new_from_slice(key)?
                    .decrypt_padded_vec_mut::<ZeroPadding>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Aes128, Mode::Ecb, Padding::NoPadding) => {
                Ok(ecb::Decryptor::<Aes128>::new_from_slice(key)?
                    .decrypt_padded_vec_mut::<NoPadding>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Aes128, Mode::Ecb, Padding::Iso7816) => {
                Ok(ecb::Decryptor::<Aes128>::new_from_slice(key)?
                    .decrypt_padded_vec_mut::<Iso7816>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Aes128, Mode::Ecb, Padding::Iso10126) => {
                Ok(ecb::Decryptor::<Aes128>::new_from_slice(key)?
                    .decrypt_padded_vec_mut::<Iso10126>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Aes128, Mode::Ecb, Padding::AnsiX923) => {
                Ok(ecb::Decryptor::<Aes128>::new_from_slice(key)?
                    .decrypt_padded_vec_mut::<AnsiX923>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Aes192, Mode::Ecb, Padding::Pkcs7) => {
                Ok(ecb::Decryptor::<Aes192>::new_from_slice(key)?
                    .decrypt_padded_vec_mut::<Pkcs7>(encrypted_data)?
                    .into())
            }

            // AES192 ECB
            (BlockCipher::Aes192, Mode::Ecb, Padding::ZeroPadding) => {
                Ok(ecb::Decryptor::<Aes192>::new_from_slice(key)?
                    .decrypt_padded_vec_mut::<ZeroPadding>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Aes192, Mode::Ecb, Padding::NoPadding) => {
                Ok(ecb::Decryptor::<Aes192>::new_from_slice(key)?
                    .decrypt_padded_vec_mut::<NoPadding>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Aes192, Mode::Ecb, Padding::Iso7816) => {
                Ok(ecb::Decryptor::<Aes192>::new_from_slice(key)?
                    .decrypt_padded_vec_mut::<Iso7816>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Aes192, Mode::Ecb, Padding::Iso10126) => {
                Ok(ecb::Decryptor::<Aes192>::new_from_slice(key)?
                    .decrypt_padded_vec_mut::<Iso7816>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Aes192, Mode::Ecb, Padding::AnsiX923) => {
                Ok(ecb::Decryptor::<Aes192>::new_from_slice(key)?
                    .decrypt_padded_vec_mut::<AnsiX923>(encrypted_data)?
                    .into())
            }

            // AES256 ECB
            (BlockCipher::Aes256, Mode::Ecb, Padding::Pkcs7) => {
                Ok(ecb::Decryptor::<Aes256>::new_from_slice(key)?
                    .decrypt_padded_vec_mut::<Pkcs7>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Aes256, Mode::Ecb, Padding::ZeroPadding) => {
                Ok(ecb::Decryptor::<Aes256>::new_from_slice(key)?
                    .decrypt_padded_vec_mut::<ZeroPadding>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Aes256, Mode::Ecb, Padding::NoPadding) => {
                Ok(ecb::Decryptor::<Aes256>::new_from_slice(key)?
                    .decrypt_padded_vec_mut::<NoPadding>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Aes256, Mode::Ecb, Padding::Iso7816) => {
                Ok(ecb::Decryptor::<Aes256>::new_from_slice(key)?
                    .decrypt_padded_vec_mut::<Iso7816>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Aes256, Mode::Ecb, Padding::Iso10126) => {
                Ok(ecb::Decryptor::<Aes256>::new_from_slice(key)?
                    .decrypt_padded_vec_mut::<Iso10126>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Aes256, Mode::Ecb, Padding::AnsiX923) => {
                Ok(ecb::Decryptor::<Aes256>::new_from_slice(key)?
                    .decrypt_padded_vec_mut::<AnsiX923>(encrypted_data)?
                    .into())
            }

            // AES CTR
            (BlockCipher::Aes128, Mode::Ctr, _) => {
                Ctr128BE::<Aes128>::new_from_slices(key, iv)?.apply_keystream(encrypted_data);
                Ok(Cow::Borrowed(encrypted_data))
            }

            (BlockCipher::Aes192, Mode::Ctr, _) => {
                Ctr128BE::<Aes192>::new_from_slices(key, iv)?.apply_keystream(encrypted_data);
                Ok(std::borrow::Cow::Borrowed(encrypted_data))
            }

            (BlockCipher::Aes256, Mode::Ctr, _) => {
                Ctr128BE::<Aes256>::new_from_slices(key, iv)?.apply_keystream(encrypted_data);
                Ok(std::borrow::Cow::Borrowed(encrypted_data))
            }

            // Blowfish CBC
            (BlockCipher::Blowfish, Mode::Cbc, Padding::Pkcs7) => {
                Ok(cbc::Decryptor::<Blowfish>::new_from_slices(key, iv)?
                    .decrypt_padded_vec_mut::<Pkcs7>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Blowfish, Mode::Cbc, Padding::ZeroPadding) => {
                Ok(cbc::Decryptor::<Blowfish>::new_from_slices(key, iv)?
                    .decrypt_padded_vec_mut::<ZeroPadding>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Blowfish, Mode::Cbc, Padding::NoPadding) => {
                Ok(cbc::Decryptor::<Blowfish>::new_from_slices(key, iv)?
                    .decrypt_padded_vec_mut::<NoPadding>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Blowfish, Mode::Cbc, Padding::Iso7816) => {
                Ok(cbc::Decryptor::<Blowfish>::new_from_slices(key, iv)?
                    .decrypt_padded_vec_mut::<Iso7816>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Blowfish, Mode::Cbc, Padding::Iso10126) => {
                Ok(cbc::Decryptor::<Blowfish>::new_from_slices(key, iv)?
                    .decrypt_padded_vec_mut::<Iso10126>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Blowfish, Mode::Cbc, Padding::AnsiX923) => {
                Ok(cbc::Decryptor::<Blowfish>::new_from_slices(key, iv)?
                    .decrypt_padded_vec_mut::<AnsiX923>(encrypted_data)?
                    .into())
            }

            // Blowfish ECB
            (BlockCipher::Blowfish, Mode::Ecb, Padding::Pkcs7) => {
                Ok(ecb::Decryptor::<Blowfish>::new_from_slice(key)?
                    .decrypt_padded_vec_mut::<Pkcs7>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Blowfish, Mode::Ecb, Padding::ZeroPadding) => {
                Ok(ecb::Decryptor::<Blowfish>::new_from_slice(key)?
                    .decrypt_padded_vec_mut::<ZeroPadding>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Blowfish, Mode::Ecb, Padding::NoPadding) => {
                Ok(ecb::Decryptor::<Blowfish>::new_from_slice(key)?
                    .decrypt_padded_vec_mut::<NoPadding>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Blowfish, Mode::Ecb, Padding::Iso7816) => {
                Ok(ecb::Decryptor::<Blowfish>::new_from_slice(key)?
                    .decrypt_padded_vec_mut::<Iso7816>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Blowfish, Mode::Ecb, Padding::Iso10126) => {
                Ok(ecb::Decryptor::<Blowfish>::new_from_slice(key)?
                    .decrypt_padded_vec_mut::<Iso10126>(encrypted_data)?
                    .into())
            }
            (BlockCipher::Blowfish, Mode::Ecb, Padding::AnsiX923) => {
                Ok(ecb::Decryptor::<Blowfish>::new_from_slice(key)?
                    .decrypt_padded_vec_mut::<AnsiX923>(encrypted_data)?
                    .into())
            }

            (BlockCipher::Blowfish, Mode::Ctr, _) => Err(CipherError::CtrBlowfish),
        }
    }

    fn validate_key(cipher: BlockCipher, key: &[u8]) -> bool {
        match cipher {
            BlockCipher::Aes128 => key.len() == 16,
            BlockCipher::Aes192 => key.len() == 24,
            BlockCipher::Aes256 => key.len() == 32,
            BlockCipher::Blowfish => (4..=56).contains(&key.len()),
        }
    }
}
