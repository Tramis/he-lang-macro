
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
    [
        COMPLEXT_1 = r#"
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
        "#
    ]
);

#[test]
fn test_make() {
    println!("{}", PRINT_PRINT_2);
}

#[test]
fn test_make_group(){
    for x in EXAMPLE_PRINT{
        println!("{x}")
    }
}
