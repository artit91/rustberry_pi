use core::fmt;

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
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    global![mini_uart].write_fmt(args).unwrap();
}