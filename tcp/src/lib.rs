use std::{
    collections::BTreeMap, io, net::TcpStream, sync::RwLock, thread::JoinHandle, time::Duration,
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

use native_tls::TlsStream;
use utils::{error::GlobalError, statuses::Task};

pub use crate::utils::*;

pub const ERR: &str = "ERR|";

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[macro_use]
extern crate lazy_static;

use std::io::{Read as IoRead, Write as IoWrite};

pub trait SetTimeout {
    fn set_read_timeout(&self, dur: Option<Duration>) -> io::Result<()>;
    fn set_write_timeout(&self, dur: Option<Duration>) -> io::Result<()>;
}

pub trait ReadAndWrite: IoRead + IoWrite + Send + Sync + SetTimeout {}

impl<T: IoRead + IoWrite + Send + Sync + SetTimeout> ReadAndWrite for T {}

impl SetTimeout for TlsStream<TcpStream> {
    fn set_read_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.get_ref().set_read_timeout(dur)
    }

    fn set_write_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.get_ref().set_write_timeout(dur)
    }
}

impl SetTimeout for TcpStream {
    fn set_read_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.set_read_timeout(dur)
    }

    fn set_write_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.set_read_timeout(dur)
    }
}

pub struct ThreadResult {
    stream: Box<dyn ReadAndWrite>,
    buffer: Option<Vec<u8>>,
}

struct TcpThread {
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
