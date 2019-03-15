use crate::asm;

use tock_registers::{registers::*, register_bitfields};

register_bitfields! {
    u64,
    ESR_EL1 [
        EC OFFSET(26) NUMBITS(6) [
            UNKNOWN     = 0x00,
            WFI         = 0x01,
            CP15_32     = 0x03,
            CP15_64     = 0x04,
            CP14_MR     = 0x05,
            CP14_LS     = 0x06,
            FP_ASIMD    = 0x07,
            CP10_ID     = 0x08,
            CP14_64     = 0x0C,
            ILL_ISS     = 0x0E,
            SVC32       = 0x11,
            SVC64       = 0x15,
            SYS64       = 0x18,
            IABT_EL0    = 0x20,
            IABT_EL1    = 0x21,
            PC_ALIGN    = 0x22,
            DABT_EL0    = 0x24,
            DABT_EL1    = 0x25,
            SP_ALIGN    = 0x26,
            FP_EXC32    = 0x28,
            FP_EXC64    = 0x2C,
            SERROR      = 0x2F,
            BREAKPT_EL0 = 0x30,
            BREAKPT_EL1 = 0x31,
            SOFTSTP_EL0 = 0x32,
            SOFTSTP_EL1 = 0x33,
            WATCHPT_EL0 = 0x34,
            WATCHPT_EL1 = 0x35,
            BKPT32      = 0x38,
            BRK64       = 0x3C
        ],
        IL OFFSET(25) NUMBITS(1) [
            Bit16 = 0,
            Bit32 = 1
        ],
        ISSValid OFFSET(24) NUMBITS(1) [
            NotValid = 0,
            Valid = 1
        ],
        ISS OFFSET(0) NUMBITS(23) []
    ]
}

#[repr(C)]
struct Context {
    x: [u64; 31],
    spsr_el1: u64,
    elr_el1: u64,
    esr_el1: ReadOnly<u64, ESR_EL1::Register>,
    far_el1: u64
}

#[allow(non_snake_case)]
impl core::fmt::Display for ESR_EL1::EC::Value {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let message = match self {
            ESR_EL1::EC::Value::UNKNOWN     => "UNKNOWN",
            ESR_EL1::EC::Value::WFI         => "WFI",
            ESR_EL1::EC::Value::CP15_32     => "CP15_32",
            ESR_EL1::EC::Value::CP15_64     => "CP15_64",
            ESR_EL1::EC::Value::CP14_MR     => "CP14_MR",
            ESR_EL1::EC::Value::CP14_LS     => "CP14_LS",
            ESR_EL1::EC::Value::FP_ASIMD    => "FP_ASIMD",
            ESR_EL1::EC::Value::CP10_ID     => "CP10_ID",
            ESR_EL1::EC::Value::CP14_64     => "CP14_64",
            ESR_EL1::EC::Value::ILL_ISS     => "ILL_ISS",
            ESR_EL1::EC::Value::SVC32       => "SVC32",
            ESR_EL1::EC::Value::SVC64       => "SVC64",
            ESR_EL1::EC::Value::SYS64       => "SYS64",
            ESR_EL1::EC::Value::IABT_EL0    => "IABT_EL0",
            ESR_EL1::EC::Value::IABT_EL1    => "IABT_EL1",
            ESR_EL1::EC::Value::PC_ALIGN    => "PC_ALIGN",
            ESR_EL1::EC::Value::DABT_EL0    => "DABT_EL0",
            ESR_EL1::EC::Value::DABT_EL1    => "DABT_EL1",
            ESR_EL1::EC::Value::SP_ALIGN    => "SP_ALIGN",
            ESR_EL1::EC::Value::FP_EXC32    => "FP_EXC32",
            ESR_EL1::EC::Value::FP_EXC64    => "FP_EXC64",
            ESR_EL1::EC::Value::SERROR      => "SERROR",
            ESR_EL1::EC::Value::BREAKPT_EL0 => "BREAKPT_EL0",
            ESR_EL1::EC::Value::BREAKPT_EL1 => "BREAKPT_EL1",
            ESR_EL1::EC::Value::SOFTSTP_EL0 => "SOFTSTP_EL0",
            ESR_EL1::EC::Value::SOFTSTP_EL1 => "SOFTSTP_EL1",
            ESR_EL1::EC::Value::WATCHPT_EL0 => "WATCHPT_EL0",
            ESR_EL1::EC::Value::WATCHPT_EL1 => "WATCHPT_EL1",
            ESR_EL1::EC::Value::BKPT32      => "BKPT32",
            ESR_EL1::EC::Value::BRK64       => "BRK64",
        };
        f.write_str(message)
    }
}

