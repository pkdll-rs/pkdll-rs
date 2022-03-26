use socks::{Authentication, ToTargetAddr};

use crate::error::{self, ProxyError};
use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream, ToSocketAddrs},
    time::Duration,
};

pub enum ProxyType {
    SOCKS4,
    SOCKS5,
    Http,
}

pub struct Creds {
    pub username: String,
    pub password: String,
}

pub struct Proxy {
    pub _type: ProxyType,
    pub addr: SocketAddr,
    pub creds: Option<Creds>,
}

// ip:port|type:user:password
impl Proxy {
    pub fn from_pk_str(proxy: String) -> Result<Proxy, ProxyError> {
        let splitted: Vec<&str> = proxy.split('|').collect();
        if splitted.len() != 2 {
            return Err(ProxyError::NotValidProxy);
        }

        let addr = match splitted[0].to_socket_addrs()?.next() {
            Some(addr) => addr,
            None => return Err(ProxyError::NotValidAddrA),
        };

        let splitted: Vec<&str> = splitted[1].split(':').collect();

        if splitted.len() > 3 {
            return Err(ProxyError::NotValidProxy);
        }

        let _type = match splitted[0] {
            "SOCKS4" => ProxyType::SOCKS4,
            "SOCKS5" => ProxyType::SOCKS5,
            "HTTPS" => ProxyType::Http,
            _ => return Err(ProxyError::UnsupportedType(splitted[0].to_owned())),
        };

        let mut creds: Option<Creds> = None;

        if splitted.len() > 1 {
            let username = splitted[1].to_owned();
            let password = splitted[2].to_owned();

            creds = Some(Creds { username, password });
        }

        Ok(Proxy { _type, addr, creds })
    }
}

pub fn connect_http<T, U>(
    proxy: T,
    target: U,
    auth: Authentication,
    timeout: Option<Duration>,
) -> Result<TcpStream, error::GlobalError>
where
    T: ToSocketAddrs,
    U: ToTargetAddr,
{
    let mut socket = if let Some(timeout) = timeout {
        let addr = proxy.to_socket_addrs().unwrap().next().unwrap();
        TcpStream::connect_timeout(&addr, timeout)?
    } else {
        TcpStream::connect(proxy)?
    };

    let target = match target.to_target_addr()? {
        socks::TargetAddr::Ip(target) => (target.ip().to_string(), target.port().to_string()),
        socks::TargetAddr::Domain(domain, port) => (domain, port.to_string()),
    };

    let auth = match auth {
        Authentication::None => "".to_owned(),
        Authentication::Password { username, password } => {
            let creds = base64::encode(&format!("{}:{}", username, password));

            format!("Proxy-Authorization: basic {}\r\n", creds)
        }
    };

    let connect_req = format!(
        "CONNECT {}:{} HTTP/1.1\r\n\
Host: {}:{}\r\n\
User-Agent: something/1.0.0\r\n\
Proxy-Connection: Keep-Alive\r\n\
{}\
\r\n",
        target.0, target.1, target.0, target.1, auth,
    );

    socket.set_read_timeout(timeout)?;
    socket.set_write_timeout(timeout)?;

    socket.write_all(connect_req.as_bytes())?;
    let mut proxy_response = Vec::new();

    loop {
        let mut buf = vec![0; 256];
        let total = socket.read(&mut buf)?;
        proxy_response.append(&mut buf);
        if total < 256 {
            break;
        }
    }

    let response_string = String::from_utf8_lossy(&proxy_response);
    let top_line = response_string
        .lines()
        .next()
        .ok_or(error::ProxyError::ProxyConnect)?;
    let status_code = top_line
        .split_whitespace()
        .nth(1)
        .ok_or(error::ProxyError::ProxyConnect)?;

    match status_code {
        "200" => Ok(socket),
        "401" | "407" => Err(error::ProxyError::ProxyUnauthorized.into()),
        _ => Err(error::ProxyError::ProxyConnect.into()),
    }
}
