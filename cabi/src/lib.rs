extern crate simplet2s;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Frees a C str.
#[no_mangle]
pub unsafe extern "C" fn simplet2s_str_free(s: *mut c_char) {
    if !s.is_null() {
        CString::from_raw(s);
    }
}

#[no_mangle]
pub unsafe extern "C" fn simplet2s_convert(s: *const c_char) -> *mut c_char {
    let s = CStr::from_ptr(s).to_str().unwrap();
    let mut r = simplet2s::convert(s);
    r.shrink_to_fit();
    let c_str = CString::new(r).unwrap();
    c_str.into_raw()
}
