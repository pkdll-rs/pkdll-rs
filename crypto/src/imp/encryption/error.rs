use aes::cipher::block_padding::UnpadError;
use digest::InvalidLength;
use rsa::{errors::Error as _RsaError, pkcs8::spki};
use thiserror::Error;

use crate::imp::hashing::error::HashError;

#[derive(Error, Debug)]
pub enum RsaError {
    #[error("invalid public key")]
    InvalidPublicKey,
    #[error("invalid private key")]
    InvalidPrivateKey,
    #[error("invalid sign mode: {0}")]
    InvalidSignMode(String),
    #[error("rsa key error: {0}")]
    SpkiError(#[from] spki::Error),
    #[error(transparent)]
    HashError(#[from] HashError),
    #[error("rsa error: {0}")]
    Other(#[from] _RsaError),
}

#[derive(Error, Debug)]
pub enum CipherError {
    #[error("invalid padding: {0}")]
    InvalidPadding(String),
    #[error("invalid mode: {0}")]
    InvalidMode(String),
    #[error("invalid key length: {0}. Only 16, 24, 32 accepted")]
    InvalidKeyLen(usize),
    #[error("CTR mode does not support blowfish")]
    CtrBlowfish,
    #[error("failed to decrypt (unpad failed or bad input data)")]
    Unpad(#[from] UnpadError),
}

impl From<InvalidLength> for CipherError {
    fn from(_: InvalidLength) -> Self {
        eprintln!("InvalidLength to AesError conversion");
        Self::InvalidKeyLen(0)
    }
}
