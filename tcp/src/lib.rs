use std::{
    collections::HashMap,
    io,
    net::TcpStream,
    sync::{Arc, RwLock},
    thread::JoinHandle,
};

mod dllmain;
mod tcp;

mod utils {
    pub mod cstring;
    pub mod errors;
    pub mod macros;
    pub mod proxy;
    pub mod tcp;
}

use errors::ConnectionError;

pub use crate::utils::*;

pub const ERR: &str = "ERR|";

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[macro_use]
extern crate lazy_static;

struct TcpThread {
    tcp_stream: Option<TcpStream>,
    join_handler_connect: Option<JoinHandle<Result<TcpStream, ConnectionError>>>,
    join_handler_write: Option<JoinHandle<io::Result<TcpStream>>>,
    thread_control: thread_control::Control,
}

lazy_static! {
    static ref CACHE: Arc<RwLock<HashMap<String, TcpThread>>> = {
        let cache = HashMap::new();
        Arc::new(RwLock::new(cache))
    };
}
