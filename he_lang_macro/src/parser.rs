use crate::types::*;

use pest_derive::Parser;
use pest::{Parser, RuleType, iterators::Pair};

#[derive(Parser)]
#[grammar = "../utils/he_lang_macro.pest"]
struct HeParser;

pub(crate) fn parse(s: &str) {
    let res: Pair<Rule> = HeParser::parse(Rule::main, s).expect("parse error").next().unwrap();

    println!("{res:#?}");

}

