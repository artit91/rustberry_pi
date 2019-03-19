pub struct Interrupt {
    link_address: Option<u64>
}

impl Interrupt {
    pub const fn new() -> Self {
        Interrupt {
            link_address: None
        }
    }
    #[inline]
    pub const fn link_address(&self) -> Option<u64> {
        self.link_address
    }
    #[inline]
    pub fn link(&mut self, link_address: u64) {
        self.interrupt_disable();
        self.link_address = Some(link_address);
        self.interrupt_enable();
    }
    #[inline]
    pub fn unlink(&mut self) {
        self.link_address = None;
    }
    #[inline]
    pub fn interrupt_disable(&self) {
        global![mini_uart].interrupt_disable();
    }
    #[inline]
    pub fn interrupt_enable(&self) {
        global![mini_uart].interrupt_enable();
    }
    #[inline]
    pub fn process(&self) {
        global![mini_uart].try_read_char();
    }
    #[inline]
    pub fn flush(&self) {
        global![mini_uart].flush();
    }
}
