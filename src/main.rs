#![no_std]
#![no_main]
#![feature(format_args_nl)]
#![feature(allocator_api)]
#![feature(alloc)]
#![feature(alloc_error_handler)]
#![feature(global_asm)]
#![feature(asm)]

global_asm!(include_str!("boot/start.S"));

#[macro_use]
mod macros;

mod asm;
mod dev;
mod sys;
mod logger;

use alloc::prelude::*;
use core::panic::PanicInfo;
use sys::alloc::*;
use sys::reactor::*;
use dev::miniuart::*;
use logger::*;

extern crate alloc;

#[global_allocator]
static mut ALLOCATOR: Allocator = Allocator::new();
static mut MINIUART: MiniUart = MiniUart::new();
static mut LOGGER: Logger = Logger {};
static mut LOOP: sys::reactor::Loop = Loop::new();

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::asm;
    println_sync!("panic: {}", info);
    loop {
        asm::wfe();
    }
}

unsafe fn echo() {
    LOOP.read_char(Box::new(|c| {
        if c != '\n' {
            print!("{}", c);
        }
        echo();
    }));
}

unsafe fn command_line() {
    LOOP.read_line(Box::new(|line| {
        // echo back, for now
        print!("\n{}\n> ", line);
        command_line();
    }));
}

#[no_mangle]
unsafe extern "C" fn _main() -> ! {
    ALLOCATOR.init();
    MINIUART.init();
    LOOP.init();

    println_sync!("Press a key to continue!");
    read_char_sync!();

    println!("Welcome!");
    print!("> ");

    echo();
    command_line();

    loop {
        // check if we have to do something
        LOOP.run();
        // sleep
        MINIUART.interrupt_enable();
        asm::wfi();
    }
}
