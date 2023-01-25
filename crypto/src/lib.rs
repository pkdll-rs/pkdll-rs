#![feature(try_trait_v2)]
#![feature(exhaustive_patterns)]

mod dllmain;
mod handlers;
mod imp;
mod utils;
mod wstring;

use std::sync::atomic::AtomicBool;

pub use crate::utils::*;

pub const ERR: &str = "ERR|";
pub const DEBUG: bool = option_env!("DLL_DEBUG").is_some();

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

pub static CONSOLE_OPEN: AtomicBool = AtomicBool::new(false);
