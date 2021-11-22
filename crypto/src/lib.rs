mod aes;
mod dllmain;
mod utils {
    pub mod cstring;
    pub mod aes;
}

pub use crate::{
    utils::*,
};

pub const ERR: &str = "ERR|";