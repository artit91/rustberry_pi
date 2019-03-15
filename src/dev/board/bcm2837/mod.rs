// https://github.com/raspberrypi/documentation/files/1888662/BCM2837-ARM-Peripherals.-.Revised.-.V2-1.pdf

use tock_registers::{registers::*, register_bitfields};

pub const MMIO_BASE: u32 = 0x3F00_0000;
pub const AUX_BASE: u32 = MMIO_BASE + 0x21_5000;
pub const GPIO_BASE: u32 = MMIO_BASE + 0x20_0000;

register_bitfields! {
    u32,
    // GPIO
    GPFSEL1 [
        FSEL15 OFFSET(15) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            Alternate5 = 0b010
        ],
        FSEL14 OFFSET(12) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            Alternate5 = 0b010
        ]
    ],
    GPPUDCLK0 [
        PUDCLK15 OFFSET(15) NUMBITS(1) [
            NoEffect = 0,
            AssertClock = 1
        ],
        PUDCLK14 OFFSET(14) NUMBITS(1) [
            NoEffect = 0,
            AssertClock = 1
        ]
    ],
    // AUX
    AUX_ENABLES [
        MINI_UART_ENABLE OFFSET(0) NUMBITS(1) [],
        SPI1_ENABLE OFFSET(1) NUMBITS(1) [],
        SPI2_ENABLE OFFSET(2) NUMBITS(1) []
    ],
    AUX_MU_IER_REG [
        INTERRUPT_ENABLE OFFSET(2) NUMBITS(1) [],
        INTERRUPT_EMPTY OFFSET(1) NUMBITS(1) [],
        INTERRUPT_HAS_BYTE OFFSET(0) NUMBITS(1) []
    ],
    AUX_MU_IIR_REG [
        FIFO_ENABLED OFFSET(6) NUMBITS(2) [
            Enabled = 0b11
        ],
        // Read
        INTERRUPT OFFSET(1) NUMBITS(2) [
            IsEmpty = 0b01,
            HasByte = 0b10,
            NoInterrupt = 0b00
        ],
        // Write
        FIFO_CLEAR OFFSET(1) NUMBITS(2) [
            Receive = 0b01,
            Transmit = 0b10,
            Both = 0b11
        ],
        INTERRUPT_NOT_PENDING OFFSET(0) NUMBITS(1) [] 
    ],
    AUX_MU_LCR_REG [
        BREAK OFFSET(6) NUMBITS(1) [],
        DATA_SIZE OFFSET(0) NUMBITS(2) [
            SevenBit = 0b00,
            EightBit = 0b11
        ]
    ],
    AUX_MU_LSR_REG [
        TRANSMIT_EMPTY OFFSET(5) NUMBITS(1) [],
        DATA_READY OFFSET(0) NUMBITS(1) []
    ],
    AUX_MU_CNTL_REG [
        TRANSMIT OFFSET(1) NUMBITS(1) [],
        RECEIVE OFFSET(0) NUMBITS(1) []
    ],
    AUX_MU_BAUD_REG [
        RATE OFFSET(0) NUMBITS(16) []
    ]
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct AUX {
    pub AUX_IRQ: ReadWrite<u32>,                                    // 0x00
    pub AUX_ENABLES: ReadWrite<u32, AUX_ENABLES::Register>,         // 0x04
    __reserved_1: [u32; 14],                                        // 0x08
    pub AUX_MU_IO_REG: ReadWrite<u32>,                              // 0x40
    pub AUX_MU_IER_REG: ReadWrite<u32, AUX_MU_IER_REG::Register>,   // 0x44
    pub AUX_MU_IIR_REG: WriteOnly<u32, AUX_MU_IIR_REG::Register>,   // 0x48
    pub AUX_MU_LCR_REG: WriteOnly<u32, AUX_MU_LCR_REG::Register>,   // 0x4C
    pub AUX_MU_MCR_REG: WriteOnly<u32>,                             // 0x50
    pub AUX_MU_LSR_REG: ReadOnly<u32, AUX_MU_LSR_REG::Register>,    // 0x54
    pub AUX_MU_MSR_REG: ReadOnly<u32>,                              // 0x58
    pub AUX_MU_SCRATCH: ReadWrite<u32>,                             // 0x5C
    pub AUX_MU_CNTL_REG: WriteOnly<u32, AUX_MU_CNTL_REG::Register>, // 0x60
    pub AUX_MU_STAT_REG: ReadOnly<u32>,                             // 0x64
    pub AUX_MU_BAUD_REG: WriteOnly<u32, AUX_MU_BAUD_REG::Register>, // 0x68
    __reserved_2: [u32; 5],                                         // 0x6C
    pub AUX_SPI1_CNTL0_REG: ReadWrite<u32>,                         // 0x80
    pub AUX_SPI1_CNTL1_REG: ReadWrite<u32>,                         // 0x84
    pub AUX_SPI1_STAT_REG: ReadWrite<u32>,                          // 0x88
    __reserved_3: u32,                                              // 0x8C
    pub AUX_SPI1_IO_REG: ReadWrite<u32>,                            // 0x90
    pub AUX_SPI1_PEEK_REG: ReadOnly<u32>,                           // 0x94
    __reserved_4: [u32; 10],                                        // 0x98
    pub AUX_SPI2_CNTL0_REG: ReadWrite<u32>,                         // 0xC0
    pub AUX_SPI2_CNTL1_REG: ReadWrite<u32>,                         // 0xC4
    pub AUX_SPI2_STAT_REG: ReadWrite<u32>,                          // 0xC8
    __reserved_5: u32,                                              // 0xCC
    pub AUX_SPI2_IO_REG: ReadWrite<u32>,                            // 0xD0
    pub AUX_SPI2_PEEK_REG: ReadOnly<u32>,                           // 0xD4
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct GPIO {
    pub GPFSEL0: ReadWrite<u32>,                        // 0x00
    pub GPFSEL1: ReadWrite<u32, GPFSEL1::Register>,     // 0x04
    pub GPFSEL2: ReadWrite<u32>,                        // 0x08
    pub GPFSEL3: ReadWrite<u32>,                        // 0x0C
    pub GPFSEL4: ReadWrite<u32>,                        // 0x10
    pub GPFSEL5: ReadWrite<u32>,                        // 0x14
    __reserved_0: u32,                                  // 0x18
    pub GPSET0: WriteOnly<u32>,                         // 0x1C
    pub GPSET1: WriteOnly<u32>,                         // 0x20
    __reserved_1: u32,                                  // 0x24
    pub GPCLR0: WriteOnly<u32>,                         // 0x28
    pub GPCLR1: WriteOnly<u32>,                         // 0x2C
    __reserved_2: u32,                                  // 0x30
    pub GPLEV0: ReadOnly<u32>,                          // 0x34
    pub GPLEV1: ReadOnly<u32>,                          // 0x38
    __reserved_3: u32,                                  // 0x3C
    pub GPEDS0: ReadWrite<u32>,                         // 0x40
    pub GPEDS1: ReadWrite<u32>,                         // 0x44
    __reserved_4: u32,                                  // 0x48
    pub GPREN0: ReadWrite<u32>,                         // 0x4C
    pub GPREN1: ReadWrite<u32>,                         // 0x50
    __reserved_5: u32,                                  // 0x54
    pub GPFEN0: ReadWrite<u32>,                         // 0x58
    pub GPFEN1: ReadWrite<u32>,                         // 0x5C 
    __reserved_6: u32,                                  // 0x60
    pub GPHEN0: ReadWrite<u32>,                         // 0x64
    pub GPHEN1: ReadWrite<u32>,                         // 0x68
    __reserved_7: u32,                                  // 0x6C
    pub GPLEN0: ReadWrite<u32>,                         // 0x70
    pub GPLEN1: ReadWrite<u32>,                         // 0x74
    __reserved_8: u32,                                  // 0x78
    pub GPAREN0: ReadWrite<u32>,                        // 0x7C
    pub GPAREN1: ReadWrite<u32>,                        // 0x80
    __reserved_9: u32,                                  // 0x84
    pub GPAFEN0: ReadWrite<u32>,                        // 0x88
    pub GPAFEN1: ReadWrite<u32>,                        // 0x8C
    __reserved_10: u32,                                 // 0x90
    pub GPPUD: ReadWrite<u32>,                          // 0x94
    pub GPPUDCLK0: ReadWrite<u32, GPPUDCLK0::Register>, // 0x98
    pub GPPUDCLK1: ReadWrite<u32>,                      // 0x9C
    __reserved_11: [u32; 4],                            // 0xA0
    __test: u32                                         // 0xB0
}
