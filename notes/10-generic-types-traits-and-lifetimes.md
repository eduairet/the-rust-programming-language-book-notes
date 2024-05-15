# Generic Types, Traits, and Lifetimes

- Generics help you to abstract types and reduce duplication.
- This code reduces code duplication by abstracting the logic without generics yet:

  ```rust
  fn largest<T>(list: &[T]) -> T {
      let mut largest = list[0];

      for &item in list.iter() {
          if item > largest {
              largest = item;
          }
      }

      largest
  }

  fn main() {
      let number_list = vec![34, 50, 25, 100, 65];

      let result = largest(&number_list);
      println!("The largest number is {}", result);

      let char_list = vec!['y', 'm', 'a', 'q'];

      let result = largest(&char_list);
      println!("The largest char is {}", result);
  }
  ```

- Generic data types are defined by angle brackets `<>` and the generic type name `T` (this is a convention, you can use any name)

  - Functions

    ```rust
    fn largest<T>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }
    ```

  - Structs

    ```rust
    struct Point<T> {
        x: T,
        y: T,
    }
    // let integer = Point { x: 5, y: 10 };
    // let float = Point { x: 1.0, y: 4.0 };
    // let wont_work = Point { x: 5, y: 4.0 };
    // x and y values must be of the same type

    struct Point<T, U> {
        x: T,
        y: U,
    }

    // let integer_and_float = Point { x: 5, y: 4.0 };
    // x and y values can be of different types now
    ```

- Enums

  ```rust
  enum Result<T, E> {
      Ok(T),
      Err(E),
  }

  // let integer = Result::Ok(5);
  // let float = Result::Ok(4.0);
  // let error = Result::Err("Error");
  ```

- Method definitions in structs can also use generics

  ```rust
  struct Point<T> {
      x: T,
      y: T,
  }

  impl<T> Point<T> { // We need to declare the generic type before the impl keyword to use it in the struct
      fn x(&self) -> &T {
          &self.x
      }
  }

  let p = Point { x: 5, y: 10 };
  println!("p.x = {}", p.x()); // p.x = 5
  ```

  - Generic types in the implementation can vary from the struct's generic type

    ```rust
    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y); // p3.x = 5, p3.y = c
    ```

- Generics don't affect performance because Rust implements generics through monomorphization, which is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

## Traits

- Traits define shared behavior in an abstract way.
- Traits are similar to interfaces in other languages.
- Traits are defined using the `trait` keyword.

  ```rust
  pub trait Summary {
      fn summarize(&self) -> String;
      // Here we define a method called summarize that returns a String
  }

  pub struct NewsArticle {
      pub headline: String,
      pub location: String,
      pub author: String,
      pub content: String,
  }

  impl Summary for NewsArticle {
      fn summarize(&self) -> String {
          // We implement the summarize method for the NewsArticle struct
          format!("{}, by {} ({})", self.headline, self.author, self.location)
      }
  }

  pub struct Tweet {
      pub username: String,
      pub content: String,
      pub reply: bool,
      pub retweet: bool,
  }

  impl Summary for Tweet {
      fn summarize(&self) -> String {
          // We implement the summarize method for the Tweet struct
          format!("{}: {}", self.username, self.content)
      }
  }

  pub fn notify(item: &impl Summary) {
      println!("Breaking news! {}", item.summarize());
  }

  let tweet = Tweet {
      username: String::from("horse_ebooks"),
      content: String::from("of course, as you probably already know, people"),
      reply: false,
      retweet: false,
  };

  notify(&tweet);
  ```

- Using a trait needs to be explicitly bring it into scope if it's not.

  ```rust
  use aggregator::{Summary, Tweet};

  fn main() {
      let tweet = Tweet {
          username: String::from("horse_ebooks"),
          content: String::from("of course, as you probably already know, people"),
          reply: false,
          retweet: false,
      };

      println!("1 new tweet: {}", tweet.summarize());
  }
  ```

  - We can only implement a trait on a type if either the trait or the type is local to our crate.

- Default implementations can be provided for traits.

  ```rust
  pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
  }

  impl Summary for NewsArticle {}
  ```

- Traits can call other methods in the same trait.

  ```rust
  pub trait Summary {
      fn summarize_author(&self) -> String;

      fn summarize(&self) -> String {
          format!("(Read more from {}...)", self.summarize_author())
      }
  }

  // We need to implement the summarize_author method for the NewsArticle struct
  // to be able to use the summarize method
  impl Summary for Tweet {
      fn summarize_author(&self) -> String {
          format!("@{}", self.username)
      }
  }
  ```

- Traits can be used as parameters.

  ```rust
  pub fn notify(item: &impl Summary) {
      println!("Breaking news! {}", item.summarize());
  }
  ```
