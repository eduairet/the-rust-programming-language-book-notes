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
- Package
  - One or more crates that provide a set of functionality
  - Contains a `Cargo.toml` file that describes how to build those crates
  - Can contain zero or one library crates and as many binary crates as you’d like (but it must contain at least one crate [either library or binary])

## Defining Modules to Control Scope and Privacy

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

  - The logic of the modules is:

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
