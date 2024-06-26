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

    ```Rust
    // Integers, signed | -2^(bit - 1) to 2^(bit - 1) - 1
    let integer_i8: i8 = -128; // 2^7
    let integer_i16: i16 = -32768; // 2^15
    let integer_i32: i32 = -2147483648; // 2^31
    let integer_i64: i64 = -9223372036854775808; // 2^63
    let integer_i128: i128 = -170141183460469231731687303715884105728; // 2^127
    let integer_isize: isize = -9223372036854775808; // 2^63

    // Integers, unsigned | 2^bit - 1
    let integer_u8: u8 = 255; // 2^8 - 1
    let integer_u16: u16 = 65535; // 2^16 - 1
    let integer_u32: u32 = 4294967295; // 2^32 - 1
    let integer_u64: u64 = 18446744073709551615; // 2^64 - 1
    let integer_u128: u128 = 340282366920938463463374607431768211455; // 2^128 - 1
    let integer_usize: usize = 18446744073709551615; // 2^64 - 1
    ```

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

    ```Rust
    // Floating point
    let float_f32: f32 = 3.141516; // 32-bit floating point
    let float_f64: f64 = 3.14159265358979323; // 64-bit floating point - More precise
    ```

  - Rust supports basic mathematical operations: `+`, `-`, `*`, `/`, `%`

    ```Rust
    // Numeric operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = 43 / 5; // Truncates the result to the nearest integer
    let remainder = 43 % 5;
    ```

- Booleans: `bool` (`true`, `false`)

  ```Rust
  // Boolean
  let boolean_true: bool = true;
  let boolean_false: bool = false;
  ```

- Characters: `char` (single Unicode character, 4 bytes)

  ```Rust
  // Character
  let char_a: char = 'a';
  let char_heart: char = '😻';
  ```

### Compound types

- Tuples

  - Group multiple values of different types
  - Fixed length
  - You can use destructuring to access values or dot notation to access a specific value
  - An empty tuple is written as `()` and is a type of unit value (no value)
    - Useful in situations where you need to represent the absence of a value

  ```Rust
  // Tuple
  let tuple: (i32, f64, u8) = (500, 6.4, 1);
  let (x, y, z) = tuple; // Destructuring
  println!("The value of y is: {}", y);
  println!("The value of tuple.0 is: {}", tuple.0);
  ```

- Arrays

  - Useful when you want your data allocated on the stack rather than the heap
  - Every element must have the same type
  - Fixed length
  - Useful when you know the number of elements at compile time

  ```Rust
  // Array
  let array: [i32; 5] = [1, 2, 3, 4, 5];
  let array2 = [3; 5]; // [3, 3, 3, 3, 3]
  let first = array[0];
  let second = array[1];
  let error = array[10]; // This will cause a panic
  // index out of bounds: the length is 5 but the index is 10
  ```

## Functions

- The `main` function is the entry point of many programs
- Functions are declared with the `fn` keyword
- Function naming convention is snake case `snake_case`
- They should be after the `main` function
- If you're using parameters, you must declare the type of each parameter

```Rust
fn main() {
    println!("Hello, world!");
    another_function(); // Function call
    function_with_parameters(5, 10); // Function call with parameters
}

fn another_function() {
    println!("Another function.");
}

fn function_with_parameters(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
```

### Statements and Expressions

- Rust is an expression-based language
- **Expressions** return a value

  - Calling a function or macro is an expression
  - Blocks of code evaluate to the last expression in them
  - Expressions do not include ending semicolons, if you add a semicolon, it becomes a statement

    ```Rust
    let y = {
      let x = 3;
      // Expression
      x + 1
    };
    println!("The value of y is: {}", y); // 4
    ```

- **Statements** are instructions that perform some action and do not return a value

  - Variable definitions are statements
  - Since declarations are statements, you can't assign a `let` statement to another variable

    ```Rust
    let y = 6; // Statement
    let x = (let y = 6); // Error
    // In other languages, this would be an expression
    // let x = y = 6;
    ```

### Function with return value

- You can return a value from a function by specifying the return type after an arrow `->`
- The return value of the function is the last expression in the block of the function body
- You can return early from a function by using the `return` keyword and specifying a value
- Most functions return `()` if you omit the return type
- Most functions return the last expression implicitly

  ```Rust
  fn five() -> i32 {
      5
  } // Implicit return, adding a semicolon would make it a statement and cause an error
  ```

## Control Flow

- Rust has three main categories of control flow:

  - `if` expressions

    ```Rust
    // If expression
    let number = 3;
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }
    ```

    - You need to use a boolean expression in the `if` statement

      ```Rust
      // Boolean expression
      let number = 3;
      if number {
          println!("Number is 3");
      } // Error
      ```

    - You can use `else if` to add more conditions

      ```Rust
      // Else if
      let number = 6;
      if number % 4 == 0 {
          println!("Number is divisible by 4");
      } else if number % 3 == 0 {
          println!("Number is divisible by 3");
      } else if number % 2 == 0 {
          println!("Number is divisible by 2");
      } else {
          println!("Number is not divisible by 4, 3, or 2");
      }
      ```

      - In rust is not recommended to use `else if` chains when you have more than 4 conditions because it can be hard to read, in this case, you should use a `match` expression

    - You can use `if` in a `let` statement

      ```Rust
      // If in a let statement
      let condition = true;
      let number = if condition { 5 } else { 6 };
      println!("The value of number is: {}", number);
      ```

      - The type of the `if` and `else` expressions must be the same

  - `loop` expressions

    ```Rust
    // Loop expression
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // It will run forever if we don't break
        }
    };
    println!("The result is {}", result);
    ```

    - When we nest loops, we can use `break` with a label to specify which loop to break

      ```Rust
      // Loop with a label
      'outer: loop {
          println!("Entered the outer loop");
          loop {
              println!("Entered the inner loop");
              break 'outer; // Breaks the outer loop
          }
          println!("This point will never be reached");
      }
      ```

    - We can use `continue` to skip the rest of the iteration and start a new one

      ```Rust
      // Continue
      for number in 1..=10 {
          if number % 2 == 0 {
              continue;
          }
          println!("The number is: {}", number);
      }
      ```

  - `while` expressions

    ```Rust
    // While expression
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
    ```

  - `for` expressions

    ```Rust
    // For expression
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }
    // With a range
    for number in (1..4).rev() {
        println!("{}", number);
    } // 3, 2, 1
    ```

## Appendix

- Reserved keywords: [Rust Reference](https://doc.rust-lang.org/reference/keywords.html)
- Allowed constant evaluations: [Rust Reference](https://doc.rust-lang.org/reference/const_eval.html)
