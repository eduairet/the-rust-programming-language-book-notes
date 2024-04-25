# Using Structs to Structure Related Data

- Strucs are custom data types that let you group multiple related values.
- Structs are similar to Classes in OOP languages.
- Defining a struct:
  ```rust
  struct User {
      username: String,
      email: String,
      sign_in_count: u64,
      active: bool,
  }
  ```
- Instantiating a struct:

  ```rust
  // I can be mut or not, depending on whether I want to change the struct fields
  let mut user1 = User {
      username: String::from("username"),
      email: String::from("mail@mail.mail"),
      active: true, // It's not necessary to specify all fields or use the same order
      sign_in_count: 1,
    };

  // Updating a struct field
  user1.email = String::from("changed@test.com");

  // Accessing fields
  println!(
    "User1: {}, {}, {}, {}",
    user1.username, user1.email, user1.active, user1.sign_in_count
  );
  ```

- We can create a helper function to create instances of a struct:

  ```Rust
  fn build_user(email: String, username: String) -> User {
      User {
          email, // This is a shorthand for email: email
          username, // This is a shorthand for username: username
          active: true,
          sign_in_count: 1,
      }
  }

  let user2 = build_user(String::from("
  ```

- Sometimes is useful to instance structs from other instance:

  ```Rust
  let user3 = User {
    username: String::from("new-user"),
    email: String::from("new@test.com"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
  };

  // Or using the struct update syntax
  let user4 = User {
    email: String::from("new02@test.com"),
    ..user1
  };
  ```

  - Be careful with this because it moves the ownership of heap stored values inside the struct like username, which is moved from `user1` to `user4`.
  - Using borrowed values like `&str` instead of `String` needs the use of lifetimes which will be explained later.

- Tuple structs are structs without named fields:

  ```Rust
  struct Color(i32, i32, i32);
  let black = Color(0, 0, 0);
  ```

- Unit-like structs are structs without any fields:

  ```Rust
  struct UnitLikeStruct;
  ```

  - They are useful when you need to implement a trait on a type but don't have any data to store in the type itself.

- Another important thing about structs is that they don't have the `Display` trait implemented by default, so we can't print them directly. We need to implement the `Debug` trait to be able to print them:

  ```Rust
  #[derive(Debug)]
  struct Rectangle {
      width: u32,
      height: u32,
  }

  let rect1 = Rectangle { width: 30, height: 50 };
  println!("rect1 is {:?}", rect1);
  // Output: rect1 is Rectangle { width: 30, height: 50 }
  // We can use !dbg to print the struct with the Debug trait as well
  dbg!(&rect1);
  // Output: [src/main.rs:15] &rect1 = Rectangle { width: 30, height: 50 }
  ```

  - The `#[derive(Debug)]` annotation is a directive to the compiler to implement the `Debug` trait for the struct automatically.
  - The `{:?}` is a placeholder for the `Debug` trait.
  - For large structs, we can use `:#?` to pretty print the struct.
  - The main difference between `println!` and `dbg!` is that `dbg!` takes ownership of the value and `println!` which takes the reference.

## Methods

- Methods are similar to functions but are defined within the context of a struct.

  ```Rust
  fn main() {
      let rect1 = Rectangle { width: 30, height: 50 };
      println!("The area of the rectangle is {} square pixels.", rect1.area());
  }

  #[derive(Debug)]
  struct Rectangle {
      width: u32,
      height: u32,
  }

  impl Rectangle {
      fn area(&self) -> u32 {
          self.width * self.height
      }
  }
  ```

- Methods are defined within an `impl` block.
- Methods can take ownership of `self`, borrow `self` immutably, or borrow `self` mutably, just like functions take ownership, borrow, or mutable borrow of their arguments.
- Rust has no equivalent to `->` like in other languages to access fields of a struct. Instead, Rust has a feature called `Deref` which allows us to overload the `*` operator to access fields of a struct.
- We can add several methods to the same `impl` block or even multiple `impl` blocks for the same struct.

  ```Rust
  impl Rectangle {
      fn area(&self) -> u32 {
          self.width * self.height
      }

      fn can_hold(&self, other: &Rectangle) -> bool {
          self.width > other.width && self.height > other.height
      }
  }

  impl Rectangle {
      fn square(size: u32) -> Rectangle {
          Rectangle { width: size, height: size }
      }
  }
  ```

## Associated Functions

- Associated functions are functions that are associated with a struct but don't take `self` as a parameter.

  ```Rust
  impl Rectangle {
      fn square(size: u32) -> Rectangle {
          Self { width: size, height: size }
      }
  }
  ```

- Associated functions are often used for constructors that will return a new instance of the struct.
