#[cfg(target_arch = "x86_64")]
mod x86_64 {
    use gimli::{Register, X86_64};

    pub type UnwindWord = usize;
    pub type UnwindPtr = usize;

    pub struct Arch;

    #[allow(unused)]
    impl Arch {
        pub const SP: Register = X86_64::RSP;
        pub const RA: Register = X86_64::RA;

        pub const UNWIND_DATA_REG: (Register, Register) = (X86_64::RAX, X86_64::RDX);
        pub const UNWIND_PRIVATE_DATA_SIZE: usize = 6;
    }
}
#[cfg(target_arch = "x86_64")]
pub use x86_64::*;

#[cfg(target_arch = "x86")]
mod x86 {
    use gimli::{Register, X86};

    pub type UnwindWord = usize;
    pub type UnwindPtr = usize;

    pub struct Arch;

    #[allow(unused)]
    impl Arch {
        pub const SP: Register = X86::ESP;
        pub const RA: Register = X86::RA;

        pub const UNWIND_DATA_REG: (Register, Register) = (X86::EAX, X86::EDX);
        pub const UNWIND_PRIVATE_DATA_SIZE: usize = 5;
    }
}
#[cfg(target_arch = "x86")]
pub use x86::*;

#[cfg(any(
    target_arch = "mips64",
    target_arch = "mips64r6", 
    target_arch = "mips",
    target_arch = "mips32r6",
))]
mod mips {
    use gimli::{Register, MIPS};

    #[cfg(not(target_abi = "abin32"))]
    pub type UnwindWord = usize;
    #[cfg(target_abi = "abin32")]
    pub type UnwindWord = u64;
    pub type UnwindPtr = usize;

    pub struct Arch;

    #[allow(unused)]
    impl Arch {
        pub const SP: Register = MIPS::SP;
        pub const RA: Register = MIPS::RA;

        pub const UNWIND_DATA_REG: (Register, Register) = (MIPS::A0, MIPS::A1);
        pub const UNWIND_PRIVATE_DATA_SIZE: usize = 2;
    }
}
#[cfg(any(
    target_arch = "mips64",
    target_arch = "mips64r6", 
    target_arch = "mips",
    target_arch = "mips32r6",
))]
pub use mips::*;

#[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))]
mod riscv {
    use gimli::{Register, RiscV};

    pub type UnwindWord = usize;
    pub type UnwindPtr = usize;

    pub struct Arch;

    #[allow(unused)]
    impl Arch {
        pub const SP: Register = RiscV::SP;
        pub const RA: Register = RiscV::RA;

        pub const UNWIND_DATA_REG: (Register, Register) = (RiscV::A0, RiscV::A1);
        pub const UNWIND_PRIVATE_DATA_SIZE: usize = 2;
    }
}
#[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))]
pub use riscv::*;

#[cfg(target_arch = "aarch64")]
mod aarch64 {
    use gimli::{AArch64, Register};

    pub type UnwindWord = usize;
    pub type UnwindPtr = usize;

    pub struct Arch;

    #[allow(unused)]
    impl Arch {
        pub const SP: Register = AArch64::SP;
        pub const RA: Register = AArch64::X30;

        pub const UNWIND_DATA_REG: (Register, Register) = (AArch64::X0, AArch64::X1);
        pub const UNWIND_PRIVATE_DATA_SIZE: usize = 2;
    }
}
#[cfg(target_arch = "aarch64")]
pub use aarch64::*;

#[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_arch = "mips64",
    target_arch = "mips",
    target_arch = "riscv64",
    target_arch = "riscv32",
    target_arch = "aarch64"
)))]
compile_error!("Current architecture is not supported");
