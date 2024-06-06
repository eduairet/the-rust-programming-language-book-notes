# Fearless Concurrency

- Thanks to Rust's ownership system and type system, Rust can solve most of the concurrency problems at compile time instead of runtime.

## Using Threads to Run Code Simultaneously

- Threads allow you to run multiple pieces of code at the same time.
- Some problems related to threads are:
  - Race conditions: two or more threads are trying to access the same data at the same time.
  - Deadlocks: two threads are waiting for each other to finish using a resource the other thread has.
  - Bugs that happen only in certain situations and are hard to reproduce and fix.
- Rust offers a 1:1 threading model, which means that each OS thread corresponds to one Rust thread.

  ```rust
  use std::thread;
  use std::time::Duration;

  fn main() {
      let handle = thread::spawn(|| {
          for i in 1..10 {
              println!("hi number {} from the spawned thread", i);
              thread::sleep(Duration::from_millis(1));
          }
      });

      for i in 1..5 {
          println!("hi number {} from the main thread", i);
          thread::sleep(Duration::from_millis(1));
      }

      handle.join().unwrap();

      /*
      Output:

      hi number 1 from the main thread
      hi number 1 from the spawned thread
      hi number 2 from the main thread
      hi number 2 from the spawned thread
      hi number 3 from the main thread
      hi number 3 from the spawned thread
      hi number 4 from the main thread
      hi number 4 from the spawned thread
      hi number 5 from the spawned thread
      hi number 6 from the spawned thread
      hi number 7 from the spawned thread
      hi number 8 from the spawned thread
      hi number 9 from the spawned thread

      Without the `join` method, we can't guarantee that the spawned thread will finish before the main thread or even finish at all.
      */

  }
  ```

- When using `spawn` we might opt to use `move` to force the closure to take ownership of the values it uses in the environment.

  ```rust
  use std::thread;

  fn main() {
      let v = vec![1, 2, 3];

      let handle = thread::spawn(move || {
          println!("Here's a vector: {:?}", v);
      });

      handle.join().unwrap();
  }
  ```
