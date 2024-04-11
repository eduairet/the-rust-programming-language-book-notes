# Common Programming Concepts

## Varialbes and Mutability

- Variables are immutable by default
- Use `let` to declare a variable
- Use `mut` to make a variable mutable
- Constants are always immutable and must be annotated with a type
  - Use `const` to declare a constant
  - Constants can be declared in any scope, including the global scope
  - Constant naming convention is to use all uppercase with underscores between words (`SOME_CONSTANT`)
- Shadowing allows to reuse a variable name
  - This will make the compiler to use the shadowed variable instead of the original one
  - Shadowing is useful to change the type of a variable without changing its name

## Data Types

- Rust is a statically hard typed language
- Rust must know the types of all variables at compile time, but the compiler can usually infer the types
- Rust has two data type categories: scalar (single value) and compound (multiple values)

### Scalar Types

- Numbers:

  - Integers:
    | Length | Signed | Unsigned |
    | ------ | ------ | -------- |
    | 8-bit | `i8` | `u8` |
    | 16-bit | `i16` | `u16` |
    | 32-bit | `i32` | `u32` |
    | 64-bit | `i64` | `u64` |
    | 128-bit| `i128` | `u128` |
    | arch | `isize`| `usize` |
    - The default integer type is `i32`
    - Signed integers can store positive and negative numbers
    - Unsigned integers can store only positive numbers
    - `arch` depends on the kind of computer your program is running on
    - Numbers can use underscores to improve readability: `let x: i32 = 1_000;`
    - You can use type suffixes to specify the type of a number: `let x = 42u32;`
  - Integer literals:
    - Decimal: `98_222`
    - Hex: `0xff`
    - Octal: `0o77`
    - Binary: `0b1111_0000`
    - Byte (u8 only): `b'A'`
  - Floating-point numbers: `f32`, `f64`
    - Numbers with a decimal point
    - Default type is `f64` because it's roughly the same speed as `f32` but is more precise
    - Rust uses the IEEE-754 standard for floating-point numbers
  - Rust supports basic mathematical operations: `+`, `-`, `*`, `/`, `%`

- Booleans: `bool` (`true`, `false`)
- Characters: `char` (single Unicode character, 4 bytes)
  - `'a' '😻'`

### Compound types

- They can group multiple values into one type

## Appendix

- Reserved keywords: [Rust Reference](https://doc.rust-lang.org/reference/keywords.html)
- Allowed constant evaluations: [Rust Reference](https://doc.rust-lang.org/reference/const_eval.html)
