pub mod utils;
pub mod configuration;
pub mod mod_loading;
pub mod il2cpp;
pub mod errors;

use jni::{
    sys::{ jint, JNI_VERSION_1_6},
    JavaVM, JNIEnv,
};
use std::{ os::raw::c_void, panic::catch_unwind };

#[macro_use]
extern crate log;

const INVALID_JNI_VERSION: jint = 0;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn JNI_OnLoad(vm: JavaVM, _: *mut c_void) -> jint {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Trace)
            .with_tag("fujsu")
    );

    let mut env: JNIEnv = vm.get_env().expect("Cannot get reference to the JNIEnv");
    vm.attach_current_thread()
        .expect("Unable to attach current thread to the JVM");

    configuration::init(&mut env);

    info!("JNI initialized!");
    
    catch_unwind(|| JNI_VERSION_1_6).unwrap_or(INVALID_JNI_VERSION)
}

#[no_mangle]
fn startup() {
    il2cpp::hook::hook_init();

    info!("il2cpp hooked");

    unsafe {
        mod_loading::load_libs();
        mod_loading::call_load();
    }

    info!("mods loaded");
}
