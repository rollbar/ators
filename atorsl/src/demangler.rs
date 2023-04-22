use crate::IsOkAnd;
use gimli::DwLang;
use std::convert::identity;
use swift::Scope;

pub fn language_of(s: &str) -> DwLang {
    if (s.starts_with("-[") || s.starts_with("+[")) && s.ends_with(']') {
        gimli::DW_LANG_ObjC
    } else if s.starts_with("_Z")
        || s.starts_with("__Z")
        || s.starts_with("___Z")
        || s.starts_with("____Z")
        || s.starts_with('?')
        || s.starts_with("@?")
    {
        gimli::DW_LANG_C_plus_plus
    } else if s.starts_with("_R") {
        gimli::DW_LANG_Rust
    } else if swift::is_mangled(s).is_ok_and(identity) {
        gimli::DW_LANG_Swift
    } else {
        DwLang(0)
    }
}

pub fn demangle(sym: &str, lang: Option<DwLang>) -> String {
    match lang.unwrap_or_else(|| language_of(sym)) {
        gimli::DW_LANG_C
        | gimli::DW_LANG_C89
        | gimli::DW_LANG_C99
        | gimli::DW_LANG_C11
        | gimli::DW_LANG_C17
        | gimli::DW_LANG_C_plus_plus
        | gimli::DW_LANG_C_plus_plus_03
        | gimli::DW_LANG_C_plus_plus_11
        | gimli::DW_LANG_C_plus_plus_14
        | gimli::DW_LANG_C_plus_plus_17
        | gimli::DW_LANG_C_plus_plus_20 => {
            if sym.starts_with('?') || sym.starts_with("@?") {
                msvc_demangler::demangle(sym, msvc_demangler::DemangleFlags::llvm()).ok()
            } else {
                cpp_demangle::Symbol::new(sym)
                    .ok()
                    .map(|s| s.to_string())
            }
        }

        gimli::DW_LANG_ObjC | gimli::DW_LANG_ObjC_plus_plus => {
            if (sym.starts_with("-[") || sym.starts_with("+[")) && sym.ends_with(']') {
                None
            } else {
                cpp_demangle::Symbol::new(sym)
                    .ok()
                    .map(|s| s.to_string())
            }
        }

        gimli::DW_LANG_Swift => swift::try_demangle(sym, Scope::Compact).ok(),

        gimli::DW_LANG_Rust => rustc_demangle::try_demangle(sym)
            .ok()
            .map(|s| s.to_string()),

        _ => None,
    }
    .unwrap_or_else(|| sym.to_string())
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
