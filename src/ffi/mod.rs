use parser;
use std::ffi::{CStr, CString};
use std::str;
use libc::c_char;

/* Utility functions */
fn str_to_ptr(input: &str) -> *const c_char {

    let c_str = CString::new(input).unwrap();
    let c_ptr = c_str.as_ptr();
    std::mem::forget(c_str);

    c_ptr
}

fn str_from_ptr<'a>(input: *const c_char) -> &'a str {
    let res = unsafe { CStr::from_ptr(input) };
    let out = res.to_str().unwrap();

    out
}

/* Extern functions */
#[no_mangle]
pub extern "C" fn mcp_from_notchian(notchian: *const c_char) -> *const c_char {

    let result = parser::srg::find_class_notchian(str_from_ptr(notchian)).unwrap().mcp_name;
    return str_to_ptr(result.as_str());
    
}

#[no_mangle]
pub extern "C" fn notchian_from_mcp(mcp: *const c_char) -> *const c_char {
    
    let result = parser::srg::find_class(str_from_ptr(mcp)).unwrap().notchian_name;
    return str_to_ptr(result.as_str());
    
}