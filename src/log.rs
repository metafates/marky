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
macro_rules! die {
    () => {{
        std::process::exit(1);
    }};
    ($($arg:tt)*) => {{
        error!($($arg)*);
        std::process::exit(1);
    }};
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

#[macro_export]
macro_rules! warn {
    () => {
        eprintln!("{}", "warning".yellow())
    };
    ($($arg:tt)*) => {
        eprintln!("{} {}", "warning:".yellow(), format!($($arg)*))
    };
}

#[macro_export]
macro_rules! note {
    () => {
        eprintln!("{}", "note".yellow())
    };
    ($($arg:tt)*) => {
        eprintln!("{} {}", "note:".yellow(), format!($($arg)*))
    };
}
