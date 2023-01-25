use md4::Md4;
use md5::Md5;
use rand::{
    rngs::{StdRng},
    SeedableRng,
};
use ripemd::{Ripemd160, Ripemd256, Ripemd320};
use rsa::{
    pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey},
    pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePublicKey, LineEnding},
    BigUint, Oaep, Pkcs1v15Encrypt, Pkcs1v15Sign, Pss, PublicKey, RsaPrivateKey,
    RsaPublicKey,
};
use sha1::Sha1;
use sha2::*;
use sha3::*;


use crate::imp::hashing::{error::HashError, make_hash};

use super::error::RsaError;

enum SignatureScheme {
    Pkcs1(Pkcs1v15Sign),
    Pss(Pss),
}

pub fn modulus_to_pem(n: &[u8], e: &[u8]) -> Result<String, RsaError> {
    let n = BigUint::from_bytes_be(n);
    let e = BigUint::from_bytes_be(e);

    let pub_key = RsaPublicKey::new(n, e)?;
    let pub_key = pub_key.to_public_key_pem(LineEnding::default())?;
    Ok(pub_key)
}

pub fn rsa_encrypt(data: &[u8], key: &str, hash_type: &str) -> Result<Vec<u8>, RsaError> {
    let pub_key = if key.contains("--BEGIN RSA") {
        match RsaPublicKey::from_pkcs1_pem(key) {
            Ok(pub_key) => pub_key,
            Err(_) => return Err(RsaError::InvalidPublicKey),
        }
    } else {
        match RsaPublicKey::from_public_key_pem(key) {
            Ok(pub_key) => pub_key,
            Err(_) => return Err(RsaError::InvalidPublicKey),
        }
    };

    let mut rng = StdRng::from_entropy();
    let encrypted = match oaep_padding_from_str(hash_type)? {
        Some(oaep) => pub_key.encrypt(&mut rng, oaep, data),
        None => pub_key.encrypt(&mut rng, Pkcs1v15Encrypt::default(), data),
    }?;
    Ok(encrypted)
}

pub fn rsa_decrypt(data: &[u8], key: &str, hash_type: &str) -> Result<Vec<u8>, RsaError> {
    let priv_key = if key.contains("--BEGIN RSA") {
        match RsaPrivateKey::from_pkcs1_pem(key) {
            Ok(priv_key) => priv_key,
            Err(_) => return Err(RsaError::InvalidPrivateKey),
        }
    } else {
        match RsaPrivateKey::from_pkcs8_pem(key) {
            Ok(priv_key) => priv_key,
            Err(_) => return Err(RsaError::InvalidPrivateKey),
        }
    };

    let decrypted = match oaep_padding_from_str(hash_type)? {
        Some(oaep) => priv_key.decrypt(oaep, data),
        None => priv_key.decrypt(Pkcs1v15Encrypt::default(), data),
    }?;

    Ok(decrypted)
}

pub fn rsa_sign(data: &[u8], key: &str, hash_type: &str, mode: &str) -> Result<Vec<u8>, RsaError> {
    let priv_key = if key.contains("--BEGIN RSA") {
        match RsaPrivateKey::from_pkcs1_pem(key) {
            Ok(priv_key) => priv_key,
            Err(_) => return Err(RsaError::InvalidPrivateKey),
        }
    } else {
        match RsaPrivateKey::from_pkcs8_pem(key) {
            Ok(priv_key) => priv_key,
            Err(_) => return Err(RsaError::InvalidPrivateKey),
        }
    };

    let hashed_data = make_hash(data, hash_type)?;

    let signed = match sign_padding_from_str(hash_type, mode)? {
        SignatureScheme::Pkcs1(padding) => priv_key.sign(padding, hashed_data.as_bytes())?,
        SignatureScheme::Pss(padding) => priv_key.sign(padding, hashed_data.as_bytes())?,
    };

    Ok(signed)
}

