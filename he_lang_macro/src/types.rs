

use crate::list::*;

type MacroFrom = List<HeType>;
type MacroTo = List<HeType>;

pub enum HeType {
    Data(HePrimitive),
    Ident { name: String, value: Option<Box<HeType>> },
    Macro { from: MacroFrom, to: MacroTo },
}

enum HePrimitive {
    Int(i32),
    String(String),
    List(Box<HePrimitive>),
    // he symbol
    Symbol,
}