use std::cell::RefCell;
use std::rc::Rc;

use crate::scope::Scope;
use crate::types::*;

use crate::io::*;

#[allow(unused)]
use ansi_term::Color::{Blue, Green, Red};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./utils/he_lang_macro.pest"]
pub struct HeParser;

pub use Rule::main as MainRule;

fn handle_parse_error(e: pest::error::Error<Rule>) {
    const UP_ARROW: char = '^';

    match e.line_col {
        pest::error::LineColLocation::Pos((x, y)) => {
            log_error!(format!(
                "parse error:\n{}\n{}",
                format!("{: >10}{}", format!("line[{}]:", x), e.line()),
                format!(
                    "{}{}{UP_ARROW}",
                    std::iter::repeat(' ').take(10).collect::<String>(),
                    std::iter::repeat('-').take(y - 1).collect::<String>()
                )
            ))
        }
        _ => {
            log_error!("line is a span, should be LineColLocation")
        }
    }
}

macro_rules! to_vec {
    ($pair:expr, $res:expr ,$have_type:pat) => {
        loop {
            match $pair.as_rule() {
                $have_type => {
                    let mut pairs = $pair.into_inner();
                    let now = pairs.next();
                    $res.push(now.unwrap());
                    if let Some(next) = pairs.next() {
                        $pair = next
                    } else {
                        break;
                    }
                }
                _ => unreachable!("\n{}", format!("{:#?}", $pair)),
            }
        }
    };
}

pub fn parse_expr(pair: Pair<Rule>) -> Expression {
    let expr = pair.into_inner().next().unwrap();
    match expr.as_rule() {
        Rule::macro_call => {
            let mut macro_call = expr.into_inner();
            let ident = macro_call
                .next()
                .unwrap()
                .into_inner()
                .next()
                .unwrap()
                .as_span()
                .as_str()
                .to_string();

            // predefined macro `string!`
            // if ident.as_str() == "string" {
            //     return Expression::Data(HePrimitive::String(
            //         macro_call.next().unwrap().as_str().to_string(),
            //     ));
            // }

            // let params = macro_call.next();
            // if params.is_some() {
            //     let mut params = params.unwrap();
            //     m_call.origin_param = params.as_str().to_string();

            //     let mut tmp_params = vec![];
            //     to_vec!(params, tmp_params, Rule::call_params);

            //     for p in tmp_params {
            //         // let p = p.into_inner().next().unwrap().as_span().as_str();
            //         m_call.params.push(Box::new(parse_expr(
            //             p.into_inner().next().unwrap(),
            //             scope.clone(),
            //         )))
            //     }
            // }

            return Expression::MacroCall(MacroCall {
                origin_param: macro_call.next().unwrap().as_str().to_string(),
                macro_name: ident,
                // params: List::new(),
            });
        }
        Rule::int => {
            let num = expr.as_span().as_str();
            match num.parse::<i32>() {
                Err(e) => {
                    log_error!(e);
                    panic!("{} into i32 overflow", num)
                }
                Ok(v) => return Expression::Data(HePrimitive::Int(v)),
            }
        }
        Rule::string => {
            let string = expr.into_inner().next().unwrap();
            return Expression::Data(HePrimitive::String(string.as_span().as_str().to_string()));
        }
        Rule::other => return Expression::Raw(expr.as_span().as_str().to_string()),
        Rule::paren => return Expression::Raw(expr.as_span().as_str().to_string()),
        _ => unreachable!("unexpected expression: {expr:#?}"),
    }
}

pub fn parse_params(mut pair: Pair<Rule>) -> Vec<Expression> {
    let mut params = vec![];
    to_vec!(pair, params, Rule::call_params);

    params
        .into_iter()
        .map(|x| parse_expr(x.into_inner().next().unwrap()))
        .collect()
}

