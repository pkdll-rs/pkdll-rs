use std::{net::SocketAddr, str::FromStr};

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
    pub fn from_pk_str(proxy: &str) -> Result<Proxy, anyhow::Error> {
        let splitted: Vec<&str> = proxy.split('|').collect();
        if splitted.len() != 2 {
            return Err(anyhow::anyhow!("not a valid proxy"));
        }

        let addr = SocketAddr::from_str(splitted[0])?;

        let splitted: Vec<&str> = splitted[1].split(':').collect();

        if splitted.len() > 3 {
            return Err(anyhow::anyhow!("not a valid proxy"));
        }
        
        let _type = match splitted[0] {
            "SOCKS4" => ProxyType::SOCKS4,
            "SOCKS5" => ProxyType::SOCKS5,
            _ => return Err(anyhow::anyhow!("unsupported proxy type: {}", splitted[0])),
        };

        let mut creds: Option<Creds> = None;

        if splitted.len() > 1 {
            let username = splitted[1].to_string();
            let password = splitted[2].to_string();

            creds = Some(Creds{username, password});
        }

        return Ok(Proxy{_type, addr, creds});
    }
}