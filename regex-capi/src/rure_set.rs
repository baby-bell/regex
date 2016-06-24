use ::error::{Error, ErrorKind};

use ::regex::bytes::RegexSet;
use ::libc::{c_char, size_t};

use ::std::slice;
use ::std::ptr;
use ::std::str;

#[repr(C)]
pub struct rure_bytes {
    pub buf: *const u8,
    pub len: size_t,
}

ffi_fn! {
    fn rure_set_compile(patterns: *const rure_bytes,
                        patterns_len: size_t,
                        error: *mut Error
    ) -> *const RegexSet {
        let pats = unsafe { slice::from_raw_parts(patterns, patterns_len) };
        let mut str_pats = Vec::with_capacity(patterns_len);

        for byte_struct in pats {
            let byte_slice = unsafe { slice::from_raw_parts(byte_struct.buf, byte_struct.len) };
            let str_pat = match str::from_utf8(byte_slice) {
                Ok(s) => s,
                Err(e) => {
                    unsafe {
                        if !error.is_null() {
                            *error = Error::new(ErrorKind::Str(e));
                        }
                        return ptr::null();
                    }
                }
            };

            str_pats.push(str_pat);
        }

        match RegexSet::new(str_pats) {
            Ok(r) => Box::into_raw(Box::new(r));
            Err(e) => {
                unsafe {
                    if !error.is_null() {
                        *error = Error::new(ErrorKind::Regex(e));
                    }
                }
                ptr::null()
            }
        }
    }
}

ffi_fn! {
    fn rure_set_free(rs: *const RureSet) {
        unsafe { Box::from_raw(rs as *mut RureSet); }
    }
}
