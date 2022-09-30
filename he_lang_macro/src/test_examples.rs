#![allow(unused)]

macro_rules! make_examples {
    ($group_name: ident, [$($name:ident = $s:expr);*]) => {
        macro_rules! make_example{
            ($name_1:ident = $s_1:expr) =>{
                pub const $name_1: &str = $s_1;
            };
        }

        $(make_example!($name = $s);)*
        pub const $group_name: &[&str] = &[$($name,)*];
    };
}

make_examples!(
    EXAMPLE_PRINT,
    [
        PRINT = "print!()";
        PRINT_PRINT = "print!(print!)";
        PRINT_PRINT_2 = "print!(print!(2))"
    ]
);

make_examples!(
    COMPLEX,
    [COMPLEXT_1 = r#"
        b! = {
            () => c!;
        }
        
        ab! = {
            () => end!;
        }
        
        ac! = {
            () => "in ac";
        }
        
        end! = {
            () => "ended";
        }
        
        make_macro! = {
            (1) => {
                ab!()()
            };
        
            (2) => {
                a(b!)()()
            };
        }
        
        // output should be
        //  
        //  "ended"
        //  "in ac"
        
        print!(make_macro!(1));
        print!(make_macro!(2));
        "#]
);

make_examples!(
    BASIC,
    [
        def_macro_1 = r#"
        a! = {
            ($a | $b| $c) => {gfdsgf};
        }
        "#;
        def_macro_2 = r#"b! = {() => ;}"#;
        def_macro_cat = r#"
        cat! = {
            ($a | $b) => $a$b;
        }
        "#;

        expression_1 = "1;";
        expression_s_123 = r#""123";"#;

        macro_call_1 = r#"a!(1 | 2 | 3);"#;
        macro_call_empty_param = r#"a!(   );"#;
        macro_call_empty_param_4 = r#"a!( | ||);"#;
        macro_call_any_param = r#"a!(fdsaf&fd438r4\));"#;
        macro_call_paren_param = r#"a!(1|(|)|2);"#
    ]
);

make_examples!(
    PRE_DEFINED,
    [
        macro_call_string = r#"string!(gfdsagdfasf90 90dsafj d| sad );"#;

        macro_call_print = r#"print!(1 | 2)"#;

        macro_call_count = r#"count!(1 | (1 | 2) | 2)"#
    ]
);

#[test]
fn test_make() {
    println!("{}", PRINT_PRINT_2);
}

#[test]
fn test_make_group() {
    for x in EXAMPLE_PRINT {
        println!("{x}")
    }
}
