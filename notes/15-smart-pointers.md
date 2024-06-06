# Smart Pointers

- A pointer is an address in memory
- A smart pointer is a data structure that acts like a pointer but also has additional metadata and capabilities
- Smart pointers:
  - Own the data they're referring to
  - They're usually implemented using structs
  - Implement the `Deref` and `Drop` traits
    - `Deref`: allows an instance of the smart pointer struct to behave like a reference
      - This means you can use the `*<Some struct with the defer trait>` operator to dereference a smart pointer instance
        - It's the equivalent to `*(<Some struct with the defer trait>.deref())`
      - Some types are implicit inferred by the compiler to be dereferenced
        - For example, when you use the `*` operator on a string literal, the compiler automatically dereferences it to a `&str`
      - The mutable dereference operator `*` is also implemented by the `DerefMut` trait
    - `Drop`: allows you to customize the code that is run when an instance of the smart pointer goes out of scope
      - This is useful for releasing resources like file handles or network connections
      - The `drop` method is called automatically when the instance goes out of scope but it can also be called with the help of `std::mem::drop` when you need to release the resources earlier
      - Trying to drop manually will result in a double free error

## Using Box<T> to Point to Data on the Heap

- `Box<T>` allows you to store data on the heap rather than the stack
- The data is cleaned up when the `Box<T>` goes out of scope
- The pointer itself is stored on the stack
- Useful when:
  - You have a type whose size can't be known at compile time
  - You have a large amount of data and you want to transfer ownership but ensure the data won't be copied when you do so
  - You want to own a value and you care only that it's a type that implements a particular trait rather than any specific type

## Rc<T>, the Reference Counted Smart Pointer

- `Rc<T>` allows you to have multiple owners of the same data
- It keeps track of the number of references to the data and when the count goes to zero, the data is cleaned up
- It's just appropriate for single-threaded scenarios
- When using `Rc<T>`, you need to use the `clone` method to increment the reference count
  - The `clone` method doesn't perform a deep copy like `clone` does for `String` or `Vec<T>`
- You can track the number of references to the data by calling the `strong_count` method

  ```rust
  use std::rc::Rc;

  let rc = Rc::new(5);
  let rc2 = Rc::clone(&rc);
  println!("rc = {}, rc2 = {}", Rc::strong_count(&rc), Rc::strong_count(&rc2)); // rc = 2, rc2 = 2
  ```

- Mutability in `Rc<T>`:

  - You can't have multiple mutable references to the same data
  - If you need to mutate the data, you can use `RefCell<T>` to achieve interior mutability

    - `RefCell<T>` allows you to mutate the data even when there are immutable references to it
    - It enforces the borrowing rules at runtime rather than compile time
    - If you violate the borrowing rules, your program will panic

      ```rust
      use std::cell::RefCell;
      use std::rc::Rc;

      let value = Rc::new(RefCell::new(5));
      let value2 = Rc::clone(&value);

      *value.borrow_mut() += 10;
      *value2.borrow_mut() += 20;

      println!("value = {:?}", value.borrow()); // value = RefCell { value: 35 }
      ```

- Even though Rust doesn't have mock objects, you can use `Rc<T>` to simulate them

  ```rust
  pub trait Messenger {
      fn send(&self, msg: &str);
  }

  pub struct LimitTracker<'a, T: Messenger> {
      messenger: &'a T,
      value: usize,
      max: usize,
  }

  impl<'a, T> LimitTracker<'a, T>
  where
      T: Messenger,
  {
      pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
          LimitTracker {
              messenger,
              value: 0,
              max,
          }
      }
      pub fn set_value(&mut self, value: usize) {
          self.value = value;

          let percentage_of_max = self.value as f64 / self.max as f64;

          if percentage_of_max >= 1.0 {
              self.messenger.send("Error: You are over your quota!");
          } else if percentage_of_max >= 0.9 {
              self.messenger.send("Urgent: You're at 90% of your quota!");
          } else if percentage_of_max >= 0.75 {
              self.messenger.send("Warning: You're at 75% of your quota!");
          }
      }
  }

  #[cfg(test)]
  mod tests {
      use super::*;
      use std::cell::RefCell;

      struct MockMessenger {
          // Implementing a mock Messenger to test the LimitTracker
          sent_messages: RefCell<Vec<String>>,
      }

      impl MockMessenger {
          fn new() -> MockMessenger {
              MockMessenger {
                  // Creating a new MockMessenger
                  sent_messages: RefCell::new(vec![]),
              }
          }
      }

      impl Messenger for MockMessenger {
          fn send(&self, message: &str) {
              self.sent_messages.borrow_mut().push(String::from(message));
          }
      }

      #[test]
      fn it_sends_an_over_75_percent_warning_message() {
          let mock_messenger = MockMessenger::new();
          let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

          limit_tracker.set_value(80);
          assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
      }
  }
  ```

- By using `RefCell` we keep track of every `RefMut<T>` and `Ref<T>`

  - `RefMut<T>` and `Ref<T>` are safe methods provided by Rust to avoid issues that can jump in our project when using `&` and `&mut`, these methods are `borrow` and `borrow_mut`

- We can mix `Rc<T>` and `RefCell<T>` to have multiple owners and interior mutability

  ```rust
  use std::cell::RefCell;
  use std::rc::Rc;

  #[derive(Debug)]
  struct Node {
      value: i32,
      children: RefCell<Vec<Rc<Node>>>,
  }

  let leaf = Rc::new(Node {
      value: 3,
      children: RefCell::new(vec![]),
  });

  let branch = Rc::new(Node {
      value: 5,
      children: RefCell::new(vec![Rc::clone(&leaf)]),
  });

  *leaf.children.borrow_mut() = vec![Rc::clone(&branch)];

  println!("leaf = {:?}", leaf);
  ```

- Reference cycles can leak memory when using `Rc<T>` and `RefCell<T>`

  ```rust
  use crate::List::{Cons, Nil};
  use std::cell::RefCell;
  use std::rc::Rc;

  #[derive(Debug)]
  enum List {
      Cons(i32, RefCell<Rc<List>>),
      Nil,
  }

  impl List {
      fn tail(&self) -> Option<&RefCell<Rc<List>>> {
          match self {
              Cons(_, item) => Some(item),
              Nil => None,
          }
      }
  }

  fn main() {
      let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

      println!("a initial rc count = {}", Rc::strong_count(&a));
      println!("a next item = {:?}", a.tail());

      let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

      println!("a rc count after b creation = {}", Rc::strong_count(&a));
      println!("b initial rc count = {}", Rc::strong_count(&b));
      println!("b next item = {:?}", b.tail());

      if let Some(link) = a.tail() {
          *link.borrow_mut() = Rc::clone(&b);
      }

      println!("b rc count after changing a = {}", Rc::strong_count(&b));
      println!("a rc count after changing a = {}", Rc::strong_count(&a));

      // Uncomment the next line to see that we have a cycle;
      // it will overflow the stack.
      // println!("a next item = {:?}", a.tail());
  }
  ```

- We can use `Weak<T>` to avoid reference cycles

  - `Weak<T>` is a non-owning reference
  - It doesn't increment the reference count
  - It's useful when you need to have a reference to a `Rc<T>` but you don't want to have ownership
  - It doesn't need to be count 0 to be cleaned up
  - You can call the `upgrade` method to get a `Rc<T>` from a `Weak<T>`

    ```rust
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    ```
