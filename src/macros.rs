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

#[macro_export]
macro_rules! global {
    [$name:ident] => ($crate::globals::$name());
}

#[macro_export]
macro_rules! register_global {
    ($name:ident, $type:path, $variable_name:ident) => (
        #[inline]
        pub fn $name() -> &'static mut $type {
            return unsafe { &mut $crate::globals::$variable_name };
        }
    );
}

#[doc(hidden)]
pub fn _read_sync() -> char {
    global![mini_uart].read_char()
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    global![logger].write_fmt(args).unwrap();
}

#[doc(hidden)]
pub fn _print_sync(args: fmt::Arguments) {
    use core::fmt::Write;
    global![mini_uart].write_fmt(args).unwrap();
}