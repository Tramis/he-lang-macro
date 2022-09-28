# he-lang-macro
he-lang 的宏实现

目标是做一个 rust 宏和 c 宏的魔幻缝合怪，毫无卫生性可言

- [he-lang-macro](#he-lang-macro)
  - [He the Great](#he-the-great)
  - [Usage](#usage)
  - [design](#design)
    - [KeyWord](#keyword)
    - [Macro](#macro)
    - [Ident](#ident)
    - [预制表](#预制表)

## He the Great
无需多言

## Usage

``` bash
he-lang abc.he
he-lang print!(1 | 2 | 3)
# TODO: interactive
```

## design

### KeyWord
1. `'|'`
 （何符号）
2. `'!'` （宏符号）
3. `'"'` / `'''` （引号，用于字符串）
4. `'()'` （括号，用于宏调用或组合值）

### Macro

宏调用时，将吃掉所有输入，并将其简单替换

```rust 
// define macro
a! = {
    ($a | $b | $c) => {
        $a | $c
    }

    ($a | $c) => {
        ($c)
    }
}


// `print!` is predefined macro
// use macro
// 
// output shoull be: `1 3`
print! (a! (1 | 2 | 3))


// nested call
// 
// output should be `3`
print! (a! (a! (1 | 2 | 3)))


// macro `print!` equals its input
// 
// output should be: 
// `1 | 2`
// `1 | 2`
print! (print!(1 | 2))

```

常量参数

```rust
a! = {
    (0) => {0}
    ($a | $b) => {
        $a + a!($b)
    }
}

```

### Ident

变量将用于单纯的字符串替换

```rust
a = 
```

### 预制表

- `print!`
```rust
print!(1 | 2)
print!(1, 2, 3)

// 

print!(print!(1 | 2))

// if input is not string / int, `print!(a)` equals `print!(string!((!a)))
// output: 
// "(!print)"
print!(print!)
```

- `string!`
```rust
// "a s v dfv fd"
print!(string!(a s v dfv fd))
// 
print!(print!)
```

- `$sep`
```rust
one_two_one! = {
    ($a, $b) => {$a $b $a}
}

make_two! = {
    ($a) => {
        one_two_one!($a, $sep)
    }
}

```
- `()`

```rust
b! = {
    c!
}

ab! = {
    () => end!
}

ac! = {
    () => "in ac"
}

end! = {
    () => "ended"
}

make_macro! = {
    (1) => {
        ab!()()
    }

    (2) => {
        a(b!)()()
    }
}

// output should be
//  
//  "ended"
//  "in ac"

print!(make_macro!(1))
print!(make_macro!(2))
```