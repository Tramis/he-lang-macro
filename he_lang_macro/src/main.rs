mod executor;
mod list;
mod log;
mod parser;
mod scope;
mod types;

mod lexer;
mod test_examples;

fn main() {
    let s = r"
    a! = {
        () => 1;
    }
    ";

    // parser::parse(test_examples::COMPLEXT_1);
    parser::parse(s);
}
