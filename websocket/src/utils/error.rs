use std::{
    io,
    net::{self, TcpStream},
};

use thiserror::Error;
use tungstenite::{stream::MaybeTlsStream, ClientHandshake};

#[derive(Debug, Error)]
pub enum ProxyError {
    #[error("not a valid proxy")]
    NotValidProxy,

    #[error("unsupported proxy type: {0}")]
    UnsupportedType(String),

    #[error("not a valid addr")]
    NotValidAddr(#[from] net::AddrParseError),

    #[error("not a valid addr")]
    NotValidAddrA,

    #[error("proxy failed to connect")]
    ProxyConnect,

    #[error("provided proxy credentials are incorrect")]
    ProxyUnauthorized,

    #[error(transparent)]
    ConnectionError(#[from] io::Error),
}

#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error(transparent)]
    IOError(#[from] io::Error),

    #[error("not a valid target")]
    NotValidAddr(#[from] net::AddrParseError),

    #[error("not a valid target")]
    NotValidAddrA,
}

#[derive(Debug, Error)]
pub enum DllError {
    #[error("ERR|connection not found")]
    ConnectionNotFound,

    #[error("ERR|no active task")]
    NoTaskRunning,

    #[error(
        "ERR|no tcp stream (either certain task is running or connection has not created yet)"
    )]
    NoTcpStream,

    #[error("ERR|unsupported message type: {0}")]
    BadMessageType(String),
}

#[derive(Debug, Error)]
pub enum GlobalError {
    #[error(transparent)]
    IO(#[from] io::Error),

    #[error(transparent)]
    Connection(#[from] ConnectionError),

    #[error(transparent)]
    Proxy(#[from] ProxyError),

    #[error(transparent)]
    Tls(#[from] native_tls::Error),

    #[error(transparent)]
    Handshake(#[from] native_tls::HandshakeError<TcpStream>),

    #[error(transparent)]
    WSHandshake(#[from] tungstenite::HandshakeError<ClientHandshake<MaybeTlsStream<TcpStream>>>),

    #[error(transparent)]
    WS(#[from] tungstenite::Error),
}
