use alloc::collections::VecDeque;
use core::fmt::Write;
use core::option::Option;
use crate::dev::board::bcm2837::*;
use crate::asm;

pub struct MiniUart {
    aux: *const AUX,
    gpio: *const GPIO,
    pub input: Option<VecDeque<u8>>,
    pub output: Option<VecDeque<u8>>,
}

impl MiniUart { 
    pub const fn new() -> MiniUart {
        MiniUart {
            aux: AUX_BASE as *const AUX,
            gpio: GPIO_BASE as *const GPIO,
            input: None,
            output: None
        }
    }
    pub fn init(&self) {
        unsafe {
            // Enable UART module (and not touching other enabled modules)
            (*self.aux).AUX_ENABLES.modify(AUX_ENABLES::MINI_UART_ENABLE::SET);

            // Disable receive and transmit
            (*self.aux).AUX_MU_CNTL_REG.write(
                AUX_MU_CNTL_REG::RECEIVE::CLEAR +
                AUX_MU_CNTL_REG::TRANSMIT::CLEAR
            );

            (*self.aux).AUX_MU_LCR_REG.write(
                // Disable the break condition
                AUX_MU_LCR_REG::BREAK::CLEAR +
                // Eight bit mode
                AUX_MU_LCR_REG::DATA_SIZE::EightBit
            );

            // Disable interrupts
            (*self.aux).AUX_MU_IER_REG.write(
                AUX_MU_IER_REG::INTERRUPT_ENABLE::CLEAR +
                AUX_MU_IER_REG::INTERRUPT_EMPTY::CLEAR +
                AUX_MU_IER_REG::INTERRUPT_HAS_BYTE::CLEAR
            );

            // Set RTS line to high
            (*self.aux).AUX_MU_MCR_REG.set(0);

            // Clear both FIFOs (receive, transmit)
            (*self.aux).AUX_MU_IIR_REG.write(AUX_MU_IIR_REG::FIFO_CLEAR::Both);

            // set baudrate: baudrate = system_clock_freq / (8 * (AUX_MU_BAUD_REG + 1))
            // 115313 = 250000000 / (8 * (270 + 1)) (this is the closest you can get to 115200)
            (*self.aux).AUX_MU_BAUD_REG.write(AUX_MU_BAUD_REG::RATE.val(270));
        }

        unsafe {
            // Set 14 and 15 pins to alternate 5 (MiniUART)
            (*self.gpio).GPFSEL1.modify(
                GPFSEL1::FSEL14::Alternate5 +
                GPFSEL1::FSEL15::Alternate5
            );

            // BCM2837-ARM-Peripherals.-.Revised.-.V2-1.pdf - page 100
            // The GPIO Pull-up/down Clock Registers control the actuation of internal pull-downs on
            // the respective GPIO pins. These registers must be used in conjunction with the GPPUD
            // register to effect GPIO Pull-up/down changes. The following sequence of events is
            // required:
            // 1. Write to GPPUD to set the required control signal (i.e. Pull-up or Pull-Down or neither
            // to remove the current Pull-up/down)
            // 2. Wait 150 cycles – this provides the required set-up time for the control signal
            // 3. Write to GPPUDCLK0/1 to clock the control signal into the GPIO pads you wish to
            // modify – NOTE only the pads which receive a clock will be modified, all others will
            // retain their previous state.
            // 4. Wait 150 cycles – this provides the required hold time for the control signal
            // 5. Write to GPPUD to remove the control signal
            // 6. Write to GPPUDCLK0/1 to remove the clock

            // 1. toggle pin
            (*self.gpio).GPPUD.set(0);

            // 2. Wait 150 cycles
            for _ in 0..150 {
                asm::nop();
            }

            // 3. We want to toggle 14 and 15
            (*self.gpio).GPPUDCLK0.write(
                GPPUDCLK0::PUDCLK14::AssertClock + GPPUDCLK0::PUDCLK15::AssertClock,
            );

            // 4. Wait 150 cycles
            for _ in 0..150 {
                asm::nop();
            }

            // 5. Removing control signal
            (*self.gpio).GPPUD.set(0);

            // 6. Removing the clock
            (*self.gpio).GPPUDCLK0.set(0);
        }

        unsafe {
            // enable transmit and receive
            (*self.aux).AUX_MU_CNTL_REG.write(
                AUX_MU_CNTL_REG::RECEIVE::SET +
                AUX_MU_CNTL_REG::TRANSMIT::SET
            );

            // enable aux interrupt
            (*(0x3F00_B210 as *mut u32)) = 1 << 29;
        }
    }
    #[inline]
    pub fn interrupt_disable(&self) {
        unsafe {
            (*self.aux).AUX_MU_IER_REG.write(
                AUX_MU_IER_REG::INTERRUPT_ENABLE::CLEAR +
                AUX_MU_IER_REG::INTERRUPT_EMPTY::CLEAR +
                AUX_MU_IER_REG::INTERRUPT_HAS_BYTE::CLEAR
            );
        }
    }
    #[inline]
    pub fn interrupt_enable(&self) {
        unsafe {
            (*self.aux).AUX_MU_IER_REG.write(
                AUX_MU_IER_REG::INTERRUPT_ENABLE::SET +
                AUX_MU_IER_REG::INTERRUPT_EMPTY::CLEAR +
                AUX_MU_IER_REG::INTERRUPT_HAS_BYTE::SET
            );
        }
    }
    #[inline]
    pub fn try_read_char(&mut self) {
        if self.input.is_none() {
            self.input = Some(VecDeque::with_capacity(255));
        }
        let input_queue = self.input.as_mut().unwrap();
        unsafe {
            if (*self.aux).AUX_MU_LSR_REG.is_set(AUX_MU_LSR_REG::DATA_READY) {
                let c = (*self.aux).AUX_MU_IO_REG.get() as u8;
                input_queue.push_back(c);
            }
        }
    }
    pub fn try_get_char(&mut self) -> Option<char> {
        if let Some(c) = self.input.as_mut()?.pop_front() {
            return Some(c as char)
        }
        None
    }
    pub fn push_char(&mut self, c: char) {
        if self.output.is_none() {
            self.output = Some(VecDeque::with_capacity(255));
        }
        let output_queue = self.output.as_mut().unwrap();
        output_queue.push_back(c as u8);
    }
    pub fn read_char(&self) -> char {
        unsafe {
            // check if we can read
            while !(*self.aux).AUX_MU_LSR_REG.is_set(AUX_MU_LSR_REG::DATA_READY) {
                asm::nop();
            }
            // we can
            let mut ret = (*self.aux).AUX_MU_IO_REG.get() as u8 as char;
            if ret == '\r' {
                ret = '\n';
            }
            ret
        }
    }
    pub fn character_available(&self) -> bool {
        return self.input.is_some() && !self.input.as_ref().unwrap().is_empty();
    }
    pub fn flush(&mut self) {
        if self.output.is_none() {
            return;
        }
        while !self.output.as_mut().unwrap().is_empty() {
            unsafe {
                while !(*self.aux).AUX_MU_LSR_REG.is_set(AUX_MU_LSR_REG::TRANSMIT_EMPTY) {
                    asm::nop();
                }
                self.interrupt_disable();
                let c = self.output.as_mut().unwrap().pop_front().unwrap();
                (*self.aux).AUX_MU_IO_REG.set(c as u32);
                self.interrupt_enable();
            }
        }
    }
}

impl Write for MiniUart {
    fn write_char(&mut self, c: char) -> core::fmt::Result {
        unsafe {
            // check if we can write
            while !(*self.aux).AUX_MU_LSR_REG.is_set(AUX_MU_LSR_REG::TRANSMIT_EMPTY) {
                asm::nop();
            }
            // we can
            (*self.aux).AUX_MU_IO_REG.set(c as u32);
        }
        Ok(())
    }
    fn write_str(&mut self, input: &str) -> core::fmt::Result {
        for c in input.chars() {
            if c == '\n' {
                self.write_char('\r')?;
            }
            self.write_char(c)?;
        }
        Ok(())
    }
}
