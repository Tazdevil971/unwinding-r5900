use crate::arch::UnwindWord;
use core::{fmt, ops};
use gimli::{MIPS, Register};

// Match DWARF_FRAME_REGISTERS in libgcc
pub const MAX_REG_RULES: usize = 188;

#[repr(C)]
#[derive(Clone, Default)]
pub struct Context {
    #[cfg(not(target_abi = "abin32"))]
    pub gp: [usize; 32],
    #[cfg(target_abi = "abin32")]
    pub gp: [u64; 32],

    #[cfg(all(
        not(feature = "soft-float"), 
        target_feature = "fp64"
    ))]
    pub fp: [u64; 32],
    #[cfg(all(
        not(feature = "soft-float"), 
        not(target_feature = "fp64")
    ))]
    pub fp: [u32; 32],
}

impl fmt::Debug for Context {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt = fmt.debug_struct("Context");

        for i in 0..=32 {
            fmt.field(
                MIPS::register_name(Register(i as u16)).unwrap(),
                &self.gp[i]  
            );
        }

        #[cfg(not(feature = "soft-float"))]
        for i in 0..=32 {
            fmt.field(
                MIPS::register_name(Register(32 + i as u16)).unwrap(),
                &self.fp[i]
            );
        }

        fmt.finish()
    }
}

impl ops::Index<Register> for Context {
    type Output = UnwindWord;

    fn index(&self, reg: Register) -> &UnwindWord {
        match reg {
            Register(0..=32) => &self.gp[reg.0 as usize],
            _ => unimplemented!()
        }
    }
}

impl ops::IndexMut<Register> for Context {
    fn index_mut(&mut self, reg: Register) -> &mut UnwindWord {
        match reg {
            Register(0..=32) => &mut self.gp[reg.0 as usize],
            _ => unimplemented!()
        }
    }
}