pub fn parse_main(mut pair: Pair<Rule>) -> Vec<HeType> {
    let mut statements = vec![];

    to_vec!(pair, statements, Rule::statements);

    let mut res = vec![];
    for statement in statements {
        let statement_inner = statement.into_inner().next().unwrap();
        match statement_inner.as_rule() {
            Rule::expression => res.push(HeType::Expression(parse_expr(statement_inner))),
            Rule::macro_def => {
                let mut macro_def = statement_inner.into_inner();
                let ident = macro_def.next().unwrap().into_inner().next().unwrap();

                let mut macro_contents = macro_def.next().unwrap();

                let mut contents = vec![];
                to_vec!(macro_contents, contents, Rule::macro_def_contents);

                for content in contents {
                    let mut content = content.into_inner();

                    // params(dollars)
                    let params = content.next().unwrap().into_inner().next();

                    let m = Macro::new(
                        ident.as_str().to_string(),
                        {
                            if let Some(mut params) = params {
                                let mut dollars = vec![];
                                to_vec!(params, dollars, Rule::dollars);

                                dollars
                                    .into_iter()
                                    .map(|x| x.as_span().as_str().to_string())
                                    .collect()
                            } else {
                                vec!["".to_string()]
                            }
                        },
                        {
                            content
                                .next()
                                .unwrap()
                                .into_inner()
                                .next()
                                .unwrap()
                                .as_span()
                                .as_str()
                                .to_string()
                        },
                    );

                    res.push(HeType::MacroDef(m.clone()));
                }
            }

            Rule::EOI => {
                // suspicious appear
            }
            _ => unreachable!("\n{}", format!("{:#?}", statement_inner)),
        }
    }

    res
}

pub fn he_parse_with_rule(s: &str, rule: Rule) -> Pair<Rule> {
    match HeParser::parse(rule, s) {
        Ok(mut v) => v.next().expect("parse result empty"),
        Err(e) => {
            log_error!(format!("parse error: {}", s));
            handle_parse_error(e);
            panic!("parse error")
        }
    }
}

#[allow(unused)]
pub(crate) mod test {
    use crate::escape::Escape;

    use super::*;
    fn he_parse_with_rule_test(s: &str, rule: Rule) -> Pair<Rule> {
        match HeParser::parse(rule, s) {
            Ok(mut v) => {
                let v = v.next().expect("parse result empty");
                log_success!(format!("{:#?}", v));
                v
            }
            Err(e) => {
                handle_parse_error(e);
                panic!("parse error")
            }
        }
    }

    pub fn test_parse_params(s: &str) {
        let scope = Rc::new(RefCell::new(Scope::new()));
        log_success!(format!(
            "{:#?}",
            parse_params(he_parse_with_rule_test(s, Rule::call_params))
        ));
    }

    pub fn test_parse_with_rule(s: &str, rule: Rule) {
        let res = HeParser::parse(rule, s);

        match res {
            Ok(mut res) => {
                log_msg!(&Green.paint(format!("{res:#?}")).to_string());

                let scope = Rc::new(RefCell::new(Scope::new()));
                let res = parse_main(res.next().unwrap());
                println!("{res:#?}")
            }
            Err(e) => {
                // log_normal(format!("{:#?}", &e));
                handle_parse_error(e);
            }
        }
    }

    #[test]
    fn test_s() {
        use crate::test_examples::*;
        log_init();
        // test_parse_with_rule(def_macro_1, Rule::main);

        // test_parse_with_rule(def_macro_2, Rule::main);

        // test_parse_with_rule(macro_call_1, Rule::main);

        // parse(expression_s_123);

        // parse(test_examples::macro_call_empty_param);

        // parse(test_examples::macro_call_any_param);

        // parse(test_examples::def_macro_cat);

        // parse(macro_call_paren_param);

        // parse(macro_call_string);

        let mut s = String::from(" | ( a | ( | ) )");
        s.escape_parenthese();

        test_parse_params(" | ( | \\(| \\))");
        // parse_with_rule(r#"fdsafd!((())"#, Rule::expression);
        // parse_with_rule(r#"fdsf$_\)"#, Rule::expression);
        // parse_with_rule(r#""#, Rule::expression);
        // parse_with_rule(macro_call_paren_param, Rule::expression);
        // test_parse_params("|");
        // test_parse_params("| ");
        // test_parse_params(" |");
        // test_parse_params(" | ");
        // test_parse_params("| |");
        // test_parse_params(" | | ");
    }

    #[test]
    fn test_basic() {
        log_init();
        use crate::test_examples::BASIC;
        for x in BASIC {
            test_parse_with_rule(x, Rule::main);
        }
    }
}
