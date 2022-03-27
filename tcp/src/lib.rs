mod dllmain;
mod tcp;

mod utils {
    pub mod cstring;
    pub mod error;
    pub mod macros;
    pub mod proxy;
    pub mod statuses;
    pub mod tcp;
    pub mod traits;
}

use once_cell::sync::Lazy;

use crate::{statuses::Task, traits::TcpThread, utils::*};
use std::{collections::BTreeMap, sync::{RwLock, Arc}, time::{Instant, Duration}, thread::{self, JoinHandle}};

pub const ERR: &str = "ERR|";

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[macro_use]
extern crate lazy_static;

use futures::Future;
use futures_cpupool::CpuPool;

static THREAD_POOL: Lazy<CpuPool> = Lazy::new(|| {
    let pool = CpuPool::new(50);
    pool
});

static CACHE: Lazy<Arc<RwLock<BTreeMap<String, TcpThread>>>> = Lazy::new(|| {
    let cache = BTreeMap::new();
    let cache = Arc::new(RwLock::new(cache));
    let copy = Arc::clone(&cache);

    /*thread::spawn(move || {
        let copy = copy;
        loop {
            cleanup(&copy);
            thread::sleep(Duration::from_secs(40))
        }
    });*/

    cache
});

/*fn cleanup(cache: &Arc<RwLock<BTreeMap<String, TcpThread>>>) {
    let mut w = cache.write().unwrap();
    w.retain(|_, v| {
        if v.ttl < Instant::now() {
            if let Some(handler) = v.join_handler.take() {
                println!("Joining: {:?}, {:?}", v, handler);
                if !v.thread_control.is_done() {
                    println!("Not finished");
                    return true;
                }
                handler.join();
                println!("Joined");
            }
            println!("Removed");
            return false;
        }
        true
    });
    println!("{}", w.len());
}*/