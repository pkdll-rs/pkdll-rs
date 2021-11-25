#[macro_export]
macro_rules! unwrap_or_err {
    ( $e:expr ) => {
        match $e {
            Ok(result) => result,
            Err(error) => {
                let mut err_string = error.to_string();
                err_string.insert_str(0, crate::ERR);
                let wstring = cstring::to_widechar(&err_string);
                return mem::ManuallyDrop::new(wstring).as_ptr();
            },
        }
    }
}

#[macro_export]
macro_rules! switch_hash_trait {
    ( $func:ident, $hash_type:expr, $($trait_arg1: ty)?, $($arg: expr),* ) => {
        match $hash_type {
            "md5" => $func::<Md5, $($trait_arg1)?>($($arg),*),
            "md4" => $func::<Md4, $($trait_arg1)?>($($arg),*),
            "sha1" => $func::<Sha1, $($trait_arg1)?>($($arg),*),
            "sha224" => $func::<Sha224, $($trait_arg1)?>($($arg),*),
            "sha256" => $func::<Sha256, $($trait_arg1)?>($($arg),*),
            "sha384" => $func::<Sha384, $($trait_arg1)?>($($arg),*),
            "sha512" => $func::<Sha512, $($trait_arg1)?>($($arg),*),
            "sha3-224" => $func::<Sha3_224, $($trait_arg1)?>($($arg),*),
            "sha3-256" => $func::<Sha3_256, $($trait_arg1)?>($($arg),*),
            "sha3-384" => $func::<Sha3_384, $($trait_arg1)?>($($arg),*),
            "sha3-512" => $func::<Sha3_512, $($trait_arg1)?>($($arg),*),
            "keccak224" => $func::<Keccak224, $($trait_arg1)?>($($arg),*),
            "keccak256" => $func::<Keccak256, $($trait_arg1)?>($($arg),*),
            "keccak384" => $func::<Keccak384, $($trait_arg1)?>($($arg),*),
            "keccak512" => $func::<Keccak512, $($trait_arg1)?>($($arg),*),
            "ripemd160" => $func::<Ripemd160, $($trait_arg1)?>($($arg),*),
            "ripemd256" => $func::<Ripemd256, $($trait_arg1)?>($($arg),*),
            "ripemd320" => $func::<Ripemd320, $($trait_arg1)?>($($arg),*),
            _ => return Err(HashError::InvalidHashType($hash_type.to_string()).into())
        }
    }
}

#[macro_export]
macro_rules! switch_hmac_trait {
    ( $func:ident, $hash_type:expr, $($trait_arg1: expr)?, $($arg: expr),* ) => {
        match $hash_type {
            "md5" => $func::<Hmac<Md5>, $($trait_arg1)?>($($arg),*),
            "md4" => $func::<Hmac<Md4>, $($trait_arg1)?>($($arg),*),
            "sha1" => $func::<Hmac<Sha1>, $($trait_arg1)?>($($arg),*),
            "sha224" => $func::<Hmac<Sha224>, $($trait_arg1)?>($($arg),*),
            "sha256" => $func::<Hmac<Sha256>, $($trait_arg1)?>($($arg),*),
            "sha384" => $func::<Hmac<Sha384>, $($trait_arg1)?>($($arg),*),
            "sha512" => $func::<Hmac<Sha512>, $($trait_arg1)?>($($arg),*),
            "sha3-224" => $func::<Hmac<Sha3_224>, $($trait_arg1)?>($($arg),*),
            "sha3-256" => $func::<Hmac<Sha3_256>, $($trait_arg1)?>($($arg),*),
            "sha3-384" => $func::<Hmac<Sha3_384>, $($trait_arg1)?>($($arg),*),
            "sha3-512" => $func::<Hmac<Sha3_512>, $($trait_arg1)?>($($arg),*),
            "keccak224" => $func::<Hmac<Keccak224>, $($trait_arg1)?>($($arg),*),
            "keccak256" => $func::<Hmac<Keccak256>, $($trait_arg1)?>($($arg),*),
            "keccak384" => $func::<Hmac<Keccak384>, $($trait_arg1)?>($($arg),*),
            "keccak512" => $func::<Hmac<Keccak512>, $($trait_arg1)?>($($arg),*),
            "ripemd160" => $func::<Hmac<Ripemd160>, $($trait_arg1)?>($($arg),*),
            "ripemd256" => $func::<Hmac<Ripemd256>, $($trait_arg1)?>($($arg),*),
            "ripemd320" => $func::<Hmac<Ripemd320>, $($trait_arg1)?>($($arg),*),
            _ => return Err(HashError::InvalidHashType($hash_type.to_string()).into())
        }
    }
}