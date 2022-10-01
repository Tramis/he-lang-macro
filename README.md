# he-lang-macro
he-lang 的宏实现

目标是做一个 rust 宏和 c 宏的魔幻缝合怪，毫无卫生性可言

- [he-lang-macro](#he-lang-macro)
  - [He the Great](#he-the-great)
  - [Usage](#usage)
  - [Overview](#overview)
  - [Macro](#macro)
  - [Predefined](#predefined)

## He the Great
无需多言

## Usage

``` bash
he-lang -f abc.he
he-lang -i print!(1 | 2 | 3)
# TODO: interactive
```

## Overview

```rust
a! = {
    () => {
        a!
    };
    ($a) => a!$a;
}

print!( a! ( a ) );

// output:
//      `a!a`
```

## Macro

- 宏调用时，将吃掉所有输入，并将其简单替换

```rust 
// define macro
a! = {
    ($a | $b | $c) => {
        $a | $c
    };

    ($a | $c) => {
        ($c)
    };
}


// `print!` is predefined macro
// use macro
// 
// output shoull be: 
// `1 | 3`
print! (a! (1 | 2 | 3));


// nested call
// 
// output:
//  `(3)`
print! (a! (a! (1 | 2 | 3)));


// macro `print!` equals its input
// 
// output: 
//  `1 | 2`
//  `1 | 2`
print! (print!(1 | 2));

```

## Predefined

- `print!`
```rust
print!(1 | 2);
print!(1, 2, 3);

// macro `print!` equals its input
// 
// output: 
//  `1 | 2`
//  `1 | 2`
print! (print!(1 | 2));

// output: 
// `print!`
print!(print!);
```

- `raw!`
```rust
// output: 
//  `a s v dfv fd`
print!(raw!(a s v dfv fd));
```

- `count!`
```rust
// output: 
//  `4`
print!(count!(1 | | |));


```
- `()`

```rust
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
```