use crate::sys::reactor::*;
use crate::dev::miniuart::*;
use crate::logger::*;
use crate::sys::alloc::*;
use crate::ALLOCATOR;
use crate::sys::exception::interrupt::*;

static mut MINIUART: MiniUart = MiniUart::new();
static mut LOGGER: Logger = Logger {};
static mut DEFAULT_LOOP: Loop = Loop::new();
static mut INTERRUPT: Interrupt = Interrupt::new();

register_global!(mini_uart, MiniUart, MINIUART);
register_global!(logger, Logger, LOGGER);
register_global!(default_loop, Loop, DEFAULT_LOOP);
register_global!(allocator, Allocator, ALLOCATOR);
register_global!(interrupt, Interrupt, INTERRUPT);

pub fn init() {
    global![allocator].init();
    global![mini_uart].init();
    global![default_loop].init();
}
