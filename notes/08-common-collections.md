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

- Strings in rust are collections of bytes.
- The `str` type is a string slice and is the only string type that is hard-coded into the language.
- The `String` is a growable, mutable, owned, UTF-8 encoded string type provided by the standard library.
- `str` and `String` are UTF-8 encoded strings.
- `String` has a lot of similarities with a vector since it's implemented as a wrapper around a vector of bytes.

```Rust
let mut s = String::new(); // Creates a new empty string.
let data = "initial contents";
let s = data.to_string(); // Converts a string slice to a String.
let s = "initial contents".to_string(); // Does the same as the previous line.
let s = String::from("initial contents"); // Does the same as the previous line.

// Since strings are UTF-8 encoded, we can include any character in them.
let hello = String::from("السلام عليكم");
let hello = String::from("Dobrý den");
let hello = String::from("Hello");
let hello = String::from("שָׁלוֹם");
let hello = String::from("नमस्ते");
let hello = String::from("こんにちは");
let hello = String::from("안녕하세요");
let hello = String::from("你好");
let hello = String::from("Olá");
let hello = String::from("Здравствуйте");
let emoji = String::from("👋");
```

- Updating a string:

  ```Rust
  let mut s = String::from("foo");
  s.push_str("bar"); // Appends a string slice to a String.
  s.push('l'); // Appends a single character to a String.

  let s1 = String::from("Hello, ");
  let s2 = String::from("world!");
  let s3 = s1 + &s2; // The + operator uses the add method.
  // &s2 coerces the &String to a &str and adds a reference to the string.
  // s1 has been moved here and can no longer be used.
  let extra_exclamation = String::from("!!!");
  s3.push_str(extra_exclamation); // .push_str() takes ownership of extra_exclamation.

  ```

  - The `push_str` method takes a string slice because we don't necessarily want to take ownership of the parameter.

  - When combining several strings, it's better to use the `format!` macro.

    ```Rust
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3); // The format! macro doesn't take ownership of any of its parameters.
    ```

- Indexing into strings is not allowed in Rust.

  ```Rust
  let s1 = String::

  let s1 = String::from("hello");
  let h = s1[0]; // This will cause a compilation error.
  ```

  - Rust strings are a collection of bytes, and indexing into a string would return a byte, not a character.
  - Each can be byte, scalar value and grapheme cluster, and it would be more complex to index into a string.
    ```rust
    "नमस्ते"
    [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135] // The bytes of the string
    ['न', 'म', 'स', '◌्', 'त', '◌े'] // The characters of the string, letters and diacritics.
    ["न", "म", "स्", "ते"] // The last two characters are grapheme clusters which combine letters and diacritics.
    ```

- You can slice a string using a range.

  ```Rust
  let hello = "Здравствуйте";
  let s = &hello[0..4]; // This will take the first four bytes of the string.
  ```

  - Be careful when slicing strings because it can cause a panic if the range is not valid.

- If you need to iterate over the characters of a string, you can use the `chars` method.

  ```Rust
  for c in "नमस्ते".chars() {
      println!("{}", c);
  }
  ```

- If you need to iterate over the bytes of a string, you can use the `bytes` method.

  ```Rust
  for b in "नमस्ते".bytes() {
      println!("{}", b);
  }
  ```

- For complex operations with strings it's better to use a crate like `unicode-segmentation`.

## Hash Maps

- A hash map allows you to associate a value with a particular key.
