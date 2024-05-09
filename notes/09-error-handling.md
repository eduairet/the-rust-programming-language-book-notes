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
