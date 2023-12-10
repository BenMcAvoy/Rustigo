#![allow(unused_macros)]

macro_rules! error {
   ($($arg:tt)*) => {
       println!("\x1b[31m[ ERROR ]\x1b[0m {}", format_args!($($arg)*))
   };
}

macro_rules! info {
   ($($arg:tt)*) => {
       println!("\x1b[32m[  INFO ]\x1b[0m {}", format_args!($($arg)*))
   };
}

macro_rules! log {
   ($($arg:tt)*) => {
       println!("\x1b[34m[  WARN ]\x1b[0m {}", format_args!($($arg)*))
   };
}
