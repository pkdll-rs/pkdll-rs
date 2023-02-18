use digest::FixedOutput;
use digest::KeyInit;
use digest::Update;

use crate::base64;
use crate::call_with_hash_generic;

use super::error::HashError;

fn _hmac<H: KeyInit + Update + FixedOutput>(data: &[u8], key: &[u8]) -> Result<Vec<u8>, HashError> {
    let mut hasher = H::new_from_slice(key)?;
    hasher.update(data);
    Ok(hasher.finalize_fixed()[..].to_vec())
}

pub fn make_hmac(
    data: &[u8],
    key: &[u8],
    hash_type: &str,
) -> Result<Vec<u8>, HashError> {
    let mac = call_with_hash_generic!(_hmac(data, key), hmac(hash_type), HashError::InvalidHashType)?;
    Ok(mac)
}
