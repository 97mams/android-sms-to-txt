use jni::objects::{JClass, JString};
use jni::sys::jint;
use jni::JNIEnv;
use std::fs::OpenOptions;
use std::io::Write;

#[no_mangle]
pub extern "system" fn
Java_com_example_smslogger_SmsWriter_appendSms(
    mut env: JNIEnv,
    _class: JClass,
    text: JString,
    path: JString,
) -> jint {
    // Convert Java strings → Rust strings safely
    let content: String = match env.get_string(&text) {
        Ok(val) => val.into(),
        Err(_) => return -1, // error reading SMS content
    };

    let file_path: String = match env.get_string(&path) {
        Ok(val) => val.into(),
        Err(_) => return -2, // error reading file path
    };

    // Open file in append mode
    let mut file = match OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_path)
    {
        Ok(file) => file,
        Err(_) => return -3, // file open error
    };

    // Write content
    if file.write_all(content.as_bytes()).is_err() {
        return -4; // write error
    }

    0 // success
}
