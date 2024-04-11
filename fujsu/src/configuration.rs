use std::ffi::CString;
use jni::JNIEnv;
use jni::objects::JObject;
use ndk_sys::AAssetManager;
use toml::Table;

pub static mut MOD_LIBS: Vec<String> = Vec::new();

pub fn init(env: &mut JNIEnv) {
    let config_str = unsafe {
        read_asset_to_string("fujsu.toml", env)
    };

    let table: Table = toml::from_str(&config_str).unwrap();

    
    if table.contains_key("mods") {
        for mod_name in table["mods"].as_array().unwrap() {
            unsafe {
                MOD_LIBS.push(mod_name.as_str().unwrap().to_string());
            }
        }
    }
    else {
        warn!("No mods provided! Fujsu will do literally nothing.");
    }
}

unsafe fn read_asset_to_string(filename: &str, env: &mut JNIEnv) -> String {
    let asset_manager = get_android_asset_manager(env);
    let filename_cstr = CString::new(filename).unwrap();
    let asset = ndk_sys::AAssetManager_open(asset_manager, filename_cstr.as_ptr(), ndk_sys::AASSET_MODE_UNKNOWN as i32);
    if asset.is_null() {
        error!("No configuration provided!");
        panic!();
    }

    let mut contents = String::new();

    const BUFFER_SIZE: usize = 1024;
    let mut buffer = [0u8; BUFFER_SIZE];

    loop {
        let bytes_read = ndk_sys::AAsset_read(asset, buffer.as_mut_ptr() as *mut std::ffi::c_void, BUFFER_SIZE);
        if bytes_read <= 0 {
            break;
        }

        let slice = &buffer[0..bytes_read as usize];
        if let Ok(s) = std::str::from_utf8(slice) {
            contents.push_str(s);
        }
    }

    ndk_sys::AAsset_close(asset);

    contents
}

fn get_android_asset_manager(env: &mut JNIEnv) -> *mut AAssetManager {
    let unity_class_name = "com/unity3d/player/UnityPlayer";
    let unity_class = &env
        .find_class(unity_class_name)
        .expect("Failed to find class com/unity3d/player/UnityPlayer");

    let current_activity_obj: JObject = env
        .get_static_field(unity_class, "currentActivity", "Landroid/app/Activity;")
        .expect("Failed to get static field currentActivity")
        .l().unwrap();

    let asset_manager = env.call_method(current_activity_obj, "getAssets", "()Landroid/content/res/AssetManager;", &[]);
    unsafe {
        return ndk_sys::AAssetManager_fromJava(env.get_native_interface(), asset_manager.unwrap().l().unwrap().as_raw());
    }
}