# Managing Growing Projects with Packages, Crates, and Modules

- Cargo offers a way to build a project with multiple files, multiple dependencies, and multiple configurations.
  - **Packages:** A Cargo feature that lets you build, test, and share crates
  - **Crates:** A tree of modules that produces a library or executable
  - **Modules and use:** Let you control the organization, scope, and privacy of paths
  - **Paths** A way of naming an item, such as a struct, function, or module
- When cargo compiles a project, it follows the next logic:
  - It looks for a `Cargo.toml` file and reads the metadata
  - It looks for a `src` directory and reads the source files
  - It compiles the project and places the resulting executable or library in the `target` directory

## Packages and Crates

- Crate
  - Smallest compilation unit in Rust
  - They can contain modules, and modules can contain other modules
  - They can contain a binary or a library
    - Binary: An executable with a `main.rs` file
    - Library: Code that can be called by other programs and has a `lib` file instead of `main.rs`
      - The command `cargo new my_library --lib` creates a library crate
- Package
  - One or more crates that provide a set of functionality
  - Contains a `Cargo.toml` file that describes how to build those crates
  - Can contain zero or one library crates and as many binary crates as you’d like (but it must contain at least one crate [either library or binary])

## Modules

- Modules let you control the organization, scope, and privacy of paths
- We can nest modules inside other modules

  ```
  backyard
  ├── Cargo.lock
  ├── Cargo.toml
  └── src
      ├── garden
      │   └── vegetables.rs
      ├── garden.rs
      └── main.rs
  ```

  - The logic of the previous module is:

    - `main.rs` is the root module
    - `garden.rs` is a module nested inside the root module
    - `vegetables.rs` is a module nested inside the `garden` module

      ```Rust
      // main.rs
      use crate::garden::vegetables::Spinach; // The use keyword brings a module into scope

      pub mod garden; // The pub keyword makes the module public

      fn main() {
        let meal = Spinach {
            name: String::from("Spinach"),
            portion: 100,
        };
        println!("I'd like to eat {}", meal.name());
      }
      ```

      - If you're using cargo to build the project, you might need to use the name of the package as the root module instead of `crate`:

        ```Rust
        use my_package::garden::vegetables::Spinach;
        ```

      - You can find the name of the package in the `Cargo.toml` file

        ```toml
        [package]
        name = "my_package"
        #...
        ```

- Modules can be public or private, and the default is private

  - Public modules can be accessed from other modules `pub mod garden;`
  - Private modules can only be accessed from their parent module `mod garden;`

- If we need to access an element from a parent module, we can use the `super` keyword

  ```Rust
  // lib.rs
  fn some_func() {}

  mod some_module {
      fn do_something() {
          wait();
          super::some_func();
      }

      fn wait() {}
  }
  ```

- Public and private items inside modules can be of any type: structs, enums, functions, etc.

  - A public struct can have private and public fields as well
  - Enums that are public have all their variants public automatically

- The `use` keyword brings a module into scope

  - We can use the `as` keyword to rename the module
  - We can use the `self` keyword to refer to the current module

    ```Rust
    use self::garden::vegetables::Spinach;

    use crate::garden::vegetables::Spinach;

    use std::collections::HashMap as HMap;

    // When we have equal names, we can use the full path to avoid conflicts
    use std::fmt;
    use std::io;

    fn function1() -> fmt::Result {
        // --snip--
    }

    fn function2() -> io::Result<()> {
        // --snip--
    }

    // Or rename the modules
    use std::fmt::Result;
    use std::io::Result as IoResult;

    fn function1() -> Result {
        --snip--
    }

    fn function2() -> IoResult<()> {
        --snip--
    }
    ```

    - Alias are by default private, but we can make them public by adding the `pub` keyword and use them in other modules

      ```Rust
      pub use std::fmt;
      ```

- External packages are added to the `Cargo.toml` file

  ```toml
  [dependencies]
  rand = "0.8.3"
  ```

  - And we'll be able to use them in our code

    ```Rust
    extern crate rand;

    use rand::Rng;

    fn main() {
        let secret_number = rand::thread_rng().gen_range(1..101);
        println!("The secret number is: {}", secret_number);
    }
    ```

  - There are a lot of external packages available in the [crates.io](https://crates.io/)

- If we need to group imports we can use
  ```Rust
  use std::{cmp::Ordering, io};
  ```
  - This is useful when we have a lot of imports from the same module
- If we need to import all the public items from a module we can use the `*` character, known as glob operator
  ```Rust
  use std::collections::*;
  ```
  - This is not recommended because it can make the code harder to read and understand
- When the project grows, we'll need to split modules in separate files
  - Each module should have its own file with the same name as the module `vegetables.rs`
- Be careful with the path style you use, in the past Rust supports the following styles:
  ```
  src/my_module.rs (what we covered)
  src/my_module/mod.rs (older style, still supported path)
  ```
  - You can use either style, but if you mix them in the same module, you'll get an error, nevertheless you can use both styles in different modules, which is not recommended
