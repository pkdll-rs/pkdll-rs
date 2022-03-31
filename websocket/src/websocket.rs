use crossbeam_channel::{Receiver as _Receiver, Sender as _Sender};
use std::time::{Duration, Instant};
use tungstenite::protocol::CloseFrame;
use tungstenite::Message;
use url::Url;

use crossbeam_channel::bounded;
use uuid::Uuid;
use winapi::um::winnt::LPCWSTR;

use crate::debug;
use crate::utils::traits::SetTimeout;
use crate::{
    cstring,
    error::{DllError, GlobalError},
    proxy::Proxy,
    statuses::DllStatus,
    traits::ThreadResult,
    unwrap_or_err,
    utils::websocket,
    Task, TcpThread, CACHE, DEBUG, THREAD_POOL,
};

pub const TTL: Duration = Duration::from_secs(30);

type Sender = _Sender<Result<ThreadResult, GlobalError>>;
type Receiver = _Receiver<Result<ThreadResult, GlobalError>>;

#[no_mangle]
pub extern "stdcall" fn connect_ip(
    url_ptr: LPCWSTR,
    proxy_addr_ptr: LPCWSTR,
    timeout_ptr: LPCWSTR,
    proxy_resolve_ptr: LPCWSTR,
) -> LPCWSTR {
    let url = cstring::from_widechar_ptr(url_ptr);
    let url = unwrap_or_err!(Url::parse(&url));
    let proxy_addr = cstring::from_widechar_ptr(proxy_addr_ptr);
    let timeout = cstring::from_widechar_ptr(timeout_ptr);
    let proxy_resolve: bool = cstring::from_widechar_ptr(proxy_resolve_ptr)
        .parse()
        .unwrap_or_default();

    let timeout: u64 = unwrap_or_err!(timeout.parse());
    let timeout = match timeout {
        0 => None,
        _ => Some(Duration::from_millis(timeout)),
    };

    let mut proxy: Option<Proxy> = None;

    if proxy_addr != ":" {
        proxy = Some(unwrap_or_err!(Proxy::from_pk_str(proxy_addr)));
    }

    debug!(
        "[Connect] URL: {}, proxy: {:?}, timeout: {:?}, proxy_resolve: {}",
        url, proxy, timeout, proxy_resolve
    );

    let (flag, control) = thread_control::make_pair();
    let (sender, recv): (Sender, Receiver) = bounded(1);

    THREAD_POOL.lock().unwrap().execute(move || {
        flag.alive();
        debug!("After alive");
        let result =
            websocket::connect(url, proxy, timeout, proxy_resolve).map_err(GlobalError::from);
        debug!("After result: {:?}", result);
        let result = sender.send(result);
        debug!("Sent: {:?}", result);
    });

    debug!("After spawn");

    let tcp_thread = TcpThread {
        stream: None,
        join_handler: Some(recv),
        thread_control: control,
        current_task: Task::Connect,
        ttl: Instant::now() + TTL,
    };

    let uuid = Uuid::new_v4().to_hyphenated().to_string();

    let mut w = unwrap_or_err!(CACHE.write());
    w.insert(uuid.clone(), tcp_thread);

    debug!("Uuid: {}", uuid);

    cstring::to_widechar_ptr(uuid)
}

#[no_mangle]
pub extern "stdcall" fn send_message(
    uuid_ptr: LPCWSTR,
    message_type_ptr: LPCWSTR,
    data_ptr: LPCWSTR,
) -> LPCWSTR {
    let uuid = cstring::from_widechar_ptr(uuid_ptr);
    if let Err(error) = stream_exists(&uuid) {
        return cstring::to_widechar_ptr(error.to_string());
    }

    let message_type = cstring::from_widechar_ptr(message_type_ptr);

    let data = cstring::from_widechar_ptr(data_ptr);

    let message = match message_type.as_str() {
        "text" => Message::Text(data),
        "binary" => Message::Binary(unwrap_or_err!(base64::decode(data))),
        message_type => {
            return cstring::to_widechar_ptr(
                DllError::BadMessageType(message_type.to_owned()).to_string(),
            )
        }
    };

    let mut w = unwrap_or_err!(CACHE.write());
    let mut tcp_thread = match w.get_mut(&uuid) {
        Some(tcp_thread) => tcp_thread,
        None => return cstring::to_widechar_ptr(DllError::ConnectionNotFound.to_string()),
    };

    tcp_thread.increase_ttl();

    let stream = tcp_thread.stream.take().unwrap();

    debug!("[Send] uuid: {}, message: {}", uuid, message);

    let (flag, control) = thread_control::make_pair();

    let (sender, recv): (Sender, Receiver) = bounded(1);

    THREAD_POOL.lock().unwrap().execute(move || {
        flag.alive();
        let result = websocket::send_message(stream, message).map_err(GlobalError::from);
        let result = sender.send(result);
        debug!("Sent: {:?}", result);
    });

    tcp_thread.thread_control = control;
    tcp_thread.join_handler = Some(recv);
    tcp_thread.current_task = Task::SendMessage;

    cstring::to_widechar_ptr(DllStatus::ThreadSpawned.as_str())
}

