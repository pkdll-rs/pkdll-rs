use std::{
    collections::BTreeMap,
    net::TcpStream,
    sync::{Arc, RwLock},
    thread::JoinHandle,
    time::Duration,
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

pub struct ThreadResult {
    tcp_stream: TcpStream,
    buffer: Option<Vec<u8>>,
}

struct TcpThread {
    tcp_stream: Option<TcpStream>,
    join_handler: Option<JoinHandle<Result<ThreadResult, GlobalError>>>,
    thread_control: thread_control::Control,
    current_task: Task,
    timeout: Option<Duration>,
}

lazy_static! {
    static ref CACHE: Arc<RwLock<BTreeMap<String, TcpThread>>> = {
        let cache = BTreeMap::new();
        Arc::new(RwLock::new(cache))
    };
}
