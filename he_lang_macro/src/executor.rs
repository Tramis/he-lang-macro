use crate::types::*;

trait Excutable {
    type Output;
    fn excute(&self) -> Self::Output;
}

impl Excutable for MacroCall {
    type Output = Expression;

    fn excute(&self) -> Self::Output {
        // predefined:

        match self.macro_name.as_str() {
            "print" => {
                unimplemented!()
            }
            "string" => {
                unreachable!("macro `string!` done while compile")
            }
            "count" => Expression::Data(HePrimitive::Int(self.params.len() as i32)),
            _ => unreachable!("unknown macro"),
        }
    }
}
