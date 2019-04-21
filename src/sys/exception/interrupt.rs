pub struct Interrupt {}

impl Interrupt {
    pub const fn new() -> Self {
        Interrupt {}
    }
    #[inline]
    pub fn interrupt_enable(&self) {
        global![mini_uart].interrupt_enable();
    }
    #[inline]
    pub fn process(&self) {
        global![mini_uart].try_read_char();
    }
}
