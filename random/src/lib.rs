mod random;
mod dllmain;

mod utils {
    pub mod cstring;
    pub mod random;
    pub mod macros;
    pub mod array;
}

pub use crate::{
    utils::*,
};

pub const ERR: &str = "ERR|";