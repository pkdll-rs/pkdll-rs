use std::{net::{TcpStream, SocketAddr}, str::FromStr, time::Duration};

use crate::proxy::Proxy;

pub fn connect(addr: String, proxy: Option<Proxy>, timeout: Option<Duration>) -> Result<TcpStream, anyhow::Error> {
    if let Some(proxy) = proxy {
        if let Some(creds) = proxy.creds {
            let stream = socks::Socks5Stream::connect_with_password(
                proxy.addr, 
                addr.as_ref(), 
                &creds.username, 
                &creds.password, 
                timeout,
            )?;

            return Ok(stream.into_inner());
        }

        let stream = socks::Socks5Stream::connect(
            proxy.addr, 
            addr.as_ref(), 
            timeout,
        )?;

        return Ok(stream.into_inner());
    }

    if let Some(timeout) = timeout {
        println!("{:?}", timeout);
        let stream = TcpStream::connect_timeout(
            &SocketAddr::from_str(&addr)?, 
            timeout,
        )?;

        return Ok(stream)
    }

    let stream = TcpStream::connect(
        &SocketAddr::from_str(&addr)?,
    )?;

    Ok(stream)
}