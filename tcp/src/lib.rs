mod dllmain;
mod tcp;

mod utils {
    pub mod cstring;
    pub mod error;
    pub mod macros;
    pub mod proxy;
    pub mod statuses;
    pub mod tcp;
    pub mod traits;
}

use crate::{statuses::Task, traits::TcpThread, utils::*};
use std::{collections::BTreeMap, sync::RwLock};

pub const ERR: &str = "ERR|";

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref CACHE: RwLock<BTreeMap<String, TcpThread>> = {
        let cache = BTreeMap::new();
        RwLock::new(cache)
    };
}
