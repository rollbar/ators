mod format;

pub mod data;
pub mod ext;
pub mod read;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to open file")]
    Io(#[from] std::io::Error),

    #[error("Error reading DWARF")]
    Gimli(#[from] gimli::Error),

    #[error("Error reading binary image object")]
    Object(#[from] object::read::Error),

    #[error("__TEXT segment not found in DWARF")]
    TextSegmentNotFound,

    #[error("Address not found")]
    AddressNotFound,

    #[error("Address is not a symbol")]
    AddressNotSymbol,
}

#[macro_export]
macro_rules! load_object {
    ($path:expr, $binding:ident) => {{
        $binding = unsafe { memmap2::Mmap::map(&std::fs::File::open(&$path)?) }?;
        Result::<object::File, atorsl::Error>::Ok(object::File::parse(&*$binding)?)
    }};
}

#[macro_export]
macro_rules! load_dwarf {
    ($object:expr, $binding:ident) => {{
        $binding = gimli::Dwarf::load(|sid| -> gimli::Result<std::borrow::Cow<[u8]>> {
            Ok(
                <object::File as object::Object>::section_by_name(&$object, sid.name())
                    .and_then(|s| {
                        <object::Section as object::ObjectSection>::uncompressed_data(&s).ok()
                    })
                    .unwrap_or(std::borrow::Cow::Borrowed(&[][..])),
            )
        })?;

        $binding.borrow(|section| {
            gimli::EndianSlice::new(
                &*section,
                <object::File as atorsl::ext::object::File>::runtime_endian(&$object),
            )
        })
    }};
}