macro_rules! code {
    (save_prelude $context_size:literal) => {
        concat!(
            "
            .set noreorder
            .set noat
            move $t0, $sp
            sub $sp, $sp, ", $context_size, " + 0x10
            .cfi_def_cfa_offset ", $context_size, " + 0x10
            sd $ra, ", $context_size, "($sp)
            .cfi_offset $ra, -16
            "
        )
    };
    (save_postlude $context_size:literal) => {
        concat!(
            "
            move $t9, $a0
            move $a0, $sp
            jalr $t9
            nop
    
            ld $ra, ", $context_size, "($sp)
            add $sp, $sp, ", $context_size, " + 0x10
            .cfi_def_cfa_offset 0
            .cfi_restore $ra
            jr $ra
            nop
            .set at
            .set reorder
            "
        )
    };
    (save_gp64) => {
        "
        sd $s0, 0x80($sp)
        sd $s1, 0x88($sp)
        sd $s2, 0x90($sp)
        sd $s3, 0x98($sp)
        sd $s4, 0xa0($sp)
        sd $s5, 0xa8($sp)
        sd $s6, 0xb0($sp)
        sd $s7, 0xb8($sp)
        sd $k0, 0xd0($sp)
        sd $k1, 0xd8($sp)
        sd $gp, 0xe0($sp)
        sd $t0, 0xe8($sp)
        sd $fp, 0xf0($sp)
        sd $ra, 0xf8($sp)
        "
    };
    (save_fp32) => {
        "
        swc1 $f0, 0x100($sp)
        swc1 $f1, 0x104($sp)
        swc1 $f2, 0x108($sp)
        swc1 $f3, 0x10c($sp)
        swc1 $f4, 0x110($sp)
        swc1 $f5, 0x114($sp)
        swc1 $f6, 0x118($sp)
        swc1 $f7, 0x11c($sp)
        swc1 $f8, 0x120($sp)
        swc1 $f9, 0x124($sp)
        swc1 $f10, 0x128($sp)
        swc1 $f11, 0x12c($sp)
        swc1 $f12, 0x130($sp)
        swc1 $f13, 0x134($sp)
        swc1 $f14, 0x138($sp)
        swc1 $f15, 0x13c($sp)
        swc1 $f16, 0x140($sp)
        swc1 $f17, 0x144($sp)
        swc1 $f18, 0x148($sp)
        swc1 $f19, 0x14c($sp)
        swc1 $f20, 0x150($sp)
        swc1 $f21, 0x154($sp)
        swc1 $f22, 0x158($sp)
        swc1 $f23, 0x15c($sp)
        swc1 $f24, 0x160($sp)
        swc1 $f25, 0x164($sp)
        swc1 $f26, 0x168($sp)
        swc1 $f27, 0x16c($sp)
        swc1 $f28, 0x170($sp)
        swc1 $f29, 0x174($sp)
        swc1 $f30, 0x178($sp)
        swc1 $f31, 0x17c($sp)
        "
    };
    (save_fp64) => {
        "
        sdc1 $f0, 0x100($sp)
        sdc1 $f1, 0x108($sp)
        sdc1 $f2, 0x110($sp)
        sdc1 $f3, 0x118($sp)
        sdc1 $f4, 0x120($sp)
        sdc1 $f5, 0x128($sp)
        sdc1 $f6, 0x130($sp)
        sdc1 $f7, 0x138($sp)
        sdc1 $f8, 0x140($sp)
        sdc1 $f9, 0x148($sp)
        sdc1 $f10, 0x150($sp)
        sdc1 $f11, 0x158($sp)
        sdc1 $f12, 0x160($sp)
        sdc1 $f13, 0x168($sp)
        sdc1 $f14, 0x170($sp)
        sdc1 $f15, 0x178($sp)
        sdc1 $f16, 0x180($sp)
        sdc1 $f17, 0x188($sp)
        sdc1 $f18, 0x190($sp)
        sdc1 $f19, 0x198($sp)
        sdc1 $f20, 0x1a0($sp)
        sdc1 $f21, 0x1a8($sp)
        sdc1 $f22, 0x1b0($sp)
        sdc1 $f23, 0x1b8($sp)
        sdc1 $f24, 0x1c0($sp)
        sdc1 $f25, 0x1c8($sp)
        sdc1 $f26, 0x1d0($sp)
        sdc1 $f27, 0x1d8($sp)
        sdc1 $f28, 0x1e0($sp)
        sdc1 $f29, 0x1e8($sp)
        sdc1 $f30, 0x1f0($sp)
        sdc1 $f31, 0x1f8($sp)
        "
    };
    (restore_prelude) => {
        "
        .set noreorder
        .set noat
        "
    };
    (restore_postlude) => {
        "
        ld $a0, 0x20($a0)
        jr $ra
        nop
        .set at
        .set reorder
        "
    };
    (restore_gp64) => {
        "
        ld $at, 0x08($a0)
        ld $v0, 0x10($a0)
        ld $v1, 0x18($a0)
        ld $a1, 0x28($a0)
        ld $a2, 0x30($a0)
        ld $a3, 0x38($a0)
        ld $t0, 0x40($a0)
        ld $t1, 0x48($a0)
        ld $t2, 0x50($a0)
        ld $t3, 0x58($a0)
        ld $12, 0x60($a0)
        ld $13, 0x68($a0)
        ld $14, 0x70($a0)
        ld $15, 0x78($a0)
        ld $s0, 0x80($a0)
        ld $s1, 0x88($a0)
        ld $s2, 0x90($a0)
        ld $s3, 0x98($a0)
        ld $s4, 0xa0($a0)
        ld $s5, 0xa8($a0)
        ld $s6, 0xb0($a0)
        ld $s7, 0xb8($a0)
        ld $t8, 0xc0($a0)
        ld $t9, 0xc8($a0)
        ld $k0, 0xd0($a0)
        ld $k1, 0xd8($a0)
        ld $gp, 0xe0($a0)
        ld $sp, 0xe8($a0)
        ld $fp, 0xf0($a0)
        ld $ra, 0xf8($a0)
        "
    };
    (restore_fp32) => {
        "
        lwc1 $f0, 0x100($a0)
        lwc1 $f1, 0x104($a0)
        lwc1 $f2, 0x108($a0)
        lwc1 $f3, 0x10c($a0)
        lwc1 $f4, 0x110($a0)
        lwc1 $f5, 0x114($a0)
        lwc1 $f6, 0x118($a0)
        lwc1 $f7, 0x11c($a0)
        lwc1 $f8, 0x120($a0)
        lwc1 $f9, 0x124($a0)
        lwc1 $f10, 0x128($a0)
        lwc1 $f11, 0x12c($a0)
        lwc1 $f12, 0x130($a0)
        lwc1 $f13, 0x134($a0)
        lwc1 $f14, 0x138($a0)
        lwc1 $f15, 0x13c($a0)
        lwc1 $f16, 0x140($a0)
        lwc1 $f17, 0x144($a0)
        lwc1 $f18, 0x148($a0)
        lwc1 $f19, 0x14c($a0)
        lwc1 $f20, 0x150($a0)
        lwc1 $f21, 0x154($a0)
        lwc1 $f22, 0x158($a0)
        lwc1 $f23, 0x15c($a0)
        lwc1 $f24, 0x160($a0)
        lwc1 $f25, 0x164($a0)
        lwc1 $f26, 0x168($a0)
        lwc1 $f27, 0x16c($a0)
        lwc1 $f28, 0x170($a0)
        lwc1 $f29, 0x174($a0)
        lwc1 $f30, 0x178($a0)
        lwc1 $f31, 0x17c($a0)
        "
    };
    (restore_fp64) => {
        "
        ldc1 $f0, 0x100($a0)
        ldc1 $f1, 0x108($a0)
        ldc1 $f2, 0x110($a0)
        ldc1 $f3, 0x118($a0)
        ldc1 $f4, 0x120($a0)
        ldc1 $f5, 0x128($a0)
        ldc1 $f6, 0x130($a0)
        ldc1 $f7, 0x138($a0)
        ldc1 $f8, 0x140($a0)
        ldc1 $f9, 0x148($a0)
        ldc1 $f10, 0x150($a0)
        ldc1 $f11, 0x158($a0)
        ldc1 $f12, 0x160($a0)
        ldc1 $f13, 0x168($a0)
        ldc1 $f14, 0x170($a0)
        ldc1 $f15, 0x178($a0)
        ldc1 $f16, 0x180($a0)
        ldc1 $f17, 0x188($a0)
        ldc1 $f18, 0x190($a0)
        ldc1 $f19, 0x198($a0)
        ldc1 $f20, 0x1a0($a0)
        ldc1 $f21, 0x1a8($a0)
        ldc1 $f22, 0x1b0($a0)
        ldc1 $f23, 0x1b8($a0)
        ldc1 $f24, 0x1c0($a0)
        ldc1 $f25, 0x1c8($a0)
        ldc1 $f26, 0x1d0($a0)
        ldc1 $f27, 0x1d8($a0)
        ldc1 $f28, 0x1e0($a0)
        ldc1 $f29, 0x1e8($a0)
        ldc1 $f30, 0x1f0($a0)
        ldc1 $f31, 0x1f8($a0)
        "
    };
}

