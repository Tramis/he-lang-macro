use std::io::Read;

use eval::link_start;
use io::{log_init, std_out};

mod eval;
mod io;
mod list;
mod parser;
mod scope;
mod types;

// mod lexer;
mod test_examples;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "input in string format")]
    input: Option<String>,

    #[arg(short, long, value_name = "input in file content format")]
    file: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    log_init();

    if let Some(file_path) = cli.file {
        use std::fs::File;
        let mut file = File::open(file_path).unwrap();
        let mut file_content = String::new();
        file.read_to_string(&mut file_content).unwrap();

        link_start(&file_content);

        return;
    }

    if let Some(raw_str) = cli.input {
        link_start(&raw_str);
        return;
    }

    std_out!("check help page: he-lang -h")
}
