use std::os::raw::c_char;
use std::mem;

use c_fixed_string::{CFixedStr, CFixedString};

/// Frees a C str.
#[no_mangle]
pub unsafe extern "C" fn simplet2s_str_free(s: *mut c_char, len: usize) {
    if !s.is_null() {
        CFixedString::from_raw_parts(s, len);
    }
}

#[no_mangle]
pub unsafe extern "C" fn simplet2s_convert(s: *const c_char, len: usize) -> *mut c_char {
    let c_str = CFixedStr::from_ptr(s, len);
    let s = String::from_utf8_lossy(c_str.as_bytes_full());
    let mut r = simplet2s::convert(&s);
    r.shrink_to_fit();
    let bytes = r.into_bytes();
    let mut c_str = CFixedString::new(bytes);
    let ptr = c_str.as_mut_ptr();
    mem::forget(c_str);
    ptr
}
