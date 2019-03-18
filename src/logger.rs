use core::fmt::Write;

pub struct Logger {}

impl Write for Logger {
    fn write_char(&mut self, c: char) -> core::fmt::Result {
        global![mini_uart].push_char(c);
        Ok(())
    }
    fn write_str(&mut self, input: &str) -> core::fmt::Result {
        for c in input.chars() {
            if c == '\r' {
                self.write_char('\n').unwrap();
            }
            self.write_char(c).unwrap();
        }
        Ok(())
    }
}
