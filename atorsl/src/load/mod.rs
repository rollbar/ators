#[macro_export]
macro_rules! load_dwarf {
    ($object:expr, $binding:ident) => {{
        $binding = gimli::Dwarf::load(|section_id| -> gimli::Result<std::borrow::Cow<[u8]>> {
            Ok($object
                .section_by_name(section_id.name())
                .and_then(|section| section.uncompressed_data().ok())
                .unwrap_or(std::borrow::Cow::Borrowed(&[][..])))
        })?;

        $binding.borrow(|section| gimli::EndianSlice::new(&*section, $object.runtime_endian()))
    }};
}

#[macro_export]
macro_rules! load_object {
    ($path:expr, $binding:ident) => {{
        let file = fs::File::open(&context.object_path)?;
        let mmap = unsafe { memmap2::Mmap::map(&file) }?;
        object::File::parse(&*mmap)?
    }};
}
