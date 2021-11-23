use md4::Md4;
use md5::Md5;
use ripemd160::Ripemd160;
use ripemd256::Ripemd256;
use ripemd320::Ripemd320;
use rsa::{BigUint, PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey, errors, pkcs1::{FromRsaPrivateKey, FromRsaPublicKey}, pkcs8::{ToPublicKey, FromPublicKey, FromPrivateKey}};
use rand::rngs::OsRng;
use sha1::Sha1;
use sha2::*;
use sha3::*;
use thiserror::Error;

use super::hash::HashError;

#[derive(Error, Debug)]
pub enum RsaError {
    #[error("invalid public key")]
    InvalidPublicKey,
    #[error("invalid private key")]
    InvalidPrivateKey,
    #[error(transparent)]
    InvalidHashType(#[from] HashError),
    #[error(transparent)]
    Error(#[from] errors::Error),
}

pub fn modulus_to_pem(n: Vec<u8>, e: Vec<u8>) -> Result<String, errors::Error> {
    let n = BigUint::from_bytes_be(n.as_slice());
    let e = BigUint::from_bytes_be(e.as_slice());

    let pub_key = RsaPublicKey::new(n, e)?;
    let pub_key = RsaPublicKey::to_public_key_pem(&pub_key)?;
    return Ok(pub_key)
}

pub fn rsa_encrypt(data: Vec<u8>, key: String, hash_type: String) -> Result<String, RsaError> {
    let pub_key: RsaPublicKey;
    if key.contains("--BEGIN RSA") {
        pub_key = match RsaPublicKey::from_pkcs1_pem(key.as_str()) {
            Ok(pub_key) => pub_key,
            Err(_) => return Err(RsaError::InvalidPublicKey),
        };
    } else {
        pub_key = match RsaPublicKey::from_public_key_pem(key.as_str()) {
            Ok(pub_key) => pub_key,
            Err(_) => return Err(RsaError::InvalidPublicKey),
        };
    }

    let padding = padding_from_str(hash_type)?;
    let mut rng = OsRng;
    let encrypted = pub_key.encrypt(&mut rng, padding, &data)?;
    return Ok(base64::encode(encrypted))
}

pub fn rsa_decrypt(data: Vec<u8>, key: String, hash_type: String) -> Result<String, RsaError> {
    let priv_key: RsaPrivateKey;
    if key.contains("--BEGIN RSA") {
        priv_key = match RsaPrivateKey::from_pkcs1_pem(key.as_str()) {
            Ok(priv_key) => priv_key,
            Err(_) => return Err(RsaError::InvalidPrivateKey),
        };
    } else {
        priv_key = match RsaPrivateKey::from_pkcs8_pem(key.as_str()) {
            Ok(priv_key) => priv_key,
            Err(_) => return Err(RsaError::InvalidPrivateKey),
        };
    }
    
    let padding = padding_from_str(hash_type)?;
    let decrypted = priv_key.decrypt(padding, &data)?;
    return Ok(String::from_utf8_lossy(&decrypted).to_string())
}

fn padding_from_str(hash_type: String) -> Result<PaddingScheme, RsaError> {
    match hash_type.len() {
        0 => Ok(PaddingScheme::new_pkcs1v15_encrypt()),
        _ => {
            let padding = match hash_type.as_str() {
                "md5" => PaddingScheme::new_oaep::<Md5>(),
                "md4" => PaddingScheme::new_oaep::<Md4>(),
                "sha1" => PaddingScheme::new_oaep::<Sha1>(),
                "sha224" => PaddingScheme::new_oaep::<Sha224>(),
                "sha256" => PaddingScheme::new_oaep::<Sha256>(),
                "sha384" => PaddingScheme::new_oaep::<Sha384>(),
                "sha512" => PaddingScheme::new_oaep::<Sha512>(),
                "sha3-224" => PaddingScheme::new_oaep::<Sha3_224>(),
                "sha3-256" => PaddingScheme::new_oaep::<Sha3_256>(),
                "sha3-384" => PaddingScheme::new_oaep::<Sha3_384>(),
                "sha3-512" => PaddingScheme::new_oaep::<Sha3_512>(),
                "keccak224" => PaddingScheme::new_oaep::<Keccak224>(),
                "keccak256" => PaddingScheme::new_oaep::<Keccak256>(),
                "keccak384" => PaddingScheme::new_oaep::<Keccak384>(),
                "keccak512" => PaddingScheme::new_oaep::<Keccak512>(),
                "ripemd160" => PaddingScheme::new_oaep::<Ripemd160>(),
                "ripemd256" => PaddingScheme::new_oaep::<Ripemd256>(),
                "ripemd320" => PaddingScheme::new_oaep::<Ripemd320>(),
                _ => return Err(RsaError::InvalidHashType(HashError::InvalidHashType(hash_type)))
            };
            Ok(padding)
        }
    }
}