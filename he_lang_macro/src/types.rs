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
    // scope should be specified when actually called
    // pub scope: Rc<RefCell<Scope>>,
    pub macro_name: String,
    pub origin_param: String,
    // remove params, parse while the macro really call
    // pub params: List<Box<Expression>>,
}

impl MacroCall {
    pub fn to_string(&self) -> String {
        let mut res = self.macro_name.clone();

        res += "! ( ";
        res += &self.origin_param;
        res += " )";
        res
    }
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
