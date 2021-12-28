mod aws;
mod dllmain;

mod utils {
    pub mod cstring;
    pub mod macros;
    pub mod aws;
}

pub use crate::{
    utils::*,
};

pub const ERR: &str = "ERR|";
pub const DEBUG: bool = false;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;