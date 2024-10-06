use std::path::Path;

use libc::RTLD_LAZY;

use crate::utils::nativelibrary::{NativeLibrary, NativeMethod};

pub static mut MODS: Vec<NativeLibrary> = Vec::new();

pub unsafe fn load_libs(package_name: &String) {
    info!("Loading mod libraries...");
    for lib in crate::configuration::MOD_LIBS.iter() {
        info!("Loading: {}", lib);

        // check if mod so exists in filesystem directory before attempting load_lib
        let raw_file_path = format!("/sdcard/fujsu/{}", lib);
        let file_path = Path::new(raw_file_path.as_str());
        if file_path.exists() {
            info!("Local library exists for {}", lib);

            let local_path = format!("/data/data/{}/files/{}", package_name, lib);
            info!("Copying library from {} to {}", file_path.display(), local_path);
            std::fs::create_dir_all(format!("/data/data/{}/files/", package_name)).ok();
            
            let res = std::fs::copy(file_path, local_path.clone());
            if let Err(e) = res {
                error!("Failed to copy library: {}", e);
                continue;
            }

            info!("Copied library to {}", local_path);

            let lib = crate::utils::nativelibrary::load_lib_with_dlerror(local_path.clone(), RTLD_LAZY);
            MODS.push(lib);

            continue;
        }
        

        let lib = crate::utils::nativelibrary::load_lib_with_dlerror(lib, RTLD_LAZY);
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
