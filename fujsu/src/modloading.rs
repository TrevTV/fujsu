use libc::RTLD_LAZY;

use crate::nativelibrary::{NativeLibrary, NativeMethod};

pub static mut MODS: Vec<NativeLibrary> = Vec::new();

pub unsafe fn load_libs() {
    info!("Loading mod libraries...");
    for lib in crate::configuration::MOD_LIBS.iter() {
        info!("Loading: {}", lib);
        let lib = crate::nativelibrary::load_lib_with_dlerror(lib, RTLD_LAZY);
        MODS.push(lib);
    }
}

pub unsafe fn call_load() {
    for lib in MODS.iter() {
        let handle: NativeMethod<fn()> = lib.sym("load").expect("Failed to find load function");
        handle();
    }
}

pub unsafe fn call_il2cpp_ready() {
    for lib in MODS.iter() {
        let handle: NativeMethod<fn()> = lib
            .sym("il2cpp_ready")
            .expect("Failed to find il2cpp_ready function");
        handle();
    }
}
