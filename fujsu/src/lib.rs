pub mod utils;
pub mod configuration;
pub mod mod_loading;
pub mod il2cpp;
pub mod errors;
pub mod perm_requester;

use jni::{
    objects::{JObject, JString}, sys::{ jint, JNI_VERSION_1_6}, JNIEnv, JavaVM
};
use std::{ os::raw::c_void, panic::catch_unwind, path::Path };

#[macro_use]
extern crate log;

const INVALID_JNI_VERSION: jint = 0;

pub static mut PACKAGE_NAME: Option<String> = None;

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
    cache_package_name(&mut env);

    if Path::new("/sdcard/fujsu/").exists() {
        perm_requester::ensure_perms(&mut env);
    }

    info!("JNI initialized!");
    
    catch_unwind(|| JNI_VERSION_1_6).unwrap_or(INVALID_JNI_VERSION)
}

#[no_mangle]
fn startup() {
    il2cpp::hook::hook_init();

    info!("il2cpp hooked");

    unsafe {
        let package_name = PACKAGE_NAME.as_ref().unwrap();
        mod_loading::load_libs(package_name);
        mod_loading::call_load();
    }

    info!("mods loaded");
}

pub fn cache_package_name(env: &mut JNIEnv) {
    let unity_class_name = "com/unity3d/player/UnityPlayer";
    let unity_class = &env
        .find_class(unity_class_name)
        .expect("Failed to find class com/unity3d/player/UnityPlayer");

    let current_activity_obj: JObject = env
        .get_static_field(unity_class, "currentActivity", "Landroid/app/Activity;")
        .expect("Failed to get static field currentActivity")
        .l()
        .unwrap();

    let package_jstr: JString = env
        .call_method(
            current_activity_obj,
            "getPackageName",
            "()Ljava/lang/String;",
            &[],
        )
        .expect("Failed to invoke getPackageName()")
        .l()
        .unwrap()
        .into();

    let package_str: String = env
        .get_string(&package_jstr)
        .expect("Failed to get string from jstring")
        .into();

    env.delete_local_ref(package_jstr)
        .expect("Failed to delete local ref");

    unsafe {
        PACKAGE_NAME = Some(package_str.clone());
    }
}