use clap::{
    arg, crate_authors, crate_version, value_parser, Arg, ArgAction, ArgGroup, ArgMatches,
    Command, ValueHint,
};
use std::path::PathBuf;

pub fn build() -> Command {
    Command::new("ators")
        .about("convert numeric addresses to symbols of binary images")
        .long_about(
            "convert numeric addresses to symbols of binary images\n\n\
            The ators command converts numeric addresses to their symbolic equivalents.  If\n\
            full debug symbol information is available, then the output of atos will include\n\
            file name and source line number information.")
        .author(crate_authors!())
        .version(crate_version!())
        .before_help(TITLE)
        .next_line_help(true)
        .help_expected(true)
        .arg_required_else_help(true)
        .args([
            arg!(object: -o "The path to a binary image file or dSYM in which to look up symbols")
                .help_heading("Arguments")
                .action(ArgAction::Set)
                .display_order(1)
                .required(true)
                .num_args(1)
                .action(ArgAction::Set)
                .value_hint(ValueHint::FilePath)
                .value_name("binary-image-file | dSYM")
                .value_parser(value_parser!(PathBuf)),
            arg!(loadAddress: -l "The load address of the binary image")
                .help_heading("Arguments")
                .action(ArgAction::Set)
                .display_order(2)
                .required(true)
                .num_args(1)
                .action(ArgAction::Set)
                .value_name("load-address")
                .value_parser(value_parser!(usize))
                .long_help(
                    "The load address of the binary image.  This value is always assumed to be\n\
                    in hex, even without a \"0x\" prefix.  The input addresses are assumed to be\n\
                    in a binary image with that load address.  Load addresses for binary images\n\
                    can be found in the \"Binary Images:\" section at the bottom of crash,\n\
                    sample, leaks, and malloc_history reports."),
            arg!(address: [address] "A list of input addresses at the end of the argument list.")
                .help_heading("Arguments")
                .action(ArgAction::Append)
                .display_order(3)
                .required(true)
                .num_args(1..)
                .last(true)
                .value_parser(value_parser!(usize))
        ])
        .group(
            ArgGroup::new("required")
                .args(["object", "loadAddress", "address"])
                .requires_all(["object", "loadAddress", "address"]))
        .args([
            arg!(arch: -a --arch <architecture> "The particular architecure of a binary image file in which to look up symbols.")
                .action(ArgAction::Set)
                .display_order(4)
                .required(false)
                .requires("object")
                .requires("loadAddress")
                .requires("address")
                .value_parser(value_parser!(String))
                .long_help(
                    "The particular architecure of a binary image file in which to look up\n\
                    symbols.  It is possible to get symbols for addresses from a different\n\
                    machine architecture than the system on which atos is running.  For example,\n\
                    when running atos on an Intel-based system, one may wish to get the symbol\n\
                    for an address that came from a backtrace of a process running on an ARM\n\
                    device.  To do so, use the -arch flag to specify the desired architecture\n\
                    (such as i386 or arm) and pass in a corresponding symbol-rich Mach-O binary\n\
                    image file with a binary image of the corresponding architecture (such as a\n\
                    Universal Binary)."),
            arg!(inline: -i --inlineFrames "Display inlined symbols")
                .action(ArgAction::SetTrue)
                .display_order(5)
                .required(false)
                .requires("object")
                .requires("loadAddress")
                .requires("address"),
            arg!(verbose: -v --verbose "Display verbose output")
                .action(ArgAction::SetTrue)
                .display_order(6)
                .required(false)
                .requires("object")
                .requires("loadAddress")
                .requires("address")
                .display_order(1000),
        ])
        .after_long_help(
            "\t\t\t\t- - -\n\n\
            A stripped, optimized version of Sketch was built as an x86_64 position-independent\n\
            executable (PIE) into /BuildProducts/Release.  Full debug symbol information is\n\
            available in Sketch.app.dSYM, which sits alongside Sketch.app.  When Sketch.app was\n\
            run, the Sketch binary (which was built at 0x100000000) was loaded at 0x10acde000.\n\
            Running 'sample Sketch' showed 3 addresses that we want to get symbol information\n\
            for -- 0x10acea1d3, 0x10ace4bea, and 0x10ace4b7a.\n\
            \n\
            Now, to symbolicate, we run atos with the -o flag specifying the path to the actual\n\
            Sketch dSYM, the -arch x86_64 flag, and the -l 0x10acde000 flag to specify the load\n\
            address.\n\
            \n\
            % ators -o ./Sketch.app.dSYM -arch x86_64 -l 0x10acde000 -- 0x10acea1d3 0x10ace4bea 0x10ace4b7a\n\
            -[SKTGraphicView drawRect:] (in Sketch) (SKTGraphicView.m:445)\n\
            -[SKTGraphic drawHandlesInView:] (in Sketch) (NSGeometry.h:110)\n\
            -[SKTGraphic drawHandleInView:atPoint:] (in Sketch) (SKTGraphic.m:490)")
}

const TITLE: &str = r#"
​           .
         .o8
 .oooo. .o888oo  .ooooo.  oooo d8b  .oooo.o
`P  )88b  888   d88' `88b `888""8P d88(  "8
 .oP"888  888   888   888  888     `"Y88b.
d8(  888  888 . 888   888  888     o.  )88b
`Y888""8o "888" `Y8bod8P' d888b    8""888P'"#;
