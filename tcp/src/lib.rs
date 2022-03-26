use std::{
    collections::BTreeMap,
    sync::RwLock,
    thread::JoinHandle, net::TcpStream,
};

mod dllmain;
mod tcp;

mod utils {
    pub mod cstring;
    pub mod error;
    pub mod macros;
    pub mod proxy;
    pub mod statuses;
    pub mod tcp;
}

use utils::{error::GlobalError, statuses::Task};

pub use crate::utils::*;

pub const ERR: &str = "ERR|";

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[macro_use]
extern crate lazy_static;

use std::io::{Read as IoRead, Write as IoWrite};

pub trait ReadAndWrite: IoRead + IoWrite + Send + Sync {}

impl<T: IoRead + IoWrite + Send + Sync> ReadAndWrite for T {}

pub struct ThreadResult {
    tcp_stream: Option<TcpStream>,
    stream: Box<dyn ReadAndWrite>,
    buffer: Option<Vec<u8>>,
}

struct TcpThread {
    tcp_stream: Option<TcpStream>,
    stream: Option<Box<dyn ReadAndWrite>>,
    join_handler: Option<JoinHandle<Result<ThreadResult, GlobalError>>>,
    thread_control: thread_control::Control,
    current_task: Task,
}

lazy_static! {
    static ref CACHE: RwLock<BTreeMap<String, TcpThread>> = {
        let cache = BTreeMap::new();
        RwLock::new(cache)
    };
}
