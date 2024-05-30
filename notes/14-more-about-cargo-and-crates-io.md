# More about Cargo and Crates.io

## Customizing Builds with Release Profiles

- The dev profile is used when you run `cargo build`.
- The release profile is used when you run `cargo build --release`.
- You can customize them in the `Cargo.toml` file.

  ```toml
  [profile.dev]
  opt-level = 0

  [profile.release]
  opt-level = 3
  ```

  - All the options you put in the `Cargo.toml` file will override the default values.

## Publishing a Crate to Crates.io

- You can publish a crate to [crates.io](https://crates.io/).
- Code should be open source.
- In order to make your crate useful to others, you should write documentation.
  - Documentation comments are written with `///`.
  - Here is an example with the most common sections added to document a function (examples, panics, errors, safety)
    ````rust
    /// Adds one to the number given.
    ///
    /// # Examples
    ///
    /// ```
    /// let arg = 5;
    /// let answer = my_crate::add_one(arg);
    ///
    /// assert_eq!(6, answer);
    /// ```
    ///
    /// # Panics
    ///
    /// This function will panic if the input is `std::i32::MAX`.
    ///
    /// # Errors
    ///
    /// This function will return an error if the input is `std::i32::MAX`.
    ///
    /// # Safety
    ///
    /// This function is not safe because it uses `unsafe` code.
    ````
- A good practice is to add tests as a matter of comments to show how the function works.
- Contained items are documented with `//!`.
  ```rust
  //! # My Crate
  //!
  //! `my_crate` is a collection of utilities to make performing certain calculations more convenient.
  //!
  ```
- When building an API it's common to bring types, functions, etc. to the top level of the crate, this way they'll appear in the documentation and they will be easier to find and use

  ```rust
  //! # Art
  //!
  //! A library for modeling artistic concepts.
  //!
  //! pub use self::kinds::PrimaryColor;
  //! pub use self::kinds::SecondaryColor;
  //! pub use self::utils::mix;
  //!
  //! pub mod kinds {
  //!    // --snip--
  //! }
  //!
  //! pub mod utils {
  //!   // --snip--
  //! }
  ```

- After you have written the documentation, you can publish the crate with `cargo publish`.

  - Create an account on crates.io.
    - Run `cargo login <API Key from crates.io>`
    - Run `cargo publish`
    - You can update the version of the crate in the `Cargo.toml` file
    - You can also add a description, license, key words, and other information to the `Cargo.toml` file

- Example of a `Cargo.toml` file with all the information added

  ```toml
  [package]
  name = "guessing_game"
  license = "MIT"
  version = "0.1.0"
  edition = "2021"
  authors = ["Your Name"]
  description = "A fun game where you guess what number the computer has chosen."
  license = "MIT OR Apache-2.0"
  keywords = ["game", "guessing"]
  ```

- Publishing a crate will make it available to everyone in the world but it will be permanently available and you can't delete it.
  - Change the version number if you want to update the crate, use semantic versioning.
- If an old version becomes obsolete, you can yank it with `cargo yank --vers 1.0.0`.
  - This can be undone with `cargo yank --vers 1.0.0 --undo`.

## Cargo Workspaces

- A workspace is a set of packages that are grouped together in a single directory.
- You can create a workspace by creating a directory with a `Cargo.toml` file that contains a `[workspace]` section.

  ```toml
  [workspace]
  members = [
      "adder",
  ]
  ```

  ```
  ├── Cargo.lock
  ├── Cargo.toml
  ├── adder
  │   ├── Cargo.toml
  │   └── src
  │       └── main.rs
  └── target
  ```

- Check [e_16_workspace](../projects/e_16_workspaces/) for an example of a workspace.
- Build all the packages from the root of the workspace with `cargo build`.
- To run a specific package in a workspace, you can use `cargo run -p <Workspace>`.
- The same logic applies to tests, you can run all the tests with `cargo test` or run a specific package with `cargo test -p <Workspace>`.
- When adding a dependency to a workspace, the dependency will be bound to all the packages in the workspace if it's added to their `Cargo.toml` files, but it won't be available to all the packages in the project.
- You can add a dependency to all the packages in the workspace by adding it to the root `Cargo.toml` file.

  ```toml
  [dependencies]
  rand = "0.8.4"
  ```

  - Or with `cargo add rand --workspace`.

## Notes

- Check all the profile options in the [Cargo documentation](https://doc.rust-lang.org/cargo/reference/profiles.html).