#[naked]
pub extern "C-unwind" fn save_context(f: extern "C" fn(&mut Context, *mut ()), ptr: *mut ()) {
    unsafe {
        #[cfg(all(
            not(feature = "soft-float"), 
            target_feature = "fp64"
        ))]
        core::arch::naked_asm!(
            code!(save_prelude 0x200),
            code!(save_gp64),
            code!(save_fp64),
            code!(save_postlude 0x200),
        );
        #[cfg(all(
            not(feature = "soft-float"), 
            not(target_feature = "fp64")
        ))]
        core::arch::naked_asm!(
            code!(save_prelude 0x180),
            code!(save_gp64),
            code!(save_fp32),
            code!(save_postlude 0x180),
        );
        #[cfg(feature = "soft-float")]
        core::arch::naked_asm!(
            code!(save_prelude 0x100),
            code!(save_gp64),
            code!(save_postlude 0x100),
        );
    }
}

pub unsafe fn restore_context(ctx: &Context) -> ! {
    unsafe {
        #[cfg(all(
            not(feature = "soft-float"), 
            target_feature = "fp64"
        ))]
        core::arch::asm!(
            code!(restore_prelude),
            code!(restore_gp64),
            code!(restore_fp64),
            code!(restore_postlude),
            in("$4") ctx,
            options(noreturn),
        );
        #[cfg(all(
            not(feature = "soft-float"), 
            not(target_feature = "fp64")
        ))]
        core::arch::asm!(
            code!(restore_prelude),
            code!(restore_gp64),
            code!(restore_fp32),
            code!(restore_postlude),
            in("$4") ctx,
            options(noreturn)
        );
        #[cfg(feature = "soft-float")]
        core::arch::asm!(
            code!(restore_prelude),
            code!(restore_gp64),
            code!(restore_postlude),
            in("$4") ctx,
            options(noreturn)
        );
    }
}