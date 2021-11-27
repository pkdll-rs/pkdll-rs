mod json;
mod dllmain;

mod utils {
    pub mod cstring;
}

pub use crate::{
    utils::*,
};

pub const ERR: &str = "ERR|";