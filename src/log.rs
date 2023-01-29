#[macro_export]
macro_rules! error {
    () => {
        println!("{}", "error".red());
    };
    ($($arg:tt)*) => {
        println!("{} {}", "error:".red(), format!($($arg)*));
    };
}
