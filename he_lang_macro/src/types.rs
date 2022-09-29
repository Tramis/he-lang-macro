use crate::list::*;

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
    pub macro_name: String,
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
