pub mod nativelibrary;
use jni::{
    sys::{ jint, JNI_VERSION_1_6},
    JavaVM,
};
use std::{ os::raw::c_void, panic::catch_unwind };

#[macro_use]
extern crate log;

const INVALID_JNI_VERSION: jint = 0;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn JNI_OnLoad(vm: JavaVM, _: *mut c_void) -> jint {
    android_log::init("libmain_rs").unwrap();

    // unnecessary for now
    // let mut env: JNIEnv = vm.get_env().expect("Cannot get reference to the JNIEnv");
    vm.attach_current_thread()
        .expect("Unable to attach current thread to the JVM");

    info!("JNI initialized!");
    
    catch_unwind(|| JNI_VERSION_1_6).unwrap_or(INVALID_JNI_VERSION)
}