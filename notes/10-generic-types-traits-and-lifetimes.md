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

- Trait bound syntax can be used to specify that a generic type must implement a trait.

  ```rust
  pub fn notify<T: Summary>(item: &T) {
      println!("Breaking news! {}", item.summarize());
  }
  ```

  - This is a better approach for more complex scenarios.
  - For example if we want to use multiple traits

    ```rust
    // Without trait bound syntax
    pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
    // With trait bound syntax
    pub fn notify<T: Summary>(item1: &T, item2: &T) {}
    // Using the + syntax
    pub fn notify(item: &(impl Summary + Display)) {}
    pub fn notify<T: Summary + Display>(item: &T) {}
    ```

- If there are a lot of trait bounds, we can use the `where` clause.

  ```rust
  // Without where clause
  fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}

  // With where clause
  fn some_function<T, U>(t: T, u: U) -> i32
  where
      T: Display + Clone,
      U: Clone + Debug,
  {}
  ```

- We can return a type that implements a trait.

  ```rust
  fn returns_summarizable() -> impl Summary {
      Tweet {
          username: String::from("horse_ebooks"),
          content: String::from("of course, as you probably already know, people"),
          reply: false,
          retweet: false,
      }
  }
  ```

  - This is only possible if we return a single type that implements the trait, in this case `Tweet`.

- We can use trait bounds to conditionally implement methods.

  ```rust
  use std::fmt::Display;

  struct Pair<T> {
      x: T,
      y: T,
  }

  impl<T> Pair<T> {
      fn new(x: T, y: T) -> Self {
          Self { x, y }
      }
  }

  impl<T: Display + PartialOrd> Pair<T> {
      fn cmp_display(&self) {
          if self.x >= self.y {
              println!("The largest member is x = {}", self.x);
          } else {
              println!("The largest member is y = {}", self.y);
          }
      }
  }
  ```

- We can implement traits that implement other traits, these are called blanket implementations.

  ```rust
  impl<T: Display> ToString for T {
  // --snip--
  }
  ```

## Lifetimes

- Preventing dangling references.

  ```rust
  {
      let r;
      {
          let x = 5;
          r = &x;
      }
      println!("r: {}", r);
  }

  // This code will not compile because the reference r is assigned to a variable x that goes out of scope.
  // r = &x;
  //     ^^ borrowed value does not live long enough
  ```

- The borrow checker prevents this by checking that references are valid.
  ```
  fn main() {
      let r;                // ---------+-- 'a
                            //          |
      {                     //          |
          let x = 5;        // -+-- 'b  |
          r = &x;           //  |       |
      }                     // -+       |
                            //          |
      println!("r: {}", r); //          |
  }                         // ---------+
  ```
  - To fix this we can use lifetimes.
    ```
    fn main() {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+
    ```
- We can use generic lifetimes in functions to specify that the references must have the same lifetime.

  ```rust
  fn main() {
      let string1 = String::from("abcd");
      let string2 = "xyz";

      let result = longest(string1.as_str(), string2);
      println!("The longest string is {}", result);
  }

  fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
      // The lifetime 'a is the same for both x and y
      if x.len() > y.len() {
          x
      } else {
          y
      }
  }

  // The lifetime 'a is the same for both x and y
  ```

  - Lifetime annotations use a single quote `'` followed by a name, usually simple and short like generic types.

- We can also use lifetime annotations in structs.

  ```rust
  struct ImportantExcerpt<'a> {
      part: &'a str,
  }

  fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
  }
  // The lifetime 'a is the same for both the struct and the reference
  ```

- There are some situations where rust identifies the lifetimes for us.

  ```rust
  fn first_word(s: &str) -> &str {
      let bytes = s.as_bytes();

      for (i, &item) in bytes.iter().enumerate() {
          if item == b' ' {
              return &s[..i];
          }
      }

      &s[..]
  }
  ```

  - In this case, the lifetime of the return value is the same as the input parameter `s`.
  - Take into account that older versions of Rust required explicit lifetime annotations in this case.

    ```rust
    fn first_word<'a>(s: &'a str) -> &'a str {
        // The lifetime 'a is the same for both the input and the output
    }
    ```

- Lifetime elision rules are a set of rules that the compiler follows to determine lifetimes automatically.

  1. Each parameter that is a reference gets its own lifetime parameter.

  ```rust
  fn first_word(s: &str) -> &str {
      s
  }
  ```

  2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.

  ```rust
  fn first_word<'a>(s: &'a str) -> &'a str {
      s
  }
  ```

  3. If there are multiple input lifetime parameters we can assign the same lifetime to multiple output lifetime parameters.

  ```rust
  fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
      x
  }
  ```

- We can use life annotations in methods.

  ```rust
  impl<'a> ImportantExcerpt<'a> {
      fn level(&self) -> i32 {
          3
      }
  }

  // With the third lifetime elision rule, we can use the same lifetime for the return value
  impl<'a> ImportantExcerpt<'a> {
      fn announce_and_return_part(&self, announcement: &str) -> &str {
          println!("Attention please: {}", announcement);
          self.part
      }
  }
  ```

- There's a special lifetime `'static` that means the reference can live for the entire duration of the program.

  ```rust
  let s: &'static str = "I have a static lifetime.";
  ```

  - When the compiler warns about a static lifetime it might be better to use a generic lifetime instead.
