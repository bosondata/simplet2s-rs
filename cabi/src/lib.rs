extern crate simplet2s;

use std::mem;
use std::ptr;
use std::ffi::CStr;
use std::os::raw::c_char;

/// Represents a string.
#[repr(C)]
pub struct FfiStr {
    pub data: *mut c_char,
    pub len: usize,
    pub owned: bool,
}

impl FfiStr {
    pub fn from_string(mut s: String) -> FfiStr {
        s.shrink_to_fit();
        let rv = FfiStr {
            data: s.as_ptr() as *mut c_char,
            len: s.len(),
            owned: true,
        };
        mem::forget(s);
        rv
    }

    pub unsafe fn free(&mut self) {
        if self.owned {
            String::from_raw_parts(self.data as *mut _, self.len, self.len);
            self.data = ptr::null_mut();
            self.len = 0;
            self.owned = false;
        }
    }
}

/// Frees a ffi str.
///
/// If the string is marked as not owned then this function does not
/// do anything.
#[no_mangle]
pub unsafe extern "C" fn simplet2s_str_free(s: *mut FfiStr) {
    if !s.is_null() {
        (*s).free()
    }
}

#[no_mangle]
pub unsafe extern "C" fn simplet2s_convert(s: *const c_char) -> FfiStr {
    let s = CStr::from_ptr(s).to_str().unwrap();
    let r = simplet2s::convert(s);
    FfiStr::from_string(r)
}
