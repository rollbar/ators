fn main() -> Result<(), cc::Error> {
    cc::Build::new()
        .files(&[
            "src/swift/demangle.cpp",
            "src/swift/lib/Demangler.cpp",
            "src/swift/lib/Context.cpp",
            "src/swift/lib/Errors.cpp",
            "src/swift/lib/ManglingUtils.cpp",
            "src/swift/lib/NodeDumper.cpp",
            "src/swift/lib/NodePrinter.cpp",
            "src/swift/lib/OldDemangler.cpp",
            "src/swift/lib/Punycode.cpp",
        ])
        .include("src/swift/include")
        .define("LLVM_DISABLE_ABI_BREAKING_CHECKS_ENFORCING", "1")
        .define("SWIFT_STDLIB_HAS_TYPE_PRINTING", None)
        .define("SWIFT_SUPPORT_OLD_MANGLING", None)
        .flag_if_supported("-fpermissive")
        .flag_if_supported("-Wno-changes-meaning")
        .flag_if_supported("-w")
        .flag("-std=c++17")
        .warnings(false)
        .cpp(true)
        .try_compile("log")
}
