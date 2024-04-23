# Understanding Ownership

- Ownership allows Rust to make memory safety guarantees without needing a garbage collector.
- If a program violates ownership rules, Rust will refuse to compile it.

## The Stack and the Heap

- The stack stores values in the order it gets them and removes them in the opposite order, last in, first out.
  - All data stored on the stack must have a known, fixed size at compile time.
  - Adding data to the stack is called pushing onto the stack.
- The heap stores data with unknown size at compile time or a size that might change.
  - When you put data on the heap
  - You request a certain amount of space
  - The operating system finds an empty spot in the heap, marks it as being in use, and returns a pointer (the address of that location)
  - This process is called allocating on the heap and is slower than storing data on the stack
  - Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there.
- Ownership keeps track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap.

## Ownership Rules

1. Each value in Rust has a variable that’s called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

## Variable Scope

- A variable is valid from the point at which it’s declared until the end of the current scope.
- The scope is the range within a program for which an item is valid.
- Strings:
  - We can work with string literals, but strings are more complex.
  - String literals are immutable and hardcoded into the program.
  - Strings are mutable and stored on the heap.
  - The `String` type is provided by the standard library.
  - The `String` type is growable, mutable, owned, UTF-8 encoded text.
  - The `String::from` function creates a new `String` from a string literal.
    ```Rust
    let mut s = String::from("hello");
    // String is the namespace, and we access its properties using ::<Property>
    s.push_str(", world!");
    println!("{}", s); // prints "hello, world!"
    ```

## Memory and Allocation

- String literals are known at compile time since they’re hardcoded into the program.
- String elements are intended to be changed, so they are allocated on the heap.
- On rust, memory is automatically returned once the variable goes out of scope, which is called `drop` and is different from other low-level languages like C or C++ where you have to manually allocate and deallocate memory.
- On values stored on the stack (like integers), the value is copied when assigned to another variable.
- On values stored on the heap (like strings), the pointer is copied, but the data is not, which is called a move, not a shallow copy like in other languages because the original variable is no longer valid.

  ```Rust
  let mut n1 = 5;
  let mut n2 = n1;
  n1 -= 1;
  n2 += 1;
  // n1 and n2 are integers, so they are stored on the stack and we can copy the values
  // this uses the copy trait
  println!("n1: {}, n2: {}", n1, n2);

  let mut s1 = String::from("hello");
  let s2 = s1;
  s1.push_str(", world!"); // Error: value borrowed here after move - we need to clone the value
  println!("{s1}\n{s2}");
  ```

  - The `s1` variable is moved to `s2`, so `s1` is no longer valid.
    - We can clone the value to keep the original variable valid.
    - The `clone` method is a deep copy, which copies the data on the heap as well.
    - Rust never automatically creates deep copies of your data

- Here are some of the values that implement the `Copy` trait:
  - All the integer types, such as `u32`.
  - The Boolean type, `bool`, with values `true` and `false`.
  - All the floating point types, such as `f64`.
  - The character type, `char`.
  - Tuples, if they only contain types that also implement `Copy`.

## Ownership and Functions

- Passing a variable to a function will move or copy, just like in an assignment.

  ```Rust
  fn main() {
      let s = String::from("hello");
      takes_ownership(s);
      // println!("{}", s); // Error: value borrowed here after move

      let x = 5;
      makes_copy(x);
      println!("{}", x);
  }
  fn takes_ownership(some_string: String) {
      println!("{}", some_string);
  }

  fn makes_copy(some_integer: i32) {
      println!("{}", some_integer);
  }
  ```

## Return Values and Scope

- Returning values can also transfer ownership.

  ```Rust
  fn main() {
      let s1 = String::from("hello");
      let (s2, len) = calculate_length(s1); // the function returns a tuple, so it transfers ownership
      println!("The length of '{}' is {}", s2, len);

      let s3 = gives_ownership(); // the function returns a value, so it transfers ownership
      let s4 = String::from("hello");
      let s5 = takes_and_gives_back(s4);
      // println!("{}", s4); // Error: value borrowed here after move
  }

  fn calculate_length(s: String) -> (String, usize) {
      let length = s.len();
      (s, length)
  }

  fn gives_ownership() -> String {
      let s = String::from("hello");
      s
  }

  fn takes_and_gives_back(s: String) -> String {
      s
  }
  ```

- If a return value is allocated on the heap, the ownership is transferred to the calling function unless the value is moved to another variable.
- If we need to return multiple values, we can use a tuple to return them all at once.

  ```Rust
  fn main() {
      let (s1, s2) = tupple_return();
      println!("{}, {}", s1, s2);
  }

  fn tupple_return() -> (String, String) {
      (String::from("hello"), String::from("world"))
  }
  ```

## References and Borrowing

