use std::{
    io::{self, BufRead, BufReader, Read, Write},
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

use crate::{
    error::{ConnectionError, GlobalError},
    proxy::{self, Proxy, ProxyType},
    ReadAndWrite, ThreadResult,
};

use native_tls::TlsConnector;
use socks::ToTargetAddr;

pub fn connect(
    target_str: String,
    proxy: Option<Proxy>,
    timeout: Option<Duration>,
    proxy_resolve: bool,
    use_tls: bool,
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

    stream.set_read_timeout(timeout)?;
    stream.set_write_timeout(timeout)?;

    if use_tls {
        let connector = TlsConnector::builder()
            .danger_accept_invalid_certs(true)
            .danger_accept_invalid_hostnames(true)
            .use_sni(false)
            .build()?;

        let tls_stream = connector.connect("", stream.try_clone()?)?;
        return Ok(ThreadResult {
            tcp_stream: Some(stream),
            stream: Box::new(tls_stream),
            buffer: None,
        });
    }

    Ok(ThreadResult {
        tcp_stream: Some(stream.try_clone()?),
        stream: Box::new(stream),
        buffer: None,
    })
}

pub fn send_data(mut stream: Box<dyn ReadAndWrite>, data: Vec<u8>) -> io::Result<ThreadResult> {
    stream.write_all(&data)?;
    stream.flush()?;
    Ok(ThreadResult {
        tcp_stream: None,
        stream,
        buffer: None,
    })
}

pub fn read_exact(mut stream: Box<dyn ReadAndWrite>, len: usize) -> io::Result<ThreadResult> {
    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf)?;
    Ok(ThreadResult {
        tcp_stream: None,
        stream,
        buffer: Some(buf),
    })
}

pub fn read_to_end(mut stream: Box<dyn ReadAndWrite>) -> io::Result<ThreadResult> {
    let mut buf = Vec::new();
    if let Err(error) = stream.read_to_end(&mut buf) {
        if error.kind() != io::ErrorKind::TimedOut {
            return Err(error);
        }
    }

    Ok(ThreadResult {
        tcp_stream: None,
        stream,
        buffer: Some(buf),
    })
}

pub fn read_until(mut stream: Box<dyn ReadAndWrite>, until: Vec<u8>) -> io::Result<ThreadResult> {
    let mut buf = Vec::new();

    if until.len() > 1 {
        buf = _read_until(BufReader::new(&mut stream), &until)?;
    } else {
        BufReader::new(&mut stream).read_until(until[0], &mut buf)?;
    }

    Ok(ThreadResult {
        tcp_stream: None,
        stream,
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
