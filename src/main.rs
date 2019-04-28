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
    println!("panic: {}", info);
    loop {
        asm::wfe();
    }
}

fn echo() {
    global![default_loop].read_char(Box::new(|c| {
        if c != '\n' {
            global![default_loop].put_char(c, Box::new(|| {}));
        }
        echo();
    }));
}

fn command_line() {
    global![default_loop].read_line(Box::new(|line| {
        use core::fmt::Write;
        // echo back, for now
        let mut s = String::new();
        write!(s, "\n{}\n> ", line).unwrap();
        global![default_loop].put_string(s, Box::new(command_line));
    }));
}

#[no_mangle]
extern "C" fn _main() -> ! {
    globals::init();

    global![interrupt].interrupt_enable();

    global![default_loop].put_string(
        "Welcome!\n> ".to_string(),
        Box::new(|| {
            echo();
            command_line();
        }
    ));

    loop {
        global![default_loop].prepare();
        while global![default_loop].is_dirty() {
            global![default_loop].run();
        }
        asm::wfi();
    }
}
