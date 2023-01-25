use std::{
    env,
    fs::OpenOptions,
    io::{self, Read, Write},
    panic,
    process::Command,
    sync::atomic::Ordering,
};

use winapi::um::{consoleapi::AllocConsole, wincon::FreeConsole};

use crate::{CONSOLE_OPEN, DEBUG};

const LOG_FILE: &str = "crypto.log";

fn wait_input() {
    let mut stdin = io::stdin();
    let _ = stdin.read(&mut [0u8]).unwrap();
}

pub fn hook_panic() {
    let default_panic = std::panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        if !CONSOLE_OPEN.load(Ordering::Relaxed) {
            unsafe {
                AllocConsole();
            }
            CONSOLE_OPEN.store(true, Ordering::Relaxed);
        }
        default_panic(panic_info);

        let mut file = match OpenOptions::new().append(true).create(true).open(LOG_FILE) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to open file for panic log: {err}");
                wait_input();
                return;
            }
        };
        if let Err(err) = writeln!(file, "{panic_info}") {
            eprintln!("Failed to write panic to file: {err}");
        }

        drop(file);

        if let Ok(dir) = env::current_dir() {
            #[allow(unused_must_use)]
            {
                Command::new("notepad").arg(dir.join(LOG_FILE)).spawn();
            }
        };

        wait_input();
        if !DEBUG && CONSOLE_OPEN.load(Ordering::Relaxed) {
            unsafe {
                FreeConsole();
            }
            CONSOLE_OPEN.store(false, Ordering::Relaxed);
        }
    }));
}
