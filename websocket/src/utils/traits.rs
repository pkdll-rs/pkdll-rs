use std::{
    fmt::Debug,
    io::{self, Read, Write},
    net::TcpStream,
    time::{Duration, Instant},
};

use crossbeam_channel::Receiver;
use tungstenite::{stream::MaybeTlsStream, WebSocket, http::Response};

use crate::websocket::TTL;

use super::{error::GlobalError, statuses::Task};

#[derive(Debug)]
pub struct ThreadResult {
    pub stream: WebSocket<MaybeTlsStream<TcpStream>>,
    pub resp: Option<Response<()>>,
    pub buffer: Option<Vec<u8>>,
}

#[derive(Debug)]
pub struct TcpThread {
    pub stream: Option<WebSocket<MaybeTlsStream<TcpStream>>>,
    pub join_handler: Option<Receiver<Result<ThreadResult, GlobalError>>>,
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

impl SetTimeout for WebSocket<MaybeTlsStream<TcpStream>> {
    fn set_read_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        match self.get_ref() {
            MaybeTlsStream::Plain(s) => {
                s.set_read_timeout(dur)
            },
            MaybeTlsStream::NativeTls(s) => {
                s.get_ref().set_read_timeout(dur)
            },
            _ => unreachable!("no rustls"),
        }
    }

    fn set_write_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        match self.get_ref() {
            MaybeTlsStream::Plain(s) => {
                s.set_write_timeout(dur)
            },
            MaybeTlsStream::NativeTls(s) => {
                s.get_ref().set_write_timeout(dur)
            },
            _ => unreachable!("no rustls"),
        }
    }
}