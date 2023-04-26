use crate::IsOkAnd;
use msvc_demangler::DemangleFlags;
use std::convert::identity;
use swift::Scope;

pub enum Lang {
    C,
    Cpp,
    ObjC,
    ObjCpp,
    Rust,
    Swift,
}

pub fn language_of(s: &str) -> Option<Lang> {
    if (s.starts_with("-[") || s.starts_with("+[")) && s.ends_with(']') {
        Some(Lang::ObjC)
    } else if s.starts_with("_Z")
        || s.starts_with("__Z")
        || s.starts_with("___Z")
        || s.starts_with("____Z")
        || s.starts_with('?')
        || s.starts_with("@?")
    {
        Some(Lang::Cpp)
    } else if s.starts_with("_R") {
        Some(Lang::Rust)
    } else if swift::is_mangled(s).is_ok_and(identity) {
        Some(Lang::Swift)
    } else {
        None
    }
}

pub fn demangle(symbol: &str) -> String {
    match language_of(symbol) {
        Some(Lang::C | Lang::Cpp) => {
            if symbol.starts_with('?') || symbol.starts_with("@?") {
                msvc_demangler::demangle(symbol, DemangleFlags::llvm()).ok()
            } else {
                cpp_demangle::Symbol::new(symbol)
                    .map(|s| s.to_string())
                    .ok()
            }
        }

        Some(Lang::ObjC | Lang::ObjCpp) => {
            if (symbol.starts_with("-[") || symbol.starts_with("+[")) && symbol.ends_with(']') {
                None
            } else {
                cpp_demangle::Symbol::new(symbol)
                    .ok()
                    .map(|s| s.to_string())
            }
        }

        Some(Lang::Swift) => swift::try_demangle(symbol, Scope::Compact).ok(),

        Some(Lang::Rust) => rustc_demangle::try_demangle(symbol)
            .map(|s| s.to_string())
            .ok(),

        None => None,
    }
    .unwrap_or_else(|| symbol.to_string())
}

pub mod swift {
    use crate::data::Error;
    use std::{
        ffi::{CStr, CString},
        os::raw::{c_char, c_int},
    };

    #[repr(C)]
    pub enum Scope {
        Compact = -1,
        Standard = 0,
        Full = 1,
    }

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

    pub fn try_demangle(symbol: &str, scope: Scope) -> Result<String, Error> {
        let mut buf = vec![0; 4096];
        let c_sym = CString::new(symbol)?;

        unsafe {
            if demangleSwiftSymbol(c_sym.as_ptr(), buf.as_mut_ptr(), buf.len(), scope) != 0 {
                Ok(CStr::from_ptr(buf.as_ptr()).to_str()?.to_string())
            } else {
                Err(Error::CannotDemangleSymbol(symbol.to_owned()))
            }
        }
    }
}
