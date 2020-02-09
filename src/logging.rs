
#[cfg(debug_assertions)]
macro_rules! log_debug {
    () => { println!() };
    ($($arg:tt)*) => { println!("[{}:{}] {}", file!(), line!(), format!($($arg)*)) };
}

#[cfg(not(debug_assertions))]
macro_rules! log_debug {
    ($($arg:tt)*) => { () }
}

macro_rules! log_info {
    () => { println!() };
    ($($arg:tt)*) => { println!("[{}] {}", "stala::log_info", format!($($arg)*)) };
}