fn oaep_padding_from_str(hash_type: &str) -> Result<Option<Oaep>, RsaError> {
    if hash_type.is_empty() {
        return Ok(None);
    }

    let padding = match hash_type {
        "md5" => Oaep::new::<Md5>(),
        "md4" => Oaep::new::<Md4>(),
        "sha1" => Oaep::new::<Sha1>(),
        "sha224" => Oaep::new::<Sha224>(),
        "sha256" => Oaep::new::<Sha256>(),
        "sha384" => Oaep::new::<Sha384>(),
        "sha512" => Oaep::new::<Sha512>(),
        "sha3-224" => Oaep::new::<Sha3_224>(),
        "sha3-256" => Oaep::new::<Sha3_256>(),
        "sha3-384" => Oaep::new::<Sha3_384>(),
        "sha3-512" => Oaep::new::<Sha3_512>(),
        "keccak224" => Oaep::new::<Keccak224>(),
        "keccak256" => Oaep::new::<Keccak256>(),
        "keccak384" => Oaep::new::<Keccak384>(),
        "keccak512" => Oaep::new::<Keccak512>(),
        "ripemd160" => Oaep::new::<Ripemd160>(),
        "ripemd256" => Oaep::new::<Ripemd256>(),
        "ripemd320" => Oaep::new::<Ripemd320>(),
        _ => return Err(HashError::InvalidHashType(hash_type.to_owned()).into()),
    };
    Ok(Some(padding))
}

fn sign_padding_from_str(hash_type: &str, mode: &str) -> Result<SignatureScheme, RsaError> {
    let padding = match mode {
        "pkcs1" => {
            let padding = match hash_type {
                "md5" => Pkcs1v15Sign::new::<Md5>(),
                "md4" => Pkcs1v15Sign::new::<Md4>(),
                "sha1" => Pkcs1v15Sign::new::<Sha1>(),
                "sha224" => Pkcs1v15Sign::new::<Sha224>(),
                "sha256" => Pkcs1v15Sign::new::<Sha256>(),
                "sha384" => Pkcs1v15Sign::new::<Sha384>(),
                "sha512" => Pkcs1v15Sign::new::<Sha512>(),
                "sha3-224" => Pkcs1v15Sign::new::<Sha3_224>(),
                "sha3-256" => Pkcs1v15Sign::new::<Sha3_256>(),
                "sha3-384" => Pkcs1v15Sign::new::<Sha3_384>(),
                "sha3-512" => Pkcs1v15Sign::new::<Sha3_512>(),
                "ripemd160" => Pkcs1v15Sign::new::<Ripemd160>(),
                "ripemd256" => Pkcs1v15Sign::new::<Ripemd256>(),
                _ => return Err(HashError::InvalidHashType(hash_type.to_owned()).into()),
            };

            SignatureScheme::Pkcs1(padding)
        }
        "pss" => {
            let _rng = StdRng::from_entropy();
            let padding = match hash_type {
                "md5" => Pss::new::<Md5>(),
                "md4" => Pss::new::<Md4>(),
                "sha1" => Pss::new::<Sha1>(),
                "sha224" => Pss::new::<Sha224>(),
                "sha256" => Pss::new::<Sha256>(),
                "sha384" => Pss::new::<Sha384>(),
                "sha512" => Pss::new::<Sha512>(),
                "sha3-224" => Pss::new::<Sha3_224>(),
                "sha3-256" => Pss::new::<Sha3_256>(),
                "sha3-384" => Pss::new::<Sha3_384>(),
                "sha3-512" => Pss::new::<Sha3_512>(),
                "keccak224" => Pss::new::<Keccak224>(),
                "keccak256" => Pss::new::<Keccak256>(),
                "keccak384" => Pss::new::<Keccak384>(),
                "keccak512" => Pss::new::<Keccak512>(),
                "ripemd160" => Pss::new::<Ripemd160>(),
                "ripemd256" => Pss::new::<Ripemd256>(),
                "ripemd320" => Pss::new::<Ripemd320>(),
                _ => return Err(HashError::InvalidHashType(hash_type.to_owned()).into()),
            };

            SignatureScheme::Pss(padding)
        }

        _ => return Err(RsaError::InvalidSignMode(mode.to_owned())),
    };

    Ok(padding)
}
