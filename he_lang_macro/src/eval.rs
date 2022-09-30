use core::cell::RefCell;
use std::rc::Rc;

use crate::{
    io::{input, log_msg, log_normal, log_success, std_out},
    list::List,
    parser::{he_parse_with_rule, parse_expr, parse_main, parse_params, Rule},
    scope::Scope,
    types::*,
};

macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

impl MacroCall {
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
    /// ## can't reform in marco itself
    ///
    /// ## EOF
    ///
    pub fn eval(&self, scope: Rc<RefCell<Scope>>) -> Expression {
        log_normal!(format!("evaling macro {self:#?}"));
        (*scope).borrow_mut().stack.push(self.macro_name.clone());

        input();
        std_out!(format!(
            "{}    {}",
            (*scope).borrow().make_tab(),
            self.to_string()
        ));

        match self.macro_name.as_str() {
            "print" => {
                let mut tmp_string = self.origin_param.clone();

                into_do_it(&mut tmp_string, scope.clone());

                std_out!(format!("`print` output: \"{tmp_string}\""));

                (*scope).borrow_mut().stack.pop();
                return Expression::Data(HePrimitive::String(tmp_string));
            }
            "string" => unreachable!("macro `string!` done while compile"),

            "count" => {
                let mut tmp_string = self.origin_param.clone();

                into_do_it(&mut tmp_string, scope.clone());

                let params = he_parse_with_rule(&tmp_string, Rule::call_params);
                let params = parse_params(params);

                (*scope).borrow_mut().stack.pop();
                Expression::Data(HePrimitive::Int(params.len() as i32))
            }

            name => {
                let mut tmp_string = self.origin_param.clone();

                into_do_it(&mut tmp_string, scope.clone());

                let params = parse_params(he_parse_with_rule(&tmp_string, Rule::call_params));

                let mac = (*scope).borrow().match_macro(name, params.len()).cloned();
                match mac {
                    Some(mac) => {
                        let res = Expression::Raw(
                            mac.replace(params.into_iter().map(|x| x.to_string()).collect()),
                        );

                        (*scope).borrow_mut().stack.pop();
                        // match the macro params
                        res
                    }
                    None => panic!(
                        "unknown macro call: \n{self:#?} with params number: {}",
                        params.len()
                    ),
                }
            }
        }
    }
}

impl Macro {
    pub fn new(name: String, from: List<String>, to: String) -> Macro {
        let mut res = Macro { name, from, to };

        res.from.sort_by(|a, b| a.len().cmp(&b.len()).reverse());

        res
    }

    pub fn replace(&self, params: Vec<String>) -> String {
        assert_eq!(
            params.len(),
            self.from.len(),
            "[in macro replace] params amount not match"
        );

        fn dfs<'a>(
            now_string: String,
            i: usize,
            names: &Vec<String>,
            values: &Vec<String>,
        ) -> String {
            if i == names.len() {
                return now_string;
            }

            let name = &names[i];
            let value = &values[i];

            if name.is_empty() {
                return dfs(now_string, i + 1, names, values);
            } else {
                return now_string
                    .split(name)
                    .map(|x| dfs(x.to_string(), i + 1, names, values))
                    .collect::<Vec<String>>()
                    .join(value);
            }
        }

        dfs(self.to.clone(), 0, &self.from, &params).to_string()
    }
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
    let re_macro_call = regex!(r"[a-zA-Z][a-zA-Z0-9]+[[:space:]]*!(.*)");

    let begin = re_macro_call.find(s);
    match begin {
        Some(begin) => {
            let pair = he_parse_with_rule(&s.get(begin.start()..).unwrap(), Rule::expression);

            log_normal!(format!("found expr: {pair:#?}"));

            let expr = parse_expr(pair.clone());
            let expr_string = pair.as_span().as_str();

            match expr {
                Expression::MacroCall(macro_call) => {
                    let eval_res = macro_call.eval(scope.clone());
                    *s = s.get(..begin.start()).unwrap().to_owned()
                        + &eval_res.to_string()
                        + match s.get(begin.start() + expr_string.len()..) {
                            Some(later) => later,
                            None => "",
                        }
                }
                _ => unreachable!("unexpected match. should be macro_call: \n{pair:#?}"),
            }

            std_out!(format!(
                "{} => {}! ( {} )",
                (*scope).borrow().make_tab(),
                (*scope).borrow().stack.last().unwrap(),
                s
            ));
            input();

            true
        }
        None => false,
    }
}

fn into_do_it(s: &mut String, scope: Rc<RefCell<Scope>>) {
    (*scope).borrow_mut().push_cnt();
    while do_it(s, scope.clone()) {
        // log_normal!(format!("do it: {s}"))
    }
    (*scope).borrow_mut().pop_cnt();
}

pub fn link_start(s: &str) {
    std_out!(s);
    let scope = Rc::new(RefCell::new(Scope::new()));
    let statements = parse_main(he_parse_with_rule(s, Rule::main));

    log_success!(format!("{statements:#?}"));

    for statement in statements {
        match statement {
            HeType::Expression(expr) => match expr {
                Expression::MacroCall(macro_call) => {
                    log_msg!(ansi_term::Color::Cyan
                        .bold()
                        .paint(format!("\n\t {}\n", macro_call.to_string())));
                    let res = macro_call.eval(scope.clone()).to_string();
                    log_success!(format!("success:\n    {s}\n => {:#?}", res));
                }
                _ => {
                    log_success!(format!("expr\n    {s}\n => {:#?}", expr.to_string()))
                }
            },
            HeType::MacroDef(macro_def) => {
                log_normal!(format!("push into scope: \n{:#?}", macro_def));
                (*scope).borrow_mut().push_macro(macro_def);
            }
        }
    }
}

#[allow(unused)]
mod test {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::io::*;
    use crate::parser::test::*;
    use crate::parser::*;
    use crate::scope::Scope;
    use crate::test_examples::*;
    use crate::types::{Expression, HeType};

    use super::*;

    #[test]
    fn test_eval() {
        log_init();

        // link_start(define_and_call);
        // link_start(recurse_call);
        // link_start(print_recurse_call);
        // link_start(nested_call);

        link_start(
            r#"
            one! = {
                () => ;
            }
            
            add1! = {
                ($a) => {
                    $a | 
                };
            }
        
        print!(count!(|));
        print!(count!(add1!(add1!(one!()))));
    "#,
        );
    }

    #[test]
    fn test_predefined() {
        log_init();
        for s in PRE_DEFINED {
            link_start(s);
        }
    }
}
