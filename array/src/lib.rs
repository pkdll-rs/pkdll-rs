mod array;
mod dllmain;

mod utils {
    pub mod cstring;
    pub mod array;
    pub mod macros;
}

pub use crate::{
    utils::*,
};

pub const ERR: &str = "ERR|";