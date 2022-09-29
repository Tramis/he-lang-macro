use crate::scope::Scope;
use crate::{list::List, log::*};
use crate::{test_examples, types::*};

use ansi_term::Color::{Blue, Green, Red};
use pest::{iterators::Pair, Parser, RuleType};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./utils/he_lang_macro.pest"]
struct HeParser;

fn handle_parse_error(e: pest::error::Error<Rule>) {
    const UP_ARROW: char = '^';

    match e.line_col {
        pest::error::LineColLocation::Pos((x, y)) => {
            log_msg(
                &Red.bold()
                    .paint(format!("{: >10}{}", format!("line[{}]:", x), e.line()))
                    .to_string(),
            );
            log_msg(
                &Red.paint(format!(
                    "{}{}{UP_ARROW}",
                    std::iter::repeat(' ').take(10).collect::<String>(),
                    std::iter::repeat('-').take(y - 1).collect::<String>()
                ))
                .to_string(),
            )
        }
        pest::error::LineColLocation::Span((x1, y1), (x2, y2)) => {
            log_msg(&Red.paint("error is a span").to_string())
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

fn parse_value(pair: Pair<Rule>) -> HeType {
    unimplemented!()
}

pub fn parse_expr(mut pair: Pair<Rule>, scope: &mut Scope) -> Expression {
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

            if ident.as_str() == "string" {
                let s = if let Some(all_params) = macro_call.next() {
                    all_params.as_str()
                } else {
                    ""
                };
                return Expression::Data(HePrimitive::String(s.to_string()));
            }

            let mut m_call = MacroCall {
                macro_name: ident,
                params: List::new(),
            };

            let params = macro_call.next();
            if params.is_some() {
                let mut params = params.unwrap();
                let mut tmp_params = vec![];
                to_vec!(params, tmp_params, Rule::call_params);

                for p in tmp_params {
                    // let p = p.into_inner().next().unwrap().as_span().as_str();
                    m_call
                        .params
                        .push(Box::new(parse_expr(p.into_inner().next().unwrap(), scope)))
                }
            }

            return Expression::MacroCall(m_call);
        }
        Rule::int => {
            let num = expr.as_span().as_str();
            match num.parse::<i32>() {
                Err(e) => {
                    log_error(e);
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
        Rule::paren => {
            return Expression::Raw(
                expr.into_inner()
                    .next()
                    .unwrap()
                    .as_span()
                    .as_str()
                    .to_string(),
            )
        }
        _ => unreachable!("unexpected expression: {}", expr.as_str()),
    }
}

fn parse_main(mut pair: Pair<Rule>, scope: &mut Scope) -> Vec<HeType> {
    let mut statements = vec![];

    to_vec!(pair, statements, Rule::statements);

    let mut res = vec![];
    for statement in statements {
        let statement_inner = statement.into_inner().next().unwrap();
        match statement_inner.as_rule() {
            Rule::expression => res.push(HeType::Expression(parse_expr(statement_inner, scope))),
            Rule::macro_def => {
                let mut macro_def = statement_inner.into_inner();
                let ident = macro_def.next().unwrap().into_inner().next().unwrap();

                let mut macro_contents = macro_def.next().unwrap();

                let mut contents = vec![];
                to_vec!(macro_contents, contents, Rule::macro_def_contents);

                for content in contents {
                    let mut m = Macro {
                        from: List::new(),
                        to: "".to_string(),
                        name: ident.as_str().to_string(),
                    };
                    let mut content = content.into_inner();
                    // params(dollars)
                    let params = content.next().unwrap().into_inner().next();

                    if params.is_some() {
                        let mut params = params.unwrap();
                        let mut dollars = vec![];

                        to_vec!(params, dollars, Rule::dollars);
                        for dollar in dollars {
                            m.from.push(
                                dollar
                                    .into_inner()
                                    .next()
                                    .unwrap()
                                    .as_span()
                                    .as_str()
                                    .to_string(),
                            )
                        }
                    }

                    // body
                    m.to = content
                        .next()
                        .unwrap()
                        .into_inner()
                        .next()
                        .unwrap()
                        .as_span()
                        .as_str()
                        .to_string();

                    res.push(HeType::MacroDef(m.clone()));
                    // push
                    scope.push_macro(m);
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

pub fn parse_with_rule(s: &str, rule: Rule) {
    let res = HeParser::parse(rule, s);

    match res {
        Ok(mut res) => {
            log_msg(&Green.paint(format!("{res:#?}")).to_string());

            let mut scope = Scope::new();
            let res = parse_main(res.next().unwrap(), &mut scope);
            println!("{res:#?}")
        }
        Err(e) => {
            // log_normal(format!("{:#?}", &e));
            handle_parse_error(e);
        }
    }
}

pub fn parse(s: &str) {
    parse_with_rule(s, Rule::main)
}

#[test]
fn test_s() {
    use crate::test_examples::*;

    // parse(def_macro_1);

    // parse(def_macro_2);

    // parse(macro_call_1);

    // parse(expression_s_123);

    // parse(test_examples::macro_call_empty_param);

    // parse(test_examples::macro_call_any_param);

    // parse(test_examples::def_macro_cat);

    // parse(macro_call_paren_param);

    // parse(macro_call_string);

    // parse_with_rule(r#"fdsafd!((())"#, Rule::expression);
    // parse_with_rule(r#"fdsf$_\)"#, Rule::expression);
    // parse_with_rule(r#""#, Rule::expression);
    // parse_with_rule(macro_call_paren_param, Rule::expression);
}

#[test]
fn test_basic() {
    use crate::test_examples::BASIC;
    for x in BASIC {
        parse(x);
    }
}
