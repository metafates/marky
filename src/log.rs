#[macro_export]
macro_rules! error {
    () => {
        eprintln!("{}", "error".red())
    };
    ($($arg:tt)*) => {
        eprintln!("{} {}", "error:".red(), format!($($arg)*))
    };
}

#[macro_export]
macro_rules! info {
    () => {
        eprintln!("{}", "info".blue())
    };
    ($($arg:tt)*) => {
        eprintln!("{} {}", "info:".blue(), format!($($arg)*))
    };
}
