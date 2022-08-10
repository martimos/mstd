#[macro_export]
macro_rules! entry_point {
    ($entry:ident) => {
        #[start]
        fn start(_argc: isize, _argv: *const *const u8) -> isize {
            let main: fn() = $entry;
            main();
            0
        }
    };
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($fmt:expr) => {
        $crate::print!(concat!($fmt, "\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::print!(concat!($fmt, "\n"), $($arg)*)
    };
}
