#[allow(unused)]
use ansi_term::Color::{Blue, Green, Red};
#[allow(unused)]
use log::{debug, error, info};
use log4rs;

pub fn log_init() {
    let init_file = include_str!("./utils/log_config.yaml");
    log4rs::init_raw_config(serde_yaml::from_str(init_file).unwrap()).unwrap();
}

/// print the original string

macro_rules! log_msg {
    ($msg: expr) => {
        log::info!("{}", ansi_term::Colour::Blue.paint(format!("{}", $msg)))
    };
}

macro_rules! log_success {
    ($msg: expr) => {
        log::debug!("{}", ansi_term::Colour::Green.paint(format!("{}", $msg)))
    };
}

macro_rules! log_normal {
    ($msg: expr) => {
        log::debug!("{}", ansi_term::Colour::Blue.paint(format!("{}", $msg)))
    };
}

macro_rules! log_error {
    ($msg: expr) => {
        log::error!(
            "{}",
            ansi_term::Colour::Red.bold().paint(format!("{}", $msg))
        )
    };
}

macro_rules! std_out {
    ($msg: expr) => {
        log::info!(
            "  {}",
            ansi_term::Colour::Green.bold().paint(format!("{}", $msg))
        )
    };
}

pub(crate) use log_error;
pub(crate) use log_msg;
pub(crate) use log_normal;
pub(crate) use log_success;
pub(crate) use std_out;

use std::io;
pub fn input() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error input");

    // TODO
    // process input cmd
}
