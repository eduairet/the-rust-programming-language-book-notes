# Enums and Pattern Matching

- Enums allow you to define a type by enumerating its possible variants.

  ```Rust
  enum IpAddrKind {
      V4,
      V6,
  }
  ```

- We can create instances of the enum variants

  ```Rust
  let four = IpAddrKind::V4;
  let six = IpAddrKind::V6;
  ```

- We can use enums as parameter types

  ```Rust
  fn route(ip_kind: IpAddrKind) {}
  route(IpAddrKind::V4);
  route(IpAddrKind::V6);
  ```

- We can also use enums with structs

  ```Rust
  struct IpAddr {
      kind: IpAddrKind,
      address: String,
  }

  let home = IpAddr {
      kind: IpAddrKind::V4,
      address: String::from("127.0.0.1"),
  };

  let loopback = IpAddr {
      kind: IpAddrKind::V6,
      address: String::from("::1"),
  };
  ```

- Nevertheless, we can use enums with data

  ```Rust
  enum IpAddr {
      V4(String),
      V6(String),
  }

  let home = IpAddr::V4(String::from("127.0.0.1"));
  let loopback = IpAddr::V6(String::from("::1"));
  ```

- And we can get as specific as we want

  ```Rust
  enum IpAddr {
      V4(u8, u8, u8, u8),
      V6(String),
  }

  let home = IpAddr::V4(127, 0, 0, 1);
  let loopback = IpAddr::V6(String::from("::1"));
  ```

- And we can add any type of data to the variants, even other enums

  ```Rust
  enum Message {
      Quit,
      Move { x: i32, y: i32 },
      Write(String),
      ChangeColor(i32, i32, i32),
  }
  ```

- We can implement methods on enums

  ```Rust
  impl Message {
      fn call(&self) {
          // method body would be defined here
      }
  }

  let m = Message::Write(String::from("hello"));
  m.call();
  ```

## The `Option` Enum

- Used when a value could be something or nothing, like a nullable value
- The `Option` enum is defined as follows

  ```Rust
  enum Option<T> { // T is a generic type
      Some(T),
      None,
  }

  let some_number = Some(5);
  let some_string = Some("a string");
  let absent_number: Option<i32> = None;
  ```

## The `match` Control Flow Operator

- The `match` operator allows us to compare a value against a series of patterns and then execute code based on which pattern matches.
  ```Rust
  enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
  }
  fn value_in_cents(coin: Coin) -> u8 {
    match coin {
         Coin::Penny => 1,
         Coin::Nickel => 5,
         Coin::Dime => 10,
         Coin::Quarter => 25,
    }
  }
  ```
- We can use the `_` placeholder to match any value
  ```Rust
  fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        _ => 0,
    }
  }
  ```
- If the arms of the match are too complex, we can use curly braces

  ```Rust
  fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        _ => 0,
    }
  }
  ```

- We can use the `match` operator to bind values to variables

  ```Rust
  #[derive(Debug)]
  enum UsState {
      Alabama,
      Alaska,
      // --snip--
  }

  enum Coin {
      Penny,
      Nickel,
      Dime,
      Quarter(UsState),
  }

  fn value_in_cents(coin: Coin) -> u8 {
      match coin {
          Coin::Penny => 1,
          Coin::Nickel => 5,
          Coin::Dime => 10,
          Coin::Quarter(state) => {
              println!("State quarter from {:?}!", state);
              25
          },
      }
  }
  ```

- We can use `match` with `Option<T>` to handle `None` cases

  ```Rust
  fn plus_one(x: Option<i32>) -> Option<i32> {
      match x {
          None => None,
          Some(x) => Some(x + 1),
      }
  }

  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);
  ```

- Matches are exhaustive, so we need to handle all cases or we'll get a compile error

  ```Rust
  let some_u8_value = 0u8;
  match some_u8_value {
      1 => println!("one"),
      3 => println!("three"),
      5 => println!("five"),
      7 => println!("seven"),
      _ => (), // without this line we would get a compile error
  }
  ```

## The `if let` Control Flow Operator

- The `if let` operator is a shorter way to write `match` when we only care about one case

  ```Rust
  let config_max = Some(3u8);
  match config_max {
      Some(max) => println!("The maximum is configured to be {max}"),
      _ => (),
  }

  // is equivalent to
  let config_max = Some(3u8);
  if let Some(max) = config_max {
      println!("The maximum is configured to be {max}");
  }
  ```

- We can use `if let` with `else` if we want to handle the other cases

  ```Rust
  let mut count = 0;
  if let Coin::Quarter(state) = coin {
      println!("State quarter from {:?}!", state);
  } else {
      count += 1;
  }

  // is equivalent to
  let mut count = 0;
  match coin {
      Coin::Quarter(state) => println!("State quarter from {:?}!", state),
      _ => count += 1,
  }
  ```
