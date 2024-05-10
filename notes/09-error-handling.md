# Error Handling

- Rust classifies errors into two groups: recoverable and unrecoverable errors.
- Recoverable errors are those that a program can handle and recover from.
- Unrecoverable errors are those that a program can't handle, like a bug in the program.
- Rust doesn't have exceptions, but it has the `Result` enum to handle recoverable errors.
  - The `Result` enum has two variants: `Ok` and `Err`.
    - The `Ok` variant indicates that the operation was successful and contains the result value.
    - The `Err` variant indicates that the operation failed and contains an error value.

## Unrecoverable Errors with `panic!`

- When rust panics, it prints a failure message, unwinds and cleans up the stack, and then exits.
- Panicking when accessing an invalid index in a vector:
  ```Rust
  let v = vec![1, 2, 3];
  v[99]; // This will panic
  ```
  - This kind of panic is called a **buffer overflow**.
- You can use the `panic!` macro to generate a panic.
  ```Rust
  panic!("error message");
  // The program will panic and print the error message with an output like this:
    // thread 'main' panicked at 'error message', src/main.rs:2:3
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
  ```
- A backtrace is a list of all the functions that have been called to get to this point.
- You can run the program with the `RUST_BACKTRACE=1` environment variable to display a backtrace.
- Panicking is good idea when something unexpected can happen, like a bug in the program, the user providing invalid input, or a file not found, etc.

## Recoverable Errors with `Result`

- The `Result` enum is defined as follows:

  ```rust
  enum Result<T, E> {
      Ok(T),
      Err(E),
  }
  ```

- We can use the `Result` enum to handle recoverable errors like this:

  ```rust
  use std::fs::File;

  fn main() {
      let f = File::open("hello.txt");

      let f = match f {
          Ok(file) => file,
          Err(error) => {
              panic!("Problem opening the file: {:?}", error) // This will panic
          },
      };

      // thread 'main' panicked at src/main.rs:9:13:
      // Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }
      // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
  }
  ```

## Matching Different Errors

- You can match different errors to handle them differently:

  ```rust
  use std::fs::File;
  use std::io::ErrorKind;

  fn main() {
      let greeting_file_result = File::open("hello.txt");

      let greeting_file = match greeting_file_result {
          Ok(file) => file, // File opened successfully
          Err(error) => match error.kind() {
              ErrorKind::NotFound => match File::create("hello.txt") {
                  Ok(fc) => fc,
                  Err(e) => panic!("Problem creating the file: {:?}", e),
              }, // File not found, create it
              other_error => {
                  panic!("Problem opening the file: {:?}", other_error);
              } // Other error
          },
      };
  }
  ```

- Or use an if let statement to handle the error:

  ```rust
  use std::fs::File;
  use std::io::ErrorKind;

  fn main() {
      let greeting_file_result = File::open("hello.txt");

      let greeting_file = match greeting_file_result {
          Ok(file) => file, // File opened successfully
          Err(error) => {
              if error.kind() == ErrorKind::NotFound {
                  match File::create("hello.txt") {
                      Ok(fc) => fc,
                      Err(e) => panic!("Problem creating the file: {:?}", e),
                  } // File not found, create it
              } else {
                  panic!("Problem opening the file: {:?}", error);
              } // Other error
          },
      };
  }
  ```

## Shortcuts for Panic on Error: `unwrap` and `expect`

- The `unwrap` method is a shortcut for panic on error:

  ```rust
  use std::fs::File;

  fn main() {
      let f = File::open("hello.txt").unwrap(); // This will panic
  }
  ```

- The `expect` method is similar to `unwrap`, but it allows you to specify the error message:

  ```rust
  use std::fs::File;

  fn main() {
      let f = File::open("hello.txt").expect("Failed to open hello.txt"); // This will panic
  }
  ```

- Calling `unwrap` or `expect` is a good choice when you're sure that the operation will succeed.

## Propagating Errors

- You can propagate errors by calling code to handle the error:

  ```rust
  use std::fs::File;
  use std::io::{self, Read};

  fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
  }
  ```

## A Shortcut for Propagating Errors: The `?` Operator

- The `?` operator is a shortcut for propagating errors:

  ```rust
  use std::fs::File;
  use std::io::{self, Read};

  fn read_username_from_file() -> Result<String, io::Error> {
      let mut username_file = File::open("hello.txt")?;
      let mut username = String::new();
      username_file.read_to_string(&mut username)?;
      Ok(username)
  }
  ```

- `?` uses the `From` trait to convert the error type to the return type of the function.
- You can chain multiple `?` operators to propagate errors and make the code more concise:

  ```rust
  use std::fs::File;
  use std::io::{self, Read};

  fn read_username_from_file() -> Result<String, io::Error> {
      let mut username = String::new();
      File::open("hello.txt")?.read_to_string(&mut username)?;
      Ok(username)
  }
  ```

## Where the `?` Operator Can Be Used

- With functions compatible with the value returned by the function.
  - Inside functions that return `Result` or `Option`.
  - Custom types that implement the `From` trait.
  - You can't mix `Result` and `Option` in the same function.
    - You can use the `ok_or` method to convert an `Option` to a `Result`.
- With the `main` function:
  - You can use the `?` operator in the `main` function if it returns `Result<(), T>`.
  - You can use the `Result` type to handle errors in the `main` function.

## Creating Custom Types for Validation

- You can create custom types to handle validation:

  ```rust
  pub struct Guess {
      value: i32,
  }

  impl Guess {
      // Constructor method
      pub fn new(value: i32) -> Guess {
          if value < 1 || value > 100 {
              panic!("Guess value must be between 1 and 100, got {}.", value);
          }

          Guess { value }
      }

      // Getter method
      pub fn value(&self) -> i32 {
          self.value
      }
  } // This will panic if the value is not between 1 and 100
  ```
