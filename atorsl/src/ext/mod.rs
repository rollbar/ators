pub mod object {
    use crate::data::{Addr, Error};
    use object::{
        macho,
        read::{self, macho::FatArch},
        Object, ObjectSegment,
    };

    pub trait File {
        fn parse_data(
            data: &[u8],
            selected_arch: Option<object::Architecture>,
        ) -> Result<object::File, Error>;

        fn parse_macho<'a, T: FatArch>(
            data: &'a [u8],
            fat_arches: &[T],
            selected_arch: Option<object::Architecture>,
        ) -> Result<object::File<'a>, Error>;

        fn vmaddr(&self) -> Result<Addr, Error>;
    }

    impl File for object::File<'_> {
        fn parse_data(
            data: &[u8],
            selected_arch: Option<object::Architecture>,
        ) -> Result<object::File, Error> {
            if let Ok(fat_arches) = macho::FatHeader::parse_arch32(data) {
                object::File::parse_macho(data, fat_arches, selected_arch)
            } else if let Ok(fat_arches) = macho::FatHeader::parse_arch64(data) {
                object::File::parse_macho(data, fat_arches, selected_arch)
            } else {
                Ok(object::File::parse(data)?)
            }
        }

        fn parse_macho<'a, T: FatArch>(
            data: &'a [u8],
            fat_arches: &[T],
            selected_arch: Option<object::Architecture>,
        ) -> Result<object::File<'a>, Error> {
            Ok(object::File::parse(read::macho::FatArch::data(
                if let Some(selected_arch) = selected_arch {
                    fat_arches
                        .iter()
                        .find(|fat_arch| selected_arch == fat_arch.architecture())
                        .ok_or(Error::CannotLoadSymbolsForArch(selected_arch))?
                } else {
                    fat_arches.first().ok_or(Error::CannotLoadSymbols)?
                },
                data,
            )?)?)
        }

        fn vmaddr(&self) -> Result<Addr, Error> {
            self.segments()
                .find_map(|seg| seg.name().ok()??.eq("__TEXT").then(|| seg.address()))
                .ok_or(Error::VmAddrTextSegmentNotFound)
                .map(Addr::from)
        }
    }

    pub trait Architecture {
        fn from_name(name: &str) -> Self;

        fn name(&self) -> String;
    }

    impl Architecture for object::Architecture {
        fn from_name(name: &str) -> Self {
            match name {
                "i386" | "x86" => Self::I386,
                "x86_64" | "x86_64h" => Self::X86_64,
                "x86_64_x32" => Self::X86_64_X32,
                "arm" | "aarch32" => Self::Arm,
                "armv4" | "armv4t" | "armv5tej" => Self::Arm,
                "armv6" | "armv6m" => Self::Arm,
                "armv7" | "armv7f" | "armv7s" | "armv7k" | "armv7m" | "armv7em" => Self::Arm,
                "armv8" | "armv8m" | "armv8r" => Self::Arm,
                "arm64" | "arm64v8" | "arm64e" | "aarch64" => Self::Aarch64,
                "arm64_32" | "arm64_32v8" => Self::Aarch64_Ilp32,
                "avr" => Self::Avr,
                "bpf" => Self::Bpf,
                "hexagon" => Self::Hexagon,
                "loongarch64" => Self::LoongArch64,
                "mips" => Self::Mips,
                "mips64" => Self::Mips64,
                "msp430" => Self::Msp430,
                "powerpc" => Self::PowerPc,
                "powerpc64" => Self::PowerPc64,
                "riscv32" => Self::Riscv32,
                "riscv64" => Self::Riscv64,
                "s390x" => Self::S390x,
                "sbf" => Self::Sbf,
                "sparc64" => Self::Sparc64,
                "wasm32" => Self::Wasm32,
                "xtensa" => Self::Xtensa,
                _ => Self::Unknown,
            }
        }

        fn name(&self) -> String {
            match self {
                Self::Aarch64 => String::from("arm64"),
                Self::Aarch64_Ilp32 => String::from("arm64_32"),
                _ => format!("{:?}", self).to_lowercase(),
            }
        }
    }
}
