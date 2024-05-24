# Minigrep

A simple implementation of the `grep` command-line utility in Rust.

## Usage

- To search for a pattern in a file, run the following command:
  ```sh
  cargo run -- <pattern> <file>
  ```
- To search for a pattern in a file with case-insensitive matching, run the following command:

  ```sh
  IGNORE_CASE=1 cargo run -- <pattern> <file>
  ```

- To search for a pattern in a file with case-insensitive matching and write the output to a file, run the following command:
  ```sh
  IGNORE_CASE=1 cargo run -- <pattern> <file> > <output_file>
  ```

## Notes

- When building any kind of project you should always need to know when to separate concerns and when to combine them.

  - Split the program into `main.rs` and `lib.rs` and move the logic to `lib.rs`.
    - If you have a small project, you can put all the code in `main.rs` and avoid creating a `lib.rs` file.
  - `main.rs` should handle the command-line parsing, read the file, call the library functions, and handle errors.

- Getting `args` from the command line with `std::env::args()`

```rust
use std::{env, fs};

fn main() {
    // First we use the env::args function to get the command line arguments that were passed to the program.
    // This function returns an iterator, so we collect the iterator into a vector so we can access the values.
    let args: Vec<String> = env::args().collect();
    // We use the dbg! macro to print the value of the args variable to the console.
    // This macro is useful for debugging because it prints the value of the variable along with the file and line number where it was called.
    dbg!(&args);
    /* Output:
        cargo run
        [src/main.rs:9:5] args = [
            "target/debug/minigrep",
        ]

        cargo run -- Test
        [src/main.rs:9:5] args = [
            "target/debug/minigrep",
            "Test",
        ]

        cargo run -- Test test.txt
        [src/main.rs:9:5] &args = [
            "target/debug/minigrep",
            "Test",
            "test.txt",
        ]
    */
}
```

- Reading the contents of a file with `fs::read_to_string()`

```rust
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    // We use the fs::read_to_string function to read the contents of the file specified in the command line arguments.
    // This function returns a Result<String>, so we use the ? operator to handle any errors that may occur.
    let contents = fs::read_to_string(&args[2])?;
    dbg!(&contents);
    /* Output:
        cargo run -- Test test.txt
        [src/main.rs:15:5] &contents = "This is a test file.\nIt contains some text.\n"
    */
}
```

- Reading environment variables with `std::env::var()`

```rust
use std::env;

fn main() {
    // We use the env::var function to read the value of the HOME environment variable.
    // This function returns a Result<String>, so we use the ? operator to handle any errors that may occur.
    let home = env::var("HOME")?;
    dbg!(&home);
    /* Output:
        [src/main.rs:6:5] &home = "/home/user"
    */
}
```

- To call the program with an environment variable, you can use the `env` command in the terminal.

  ```sh
  env HOME=/home/user cargo run
  ```

- Making a file with the output of the program

  ```sh
  cargo run > output.txt
  ```

- Using `clone()` to solve borrowing issues is often a bad idea because it can be slow and use a lot of memory.
