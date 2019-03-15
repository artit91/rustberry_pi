
use core::alloc::{Layout, GlobalAlloc};
use core::cell::Cell;

#[alloc_error_handler]
fn alloc_error_handler(_layout: Layout) -> ! {
    panic!("ALLOC_ERROR")
}

pub struct Allocator {
    next: Cell<usize>,
    end: Cell<usize>,
}

impl Allocator {
    pub const fn new() -> Allocator {
        Allocator {
            next: Cell::new(0),
            end: Cell::new(0),
        }
    }
    pub fn init(&self) {
        use crate::dev::board::bcm2837::*;
        extern "C" {
            static __end: u64;
        }
        self.next.set(unsafe { &__end as *const _ as usize});
        self.end.set((MMIO_BASE - 1) as usize);
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let alignment = layout.align();
        let start = (self.next.get() + alignment - 1) & !(alignment - 1);;
        let end = start + layout.size();
        // println_sync!(
        //     "[i] Allocated Addr {:#010X} Size {:#X}",
        //     start,
        //     layout.size()
        // );
        if end <= self.end.get() {
            self.next.set(end);
            return start as *mut u8;
        }
        core::ptr::null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // TODO: proper allocator
        // println_sync!(
        //     "[i] Deallocated Addr {:#010X} Size {:#X}",
        //     _ptr as usize,
        //     _layout.size()
        // );
    }
}
