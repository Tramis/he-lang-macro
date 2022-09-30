use ansi_term::Color::{Blue, Green, Red};
use std::fmt::Display;

/// print the original string
pub fn log_msg(s: &str) {
    println!("{s}")
}

pub fn log_success(s: impl Display) {
    println!("{}", Green.paint(format!("[SUCCESS] {s}")));
}

pub fn log_normal(s: impl Display) {
    println!("{}", Blue.paint(format!("[INFO] {s}")));
}

pub fn log_error(s: impl Display) {
    println!("{}", Red.paint(format!("[ERROR] {s}")));
}

pub fn handle_error<T, E: Display>(e: std::result::Result<T, E>) {
    if let Err(e) = e {
        log_error(e);
    } else {
        panic!("handle_error is handling an error which is not an error")
    }
}
