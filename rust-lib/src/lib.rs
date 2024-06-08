use libc::c_char;
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn say_hello(input: *const c_char) {
    let mut name = "C/C++";

    if !input.is_null() {
        unsafe {
            name = CStr::from_ptr(input).to_str().unwrap();
        }
    }

    println!("Hello {}, from Rust!", name);
}
