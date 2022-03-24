use crate::errors::ProxyError;
use std::net::{SocketAddr, ToSocketAddrs};

pub enum ProxyType {
    SOCKS4,
    SOCKS5,
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
