mod dllmain;
mod time;

mod utils {
    pub mod cstring;
    pub mod macros;
}

pub use crate::utils::*;

pub const ERR: &str = "ERR|";
pub const DEBUG: bool = false;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
