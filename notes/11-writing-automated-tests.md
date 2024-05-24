# Writing Automated Tests

- Testing in Rust follows the same principles as testing in other languages.
  - Arrange: Set up the test environment.
  - Act: Perform the test.
  - Assert: Check the results.
- Test in rust have the following attributes:
  - `#[cfg(test)]` attribute to indicate that the module contains tests.
  - `#[test]` attribute to indicate that a function is a test function.
  - `assert!` macro to check the results (there are other macros like `assert_eq!`)
- To run the tests, use the `cargo test` command.

  - You can also run a single or multiple test by using the `cargo test <name>` flag.

    ```sh
    cargo test add
    ```

    - This will run all tests that start with the word `add`.

  - Be careful when running test that use threads. The `cargo test` command will run tests in parallel by default. This can cause issues when tests are not isolated from each other. To run tests sequentially, use the `-- --test-threads=1` flag.

    ```sh
    cargo test -- --test-threads=1
    ```

  - If we need to show the output of a function in a test we can use the show output flag.

    ```sh
    cargo test -- --show-output
    ```

  - Check `cargo test --help` for more options.

- Here is an example of a test function:

  ```rust
  #[cfg(test)]
  mod tests {
      #[test]
      fn test_addition() {
          assert!(2 + 2 == 4);
      }
  }
  ```

  - Result:

    ```
    running 1 test
    test tests::it_works ... ok

    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

        Running unittests src/main.rs (target/debug/deps/e_13_tests-5f3be77a3ee3d518)

    running 0 tests

    test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

    Doc-tests e_13_tests

    running 0 tests

    test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    ```

- Failings tests will show an error message.

  ```
  running 2 tests
  test tests::it_works ... ok
  test tests::it_fails ... FAILED

  failures:

  ---- tests::it_fails stdout ----
  thread 'tests::it_fails' panicked at src/lib.rs:12:9:
  assertion `left == right` failed
  left: 4
  right: 5
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


  failures:
      tests::it_fails

  test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

  error: test failed, to rerun pass `--lib`
  ```

- If you need to add custom failure messages, you can use the third argument of the macro.

  ```rust
  #[cfg(test)]
  mod tests {
      #[test]
      fn test_addition() {
          assert!(2 + 2 == 5, "2 + 2 != 5");
      }
  }
  ```

  - Result:

    ```
    running 1 test
    test tests::it_works ... FAILED

    failures:

    ---- tests::it_works stdout ----
    thread 'tests::it_works' panicked at 2 + 2 != 5: 2 + 2 != 5, src/lib.rs:12:9
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    ```

- There will be situations when we need the test to panic. In those cases, we can use the `#[should_panic]` attribute to handle the panic.

  ```rust
  #[cfg(test)]
  mod tests {
      #[test]
      #[should_panic]
      fn test_panic() {
          panic!("This test will panic");
      }
  }
  ```

  - If we need a custom message, we can use the `expected` argument.

    ```rust
    #[cfg(test)]
    mod tests {
        #[test]
        #[should_panic(expected = "This test will panic")]
        fn test_panic() {
            panic!("This test will panic");
        }
    }
    ```

    - This is especially useful when there are several different panic scenarios.

- We can use Result<T, E> to test functions that return a Result.

  ```rust
  #[cfg(test)]
  mod tests {
      fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
          if b == 0 {
              return Err("Division by zero");
          }
          Ok(a / b)
      }

      #[test]
      fn test_divide() {
          assert_eq!(divide(10, 2), Ok(5));
          assert_eq!(divide(10, 0), Err("Division by zero"));
      }
  }
  ```

- You can ignore tests by using the `#[ignore]` attribute.

  ```rust
  #[cfg(test)]
  mod tests {
      #[test]
      #[ignore]
      fn test_ignore() {
          assert!(2 + 2 == 4);
      }
  }
  ```

  - To run ignored tests, use the `--ignored` flag.

    ```sh
    cargo test -- --ignored
    ```

## Test Organization

- Tests can be organized in different ways. Here are some common patterns:

  - **Unit tests**: Test individual functions or methods, usually in the `lib.rs` or any crate, within their functions.

    - Unit tests are the most common type of tests. They are used to test individual functions or methods. Here is an example of a unit test:

    ```rust
    #[cfg(test)] // This attribute is used to indicate that the module contains tests, and it is only compiled when running tests.
    mod tests {
        fn add(a: i32, b: i32) -> i32 {
            a + b
        }

        #[test] // It will run the test when we run `cargo test`.
        fn test_add() {
            assert_eq!(add(2, 2), 4);
        }
    }
    ```

  - **Integration tests**: Test the interaction between multiple functions or modules, usually in the `tests` module

    - They're in the `tests` directory and are used to test the interaction between multiple functions or modules.
    - Each file in the `tests` directory is compiled as a separate crate.
      ```
      project
      ├── Cargo.toml
      ├── src
      │   ├── lib.rs
      │   └── main.rs
      └── tests
          ├── common
          │   └── mod.rs
          ├── test1.rs
          └── test2.rs
      ```
    - Here is an example of an integration test:

      ```rust
      use my_crate::add; // Import the function from the crate

      #[test]
      fn test_add() {
          assert_eq!(add(2, 2), 4);
      }
      ```

    - To run integration tests, use the `cargo test --test <intecration test>` command.

      ```sh
      cargo test --test test1
      ```

    - To keep the integration tests organized you can create a common module with helper functions that can be used in multiple tests.

      ```rust
      // tests/common/mod.rs file - Use it this way to prevent the compiler to test common.rs
      pub fn fixture() -> i32 {
          // Set up the test environment
      }
      ```

- Take into account that Rust allows you to test private functions
