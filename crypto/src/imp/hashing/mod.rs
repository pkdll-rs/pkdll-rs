pub mod error;
mod hash;
mod hmac;
pub use self::hmac::make_hmac;
pub use hash::make_hash;
