mod executor;
mod parser;
mod types;
mod list;
mod log;

mod test_examples;
mod lexer;

fn main() {
    let s = r"
    a! = {
        () => 1;
    }
    ";

    // parser::parse(test_examples::COMPLEXT_1);
    parser::parse(s);
}
