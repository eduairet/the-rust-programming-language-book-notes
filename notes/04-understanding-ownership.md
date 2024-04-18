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
