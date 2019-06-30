use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_void};

#[no_mangle]
pub extern fn get_number() -> u32 {
    return 46;
}

#[no_mangle]
pub extern fn add_numbers(a: u32, b: u32) -> u32 {
    return a + b;
}

#[no_mangle]
pub extern fn divide_numbers(a: f32, b: f32) -> f32 {
    return a / b;
}