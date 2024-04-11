use jni::{
    objects::{JClass, JString},
    sys::{jboolean, JavaVM},
    JNIEnv,
};
use std::path::PathBuf;
use std::ffi::CStr;

use crate::nativelibrary;

#[no_mangle]
fn load(env: JNIEnv, _: JClass, _: JString) -> jboolean {
    load_custom(&env);
    load_lib_unity(&env);
    return 1;
}

#[no_mangle]
fn unload(_: JNIEnv, _: JClass) {
    info!("unload");
}

fn load_custom(env: &JNIEnv) {
    let custom_lib = nativelibrary::load_lib(&PathBuf::from("libfujsu.so"), libc::RTLD_LAZY)
        .unwrap_or_else(|e| {
            error!("Failed to load libfujsu.so: {}", e.to_string());

            let dl_error = unsafe { libc::dlerror() };
            let error_message = unsafe {
                CStr::from_ptr(dl_error)
            };
            let formatted_string = error_message.to_string_lossy();
            error!("dlerror: {}", formatted_string);
            panic!();
        });

    let on_load: nativelibrary::NativeMethod<fn(*mut JavaVM, *mut libc::c_void)> = custom_lib
        .sym("JNI_OnLoad")
        .unwrap_or_else(|e| {
            error!("Failed to find JNI_OnLoad: {}", e.to_string());
            panic!();
        });

    (on_load)(env.get_java_vm().expect("msg").get_java_vm_pointer(), std::ptr::null_mut());

    let initialize: nativelibrary::NativeMethod<fn()> = custom_lib
        .sym("startup")
        .unwrap_or_else(|e| {
            info!("Failed to find Initialize: {}", e.to_string());
            panic!();
        });
        
    (initialize)();
}

fn load_lib_unity(env: &JNIEnv) {
    let unity_lib = nativelibrary::load_lib(&PathBuf::from("libunity.so"), libc::RTLD_NOW | libc::RTLD_GLOBAL)
        .expect("Couldn't load libunity!");

    let on_load: nativelibrary::NativeMethod<fn(*mut JavaVM, *mut libc::c_void)> = unity_lib
        .sym("JNI_OnLoad")
        .expect("Couldn't find JNI_OnLoad!");

    (on_load)(
        env.get_java_vm().expect("Failed to get JavaVM from JNIEnv").get_java_vm_pointer(),
        std::ptr::null_mut()
    );
}