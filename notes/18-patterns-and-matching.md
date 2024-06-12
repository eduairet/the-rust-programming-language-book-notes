# Patterns and Matching

- Patterns are a way to match against the structure of types.

  ```rust
  // match option
  match x {
    None => None, // Return None if x is None
    Some(i) => Some(i + 1), // Return Some(i + 1) if x is Some(i)
  }

  // match arms
  match x {
      1 => println!("one"), // each arm is a pattern
      2 => println!("two"),
      3 => println!("three"),
      _ => println!("anything"), // _ is a catchall pattern
  }

  // if
  if let Some(i) = x {
      println!("{}", i);
  } else {
      println!("None");
  }

  // while let
  let mut stack = vec![1, 2, 3];
  while let Some(top) = stack.pop() {
      println!("{}", top);
  }

  // for
  let v = vec!['a', 'b', 'c'];
  for (index, value) in v.iter().enumerate() {
      println!("{} is at index {}", value, index);
  }

  // tuple destructuring
  let (x, y, z) = (1, 2, 3); // x = 1, y = 2, z = 3
  ```

  - `match` is exhaustive, so you must cover all possible cases.
  - `if let` doesn't check exhaustiveness.
  - When destructuring a tuple, you can use `_` to ignore some values.

## Refutability: Whether a Pattern Might Fail to Match

- Patterns can be refutable or irrefutable.

  - Refutable patterns can fail to match.
  - Irrefutable patterns always match.

    ```rust
    // refutable
    if let Some(x) = a_value {
        println!("{}", x);
    }
    // irrefutable
    let x = 5;
    ```

- Yoy can use pipes `|` to match multiple patterns.

  ```rust
  let x = Some(5);
  let y = 10;

  match x {
      Some(50) | Some(51) => println!("Got 50 or 51"),
      Some(y) => println!("Matched, y = {:?}", y),
      _ => println!("Default case, x = {:?}", x),
  }
  ```

- Or match expressions with refutable patterns.

  ```rust
  let x = Some(5);
  let y = 10;

  // Match guard
  match x {
      Some(50) => println!("Got 50"),
      Some(n) if n == y => println!("Matched, n = {:?}", n),
      _ => println!("Default case, x = {:?}", x),
  }

  // Match guard with multiple values
  let x = 4;
  let y = false;

  match x {
      4 | 5 | 6 if y => println!("yes"),
      _ => println!("no"),
  }
  ```

- Or .. to match a range.

  ```rust
  let x = 1;

  match x {
      1..=5 => println!("one through five"),
      _ => println!("anything"),
  }
  ```

- We can make more sophisticated destructuring patterns

  ```rust
  struct Point {
      x: i32,
      y: i32,
  }

  let p = Point { x: 0, y: 7 };

  let Point { x: a, y: b } = p;
  assert_eq!(0, a);
  assert_eq!(7, b);

  // or
  match p {
      Point { x, y: 0 } => println!("On the x axis at {}", x),
      Point { x: 0, y } => println!("On the y axis at {}", y),
      Point { x, y } => println!("On neither axis at ({}, {})", x, y),
  }
  ```

- Destructuring is also possible in enums.

  ```rust
  enum Message {
      Quit,
      Move { x: i32, y: i32 },
      Write(String),
      ChangeColor(i32, i32, i32),
  }

  let msg = Message::ChangeColor(0, 160, 255);

  match msg {
      Message::Quit => {
          println!("The Quit variant has no data to destructure.")
      }
      Message::Move { x, y } => {
          println!(
              "Move in the x direction {} and in the y direction {}",
              x, y
          );
      }
      Message::Write(text) => println!("Text message: {}", text),
      Message::ChangeColor(r, g, b) => {
          println!(
              "Change the color to red {}, green {}, and blue {}",
              r, g, b
          )
      }
  }
  ```

- And nested structs or enums are also possible.

  ```rust
  enum Color {
      Rgb(i32, i32, i32),
      Hsv(i32, i32, i32),
  }

  enum Message {
      Quit,
      Move { x: i32, y: i32 },
      Write(String),
      ChangeColor(Color),
  }

  let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

  match msg {
      Message::ChangeColor(Color::Rgb(r, g, b)) => {
          println!(
              "Change the color to red {}, green {}, and blue {}",
              r, g, b
          )
      }
      Message::ChangeColor(Color::Hsv(h, s, v)) => {
          println!(
              "Change the color to hue {}, saturation {}, and value {}",
              h, s, v
          )
      }
      _ => ()
  }
  ```

- Structs and tuples can be destructured as well.

  ```rust
  let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
  ```

- Even though we've seen the `_` match anything, we can use more complex patterns to ignore some parts of a value.

  - Using `_` to ignore a value while destructuring a tuple or struct.

    ```rust
    let (x, _, z) = (1, 2, 3);

    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
    ```

    - `_` is also used to prefix unused variables.

      ```rust
      let _x = 5;

      let s = Some(String::from("Hello!"));

      if let Some(_) = s {
          println!("found a string");
      }

      println!("{:?}", s);
      ```

- You can use `..` to ignore a range of values.

  - The `..` pattern ignores all values not explicitly matched between two values.

    ```rust
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
    ```

  - Or the remaining values.

    ```rust
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, ..) => {
            println!("First number is {}", first);
        }
    }
    ```

  - You can't use `..` more than once in a pattern.

- We also have `@` to bind a value to a variable.

  ```rust
  enum Message {
      Hello { id: i32 },
  }

  let msg = Message::Hello { id: 5 };

  match msg {
      Message::Hello {
          id: id_variable @ 3..=7, // bind the value to id_variable
      } => {
          println!("Found an id in range: {}", id_variable)
      }
      Message::Hello { id: 10..=12 } => {
          println!("Found an id in another range")
      }
      Message::Hello { id } => {
          println!("Found some other id: {}", id)
      }
  }
  ```
