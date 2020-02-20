
#[cfg(feature = "verbose")]
macro_rules! log_debug {
    () => { println!() };
    ($($arg:tt)*) => { 
        let s: &'static str = file!();
        let filename = s.split('/').last().unwrap_or("");
        let label = format!("{}:{}", filename, line!());
        println!("[{:>18}] {}", label, format!($($arg)*)) 
    };
}

#[cfg(not(feature = "verbose"))]
macro_rules! log_debug {
    ($($arg:tt)*) => { () }
}

macro_rules! log_info {
    () => { println!() };
    ($($arg:tt)*) => { println!("[{}] {}", "filum", format!($($arg)*)) };
}
