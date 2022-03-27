#[derive(Debug)]
pub enum Task {
    Connect,
    SendData,
    RecvExact,
    RecvUntil,
    RecvEnd,
}

impl Task {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Task::Connect => "CONNECTED",
            Task::SendData => "SENT",
            Task::RecvExact | Task::RecvUntil | Task::RecvEnd => "RECEIVED",
        }
    }
}

pub enum DllStatus {
    NotYetReady,
    ThreadSpawned,
    Ok,
}

impl DllStatus {
    pub fn as_str(&self) -> &'static str {
        match *self {
            DllStatus::NotYetReady => "WAIT",
            DllStatus::ThreadSpawned => "THREAD_SPAWNED",
            DllStatus::Ok => "OK",
        }
    }
}
