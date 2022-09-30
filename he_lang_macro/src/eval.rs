use core::cell::RefCell;
use std::rc::Rc;

use regex::Regex;

use crate::{
    log::log_msg,
    parser::{he_parse_with_rule, parse_expr, Rule},
    scope::{self, Scope},
    types::*,
};

trait Eval {
    type Output;
    fn eval(&self) -> Self::Output;
}

macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

/// ## eval the left-most macro call
///
/// ```no_rust
///  |  outer!(xxxx_a!(yyyy)_xxx)
///  |              --------
///  -> outer!(xxxx_zzz_xxxx)
///                 ---
/// ```
fn do_it(s: &mut String, scope: Rc<RefCell<Scope>>) -> bool {
    let re_macro_call = regex!(r"[a-zA-Z]+[[:space:]]*!(.*)");

    let begin = re_macro_call.find(s);
    match begin {
        Some(begin) => {
            let mut pair = he_parse_with_rule(&s.get(begin.start()..).unwrap(), Rule::macro_call);
            let pair = pair.next().unwrap();

            let expr = parse_expr(pair.clone(), scope.clone());
            let expr_string = pair.as_span().as_str();

            match expr {
                Expression::MacroCall(macro_call) => {
                    let eval_res = macro_call.eval();
                    *s = s.get(..begin.start()).unwrap().to_owned()
                        + &eval_res.to_string()
                        + match s.get(begin.start() + expr_string.len()..) {
                            Some(later) => later,
                            None => "",
                        }
                }
                _ => unreachable!("unexpected match. should be macro_call: \n{pair:#?}"),
            }

            true
        }
        None => false,
    }
}

impl Eval for MacroCall {
    type Output = Expression;

    /// # eval the macro call
    ///
    /// in some rules. usually rules make sense.
    /// but in some ways it really did do weired things.
    /// Im glad to write this code.
    ///
    ///
    /// ---
    ///
    /// ## inside first
    /// ```no_rust
    ///     a!(a | b!(xxx))
    ///  -> a!(a | yyy)
    /// ```
    ///
    /// ## left first
    /// ```no_rust
    ///     a!(b!(xxx) | c!(yyy))
    ///  -> a!(zzz | c!(yyy))
    /// ```
    ///
    /// ## longest parameter first match
    /// ```no_rust
    ///     a! = {
    ///         ($a | $ab) => {
    ///             $ab
    ///         };
    ///     }
    ///
    ///     print!(a!(1 | 2))
    ///  -> print!(2)
    /// ```
    /// ## first quote parenthese, longer escape
    /// ```no_rust
    ///     a! = {
    ///         ($a | $ab) => {
    ///             ($a)b
    ///         };
    ///     }
    ///
    ///     b! = {
    ///         ($a, $ab) => {
    ///             (($a))b
    ///         };
    ///     }
    ///
    ///     print!(a!(1 | 2))
    ///  -> print!(1b)
    ///
    ///     print!(b!(1 | 2))
    ///  -> print!((1)b)
    /// ```
    /// 
    /// ## reinterpret after eval
    /// ```no_rust
    ///     a! = {
    ///         () => (;
    ///     }
    /// 
    ///     b! = {
    ///         () => );
    ///     }
    /// 
    ///     count!( a!() | b!() )
    ///  -> count!( ( | ) )
    ///  -> 1
    /// ```
    /// 
    /// 
    /// ## EOF
    ///
    fn eval(&self) -> Self::Output {
        static A: i32 = 1;

        // predefined:

        match self.macro_name.as_str() {
            "print" => {
                let mut tmp_string = self.origin_param.clone();

                while do_it(&mut tmp_string, self.scope.clone()) {
                    log_msg(&tmp_string)
                }

                return Expression::Data(HePrimitive::String(self.origin_param.clone()));
            }
            "string" => unreachable!("macro `string!` done while compile"),

            "count" => {
                let mut tmp_string = self.origin_param.clone();

                while do_it(&mut tmp_string, self.scope.clone()) {
                    log_msg(&tmp_string)
                }

                let params = he_parse_with_rule(&tmp_string, Rule::call_params);

                Expression::Data(HePrimitive::Int(self.params.len() as i32))
            },

            // name => match (*self.scope).borrow().match_macro(name, self.params.len()) {
            //     Some(mac) => {
            //         let tmp_string = self.origin_string.clone();

            //         while do_it(&mut tmp_string, self.scope.clone()) {
            //             log_msg(&tmp_string);
            //         }

            //         // match the macro params
            //     }
            //     None => panic!(
            //         "unknown macro call: {} with params number: {}",
            //         name,
            //         self.params.len()
            //     ),
            // },

            _ => unreachable!("unknown macro"),
        }
    }
}
