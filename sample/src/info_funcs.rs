use wchar::{wchz};
use winapi::um::consoleapi::AllocConsole;

const AUTHOR: &[u16] = wchz!("_Skill_");
const VER: &[u16] = wchz!("0.1");
const DESC: &[u16] = wchz!("desc");
const DEBUG: bool = true;

#[no_mangle]
pub extern "stdcall" fn info_getAuthor() ->  *const u16 {
    if DEBUG {
        unsafe {
            AllocConsole();
        }
    }
    return AUTHOR.as_ptr();
}

#[no_mangle]
pub extern "stdcall" fn info_getVersion() ->  *const u16 {
    return VER.as_ptr();
}

#[no_mangle]
pub extern "stdcall" fn info_getDescription() ->  *const u16 {
    return DESC.as_ptr();
}
