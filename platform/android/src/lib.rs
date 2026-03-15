use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jstring, jint};
use std::path::Path;
// আমাদের কোর ইঞ্জিন থেকে start_download ইম্পোর্ট করা
use vampire_core::controller::start_download;

/// এই নামের স্টাইলটি (Java_package_name_ClassName_methodName) JNI-এর জন্য বাধ্যতামূলক
#[no_mangle]
pub extern "system" fn Java_com_vampiresquad_vampiredl_MainActivity_startDownload(
    mut env: JNIEnv,
    _class: JClass,
    url: JString,
    save_path: JString,
    threads: jint,
) -> jstring {
    
    // Java এর স্ট্রিংগুলোকে Rust এর স্ট্রিংয়ে কনভার্ট করা
    let url_str: String = env.get_string(&url).unwrap().into();
    let path_str: String = env.get_string(&save_path).unwrap().into();

    // Android-এ ডিফল্টভাবে Tokio রানটাইম থাকে না, তাই আমাদের একটি তৈরি করে নিতে হবে
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    // কোর ইঞ্জিনকে কল করা হচ্ছে
    let result = rt.block_on(async {
        start_download(&url_str, Path::new(&path_str), threads as u64).await
    });

    // রেজাল্ট অনুযায়ী মেসেজ তৈরি করা
    let response_msg = match result {
        Ok(_) => "✅ Download completed successfully via Rust Core Engine!".to_string(),
        Err(e) => format!("❌ Download failed: {}", e),
    };

    // Rust এর মেসেজকে আবার Java স্ট্রিংয়ে কনভার্ট করে পাঠিয়ে দেওয়া
    let output = env.new_string(response_msg).unwrap();
    output.into_raw()
}
