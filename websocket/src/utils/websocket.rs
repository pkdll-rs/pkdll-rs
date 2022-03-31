use std::{
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

use crate::{
    error::{ConnectionError, GlobalError},
    proxy::{self, Proxy, ProxyType},
    traits::ThreadResult,
};

use native_tls::TlsConnector;
use socks::ToTargetAddr;
use tungstenite::{
    client_tls_with_config, stream::MaybeTlsStream, Connector, Message, Result, WebSocket,
};
use url::Url;

pub fn connect(
    url: Url,
    proxy: Option<Proxy>,
    timeout: Option<Duration>,
    proxy_resolve: bool,
) -> Result<ThreadResult, GlobalError> {
    let host_port = (
        url.host_str().unwrap_or_default(),
        url.port_or_known_default().unwrap_or_default(),
    );

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
                true => host_port.to_target_addr()?,
                false => match host_port.to_socket_addrs()?.next() {
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
                ProxyType::Http => proxy::connect_http(proxy.addr, target, auth, timeout)?,
            }
        }

        None => {
            let target = match host_port.to_socket_addrs()?.next() {
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

    let mut connector: Option<Connector> = None;

    if url.scheme() == "wss" {
        connector = Some(tungstenite::Connector::NativeTls(
            TlsConnector::builder()
                .danger_accept_invalid_certs(true)
                .danger_accept_invalid_hostnames(true)
                .use_sni(false)
                .build()?,
        ));
    }

    let (ws, resp) =
        client_tls_with_config(url, stream, None, connector).map_err(GlobalError::from)?;

    Ok(ThreadResult {
        stream: ws,
        resp: Some(resp),
        buffer: None,
    })
}

pub fn send_message(
    mut stream: WebSocket<MaybeTlsStream<TcpStream>>,
    message: Message,
) -> Result<ThreadResult> {
    stream.write_message(message)?;
    Ok(ThreadResult {
        stream,
        resp: None,
        buffer: None,
    })
}

pub fn read_message(mut stream: WebSocket<MaybeTlsStream<TcpStream>>) -> Result<ThreadResult> {
    let message = stream.read_message()?;
    Ok(ThreadResult {
        stream,
        resp: None,
        buffer: Some(message),
    })
}
