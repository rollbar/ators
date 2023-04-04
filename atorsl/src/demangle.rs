pub mod swift {
    use std::{
        ffi::{CStr, CString},
        os::raw::{c_char, c_int},
    };

    extern "C" {
        fn isMangledSwiftSymbol(sym: *const c_char) -> c_int;
        fn demangleSwiftSymbol(sym: *const c_char, buf: *mut c_char, buf_len: usize) -> c_int;
    }

    pub fn is_mangled(symbol: &str) -> bool {
        CString::new(symbol)
            .map(|symbol| unsafe { isMangledSwiftSymbol(symbol.as_ptr()) != 0 })
            .unwrap_or(false)
    }

    pub fn demangle(symbol: &str) -> Option<String> {
        let mut buffer = vec![0; 4096];

        CString::new(symbol).ok().and_then(|symbol| unsafe {
            if demangleSwiftSymbol(symbol.as_ptr(), buffer.as_mut_ptr(), buffer.len()) != 0 {
                Some(
                    CStr::from_ptr(buffer.as_ptr())
                        .to_string_lossy()
                        .to_string(),
                )
            } else {
                None
            }
        })
    }
}
