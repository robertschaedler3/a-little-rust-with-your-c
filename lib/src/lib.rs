use libc::c_char;
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn say_hello(name: *const c_char) {
    let name = {
        if name.is_null() {
            "World".to_string()
        } else {
            unsafe { CStr::from_ptr(name).to_str().unwrap().to_string() }
        }
    };

    println!("Hello, {}!", name);
}
