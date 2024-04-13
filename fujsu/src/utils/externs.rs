use libc::c_void;

use super::hooking;

#[no_mangle]
pub extern "C" fn hook(target: *mut c_void, detour: *mut c_void) -> *mut c_void {
    let trampoline = hooking::hook(target as usize, detour as usize);
    if trampoline.is_err() {
        error!("Failed to hook: {:?}", trampoline.err().unwrap());
        panic!();
    }
    return trampoline.unwrap() as *mut c_void;
}

#[no_mangle]
pub extern "C" fn unhook(target: *mut c_void) {
    hooking::unhook(target as usize).expect("Failed to unhook");
}