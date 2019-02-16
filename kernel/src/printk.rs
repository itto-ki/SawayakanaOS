macro_rules! print {
    ($($arg:tt)*) => ({
        unsafe {
            drivers::vga::VGA_BUFFER.print(format_args!($($arg)*));
        }
    });
}

macro_rules! printk {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}
