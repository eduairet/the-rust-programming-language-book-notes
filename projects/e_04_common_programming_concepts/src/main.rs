fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    // If we try to change the value of x, we will get a compile-time error
    // x = 6;
    // ```
    // 3 |     println!("The value of x is: {}", x);
    // 4 |     x = 6;
    //   |     ^^^^^ cannot assign twice to immutable variable
    // ```

    let mut y = 10;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is: {}", y);
    // Here we can see that we can change the value of y because it is mutable.
    // The value of y is: 5
    // The value of y is: 6

    const LANGUAGE: &str = "Rust";
    const DAYS_OF_TWO_WEEKS: u32 = 7 * 2; // Constants can be assigned expressions

    let shadowed_variable = DAYS_OF_TWO_WEEKS;
    {
        // You can shadow on different scopes
        let shadowed_variable = LANGUAGE;
        println!(
            "The value of inner scope shadowed_variable is: {}",
            shadowed_variable
        );
    }
    println!("The value of shadowed_variable is: {}", shadowed_variable);
    let shadowed_variable = LANGUAGE; // we can change the type of the variable when shadowing
    println!("The value of shadowed_variable is: {}", shadowed_variable);

    // Scalar types

    // Integers, signed | -2^(bit - 1) to 2^(bit - 1) - 1
    let scalar_integer_i8: i8 = -128; // 2^7
    let scalar_integer_i16: i16 = -32768; // 2^15
    let scalar_integer_i32: i32 = -2147483648; // 2^31
    let scalar_integer_i64: i64 = -9223372036854775808; // 2^63
    let scalar_integer_i128: i128 = -170141183460469231731687303715884105728; // 2^127
    let scalar_integer_isize: isize = -9223372036854775808; // 2^63

    // Integers, unsigned | 2^bit - 1
    let scalar_integer_u8: u8 = 255; // 2^8 - 1
    let scalar_integer_u16: u16 = 65535; // 2^16 - 1
    let scalar_integer_u32: u32 = 4294967295; // 2^32 - 1
    let scalar_integer_u64: u64 = 18446744073709551615; // 2^64 - 1
    let scalar_integer_u128: u128 = 340282366920938463463374607431768211455; // 2^128 - 1
    let scalar_integer_usize: usize = 18446744073709551615; // 2^64 - 1

    // Floating point
    let scalar_float_f32: f32 = 3.141516; // 32-bit floating point
    let scalar_float_f64: f64 = 3.14159265358979323; // 64-bit floating point - More precise

    // Numeric operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = 43 / 5; // Truncates the result to the nearest integer
    let remainder = 43 % 5;

    let scalar_boolean: bool = true;
    let scalar_character: char = 'a';

    // Compound types

    // Tuple
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tuple; // Destructuring
    println!("The value of y is: {}", y);
    println!("The value of tuple.0 is: {}", tuple.0);

    // Array
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let first = array[0];
    let second = array[1];
    // let error = array[10]; // This will cause a panic

    // Functions
    some_function();
    function_with_parameters(5, 10);

    // Statements and expressions
    let x = 5; // Statement

    // let x = (let y = 6);
    // You can't assign a statement to a variable
    // you'll get a compile-time error

    let y = {
        let x = 3;
        // Expression
        x + 1
    };
    println!("The value of y is: {}", y);

    let z = five();
    println!("The value of z is: {}", z);

    // Control flow

    // If
    let number = 3;
    if number < 5 {
        println!("The number is less than 5"); // This will be printed
    } else if number == 5 {
        println!("The number is equal to 5");
    } else {
        println!("The number is greater than or equal to 5");
    }

    // If in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // Loops

    // Loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The value of result is: {}", result);

    'outer: loop {
        println!("Entered the outer loop");
        loop {
            println!("Entered the inner loop");
            break 'outer; // Breaks the outer loop
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");

    // While
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    // For
    let array = [10, 20, 30, 40, 50];
    for element in array {
        println!("The value of element is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}", number);
    }
}

// Functions
fn some_function() {
    println!("returned");
}

fn function_with_parameters(x: i32, y: i32) {
    println!("The coord is: (x = {x}, y = {y})");
}

fn five() -> i32 {
    5
}
