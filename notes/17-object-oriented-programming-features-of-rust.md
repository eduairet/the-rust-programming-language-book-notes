# Object Oriented Programming Features of Rust

- Rust provide Object Oriented Programming features like inheritance, polymorphism, encapsulation, etc.

  - Encapsulation: Rust provides encapsulation by using `struct` and `enum` to group data and functions that operate on that data.

    ```rust
    pub struct AverageCollection {
        // These are encapsulated properties since they're private
        list: Vec<i32>,
        average: f64,
    }

    // The methods provide an API to interact with the encapsulated data
    impl AveragedCollection {
        pub fn add(&mut self, value: i32) {
            self.list.push(value);
            self.update_average();
        }

        pub fn remove(&mut self) -> Option<i32> {
            let result = self.list.pop();
            match result {
                Some(value) => {
                    self.update_average();
                    Some(value)
                }
                None => None,
            }
        }

        pub fn average(&self) -> f64 {
            self.average
        }

        fn update_average(&mut self) {
            let total: i32 = self.list.iter().sum();
            self.average = total as f64 / self.list.len() as f64;
        }
    }
    ```

- Inheritance: Rust doesn't have inheritance like other OOP languages. Instead, it uses traits to define shared behavior between types.

  ```rust
  pub trait Draw {
      fn draw(&self);
  }

  pub struct Screen {
      pub components: Vec<Box<dyn Draw>>,
  }

  impl Screen {
      pub fn run(&self) {
          for component in self.components.iter() {
              component.draw();
          }
      }
  }
  ```

- Polymorphism: Rust uses traits to define shared behavior between types. This allows for polymorphism by defining a trait that can be implemented by different types.

  ```rust
  // -- snip --
  struct SelectBox {
      width: u32,
      height: u32,
      options: Vec<String>,
  }
  // Use the `Draw` trait to define the behavior of the `SelectBox` type
  impl Draw for SelectBox {
      fn draw(&self) {
          println!("Drawing SelectBox");
      }
  }
  ```

- Polymorphism can be achieved using dynamic dispatch or static dispatch.
  - When using dynamic dispatch, Rust uses trait objects to enable polymorphism. Trait objects are created by boxing a type that implements a trait. This allows for different types to be stored in a single data structure.
  - Static dispatch is used when the concrete type is known at compile time. This allows the compiler to optimize the code by inlining the method calls.

## Implementing an Object-Oriented Design Pattern

- States are represented by state objects that implement a trait.

  ```rust
  pub trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
  }

  pub struct PendingReview {}

  impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
      self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
      Box::new(Approved {})
    }
  }

  pub struct Approved {}

  impl State for Approved {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
      self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
      self
    }
  }
  ```

- The state pattern might have some trade-offs, like the complexity of managing the state transitions and the boilerplate code required to implement the pattern.
- Sometimes it would be better to encode States and Behaviors as types.
