#[macro_export]
macro_rules! printf {
    ($($fmt:tt)*) => {{
        use std::io::{stdout, Write};
        print!($($fmt)*);
        stdout().flush().unwrap();
    }};
}

#[macro_export]
macro_rules! printlnf {
    ($($fmt:tt)*) => {{
        use std::io::{stdout, Write};
        println!($($fmt)*);
        stdout().flush().unwrap();
    }};
}

pub mod app;
pub mod draw;
pub mod key;
pub mod log;
pub mod panel;
pub mod style;
pub mod theme;
pub mod unit;
pub mod widget;

#[test]
fn test() {
    printf!("Hello world!");
}
