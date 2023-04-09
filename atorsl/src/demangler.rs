use crate::IsOkAnd;
use std::convert::identity;

#[repr(C)]
pub enum Scope {
    None,
    Compact,
    Standard,
    Full,
}

pub fn demangle(symbol: &str) -> &str {
    if swift::is_mangled(symbol).is_ok_and(identity) {
        swift::try_demangle(symbol, Scope::Standard).unwrap_or(symbol)
    } else {
        symbol
    }
}

pub mod swift {
    use super::Scope;
    use crate::data::Error;
    use std::{
        ffi::{CStr, CString},
        os::raw::{c_char, c_int},
    };

    extern "C" {
        fn isMangledSwiftSymbol(sym: *const c_char) -> c_int;

        fn demangleSwiftSymbol(
            sym: *const c_char,
            buf: *mut c_char,
            buf_len: usize,
            scope: Scope,
        ) -> c_int;
    }

    pub fn is_mangled(symbol: &str) -> Result<bool, Error> {
        unsafe { Ok(isMangledSwiftSymbol(CString::new(symbol)?.as_ptr()) != 0) }
    }

    pub fn try_demangle(symbol: &str, scope: Scope) -> Result<&str, Error> {
        let mut buf = vec![0; 4096];
        let c_sym = CString::new(symbol)?;

        unsafe {
            if demangleSwiftSymbol(c_sym.as_ptr(), buf.as_mut_ptr(), buf.len(), scope) != 0 {
                Ok(CStr::from_ptr(buf.as_ptr()).to_str()?)
            } else {
                Err(Error::CannotDemangleSymbol(symbol.to_owned()))
            }
        }
    }
}
