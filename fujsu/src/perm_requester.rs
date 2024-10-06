use jni::{
    objects::{JObject, JValueGen},
    sys::jint,
    JNIEnv,
};

// this module is mostly based on the libmainloader from sc2ad
// https://github.com/sc2ad/LibMainLoader/blob/master/src/fileutils.cpp

pub fn ensure_perms(env: &mut JNIEnv) -> bool {
    let unity_class_name = "com/unity3d/player/UnityPlayer";
    let unity_class = &env
        .find_class(unity_class_name)
        .expect("Failed to find class com/unity3d/player/UnityPlayer");

    let current_activity_obj: JObject = env
        .get_static_field(unity_class, "currentActivity", "Landroid/app/Activity;")
        .expect("Failed to get static field currentActivity")
        .l()
        .unwrap();

    if current_activity_obj.is_null() {
        info!("currentActivity is null");
        if env.exception_check().unwrap() {
            env.exception_describe().unwrap();
            env.exception_clear().unwrap();
        }
        return false;
    }

    // causes the app to freeze for some reason
    /* if ensure_perms_with_unity(env, &current_activity_obj) {
        return true;
    }

    info!("Failed to request permissions with Unity. Check System.err.");
    if env.exception_check().unwrap() {
        env.exception_describe().unwrap();
        env.exception_clear().unwrap();
    } */

    if ensure_perms_with_package_name(env, &current_activity_obj) {
        return true;
    }

    info!("Failed to request permissions with intent. Check System.err.");
    if env.exception_check().unwrap() {
        env.exception_describe().unwrap();
        env.exception_clear().unwrap();
    }

    false
}

pub fn ensure_perms_with_package_name(env: &mut JNIEnv, current_activity_obj: &JObject) -> bool {
    const TRIES: i8 = 3;
    const DELAY: u64 = 5000;

    let env_class = env
        .find_class("android/os/Environment")
        .expect("Failed to find class android/os/Environment");

    let uri_class = env
        .find_class("android/net/Uri")
        .expect("Failed to find class android/net/Uri");

    let intent_class = env
        .find_class("android/content/Intent")
        .expect("Failed to find class android/content/Intent");

    for _ in 0..TRIES {
        let result = env
            .call_static_method(&env_class, "isExternalStorageManager", "()Z", &[])
            .unwrap()
            .z()
            .unwrap();

        if env.exception_check().unwrap() {
            return false;
        }

        if result {
            return true;
        }

        let action_name = env
            .new_string("android.settings.MANAGE_APP_ALL_FILES_ACCESS_PERMISSION")
            .unwrap();
        let nrm_package_name = format!("package:{}", unsafe { crate::PACKAGE_NAME.as_ref().unwrap() });
        let package_name = env.new_string(nrm_package_name).unwrap();

        let uri = env
            .call_static_method(
                &uri_class,
                "parse",
                "(Ljava/lang/String;)Landroid/net/Uri;",
                &[JValueGen::from(&package_name)],
            )
            .unwrap()
            .l()
            .unwrap();

        let intent = env
            .new_object(
                &intent_class,
                "(Ljava/lang/String;Landroid/net/Uri;)V",
                &[JValueGen::from(&action_name), JValueGen::from(&uri)],
            )
            .unwrap();

        if intent.is_null() {
            info!("Failed to create new Intent");
            return false;
        }

        let res = env.call_method(
            current_activity_obj,
            "startActivity",
            "(Landroid/content/Intent;)V",
            &[JValueGen::from(&intent)],
        );

        if res.is_err() {
            info!("Failed to call startActivity");
            info!("{}", &res.err().unwrap().to_string());
            return false;
        }

        if env.exception_check().unwrap() {
            info!("Failed to call startActivity");
            return false;
        }

        std::thread::sleep(std::time::Duration::from_millis(DELAY));
    }

    true
}

pub fn ensure_perms_with_unity(env: &mut JNIEnv, current_activity_obj: &JObject) -> bool {
    const PERMS_TO_REQUEST: [&str; 2] = [
        "android.permission.WRITE_EXTERNAL_STORAGE",
        "android.permission.READ_EXTERNAL_STORAGE",
    ];

    let unity_perms_class = env
        .find_class("com/unity3d/player/UnityPermissions")
        .expect("Failed to find class com/unity3d/player/UnityPermissions");

    let wait_perm_class = env
        .find_class("com/unity3d/player/UnityPermissions$ModalWaitForPermissionResponse")
        .expect("Failed to find class com/unity3d/player/UnityPermissions$ModalWaitForPermissionResponse");

    let wait_perm = env
        .new_object(wait_perm_class, "()V", &[])
        .expect("Failed to create new ModalWaitForPermissionResponse");

    let permissions_array = env
        .new_object_array(2, "java/lang/String", JObject::null())
        .expect("Failed to create new String array");

    for (i, perm) in PERMS_TO_REQUEST.iter().enumerate() {
        let perm_str = env.new_string(perm).expect("Failed to create new string");

        env.set_object_array_element(&permissions_array, i as jint, perm_str)
            .expect("Failed to set object array element");
    }

    env.call_static_method(
        unity_perms_class,
        "requestUserPermissions",
        "(Landroid/app/Activity;[Ljava/lang/String;Lcom/unity3d/player/IPermissionRequestCallbacks;)V",
        &[JValueGen::from(current_activity_obj), JValueGen::from(&permissions_array), JValueGen::from(&wait_perm)]
    ).expect("Failed to call requestUserPermissions");

    if env.exception_check().unwrap() {
        return false;
    }

    env.call_method(wait_perm, "waitForResponse", "()V", &[])
        .expect("Failed to call waitForResponse");

    !env.exception_check().unwrap()
}