

pub trait Escape {
    fn escape_parenthese(&mut self);

    fn unescape(&mut self);
}

impl Escape for String {
    fn escape_parenthese(&mut self) {
        let mut res = String::new();

        let mut stk = 0;
        for ch in self.chars() {
            match ch {
                '(' => {
                    if stk > 0 {
                        res.push('\\');
                    }
                    stk += 1;
                    res.push('(')
                }

                ')' => {
                    if stk < 0 {
                        panic!("invalid pathrenthese match, have more ')' than '(' before it.")
                    }
                    stk -= 1;
                    if stk > 0 {
                        res.push('\\');
                    }
                    res.push(')')
                }

                ch => res.push(ch),
            }
        }

        *self = res
    }

    fn unescape(&mut self) {
        *self = self.replace("\\(", "(");
        *self = self.replace("\\)", ")");
    }
}
