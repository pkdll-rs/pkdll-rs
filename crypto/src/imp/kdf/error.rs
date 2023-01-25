use bcrypt::BcryptError;
use scrypt::errors::{InvalidOutputLen, InvalidParams};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KdfError {
    #[error("bcrypt error: {0}")]
    Bcrypt(#[from] BcryptError),
    #[error("scrypt error: {0}")]
    Scrypt(#[from] InvalidOutputLen),
    #[error("scrypt invalid config: {0}")]
    ScryptInvalidConf(#[from] InvalidParams),
}
