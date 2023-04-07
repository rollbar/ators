use crate::IsOkAnd;
use std::convert::identity;

pub fn demangle(symbol: &str) -> &str {
    if swift::is_mangled(symbol).is_ok_and(identity) {
        swift::try_demangle(symbol).unwrap_or(symbol)
    } else {
        symbol
    }
}

pub mod swift {
    use std::{
        ffi::{CStr, CString},
        os::raw::{c_char, c_int},
    };

    use crate::Error;

    extern "C" {
        fn isMangledSwiftSymbol(sym: *const c_char) -> c_int;
        fn demangleSwiftSymbol(sym: *const c_char, buf: *mut c_char, buf_len: usize) -> c_int;
    }

    pub fn is_mangled(symbol: &str) -> Result<bool, Error> {
        unsafe { Ok(isMangledSwiftSymbol(CString::new(symbol)?.as_ptr()) != 0) }
    }

    pub fn try_demangle(symbol: &str) -> Result<&str, Error> {
        let mut buffer = vec![0; 4096];
        let c_symbol = CString::new(symbol)?;

        unsafe {
            if demangleSwiftSymbol(c_symbol.as_ptr(), buffer.as_mut_ptr(), buffer.len()) != 0 {
                Ok(CStr::from_ptr(buffer.as_ptr()).to_str()?)
            } else {
                Err(Error::CannotDemangleSymbol(symbol.to_owned()))
            }
        }
    }
}
