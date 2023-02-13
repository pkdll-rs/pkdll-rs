#[macro_export]
macro_rules! call_with_hash_generic {
    ($top_func:ident$(::$tail_func:ident)* ($($arg: expr),*), hmac($hash_type:expr), $top_err:ident$(::$tail_err:ident)*) => {
        call_with_hash_generic!(@match_hash_str, $top_func$(::$tail_func)*($($arg),*), $hash_type, $top_err$(::$tail_err)*, use_hmac)
    };

    ($top_func:ident$(::$tail_func:ident)* ($($arg: expr),*), $hash_type:expr, $top_err:ident$(::$tail_err:ident)*) => {
        call_with_hash_generic!(@match_hash_str, $top_func$(::$tail_func)*($($arg),*), $hash_type, $top_err$(::$tail_err)*)
    };

    (@call, $top_func:ident$(::$tail_func:ident)* ($($arg: expr),*), $hash_type:path $(, $use_hmac:tt)?) => {
        $top_func$(::$tail_func)*::<call_with_hash_generic!(@get_generic, $hash_type $(, $use_hmac)?)>($($arg),*)
    };

    (@get_generic, $hash_type:path) => {
        $hash_type
    };

    (@get_generic, $hash_type:path, $_:tt) => {
        ::hmac::Hmac<$hash_type>
    };

    (@match_hash_str, $top_func:ident$(::$tail_func:ident)* ($($arg: expr),*), $hash_type:expr, $top_err:ident$(::$tail_err:ident)* $(, $use_hmac:tt)?) => {
        match $hash_type {
            "md5" => call_with_hash_generic!(@call, $top_func$(::$tail_func)* ($($arg),*), ::md5::Md5 $(, $use_hmac)?),
            "md4" => call_with_hash_generic!(@call, $top_func$(::$tail_func)* ($($arg),*), ::md4::Md4 $(, $use_hmac)?),
            "sha1" => call_with_hash_generic!(@call, $top_func$(::$tail_func)* ($($arg),*), ::sha1::Sha1 $(, $use_hmac)?),
            "sha224" => call_with_hash_generic!(@call, $top_func$(::$tail_func)* ($($arg),*), ::sha2::Sha224 $(, $use_hmac)?),
            "sha256" => call_with_hash_generic!(@call, $top_func$(::$tail_func)* ($($arg),*), ::sha2::Sha256 $(, $use_hmac)?),
            "sha384" => call_with_hash_generic!(@call, $top_func$(::$tail_func)* ($($arg),*), ::sha2::Sha384 $(, $use_hmac)?),
            "sha512" => call_with_hash_generic!(@call, $top_func$(::$tail_func)* ($($arg),*), ::sha2::Sha512 $(, $use_hmac)?),
            "sha3-224" => call_with_hash_generic!(@call, $top_func$(::$tail_func)* ($($arg),*), ::sha3::Sha3_224 $(, $use_hmac)?),
            "sha3-256" => call_with_hash_generic!(@call, $top_func$(::$tail_func)* ($($arg),*), ::sha3::Sha3_256 $(, $use_hmac)?),
            "sha3-384" => call_with_hash_generic!(@call, $top_func$(::$tail_func)* ($($arg),*), ::sha3::Sha3_384 $(, $use_hmac)?),
            "sha3-512" => call_with_hash_generic!(@call, $top_func$(::$tail_func)* ($($arg),*), ::sha3::Sha3_512 $(, $use_hmac)?),
            "keccak224" => call_with_hash_generic!(@call, $top_func$(::$tail_func)* ($($arg),*), ::sha3::Keccak256 $(, $use_hmac)?),
            "keccak256" => call_with_hash_generic!(@call, $top_func$(::$tail_func)* ($($arg),*), ::sha3::Keccak256 $(, $use_hmac)?),
            "keccak384" => call_with_hash_generic!(@call, $top_func$(::$tail_func)* ($($arg),*), ::sha3::Keccak384 $(, $use_hmac)?),
            "keccak512" => call_with_hash_generic!(@call, $top_func$(::$tail_func)* ($($arg),*), ::sha3::Keccak512 $(, $use_hmac)?),
            "ripemd128" => call_with_hash_generic!(@call, $top_func$(::$tail_func)* ($($arg),*), ::ripemd::Ripemd128 $(, $use_hmac)?),
            "ripemd160" => call_with_hash_generic!(@call, $top_func$(::$tail_func)* ($($arg),*), ::ripemd::Ripemd160 $(, $use_hmac)?),
            "ripemd256" => call_with_hash_generic!(@call, $top_func$(::$tail_func)* ($($arg),*), ::ripemd::Ripemd256 $(, $use_hmac)?),
            "ripemd320" => call_with_hash_generic!(@call, $top_func$(::$tail_func)* ($($arg),*), ::ripemd::Ripemd320 $(, $use_hmac)?),
            _ => return ::core::result::Result::Err($top_err$(::$tail_err)*($hash_type.to_owned())),
        }
    };
}
