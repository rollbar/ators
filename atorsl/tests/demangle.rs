use atorsl::demangler;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[test]
fn test_demangle() {
    File::open(format!(
        "{}/../fixtures/manglings",
        env!("CARGO_MANIFEST_DIR")
    ))
    .map(|file| BufReader::new(file).lines())
    .expect("Buffer reader to be added to manglings reader")
    .map(|result| result.expect("Buffer reader line to be ok"))
    .filter(|line| !line.is_empty() && !line.starts_with("//")) // Remove empties and comments
    .map(|line| {
        line.split_once(" ---> ")
            .map(|p| (p.0.to_string(), p.1.to_string()))
            .expect(format!("Line be splitted at '--->': {:?}", line).as_str())
    })
    .map(|(mangled, demangled)| {
        (
            demangler::swift::try_demangle(&mangled, demangler::swift::Scope::Standard)
                .expect(format!("can't demangle: {}", mangled).as_str())
                .to_owned(),
            demangled,
        )
    })
    .for_each(|demangled| assert_eq!(demangled.0, demangled.1));
}
