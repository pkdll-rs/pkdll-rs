mod array;
mod dllmain;
mod random;
mod range;

mod utils {
    pub mod cstring;
    pub mod macros;
}

pub use crate::utils::*;

pub const ERR: &str = "ERR|";
const DEBUG: bool = false;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
