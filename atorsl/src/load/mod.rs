#[macro_export]
macro_rules! load_dwarf {
    ($object:expr, $binding:ident) => {{
        $binding = gimli::Dwarf::load(|section_id| -> anyhow::Result<std::borrow::Cow<[u8]>> {
            Ok($object
                .section_by_name(section_id.name())
                .and_then(|section| section.uncompressed_data().ok())
                .unwrap_or(std::borrow::Cow::Borrowed(&[][..])))
        })?;

        $binding.borrow(|section| gimli::EndianSlice::new(&*section, $object.runtime_endian()))
    }};
}
