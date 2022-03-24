use std::{
    io::{self, Write},
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

use crate::{errors::ConnectionError, proxy::Proxy};

pub fn connect(
    addr: String,
    proxy: Option<Proxy>,
    timeout: Option<Duration>,
) -> Result<TcpStream, ConnectionError> {
    let addr = match addr.to_socket_addrs()?.next() {
        Some(addr) => addr,
        None => return Err(ConnectionError::NotValidAddrA),
    };

    if let Some(proxy) = proxy {
        if let Some(creds) = proxy.creds {
            let stream = socks::Socks5Stream::connect_with_password(
                proxy.addr,
                addr,
                &creds.username,
                &creds.password,
                timeout,
            )?;

            return Ok(stream.into_inner());
        }

        let stream = socks::Socks5Stream::connect(proxy.addr, addr, timeout)?;

        return Ok(stream.into_inner());
    }

    if let Some(timeout) = timeout {
        let stream = TcpStream::connect_timeout(&addr, timeout)?;

        return Ok(stream);
    }

    let stream = TcpStream::connect(addr)?;

    Ok(stream)
}

pub fn send_data(mut tcp_stream: TcpStream, data: Vec<u8>) -> io::Result<TcpStream> {
    tcp_stream.write_all(&data)?;
    tcp_stream.flush()?;
    Ok(tcp_stream)
}
