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

- Another approach to ensure safe concurrency is to use message passing between threads. Rust provides channels for this purpose.
- A channel has two parts: a transmitter and a receiver. The transmitter sends messages, and the receiver receives them.
- A channel is closed when either the transmitter or the receiver is dropped.

  ```rust
  use std::sync::mpsc;
  use std::thread;

  fn main() {
      let (tx, rx) = mpsc::channel();

      thread::spawn(move || {
          let val = String::from("hi");
          tx.send(val).unwrap();
      });

      let received = rx.recv().unwrap(); // Returns a Result<T, E>
      // we can also use try_recv() to check if the channel has received a value or not
      // let received = rx.try_recv().unwrap();
      // this is useful when the thread is still running and we want to check if the channel has received a value or not
      println!("Got: {}", received);
  }
  ```

## Using Message Passing to Transfer Data Between Threads

- We can also clone the transmitter to send messages from multiple threads.

  ```rust
  use std::sync::mpsc;
  use std::thread;
  fn main() {
      let (tx, rx) = mpsc::channel();
      let tx1 = mpsc::Sender::clone(&tx);
      thread::spawn(move || {
          let val = String::from("hi");
          tx.send(val).unwrap();
      });
      thread::spawn(move || {
          let val = String::from("hello");
          tx1.send(val).unwrap();
      });
      let received = rx.recv().unwrap();
      let received1 = rx.recv().unwrap();
      println!("Got: {}", received);
      println!("Got: {}", received1);
  }
  ```

## Shared State Concurrency

- Sharing memory between threads is like sharing ownership, it needs to be cleaned up properly.
- In Rust we can use `Mutex` (mutual exclusions) to ensure that only one thread accesses the data at a time.
- Mutual exclusions are hard to use correctly because we might forget to unlock the mutex, which will cause a deadlock.

  ```rust
  use std::sync::Mutex;

  fn main() {
      let m = Mutex::new(5);

      {
          let mut num = m.lock().unwrap();
          *num = 6; // Mutex is a smart pointer, so we need to dereference it
      }

      println!("m = {:?}", m);
  }
  ```

- When trying to use multiple threads we need to use `Arc` (atomic reference counting) to share ownership between threads.

  ```rust
  use std::sync::{Arc, Mutex}; // Arc is a thread-safe reference counting pointer, it's like Rc but for threads
  use std::thread;

  fn main() {
      let counter = Arc::new(Mutex::new(0));
      let mut handles = vec![];

      for _ in 0..10 {
          let counter = Arc::clone(&counter);
          let handle = thread::spawn(move || {
              let mut num = counter.lock().unwrap();

              *num += 1;
          });
          handles.push(handle);
      }

      for handle in handles {
          handle.join().unwrap();
      }

      println!("Result: {}", *counter.lock().unwrap());
  }
  ```

- Check the atomic module in the standard library for more atomic types.

## Extensible Concurrency with the `Sync` and `Send` Traits

- We can transfer ownership between threads using the `Send` trait.
- We can share ownership between threads using the `Sync` trait.
- Rust's standard library provides `Send` and `Sync` implementations for most types
- Implementing `Send` and `Sync` manually is generally unsafe.

## References

- [The Rustonomicon](https://doc.rust-lang.org/stable/nomicon)
