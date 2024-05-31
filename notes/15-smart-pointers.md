# Smart Pointers

- A pointer is an address in memory
- A smart pointer is a data structure that acts like a pointer but also has additional metadata and capabilities
- Smart pointers:
  - Own the data they're referring to
  - They're usually implemented using structs
  - Implement the `Deref` and `Drop` traits
    - `Deref`: allows an instance of the smart pointer struct to behave like a reference
      - This means you can use the `*<Some struct with the defer trait>` operator to dereference a smart pointer instance
        - It's the equivalent to `*(<Some struct with the defer trait>.deref())`
      - Some types are implicit inferred by the compiler to be dereferenced
        - For example, when you use the `*` operator on a string literal, the compiler automatically dereferences it to a `&str`
      - The mutable dereference operator `*` is also implemented by the `DerefMut` trait
    - `Drop`: allows you to customize the code that is run when an instance of the smart pointer goes out of scope

## Using Box<T> to Point to Data on the Heap

- `Box<T>` allows you to store data on the heap rather than the stack
- The data is cleaned up when the `Box<T>` goes out of scope
- The pointer itself is stored on the stack
- Useful when:
  - You have a type whose size can't be known at compile time
  - You have a large amount of data and you want to transfer ownership but ensure the data won't be copied when you do so
  - You want to own a value and you care only that it's a type that implements a particular trait rather than any specific type