- We can pass a reference to a variable to a function instead of transferring ownership, which is called borrowing.

  ```Rust
  fn main() {
      let s1 = String::from("hello");
      let len = calculate_length(&s1); // the function takes a reference, so it borrows the value
      println!("The length of '{}' is {}", s1, len);
  }

  fn calculate_length(s: &String) -> usize {
      s.len()
  }
  ```

- Dereferencing a reference is done with the `*` operator.
- References are immutable by default, but we can make them mutable by using `&mut`.

  ```Rust
  fn main() {
      let mut s = String::from("hello");
      change(&mut s); // Without the mut keyword, we would get an error
      println!("{}", s);
  }

  fn change(s: &mut String) {
      s.push_str(", world!");
  }
  ```

- We can have only one mutable reference to a variable in a scope
- We can have multiple immutable references to a variable in a scope
- We can't have a mutable reference and an immutable reference to a variable in a scope

  ```Rust
  let mut s = String::from("hello");
  let r1 = &s;
  let r2 = &s;
  let r3 = &mut s; // Error: cannot borrow `s` as mutable because it is also borrowed as immutable

  println!("{}, {}, and {}", r1, r2, r3);

  let mut s = String::from("hello");
  let r1 = &mut s;
  let r2 = &mut s; // Error: cannot borrow `s` as mutable more than once at a time
  ```

  - If the reference is dropped before the mutable reference, we can create a new mutable reference.

    ```Rust
    let mut s = String::from("hello");
    let r1 = &s;
    println!("{}", r1); // r1 goes out of scope here

    let r2 = &mut s; // no problem
    ```

- This behavior prevents a data race (two or more pointers accessing the same data at the same time) at compile time.
- With scope we can control the lifetime of references.

  ```Rust
  let mut s = String::from("hello");

  {
      let r1 = &mut s;
  } // r1 goes out of scope here, so we can make a new reference with no problems

  let r2 = &mut s;
  ```

## Dangling References

- Rust prevents dangling references by enforcing the borrowing rules at compile time.
  - A dangling reference is a pointer that references a location in memory that may have been given to someone else, so the data is invalid.

```Rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s // Returns a reference to the String, but the String is dropped at the end of the function
    // we should return the String itself s
}
```

- The `s` variable goes out of scope at the end of the function, so the reference is invalid.

## The Slice Type

- Slices allow us to reference a contiguous sequence of elements in a collection rather than the whole collection.
- Slices don't have ownership since they are references.

  ```Rust
  fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // word will get the value 5
    s.clear(); // s = ""
    println!("The first word length is: {}", word); // The first word is: 5
  }

  fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // Convert the string to an array of bytes

    // Iterating over the bytes of the string
    for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
        return i; // Return the index of the first space
      }
    }

    s.len() // Return the length of the string if there is no space
  }
  ```

  - When working with strings, it's better to use slices instead of indexes to avoid runtime errors.

    ```Rust
      // String Slices
    let s = String::from("hello world");
    let hello = &s[0..5]; // [start..end] - end is exclusive
    let world = &s[6..11];
    println!("{} {}", hello, world);

    let hello = &s[..5]; // start is 0
    let world = &s[6..]; // end is the length of the string
    println!("{} {}", hello, world);

    let s = "hello world";
    let hello = &s[..]; // start is 0 and end is the length of the string
    println!("{}", hello);

    // Improved first_word function
    fn first_word(s: &String) -> &str {
      let bytes = s.as_bytes();

      for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
          return &s[..i];
        }
      }

      &s[..]
    }
    ```

    - A better approach that can accept both `&String` and `&str` is to use the `&str` type.

      ```Rust
      fn main() {
        let my_string = String::from("hello world");

        // `first_word` works on slices of `String`s, whether partial
        // or whole.
        let word = first_word(&my_string[0..6]);
        let word = first_word(&my_string[..]);
        // `first_word` also works on references to `String`s, which
        // are equivalent to whole slices of `String`s.
        let word = first_word(&my_string);

        let my_string_literal = "hello world";

        // `first_word` works on slices of string literals,
        // whether partial or whole.
        let word = first_word(&my_string_literal[0..6]);
        let word = first_word(&my_string_literal[..]);

        // Because string literals *are* string slices already,
        // this works too, without the slice syntax!
        let word = first_word(my_string_literal);
      }

      fn first_word(s: &str) -> &str { // This function can accept both &String and &str
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
          if item == b' ' {
            return &s[..i];
          }
        }

        &s[..]
      }
      ```

- We should be careful when using slices because the reference to the slice is invalidated if the original variable is modified.

  ```Rust
  fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // “immutable borrow occurs here”
    s.clear(); // Error: mutable borrow occurs here
    println!("The first word is: {}", word);
  }
  ```

## Other Slice Types

- We can use the slice syntax to create slices of other types that implement the `Copy` trait.

  ```Rust
  let a = [1, 2, 3, 4, 5];
  let slice = &a[1..3]; // slice: &[i32]
  ```
