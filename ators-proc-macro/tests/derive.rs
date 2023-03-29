#[test]
fn derive_tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/newtype.rs");
}
