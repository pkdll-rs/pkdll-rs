pub mod error;
mod hash;
mod hmac;
mod macros;
pub use self::hmac::make_hmac;
pub use hash::make_hash;
