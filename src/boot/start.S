// it should be aligned to 128 bytes
.macro CONTEXT_SWITCH handler
.balign 0x80
    // allocating memory on the stack (16 byte alignment * 18)
    sub sp, sp, #16 * 18

    // store pairs: (a, b) [dest, slot]
    stp  x0,  x1, [sp, #16 *  0]
    stp  x2,  x3, [sp, #16 *  1]
    stp  x4,  x5, [sp, #16 *  2]
    stp  x6,  x7, [sp, #16 *  3]
    stp  x8,  x9, [sp, #16 *  4]
    stp x10, x11, [sp, #16 *  5]
    stp x12, x13, [sp, #16 *  6]
    stp x14, x15, [sp, #16 *  7]
    stp x16, x17, [sp, #16 *  8]
    stp x18, x19, [sp, #16 *  9]
    stp x20, x21, [sp, #16 * 10]
    stp x22, x23, [sp, #16 * 11]
    stp x24, x25, [sp, #16 * 12]
    stp x26, x27, [sp, #16 * 13]

    stp x28, x29, [sp, #16 * 14]

    // move the content of the program status register to x1
    mrs x1, SPSR_EL1
    // move the content of the exception link register to x2
    mrs x2, ELR_EL1
    // move the content of the syndrome register to x3 
    mrs x3, ESR_EL1
    // move the content of the fault address register to x4
    mrs x4, FAR_EL1

    // storing x30, x1, x2, x3 on the stack
    stp x30,  x1, [sp, #16 * 15]
    stp  x2,  x3, [sp, #16 * 16]
    str  x4,      [sp, #16 * 17]

    // setting the x0 to the stack pointer
    mov x0, sp
    // branch with link (copying the next address to the link register)
    bl \handler
    // restoring what was before
    b __exception_vectors_restore
.endm

// defining the exeption vectors (ax = allocatable + executable)
.section .exception_vectors, "ax"
.global __exception_vectors_start
__exception_vectors_start:
    CONTEXT_SWITCH current_elx_sp0_synchronous // 0x000
    CONTEXT_SWITCH current_elx_sp0_irq         // 0x080
    CONTEXT_SWITCH current_elx_sp0_fiq         // 0x100
    CONTEXT_SWITCH current_elx_sp0_serror      // 0x180

    CONTEXT_SWITCH current_elx_synchronous     // 0x200
    CONTEXT_SWITCH current_elx_irq             // 0x280
    CONTEXT_SWITCH current_elx_fiq             // 0x300
    CONTEXT_SWITCH current_elx_serror          // 0x380

    CONTEXT_SWITCH lower_aarch64_synchronous   // 0x400
    CONTEXT_SWITCH lower_aarch64_irq           // 0x480
    CONTEXT_SWITCH lower_aarch64_fiq           // 0x500
    CONTEXT_SWITCH lower_aarch64_serror        // 0x580

    CONTEXT_SWITCH lower_aarch32_synchronous   // 0x600
    CONTEXT_SWITCH lower_aarch32_irq           // 0x680
    CONTEXT_SWITCH lower_aarch32_fiq           // 0x700
    CONTEXT_SWITCH lower_aarch32_serror        // 0x780

.globl __exception_vectors_enable_irq
__exception_vectors_enable_irq:
    // clear interrupt masks => enable interrupts
    msr DAIFClr, #2
    ret

.global __exception_vectors_restore
__exception_vectors_restore:
    // popping items from the stack
    ldr x19,      [sp, #16 * 17]
    ldp x20, x21, [sp, #16 * 16]
    ldp x30, x22, [sp, #16 * 15]

    // restore registers
    msr  FAR_EL1, x19
    msr  ELR_EL1, x20
    msr  ESR_EL1, x21
    msr SPSR_EL1, x22

    ldp  x0,  x1, [sp, #16 *  0]
    ldp  x2,  x3, [sp, #16 *  1]
    ldp  x4,  x5, [sp, #16 *  2]
    ldp  x6,  x7, [sp, #16 *  3]
    ldp  x8,  x9, [sp, #16 *  4]
    ldp x10, x11, [sp, #16 *  5]
    ldp x12, x13, [sp, #16 *  6]
    ldp x14, x15, [sp, #16 *  7]
    ldp x16, x17, [sp, #16 *  8]
    ldp x18, x19, [sp, #16 *  9]
    ldp x20, x21, [sp, #16 * 10]
    ldp x22, x23, [sp, #16 * 11]
    ldp x24, x25, [sp, #16 * 12]
    ldp x26, x27, [sp, #16 * 13]
    ldp x28, x29, [sp, #16 * 14]

    // restore sp
    add sp, sp, #16 * 18

    // return from an exception
    eret

.section ".text.boot", "ax"
.global __start
__start:
    // get the core number
    mrs x0, MPIDR_EL1
    // We only need the first 2 bits
    and x0, x0, 0b11
    // Compare and park if not zero (not core 0) (jump forward to label 3)
    cbnz x0, 3f
    // Get the current exception level
    mrs	x1, CurrentEL
    // CurrentEL stored with offset 2
    cmp x1, #(2 << 2)
    // if CurrentEL != 2 then park
    b.ne 3f

    // load the address of __start and set the stack
    adr x2, __start
    msr SP_EL1, x2

    // EL1 is AArch64 bit (RW -  Register width control bit)
    mov x2, #(1 << 31)
    // Set the Hypervisor Configuration Register
    msr HCR_EL2, x2

    // 1 means disabled for D A I F
    // Debug | Abort | Interrupt | Fast interrupt | Reserved | AArch64 | SP_EL1
    mov x3, 0b1101000101
    msr	SPSR_EL2, x3

    // jumps to 1: in EL1 by setting the link register
    adr x4, 1f
    msr	ELR_EL2, x4

    // sets the exception handlers
    adr x5, __exception_vectors_start 
    msr VBAR_EL1, x5
    isb SY

    // return from the exception
    eret

    //reset bss and enable interrupts
1:  adr x0, __bss_start
    adr x1, __bss_end
    // store zero on the address of x0 and x0 + 1
2:  stp xzr, xzr, [x0], #16
    // compare x0 with the end address
    cmp x0, x1
    // jump back if lower than
    b.lt 2b
    // jump and set the link register
    bl _main
    // park core
3:  wfe
    b 2b
