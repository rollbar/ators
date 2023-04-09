use crate::context::Loc;
use clap::{
    crate_authors, crate_description, crate_name, crate_version, value_parser, Arg, ArgAction,
    ArgGroup, Command, ValueHint,
};
use std::{fmt, path::PathBuf};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Opt {
    Object,
    LoadAddr,
    SlideAddr,
    Offset,
    Addr,
    AddrFile,
    Arch,
    Inline,
    Scope,
    Delimiter,
    FullPath,
}

impl fmt::Display for Opt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl From<Opt> for clap::Id {
    fn from(value: Opt) -> Self {
        Self::from(value.to_string())
    }
}

pub fn build() -> Command {
    Command::new(crate_name!())
        .about(crate_description!())
        .long_about(
            "convert numeric addresses to symbols of binary images\n\n\
            The ators command converts numeric addresses to their symbolic equivalents.  If\n\
            full debug symbol information is available, then the output of atos will include\n\
            file name and source line number information.")
        .author(crate_authors!())
        .version(crate_version!())
        .before_help(TITLE)
        .arg_required_else_help(true)
        .args([
            Arg::new(Opt::Object)
                .short('o')
                .help_heading("Arguments")
                .help("The path to a binary image or dSYM in which to look up symbols")
                .required(true)
                .value_hint(ValueHint::FilePath)
                .value_name("binary|dSYM")
                .value_parser(value_parser!(PathBuf)),
            Arg::new(Opt::LoadAddr)
                .short('l')
                .help_heading("Arguments")
                .help("The load address of the binary image")
                .group("loc")
                .value_name("load-address")
                .value_parser(|addr: &str| addr.parse().map(Loc::Load))
                .long_help(
                    "The load address of the binary image.  This value is always assumed to be\n\
                    in hex, even without a \"0x\" prefix.  The input addresses are assumed to be\n\
                    in a binary image with that load address.  Load addresses for binary images\n\
                    can be found in the \"Binary Images:\" section at the bottom of crash,\n\
                    sample, leaks, and malloc_history reports."),
            Arg::new(Opt::SlideAddr)
                .short('s')
                .help_heading("Arguments")
                .help("The slide value of the binary image")
                .group("loc")
                .value_name("slide")
                .value_parser(|addr: &str| addr.parse().map(Loc::Slide))
                .long_help(
                    "The slide value of the binary image.  This is the difference between the\n\
                    load address of a binary image, and the address at which the binary image\n\
                    was built.  This slide value is subtracted from the input addresses.  It is\n\
                    usually easier to directly specify the load address with the -l argument\n\
                    than to manually calculate a slide value."),
            Arg::new(Opt::AddrFile)
                .short('f')
                .help_heading("Arguments")
                .help("Input file with white-separated numeric addresses.")
                .group("input")
                .value_hint(ValueHint::FilePath)
                .value_name("address-input-file")
                .value_parser(value_parser!(PathBuf))
                .long_help(
                    "Use this argument to specify the path of an input file containing\n\
                    whitespace-separated numeric addresses."),
            Arg::new(Opt::Addr)
                .last(true)
                .help_heading("Arguments")
                .help("\tA list of input addresses at the end of the argument list.")
                .group("input")
                .action(ArgAction::Append)
                .required(true)
                .num_args(1..)
                .value_name("address...")
                .value_parser(str::parse::<atorsl::data::Addr>),
            ])
        .group(ArgGroup::new("loc").required(true))
        .group(ArgGroup::new("input").required(true))
        .args([
            Arg::new(Opt::Arch)
                .long("arch")
                .help("The architecure of a binary image in which to look up symbols")
                .value_name("architecture")
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
            Arg::new(Opt::Scope).long("scope")
                .help("The scope of demangled information to display for mangled symbols.")
                .num_args(1)
                .value_name("scope")
                .value_parser(["none", "compact", "std", "full"])
                .default_value("std")
                .long_help(
                    "The scope of demangled information to display for mangled symbols.  Use
                    'none' to avoid demangling mangled symbols."
                ),
            Arg::new(Opt::Delimiter)
                .short('d')
                .help("Delimiter when outputting inline frames. Defaults to newline.")
                .value_name("delimiter")
                .value_parser(value_parser!(String))
                .default_value(""),
            Arg::new(Opt::Inline)
                .short('i')
                .long("inlineFrames")
                .help("Display inlined symbols")
                .action(ArgAction::SetTrue),
            Arg::new(Opt::FullPath)
                .long("fullPath")
                .help("Print the full path of the source files")
                .action(ArgAction::SetTrue),
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