#[no_mangle]
pub extern "stdcall" fn read_message(uuid_ptr: LPCWSTR) -> LPCWSTR {
    let uuid = cstring::from_widechar_ptr(uuid_ptr);
    if let Err(error) = stream_exists(&uuid) {
        return cstring::to_widechar_ptr(error.to_string());
    }

    let mut w = unwrap_or_err!(CACHE.write());
    let mut tcp_thread = match w.get_mut(&uuid) {
        Some(tcp_thread) => tcp_thread,
        None => return cstring::to_widechar_ptr(DllError::ConnectionNotFound.to_string()),
    };

    tcp_thread.increase_ttl();

    let stream = tcp_thread.stream.take().unwrap();

    debug!("[ReadMessage] uuid: {}", uuid);

    let (flag, control) = thread_control::make_pair();

    let (sender, recv): (Sender, Receiver) = bounded(1);

    THREAD_POOL.lock().unwrap().execute(move || {
        flag.alive();
        let result = websocket::read_message(stream).map_err(GlobalError::from);
        let result = sender.send(result);
        debug!("Sent: {:?}", result);
    });

    tcp_thread.thread_control = control;
    tcp_thread.join_handler = Some(recv);
    tcp_thread.current_task = Task::ReadMessage;

    cstring::to_widechar_ptr(DllStatus::ThreadSpawned.as_str())
}

#[no_mangle]
pub extern "stdcall" fn disconnect(
    uuid_ptr: LPCWSTR,
    code_ptr: LPCWSTR,
    reason_ptr: LPCWSTR,
) -> LPCWSTR {
    let uuid = cstring::from_widechar_ptr(uuid_ptr);

    let code = cstring::from_widechar_ptr(code_ptr);

    let reason = cstring::from_widechar_ptr(reason_ptr);

    let code: Option<CloseFrame> = match code.parse::<u16>() {
        Ok(code) => Some(CloseFrame {
            code: code.into(),
            reason: reason.into(),
        }),
        Err(_) => None,
    };

    debug!("[Disconnect] uuid: {}", uuid);

    let mut w = unwrap_or_err!(CACHE.write());
    let tcp_thread = match w.get_mut(&uuid) {
        Some(tcp_thread) => tcp_thread,
        None => return cstring::to_widechar_ptr(DllError::ConnectionNotFound.to_string()),
    };

    if let Some(mut stream) = tcp_thread.stream.take() {
        let result = stream.close(code);
        debug!("Close result: {:?}", result);
        unwrap_or_err!(result);
    }

    match w.remove(&uuid) {
        Some(_) => cstring::to_widechar_ptr(DllStatus::Ok.as_str()),
        None => cstring::to_widechar_ptr(DllError::ConnectionNotFound.to_string()),
    }
}

