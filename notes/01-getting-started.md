# Getting Started

## Installing Rust

- First [install rustup](https://www.rust-lang.org/learn/get-started), a toolchain manager for Rust
  ```SHELL
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
  - If everything goes well, you should see a message like this:
    ```SHELL
    Rust is installed now. Great!
    ```
  - Check the installed version with:
    ```SHELL
    rustc --version
    # rustc 1.77.1 (7cf61ebde 2024-03-27)
    ```
- Then you'll need a linker to join the compiled outputs in one file
  - If it's not installed, you can install it with:
    ```SHELL
    xcode-select --install
    ```
- If you need to update just run:
  ```SHELL
  rustup update
  ```
- Uninstall Rust:
  ```SHELL
  rustup self uninstall
  ```
- Check the documentation offline using:
  ```SHELL
  rustup doc
  ```

## Compiling and Running a Rust Program

- Create a new file with the `.rs` extension, for example, `main.rs`
- Write the code [^1]
  ```RUST
  // First we define the main function, rust scopes are defined by curly braces
  fn main() {
      println!("Hello, world!"); // This line prints the text "Hello, world!" to the console using the println! macro
      // Notice that the line ends with a semicolon (;), this is how we end expressions in Rust and it's mandatory
  }
  ```
  - `main()` is the very first function that runs in every Rust program

[^1]: If you need to format the code, you can use the `rustfmt` tool `rustfmt main.rs` or your favorite editor extension

- Compile the program with:
  ```SHELL
  rustc main.rs
  ```
  - This will generate an executable file named `main`
- Run the program with:
  ```SHELL
  ./main
  ```
  - This file can be run on any system without the need for Rust to be installed
- You should see the output:
  ```SHELL
  Hello, world!
  ```

## Cargo

- Cargo is the Rust package manager and build system
- It's the recommended way to manage your Rust projects
- Cargo comes with rustup, check the installation with:
  ```SHELL
  cargo --version
  # cargo 1.77.1 (e52e36006 2024-03-26)
  ```
- Create a project with cargo:

  ```SHELL
  cargo new hello_cargo
  cd hello_cargo
  ```

  - This will create a new directory named `hello_cargo` with the following structure:
    ```SHELL
    hello_cargo
    ├── Cargo.toml
    └── src
        └── main.rs
    ```
  - `Cargo.toml` is the manifest file that describes the project and its dependencies

    ```TOML
    [package]
    name = "hello_cargo"
    version = "0.1.0"
    edition = "2021"

    # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

    [dependencies]
    ```

    - If you need to add dependencies, you can add them to the `[dependencies]` section or use the `cargo add` command
      ```SHELL
      cargo add <dependency>
      ```
    - Notice that cargo uses Semantic Versioning, so the version number is divided into three parts: `MAJOR.MINOR.PATCH`
    - If you need to check the documentation of a crate, you can use:
      ```SHELL
      cargo doc --open
      ```
    - This will generate the documentation for the project and its crates and open it in the browser

  - The `src` directory is where the source code files are stored
    - `src/main.rs` is the main file of the project
  - If the project is not a `git` repository it will be initialized automatically
    - Explicitly initialize with or without a git repository using `cargo new hello_cargo --vcs git` or `cargo new hello_cargo --vcs none`

- Build the project with:
  ```SHELL
  cargo build
  ```
  - This will create a `target` directory with the compiled files, try running the executable with:
    ```SHELL
    ./target/debug/hello_cargo
    ```
- A better way to run the project is with:
  ```SHELL
  cargo run
  ```
  - This will compile and run the project in one step, if the project didn't change, it will only run the executable
- There's also a `check` command that only checks the project without building it, this is useful to check for errors without waiting for the build process
  ```SHELL
  cargo check
  ```
- To build the project for release, use:
  ```SHELL
  cargo build --release
  ```
  - This will create an optimized executable in the `target/release` directory

## Syntax introduction

- Rust is a statically typed language, meaning that it must know the types of all variables at compile time
- Variables are immutable by default, to make them mutable use the `mut` keyword
- Rust uses snake case for variable and function names
- The `::` syntax in Rust is used for both associated functions and namespaces
- Comments are created with `//` for single-line comments and `/* */` for multi-line comments
- Macros are called with `!` at the end of the name
- The `&` symbol is used to create references, which allow you to refer to some value without taking ownership of it, this is called borrowing
- Code example:

  ```RUST
  use std::io; // Bring the io library into scope

  // fn is the keyword to declare a function
  fn main() {
      println!("Guess the number!"); // Print a message to the console with the println! macro
      println!("Please input your guess.");

      let mut guess = String::new(); // Create a mutable variable named guess and bind it to a new, empty instance of a String

      // We can also use std::io::stdin() instead of io::stdin() if we don't want to bring the io library into scope
      io::stdin()
          .read_line(&mut guess)
          .expect("Failed to read line");
      // Read a line from the standard input and bind it to the guess variable, the .expect() method is called on the Result type returned by read_line to handle any errors

      let guess: u32 = guess.trim().parse().expect("Please type a number!"); // Parse the string into a number, shadowing the previous value of guess

      println!("You guessed: {guess}"); // Print the value of the guess variable to the console using a placeholder {}
  }
  ```
