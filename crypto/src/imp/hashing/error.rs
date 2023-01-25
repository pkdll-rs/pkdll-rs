use digest::InvalidLength;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HashError {
    #[error("invalid hash type: {0}")]
    InvalidHashType(String),
    #[error("invalid hmac key provided")]
    BadHmacKey(#[from] InvalidLength),
}