#[no_mangle]
pub extern "stdcall" fn task_status(uuid_ptr: LPCWSTR) -> LPCWSTR {
    let uuid = cstring::from_widechar_ptr(uuid_ptr);

    if let Err(error) = is_task_running(&uuid) {
        return cstring::to_widechar_ptr(error.to_string());
    }

    let mut r = unwrap_or_err!(CACHE.write());
    let mut tcp_thread = match r.get_mut(&uuid) {
        Some(stream) => stream,
        None => return cstring::to_widechar_ptr(DllError::ConnectionNotFound.to_string()),
    };

    if !tcp_thread.thread_control.is_done() {
        return cstring::to_widechar_ptr(DllStatus::NotYetReady.as_str());
    }

    tcp_thread.increase_ttl();

    let thread_result = unwrap_or_err!(tcp_thread.join_handler.take().unwrap().recv().unwrap());

    tcp_thread.stream = Some(thread_result.stream);

    debug!("[TaskStatus] uuid: {}, tcp_thread: {:?}", uuid, tcp_thread);

    if let Task::ReadMessage = tcp_thread.current_task {
        let message = thread_result.buffer.unwrap();
        let message = match message {
            Message::Text(text) => "TEXT|".to_owned() + &text,
            Message::Binary(bin) => "BINARY|".to_owned() + &base64::encode(bin),
            _ => message.to_string(),
        };

        return cstring::to_widechar_ptr(message);
    }

    cstring::to_widechar_ptr(tcp_thread.current_task.as_str())
}

#[no_mangle]
pub extern "stdcall" fn set_read_timeout(uuid_ptr: LPCWSTR, timeout_ptr: LPCWSTR) -> LPCWSTR {
    let uuid = cstring::from_widechar_ptr(uuid_ptr);
    let timeout = cstring::from_widechar_ptr(timeout_ptr);

    let timeout = match unwrap_or_err!(timeout.parse::<u64>()) {
        0 => None,
        timeout => Some(Duration::from_millis(timeout)),
    };

    let r = unwrap_or_err!(CACHE.read());
    let tcp_thread = match r.get(&uuid) {
        Some(tcp_thread) => tcp_thread,
        None => return cstring::to_widechar_ptr(DllError::ConnectionNotFound.to_string()),
    };

    if tcp_thread.stream.is_none() {
        return cstring::to_widechar_ptr(DllError::NoTcpStream.to_string());
    }

    debug!("[SetReadTimeout] uuid: {}, timeout: {:?}", uuid, timeout);

    unwrap_or_err!(tcp_thread
        .stream
        .as_ref()
        .unwrap()
        .set_read_timeout(timeout));
    cstring::to_widechar_ptr("OK")
}

#[no_mangle]
pub extern "stdcall" fn set_write_timeout(uuid_ptr: LPCWSTR, timeout_ptr: LPCWSTR) -> LPCWSTR {
    let uuid = cstring::from_widechar_ptr(uuid_ptr);
    let timeout = cstring::from_widechar_ptr(timeout_ptr);

    let timeout = match unwrap_or_err!(timeout.parse::<u64>()) {
        0 => None,
        timeout => Some(Duration::from_millis(timeout)),
    };

    let r = unwrap_or_err!(CACHE.read());
    let stream = match r.get(&uuid) {
        Some(stream) => stream,
        None => return cstring::to_widechar_ptr(DllError::ConnectionNotFound.to_string()),
    };

    if stream.stream.is_none() {
        return cstring::to_widechar_ptr(DllError::NoTcpStream.to_string());
    }

    debug!("[SetWriteTimeout] uuid: {}, timeout: {:?}", uuid, timeout);

    unwrap_or_err!(stream.stream.as_ref().unwrap().set_write_timeout(timeout));
    cstring::to_widechar_ptr("OK")
}

fn is_task_running(uuid: &str) -> Result<(), DllError> {
    let has_join_handler = {
        let r = CACHE.read().unwrap();
        let tcp_thread = match r.get(uuid) {
            Some(tcp_thread) => tcp_thread,
            None => return Err(DllError::ConnectionNotFound),
        };

        tcp_thread.join_handler.is_some()
    };

    if !has_join_handler {
        return Err(DllError::NoTaskRunning);
    }

    Ok(())
}

fn stream_exists(uuid: &str) -> Result<(), DllError> {
    let has_stream = {
        let r = CACHE.read().unwrap();
        let tcp_thread = match r.get(uuid) {
            Some(tcp_thread) => tcp_thread,
            None => return Err(DllError::ConnectionNotFound),
        };

        tcp_thread.stream.is_some()
    };

    if !has_stream {
        return Err(DllError::NoTcpStream);
    }

    Ok(())
}
