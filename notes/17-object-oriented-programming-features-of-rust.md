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