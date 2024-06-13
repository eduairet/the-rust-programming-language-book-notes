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
