use std::ffi::{c_char, c_uint, CString};

#[no_mangle]
pub extern "C" fn hello_from_rust() -> *const c_char {
    CString::new("Hello from rust")
        .expect("This shouldn't fail")
        .into_raw() // Moves ownership to Cs
}

#[repr(C)]
pub struct PrimitiveHdlNode {
    pub name: *const c_char,
    pub num_inputs: c_uint,
    pub num_outputs: c_uint,
}

#[no_mangle]
pub extern "C" fn gen_primitive_hdl_node(
    name: *const c_char,
    num_inputs: c_uint,
    num_outputs: c_uint,
) -> *mut PrimitiveHdlNode {
    let mut node = PrimitiveHdlNode {
        name,
        num_inputs,
        num_outputs,
    };
    &mut node as *mut PrimitiveHdlNode
}
