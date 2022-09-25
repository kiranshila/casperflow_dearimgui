use std::ffi::{c_char, CString};

#[no_mangle]
pub extern "C" fn hello_from_rust() -> *const c_char {
    CString::new("Hello from rust")
        .expect("This shouldn't fail")
        .into_raw() // Moves ownership to Cs
}
