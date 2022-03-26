use std::{
    io::{self, Read, Write},
    net::TcpStream,
    thread::JoinHandle,
    time::Duration,
};

use native_tls::TlsStream;

use super::{error::GlobalError, statuses::Task};

pub struct ThreadResult {
    pub stream: Box<dyn ReadAndWrite>,
    pub buffer: Option<Vec<u8>>,
}

pub struct TcpThread {
    pub stream: Option<Box<dyn ReadAndWrite>>,
    pub join_handler: Option<JoinHandle<Result<ThreadResult, GlobalError>>>,
    pub thread_control: thread_control::Control,
    pub current_task: Task,
}

pub trait SetTimeout {
    fn set_read_timeout(&self, dur: Option<Duration>) -> io::Result<()>;
    fn set_write_timeout(&self, dur: Option<Duration>) -> io::Result<()>;
}

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

pub trait ReadAndWrite: Read + Write + SetTimeout + Send + Sync {}

impl<T: Read + Write + SetTimeout + Send + Sync> ReadAndWrite for T {}
