# Common Collections

- Collections are data structures that can hold multiple values, and they are stored on the heap.
- The standard library provides a number of useful data structures called collections.

## Vectors

- Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory.
- Vectors can only store values of the same type.

```Rust
let v: Vec<i32> = Vec::new(); // Creates an empty vector of 32-bit integers.
let v = vec![1, 2, 3]; // Creates a vector with three elements and infers the type.

let mut v = Vec::new(); // It needs to be mutable to push elements into it.
v.push(5); // When pushing the first element, Rust infers the type of the vector.
v.push(6); // Pushes 5 and 6 into the vector.
println!("The second element is: {}", v[1]); // Accesses the second element.
println!("The first element is: {}", v.get(0).unwrap()); // Accesses the first element with the get method.
println!("The third element is: {}", v[2]); // Panics if the index is out of bounds.
// In these cases is better to use the get method to return an Option.
match v.get(2) {
    Some(third) => println!("third = {}", third),
    None => println!("There is no third element."),
}
```

- Inmutable references can be used to access elements of a vector.

  ```Rust
  let v = vec![100, 32, 57];
  let third: &i32 = &v[2];
  println!("The third element is {}", third);

  // If we try to push an element into the vector, the reference will be invalid.
  v.push(5);

  println!("The third element is {}", third); // This will cause a compilation error.
  ```

  - When adding a new element to the vector, it might be reallocated to a new location in memory, and the reference will be invalid.

- Iterating over the elements of a vector:

  ```Rust
  let v = vec![100, 32, 57];
  // Iterating over the elements of the vector.
  for i in &v {
      println!("{}", i);
  }

  // Iterating over the elements of the vector and changing them.
  let mut v = vec![100, 32, 57];
  for i in &mut v {
      *i += 50; // We need to dereference * the value first to be able to change it.
  }
  ```

- If we need to store elements of different types, we can use an enum.

  ```Rust
  enum SpreadsheetCell {
      Int(i32),
      Float(f64),
      Text(String),
  }

  let row = vec![
      SpreadsheetCell::Int(3),
      SpreadsheetCell::Text(String::from("blue")),
      SpreadsheetCell::Float(10.12),
  ];
  ```

- Vector as many other objects are dropped when they go out of scope.

## Strings

- Collection of characters.

## Hash Maps

- A hash map allows you to associate a value with a particular key.
