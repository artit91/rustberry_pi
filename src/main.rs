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
mod globals;

use alloc::prelude::*;
use core::panic::PanicInfo;
use sys::alloc::*;

extern crate alloc;

#[global_allocator]
static mut ALLOCATOR: Allocator = Allocator::new();

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::asm;
    println_sync!("panic: {}", info);
    loop {
        asm::wfe();
    }
}

fn echo() {
    global![default_loop].read_char(Box::new(|c| {
        if c != '\n' {
            print!("{}", c);
        }
        echo();
    }));
}

fn command_line() {
    global![default_loop].read_line(Box::new(|line| {
        // echo back, for now
        print!("\n{}\n> ", line);
        command_line();
    }));
}

extern "C" fn _loop() -> ! {
    loop {
        // check if we have to do something
        global![default_loop].run();
        // set interrupt link
        global![interrupt].link(_loop as *const u8 as u64);
        // flush
        global![default_loop].flush();
        asm::wfi();
    }
}

#[no_mangle]
extern "C" fn _main() -> ! {
    globals::init();

    global![interrupt].interrupt_enable();

    echo();
    command_line();

    println!("Welcome!");
    print!("> ");

    _loop();
}
