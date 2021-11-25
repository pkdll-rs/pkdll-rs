use digest::Digest;
pub use sha2::*;
pub use sha3::*;
pub use md5::Md5;
pub use md4::Md4;
pub use sha1::Sha1;
pub use ripemd160::Ripemd160;
pub use ripemd256::Ripemd256;
pub use ripemd320::Ripemd320;

use thiserror::Error;

use crate::switch_hash_trait;

#[derive(Error, Debug)]
pub enum HashError {
    #[error("hash type unsupported yet: `{0}`")]
    InvalidHashType(String),
}

fn _hash<D: Digest + Sync>(data: Vec<u8>) -> Result<Vec<u8>, HashError>{
    let mut hasher = <D>::new();
    hasher.update(&data);
    Ok(hasher.finalize().to_vec())
}

pub fn make_hash(data: Vec<u8>, hash_type: &str) -> Result<Vec<u8>, HashError> {
    let hashed = switch_hash_trait!(_hash, hash_type, , data)?;
    Ok(hashed)
}