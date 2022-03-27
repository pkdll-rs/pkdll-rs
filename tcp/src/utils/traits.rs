use std::{
    io::{self, Read, Write},
    net::TcpStream,
    thread::JoinHandle,
    time::{Duration, Instant}, fmt::Debug,
};

use futures_cpupool::CpuFuture;
use native_tls::TlsStream;

use crate::tcp::TTL;

use super::{error::GlobalError, statuses::Task};

#[derive(Debug)]
pub struct ThreadResult {
    pub stream: Box<dyn ReadAndWrite>,
    pub buffer: Option<Vec<u8>>,
}

#[derive(Debug)]
pub struct TcpThread {
    pub stream: Option<Box<dyn ReadAndWrite>>,
    pub join_handler: Option<CpuFuture<ThreadResult, GlobalError>>,
    pub thread_control: thread_control::Control,
    pub current_task: Task,
    pub ttl: Instant,
}

impl TcpThread {
    pub fn increase_ttl(&mut self) {
        self.ttl = Instant::now() + TTL;
    }
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

pub trait ReadAndWrite: Read + Write + SetTimeout + Send + Sync + Debug {}

impl<T: Read + Write + SetTimeout + Send + Sync + Debug> ReadAndWrite for T {}
