use std::{ffi::{c_char, CStr}, ptr::null_mut, sync::RwLock};
use libc::{c_void, RTLD_LAZY};
use lazy_static::lazy_static;
use crate::{mod_loading, utils::{hooking::{self, NativeHook}, nativelibrary}};

type InitFnIl2Cpp = fn(*const c_char) -> *mut c_void;

lazy_static! {
    pub static ref INIT_HOOK: RwLock<NativeHook<InitFnIl2Cpp>> =
        RwLock::new(NativeHook::new(null_mut(), null_mut()));
}

pub fn hook_init() {
    info!("Hooking il2cpp_init...");
    let lib = nativelibrary::load_lib_with_dlerror("libil2cpp.so", RTLD_LAZY);
    let init_ptr = lib
        .sym_raw("il2cpp_init")
        .expect("Failed to get il2cpp_init pointer");

    info!("il2cpp_init: 0x{:x}", init_ptr as *const usize as usize);

    let mut init_hook = INIT_HOOK.try_write().expect("Failed to write INIT_HOOK");
    *init_hook = hooking::NativeHook::new(init_ptr, detour as *mut c_void);
    init_hook.hook().expect("Failed to hook il2cpp_init");
}

fn detour(name: *const c_char) -> *mut c_void {
    let trampoline = INIT_HOOK.try_read().expect("Failed to read INIT_HOOK");
    let domain = trampoline(name);

    info!("il2cpp domain name: {}", unsafe {CStr::from_ptr(name)}.to_str().unwrap());

    trampoline.unhook().expect("Failed to unhook il2cpp_init");

    unsafe { mod_loading::call_il2cpp_ready() };

    return domain;
}
