#[cfg(target_os = "android")]
extern crate android_logger;
extern crate jni;
#[cfg(target_os = "android")]
#[macro_use]
extern crate log;

#[cfg(target_os = "android")]
use log::LogLevel;
use jni::JNIEnv;
use jni::objects::{JClass, JObject};
use jni::sys::{jint, jobjectArray};

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn JNI_OnLoad() -> jint {
    #[cfg(target_os = "android")]
    {
        android_logger::init_once(LogLevel::Debug);
        info!("Native library loaded");
    }

    jni::sys::JNI_VERSION_1_6
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_supercurio_androidrustjnitests_MainActivity_nativeTest(
    env: JNIEnv,
    _: JClass,
    array: jobjectArray,
) {
    let len = env.get_array_length(array)
        .expect("Couldn't get array length");

    #[cfg(target_os = "android")]
    info!("array len: {}", len);

    for i in 0..len {
        let item = env.get_object_array_element(array, i)
            .expect("Coudln't get object");

        let value = env.get_field(item, "value", "I")
            .expect("Coudln't get item's value")
            .i()
            .unwrap();

        if value % 50 == 0 {
            println!("at item value: {}", value);
            #[cfg(target_os = "android")]
            info!("at item value: {}", value);
        }

        env.delete_local_ref(JObject::from(item))
            .expect("Unable to delete item's local ref");
    }
}
