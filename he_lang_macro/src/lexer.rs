use crate::log::handle_error;
use logos::Logos;
use strum_macros::IntoStaticStr;

mod lex_callbacks {
    use super::*;
    use logos::Lexer;
    pub fn int_check_overflow(lex: &mut Lexer<Token>) -> Option<i32> {
        let slice = lex.slice();

        match slice.parse::<i32>() {
            Ok(v) => Some(v),
            e @ Err(_) => {
                handle_error(e);
                None
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MacroToken {
    Print,
    String,
    Normal(String),
}

#[derive(Debug, PartialEq)]
pub enum ParamToken {
    Space,
    Sep,
    Normal(String),
}

#[derive(Logos, Debug, PartialEq, IntoStaticStr)]
pub enum Token {
    /// ## `|`: he symbol
    /// it's the heart of he-lang.
    #[token("|")]
    HeSymbol,

    // params
    #[token("\\(")]
    EscapeLeftParam,

    #[token("\\)")]
    EscapeRightParam,

    #[token("(")]
    LeftParam,

    #[token(")")]
    RightParam,

    #[token("\"")]
    #[token("\'")]
    Quote,

    // int
    #[regex(r"[0-9]+", lex_callbacks::int_check_overflow)]
    Int(i32),

    /// params of macro are defined like `$aaa`, `$Bat`
    #[token("$s", |_|ParamToken::Space)]
    #[token("$sep", |_|ParamToken::Sep)]
    #[regex(r"\$[a-zA-Z]", |lex|ParamToken::Normal(lex.slice()[1..].to_string()))]
    DollarIdent(ParamToken),

    /// ## macros
    /// here are some predefined macros:
    /// - `print!`
    /// - `string!`
    #[token("print!", |_| MacroToken::Print)]
    #[token("string!", |_| MacroToken::String)]
    #[regex("[a-zA-Z]+!", |lex|MacroToken::Normal(lex.slice()[..lex.slice().len()].to_string()))]
    MacroIdent(MacroToken),

    /// ## ident
    /// todo
    #[regex("[a-zA-Z]+", |lex|lex.slice().to_string())]
    Ident(String),

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

#[allow(unused)]
pub fn lex(s: String) -> Vec<Token> {
    Logos::lexer(s.as_str()).into_iter().collect()
}

#[allow(unused)]
fn print_lex(s: &str) {
    let mut lex = Token::lexer(s);

    while let Some(now) = lex.next() {
        let type_name: &str = now.into();
        println!("'{}':\t{type_name}", lex.slice());
    }
}

#[test]
fn test_lex() {
    use super::test_examples;

    print_lex(test_examples::PRINT_PRINT_2);
}

#[test]
fn test_escape() {
    print_lex("string!(12321\\()\\))")
}

#[test]
fn test_macro_and_ident() {
    print_lex("a!(a)");
}

#[test]
fn test_string_and_sep() {
    print_lex("a!(1 | \" 1243$s\" | $sep | )");
}
