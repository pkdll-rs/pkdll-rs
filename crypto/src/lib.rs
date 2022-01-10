mod aes;
mod blowfish;
mod dllmain;
mod hash;
mod hmac;
mod kdf;
mod random;
mod rc4;
mod rsa;
mod xor;

mod utils {
    pub mod aes;
    pub mod blowfish;
    pub mod cipher;
    pub mod cstring;
    pub mod hash;
    pub mod hmac;
    pub mod kdf;
    pub mod macros;
    pub mod random;
    pub mod rsa;
    pub mod xor;
}

pub use crate::utils::*;

pub const ERR: &str = "ERR|";

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
