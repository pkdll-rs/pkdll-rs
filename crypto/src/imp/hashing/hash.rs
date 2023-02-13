use digest::Digest;
use sha2::*;

use crate::{base64, call_with_hash_generic};

use super::error::HashError;

fn _hash<D: Digest>(data: &[u8]) -> Vec<u8> {
    let mut hasher = <D>::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

pub fn make_hash(data: impl AsRef<[u8]>, hash_type: &str) -> Result<String, HashError> {
    let data = data.as_ref();
    let hashed = call_with_hash_generic!(_hash(data), hash_type, HashError::InvalidHashType);
    Ok(base64::encode(hashed))
}
