

use crate::list::*;


pub enum HeType {
    Data(HePrimitive),
    Ident { name: String, value: Option<Box<HeType>> },
    Macro(Macro),
    Expression(Expression)
}

pub struct Macro{
    from: List<HeType>,
    to: String
}

pub enum Expression{
    /// macro call
    Call{
        macro_name: Macro,
        params: List<HeType>
    },
    /// parentheses
    Parenthese(Option<Box<Expression>>),

}

pub enum HePrimitive {
    Int(i32),
    String(String),
    List(Box<HePrimitive>),
    Other,
}

pub trait MyType{
    type Type;
    
}