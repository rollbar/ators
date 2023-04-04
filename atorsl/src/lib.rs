pub mod addr;
pub mod context;
pub mod demangle;
pub mod error;
pub mod ext;
pub mod lookup;

pub use addr::Addr;
pub use context::Context;
pub use error::Error;
pub use lookup::Symbolicate;

/// Loads a binary image file.
///
/// # Safety
///
/// The caller must ensure that the object file loaded isn't subsequently modified, applications
/// must consider the risk and take appropriate precautions such as file permissions, locks or
/// process-private (e.g. unlinked) files.
///
/// Modifications to the loaded object file is undefined behavior.
#[macro_export]
macro_rules! load_object {
    ($path:expr, $binding:ident) => {{
        // SAFETY: All file-backed memory map constructors are marked `unsafe` because of the
        // potential for *Undefined Behavior* (UB) using the map if the underlying file is
        // subsequently modified, in or out of process. Applications must consider the risk and
        // take appropriate precautions when using file-backed maps. Solutions such as file
        // permissions, locks or process-private (e.g. unlinked) files exist but are platform
        // specific and limited.
        $binding = unsafe { memmap2::Mmap::map(&std::fs::File::open(&$path)?) }?;
        Result::<object::File, atorsl::Error>::Ok(object::File::parse(&*$binding)?)
    }};
}

/// Loads a binary image object as DWARF.
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
                if <object::File as object::Object>::is_little_endian($object) {
                    gimli::RunTimeEndian::Little
                } else {
                    gimli::RunTimeEndian::Big
                },
            )
        })
    }};
}

/// Commonly used DWARF sections, and other common information.
pub(crate) type Dwarf<'input> = gimli::Dwarf<gimli::EndianSlice<'input, gimli::RunTimeEndian>>;

/// Commonly used information for a unit in the `.debug_info` or `.debug_types` sections.
pub(crate) type Unit<'input> =
    gimli::Unit<gimli::EndianSlice<'input, gimli::RunTimeEndian>, usize>;

/// The common fields for the headers of compilation units and type units.
pub(crate) type UnitHeader<'input> =
    gimli::UnitHeader<gimli::EndianSlice<'input, gimli::RunTimeEndian>, usize>;

/// The value of an attribute in a `DebuggingInformationEntry`.
pub(crate) type AttrValue<'input> =
    gimli::AttributeValue<gimli::EndianSlice<'input, gimli::RunTimeEndian>>;

/// A Debugging Information Entry (DIE).
///
/// DIEs have a set of attributes and optionally have children DIEs as well.
pub(crate) type Entry<'abbrev, 'unit, 'input> = gimli::DebuggingInformationEntry<
    'abbrev,
    'unit,
    gimli::EndianSlice<'input, gimli::RunTimeEndian>,
    usize,
>;
