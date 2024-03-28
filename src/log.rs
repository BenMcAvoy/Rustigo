#![allow(unused_macros)]

/// A macro to log an error message.
macro_rules! error {
   ($($arg:tt)*) => {
       println!("\x1b[31m[ ERROR ]\x1b[0m {}", format_args!($($arg)*))
   };
}

/// A macro to log an info message.
macro_rules! info {
   ($($arg:tt)*) => {
       println!("\x1b[32m[  INFO ]\x1b[0m {}", format_args!($($arg)*))
   };
}

/// A macro to log a warning message.
macro_rules! warn {
   ($($arg:tt)*) => {
       println!("\x1b[34m[  WARN ]\x1b[0m {}", format_args!($($arg)*))
   };
}
