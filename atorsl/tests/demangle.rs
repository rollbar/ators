use atorsl::demangler;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[test]
fn test_demangle() {
    File::open(format!(
        "{}/../fixtures/manglings.txt",
        env!("CARGO_MANIFEST_DIR")
    ))
    .map(|file| BufReader::new(file).lines())
    .expect("Buffer reader to be added to manglings reader")
    .map(|result| result.expect("Buffer reader line to be ok"))
    .filter(|line| !line.is_empty() && !line.starts_with("//")) // Remove empties and comments
    .map(|line| {
        line.split_once(" ---> ")
            .map(|p| (p.0.to_string(), p.1.to_string()))
            .unwrap_or_else(|| panic!("Line be splitted at '--->': {:?}", line))
    })
    .map(|(mangled, demangled)| {
        (
            demangler::swift::try_demangle(&mangled, demangler::swift::Scope::Standard)
                .unwrap_or_else(|e| panic!("can't demangle: {}\n\terror: {}", mangled, e)),
            demangled,
        )
    })
    .for_each(|demangled| assert_eq!(demangled.0, demangled.1));
}
