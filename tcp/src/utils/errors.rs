use std::{io, net};

use thiserror::Error;

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

    #[error("WAIT")]
    NotYetReady,

    #[error("the connection has not created yet")]
    NotYetConnected,
}
