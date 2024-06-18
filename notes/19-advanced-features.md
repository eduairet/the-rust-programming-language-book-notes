# Advanced Features

## Unsafe Rust

- Unsafe Rust is a set of features that are not covered by the safety guarantees of the compiler.
- Unsafe Rust is not necessarily dangerous, but it allows you to do things that are not possible in safe Rust.
- Hardware is inherently unsafe, and unsafe Rust allows you to interact with hardware in a safe way.
- Unsafe Rust is used to:

  - Dereference raw pointers.

    - Raw pointers are a type of pointer that do not have the usual guarantees that references have.
    - Raw pointers can:

      - Be mutable or immutable, written as `*mut T` and `*const T` respectively, letting them ignore the borrowing rules
      - Be null.
      - Not implement any automatic cleanup.
      - Not have any guarantees about what they are pointing to.

      ```rust
      fn main() {
          let mut num = 5;

          // We can create raw pointers in safe code using as
          let r1 = &num as *const i32;
          let r2 = &mut num as *mut i32;

          unsafe {
              // We can only dereference raw pointers in unsafe code
              println!("r1 is: {}", *r1);
              println!("r2 is: {}", *r2);
          }

          unsafe {
              *r2 = 10;
          }

          println!("num is: {}", num);

          // We can create raw pointers from arbitrary memory addresses in safe code using as
          let address = 0x012345usize;
          let r = address as *const i32; // Usually this is a bad idea
      }
      ```

  - Call unsafe functions or methods.

    - Unsafe functions and methods are a way to tell the compiler that you, the programmer, have taken care of upholding the invariants that the compiler checks for.

      ```rust
      unsafe fn dangerous() {}

      unsafe {
          dangerous();
      }
      ```

    - We can call outside functions that are written in other languages, such as C.

      ```rust
      extern "C" {
          fn abs(input: i32) -> i32;
      }

      fn main() {
          unsafe {
              println!("Absolute value of -3 according to C: {}", abs(-3));
          }
      }
      ```

    - Or prepare a rust function to be called from another language.

      ```rust
      #[no_mangle]
      pub extern "C" fn call_from_c() {
          println!("Just called a Rust function from C!");
      }
      ```

  - Access or modify a mutable static variable.

    - Mutable static variables are similar to constants, but they are mutable.

      ```rust
      static mut COUNTER: u32 = 0;

      fn add_to_count(inc: u32) {
          unsafe {
              COUNTER += inc;
          }
      }

      fn main() {
          add_to_count(3);

          unsafe {
              println!("COUNTER: {}", COUNTER); // Better to use with concurrency techniques
          }
      }
      ```

  - Implement unsafe traits.

    - A trait is unsafe when at least one of its methods has some invariant that the compiler can't verify.

      ```rust
      unsafe trait Foo {
          // methods go here
      }

      unsafe impl Foo for i32 {
          // method implementations go here
      }
      ```

  - Access fields of unions.

    - Unions are similar to structs, but they can only store one of their fields at a time.

      ```rust
      union IntOrFloat {
          i: i32,
          f: f32,
      }

      fn main() {
          let mut iof = IntOrFloat { i: 123 };

          unsafe {
              println!("iof.i: {}", iof.i);
          }

          iof.i = 234;

          unsafe {
              println!("iof.i: {}", iof.i);
          }
      }
      ```

      - Unions are useful when you need to store a value that could be one of several types, but you don't know which one until runtime.

- It is important to keep unsafe code as small as possible and to encapsulate it in safe abstractions.

  ```rust
  use std::slice;

  fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
      let len = values.len();
      let ptr = values.as_mut_ptr();

      assert!(mid <= len);

      unsafe {
          (
              slice::from_raw_parts_mut(ptr, mid),
              slice::from_raw_parts_mut(ptr.add(mid), len - mid),
          )
      }
  }
  ```

## Advanced Traits

- Associate types are a way to define a type placeholder inside a trait definition to be specified later.

  ```rust
  // Associated types
  pub trait Iterator {
      type Item;

      fn next(&mut self) -> Option<Self::Item>;
  } // Connection between the type Item and the type that the next method returns
  ```

- When using default generic type parameters we can use operation overloading to define the behavior of the trait.

  ```rust
  use std::ops::Add;

  #[derive(Debug, Copy, Clone, PartialEq)]
  struct Point {
      x: i32,
      y: i32,
  }

  impl Add for Point {
      type Output = Point;

      fn add(self, other: Point) -> Point {
          Point {
              x: self.x + other.x,
              y: self.y + other.y,
          }
      }
  }

  fn main() {
      assert_eq!(
          Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
          Point { x: 3, y: 3 }
      );
  }
  ```

  - Where the generic type trait looks like this

    ```rust
    trait Add<Rhs=Self> {
        type Output;

        fn add(self, rhs: Rhs) -> Self::Output;
        // Rhs comes from Right Hand Side and is the default type for the trait
    }
    ```

- Fully qualified syntax is used when we want to call a method with the same name from different traits.

  ```rust
  trait Pilot {
      fn fly(&self);
  }

  trait Wizard {
      fn fly(&self);
  }

  struct Human;

  impl Pilot for Human {
      fn fly(&self) {
          println!("This is your captain speaking.");
      }
  }

  impl Wizard for Human {
      fn fly(&self) {
          println!("Up!");
      }
  }

  impl Human {
      fn fly(&self) {
          println!("*waving arms furiously*");
      }
  }

  fn main() {
      let person = Human;
      Pilot::fly(&person); // This is your captain speaking.
      Wizard::fly(&person); // Up!
      person.fly(); // *waving arms furiously*
  }
  ```

- Supertraits are used to define a trait that inherits from another trait.

  ```rust
  use std::fmt;

  trait OutlinePrint: fmt::Display {
      fn outline_print(&self) {
          let output = self.to_string();
          let len = output.len();
          println!("{}", "*".repeat(len + 4));
          println!("*{}*", " ".repeat(len + 2));
          println!("* {} *", output);
          println!("*{}*", " ".repeat(len + 2));
          println!("{}", "*".repeat(len + 4));
      }
  }

  struct Point {
      x: i32,
      y: i32,
  }

  impl OutlinePrint for Point {}

  impl fmt::Display for Point {
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
          write!(f, "({}, {})", self.x, self.y)
      }
  }
  ```

- The new type pattern is used to create a new type that is distinct from its original type.

  ```rust
  use std::fmt;

  struct Wrapper(Vec<String>);

  impl fmt::Display for Wrapper {
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
          write!(f, "[{}]", self.0.join(", ")) // self.0 is the inner value of the Wrapper
      }
  }

  fn main() {
      let w = Wrapper(vec![String::from("hello"), String::from("world")]);
      println!("w = {}", w);
  }
  ```

## Advanced Types
