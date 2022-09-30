use std::{cell::RefCell, rc::Rc};

use crate::{list::*, scope::Scope};

#[derive(Debug, Clone)]
pub enum HeType {
    // IdentDef { name: String, value: Option<Box<HeType>> },
    MacroDef(Macro),
    Expression(Expression),
}

#[derive(Debug, Clone)]
pub struct Macro {
    pub name: String,
    pub from: List<String>,
    pub to: String,
}

#[derive(Debug, Clone)]
pub struct MacroCall {
    pub scope: Rc<RefCell<Scope>>,
    pub macro_name: String,
    pub origin_param: String,
    pub params: List<Box<Expression>>,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Data(HePrimitive),
    /// macro call
    MacroCall(MacroCall),
    /// parentheses
    Raw(String),
}

#[derive(Debug, Clone)]
pub enum HePrimitive {
    Int(i32),
    String(String),
}

impl HePrimitive {
    pub fn to_string(&self) -> String {
        match self {
            Self::Int(i) => i.to_string(),
            Self::String(s) => s.clone(),
        }
    }
}

impl Expression {
    pub fn to_string(&self) -> String {
        match self {
            Self::Data(a) => a.to_string(),
            Self::Raw(s) => s.clone(),
            Self::MacroCall(m) => m.origin_param.clone(),
        }
    }
}
