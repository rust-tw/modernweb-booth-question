macro_rules! rprintln {
    () => (print!("\r\n"));
    ($($arg:tt)*) => ({
        print!($($arg)*);
        rprintln!();
    })
}
