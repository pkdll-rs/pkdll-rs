use base64::{engine::general_purpose::STANDARD_NO_PAD as BASE64, Engine};
use digest::core_api::BlockSizeUser;
use hmac::Mac;
use hmac::SimpleHmac;
use md4::Md4;
use md5::Md5;
use ripemd::Ripemd128;
use ripemd::Ripemd160;
use ripemd::Ripemd256;
use ripemd::Ripemd320;
use sha1::Sha1;
use sha2::*;
use sha3::*;

use super::error::HashError;

fn _hmac<D: Digest + BlockSizeUser>(data: &[u8], key: &[u8]) -> Result<Vec<u8>, HashError> {
    let mut hasher = SimpleHmac::<D>::new_from_slice(key)?;
    hasher.update(data);
    Ok(hasher.finalize().into_bytes().to_vec())
}

pub fn make_hmac(
    data: impl AsRef<[u8]>,
    key: impl AsRef<[u8]>,
    hash_type: &str,
) -> Result<String, HashError> {
    let data = data.as_ref();
    let key = key.as_ref();
    let mac = match hash_type {
        "md5" => _hmac::<Md5>(data, key),
        "md4" => _hmac::<Md4>(data, key),
        "sha1" => _hmac::<Sha1>(data, key),
        "sha224" => _hmac::<Sha224>(data, key),
        "sha256" => _hmac::<Sha256>(data, key),
        "sha384" => _hmac::<Sha384>(data, key),
        "sha512" => _hmac::<Sha512>(data, key),
        "sha3-224" => _hmac::<Sha3_224>(data, key),
        "sha3-256" => _hmac::<Sha3_256>(data, key),
        "sha3-384" => _hmac::<Sha3_384>(data, key),
        "sha3-512" => _hmac::<Sha3_512>(data, key),
        "keccak224" => _hmac::<Keccak256>(data, key),
        "keccak256" => _hmac::<Keccak256>(data, key),
        "keccak384" => _hmac::<Keccak384>(data, key),
        "keccak512" => _hmac::<Keccak512>(data, key),
        "ripemd128" => _hmac::<Ripemd128>(data, key),
        "ripemd160" => _hmac::<Ripemd160>(data, key),
        "ripemd256" => _hmac::<Ripemd256>(data, key),
        "ripemd320" => _hmac::<Ripemd320>(data, key),
        _ => return Err(HashError::InvalidHashType(hash_type.to_owned())),
    }?;
    Ok(BASE64.encode(mac))
}