impl core::fmt::Display for Context {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let t = if let Some(ec) = self.esr_el1.read_as_enum::<ESR_EL1::EC::Value>(ESR_EL1::EC) {
            ec
        } else {
            ESR_EL1::EC::Value::UNKNOWN
        };
        write!(
            f,
            "\
            ESR    {}/{:#X}\n\
            FAR    {:#017X}\n\
            ELR    {:#X}\n\
            PSTATE {:#X}\
            ",
            t,
            self.esr_el1.get(),
            self.far_el1,
            self.elr_el1,
            self.spsr_el1
        )
    }
}

#[no_mangle]
unsafe extern "C" fn current_elx_sp0_synchronous(c: &mut Context) {
    println_sync!("current_elx_sp0_synchronous\n{}", c);
    loop {
        asm::wfe();
    }
}

#[no_mangle]
unsafe extern "C" fn current_elx_sp0_irq(c: &mut Context) {
    println_sync!("current_elx_sp0_irq\n{}", c);
    loop {
        asm::wfe();
    }
}

#[no_mangle]
unsafe extern "C" fn current_elx_sp0_fiq(c: &mut Context) {
    println_sync!("current_elx_sp0_fiq\n{}", c);
    loop {
        asm::wfe();
    }
}

#[no_mangle]
unsafe extern "C" fn current_elx_sp0_serror(c: &mut Context) {
    println_sync!("current_elx_sp0_serror\n{}", c);
    loop {
        asm::wfe();
    }
}

#[no_mangle]
unsafe extern "C" fn current_elx_synchronous(c: &mut Context) {
    println_sync!("current_elx_synchronous\n{}", c);
    loop {
        asm::wfe();
    }
}

#[no_mangle]
unsafe extern "C" fn current_elx_irq(_c: &mut Context) {
    crate::MINIUART.interrupt_disable();
    crate::MINIUART.io();
}

#[no_mangle]
unsafe extern "C" fn current_elx_fiq(c: &mut Context) {
    println_sync!("current_elx_fiq\n{}", c);
    loop {
        asm::wfe();
    }
}

#[no_mangle]
unsafe extern "C" fn current_elx_serror(c: &mut Context) {
    println_sync!("current_elx_serror\n{}", c);
    loop {
        asm::wfe();
    }
}

#[no_mangle]
unsafe extern "C" fn lower_aarch64_synchronous(c: &mut Context) {
    println_sync!("lower_aarch64_synchronous\n{}", c);
    loop {
        asm::wfe();
    }
}

#[no_mangle]
unsafe extern "C" fn lower_aarch64_irq(c: &mut Context) {
    println_sync!("lower_aarch64_irq\n{}", c);
    loop {
        asm::wfe();
    }
}

#[no_mangle]
unsafe extern "C" fn lower_aarch64_fiq(c: &mut Context) {
    println_sync!("lower_aarch64_fiq\n{}", c);
    loop {
        asm::wfe();
    }
}

#[no_mangle]
unsafe extern "C" fn lower_aarch64_serror(c: &mut Context) {
    println_sync!("lower_aarch64_serror\n{}", c);
    loop {
        asm::wfe();
    }
}

#[no_mangle]
unsafe extern "C" fn lower_aarch32_synchronous(c: &mut Context) {
    println_sync!("lower_aarch32_synchronous\n{}", c);
    loop {
        asm::wfe();
    }
}

#[no_mangle]
unsafe extern "C" fn lower_aarch32_irq(c: &mut Context) {
    println_sync!("lower_aarch32_irq\n{}", c);
    loop {
        asm::wfe();
    }
}

#[no_mangle]
unsafe extern "C" fn lower_aarch32_fiq(c: &mut Context) {
    println_sync!("lower_aarch32_fiq\n{}", c);
    loop {
        asm::wfe();
    }
}

#[no_mangle]
unsafe extern "C" fn lower_aarch32_serror(c: &mut Context) {
    println_sync!("lower_aarch32_serror\n{}", c);
    loop {
        asm::wfe();
    }
}
