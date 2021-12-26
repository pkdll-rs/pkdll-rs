use std::{net::TcpStream, collections::HashMap, sync::Mutex};

mod dllmain;
mod tcp;

mod utils {
    pub mod cstring;
    pub mod proxy;
    pub mod macros;
    pub mod tcp;
}

pub use crate::{
    utils::*,
};

pub const ERR: &str = "ERR|";

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref CACHE: Mutex<HashMap<String, TcpStream>> = {
        let cache = HashMap::new();
        Mutex::new(cache)
    };
}