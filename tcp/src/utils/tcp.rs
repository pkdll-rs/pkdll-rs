use std::{
    io::{self, BufRead, BufReader, Read, Write},
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

use crate::{
    error::{ConnectionError, GlobalError},
    proxy::{self, Proxy, ProxyType},
    ThreadResult,
};

use socks::ToTargetAddr;

pub fn connect(
    target_str: String,
    proxy: Option<Proxy>,
    timeout: Option<Duration>,
    proxy_resolve: bool,
) -> Result<ThreadResult, GlobalError> {
    let stream = match proxy {
        Some(ref proxy) => {
            let mut auth = socks::Authentication::None;
            if let Some(ref creds) = proxy.creds {
                auth = socks::Authentication::Password {
                    username: &creds.username,
                    password: &creds.password,
                }
            }

            let target = match proxy_resolve {
                true => target_str.as_str().to_target_addr()?,
                false => match target_str.to_socket_addrs()?.next() {
                    Some(target) => target,
                    None => return Err(ConnectionError::NotValidAddrA.into()),
                }
                .to_target_addr()?,
            };

            match proxy._type {
                ProxyType::SOCKS4 => {
                    socks::Socks4Stream::connect(proxy.addr, target, "", timeout)?.into_inner()
                }
                ProxyType::SOCKS5 => {
                    socks::Socks5Stream::connect(proxy.addr, target, auth, timeout)?.into_inner()
                }
                ProxyType::HTTP => proxy::connect_http(proxy.addr, target, auth, timeout)?,
            }
        }

        None => {
            let target = match target_str.to_socket_addrs()?.next() {
                Some(target) => target,
                None => return Err(ConnectionError::NotValidAddrA.into()),
            };

            match timeout {
                Some(timeout) => TcpStream::connect_timeout(&target, timeout)?,
                None => TcpStream::connect(target)?,
            }
        }
    };

    Ok(ThreadResult {
        tcp_stream: stream,
        buffer: None,
    })
}

pub fn send_data(mut tcp_stream: TcpStream, data: Vec<u8>) -> io::Result<ThreadResult> {
    tcp_stream.write_all(&data)?;
    tcp_stream.flush()?;
    Ok(ThreadResult {
        tcp_stream,
        buffer: None,
    })
}

pub fn read_exact(mut tcp_stream: TcpStream, len: usize) -> io::Result<ThreadResult> {
    let mut buf = vec![0u8; len];
    tcp_stream.read_exact(&mut buf)?;
    Ok(ThreadResult {
        tcp_stream,
        buffer: Some(buf),
    })
}

pub fn read_to_end(mut tcp_stream: TcpStream) -> io::Result<ThreadResult> {
    let mut buf = Vec::new();
    if let Err(error) = tcp_stream.read_to_end(&mut buf) {
        if error.kind() != io::ErrorKind::TimedOut {
            return Err(error);
        }
    }

    Ok(ThreadResult {
        tcp_stream,
        buffer: Some(buf),
    })
}

pub fn read_until(tcp_stream: TcpStream, until: Vec<u8>) -> io::Result<ThreadResult> {
    let mut buf = Vec::new();

    if until.len() > 1 {
        buf = _read_until(BufReader::new(&tcp_stream), &until)?;
    } else {
        BufReader::new(&tcp_stream).read_until(until[0], &mut buf)?;
    }

    Ok(ThreadResult {
        tcp_stream,
        buffer: Some(buf),
    })
}

fn _read_until<R: Read>(mut stream: R, delim: &[u8]) -> io::Result<Vec<u8>> {
    let mut data = vec![];
    loop {
        let mut buf = [0u8; 1];
        let n = stream.read(&mut buf)?;
        if n != 1 {
            break;
        }
        data.push(buf[0]);
        if data.len() >= delim.len() && &data[data.len() - delim.len()..] == delim {
            return Ok(data);
        }
    }
    Ok(data)
}
