use core::fmt;

#[macro_export]
macro_rules! read_char_sync {
    () => ($crate::macros::_read_sync());
}

#[macro_export]
macro_rules! print_sync {
    ($($arg:tt)*) => ($crate::macros::_print_sync(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println_sync {
    () => (print!("\n"));
    ($($arg:tt)*) => ({
        $crate::macros::_print_sync(format_args_nl!($($arg)*));
    })
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::macros::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => ({
        $crate::macros::_print(format_args_nl!($($arg)*));
    })
}

#[doc(hidden)]
pub fn _read_sync() -> char {
    unsafe {
        crate::MINIUART.read_char()
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    unsafe {
        crate::LOGGER.write_fmt(args).unwrap();
    }
}

#[doc(hidden)]
pub fn _print_sync(args: fmt::Arguments) {
    use core::fmt::Write;
    unsafe {
        crate::MINIUART.write_fmt(args).unwrap();
    }
}
