# Functional Language Features: Iterators and Closures

## Closures

- Closures are anonymous functions that can capture their environment.

```rust
fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
    user_preference.unwrap_or_else(|| self.most_stocked())
} // The closure || captures the `self` parameter, if the closure would need parameters, they would be inside the pipes.
```

- The difference between functions and closures is that closures can capture their environment, meaning they can use variables from the scope in which they are defined.
- It's rare that closures have explicit types but you can add them if you want to.

```rust
let expensive_closure = |num: u32| -> u32 {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

- Closures can capture values from their environment in three ways:

  - Borrowing:

    ```Rust
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
    ```

  - Mutably borrowing:

    ```Rust
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(4);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
    ```

  - Moving ownership:

    ```Rust
    use std::thread;

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // The move keyword forces the closure to take ownership of the values it uses.
    thread::spawn(move || {
      println!("From thread: {:?}", list)
    }).join().unwrap();
    ```

- Moving captured values out of closures and the `Fn` trait:

  - The `FnOnce` trait is used when the closure takes ownership of the values it captures.

    ```Rust
    impl<T> Option<T> {
        pub fn unwrap_or_else<F>(self, f: F) -> T
        where
            F: FnOnce() -> T,
        {
            match self {
                Some(x) => x,
                None => f(),
            }
        }
    }
    ```

  - The `FnMut` trait is used when the closure mutably borrows the values it captures.

    ```Rust
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!(
        "{:#?}, sorted in {num_sort_operations} operations",
        list
    );
    ```

## Iterators

- Iterators are lazy and do not perform any operations until they are consumed.

  ```rust
  let list = vec![1, 2, 3];

  let mut iter = list.iter();

  for i in iter {
      println!("{}", i);
  }
  ```

- All iterators implement the `Iterator` trait.

  ```rust
  trait Iterator {
      type Item;

      fn next(&mut self) -> Option<Self::Item>;
  }
  ```

- Take in count that when you iterate over an `iter` object, each step uses the `next` method behind the scenes.

  ```rust
  let mut list = vec![1, 2, 3];
  let mut iter = list.iter();

  assert_eq!(iter.next(), Some(&1));
  assert_eq!(iter.next(), Some(&2));
  assert_eq!(iter.next(), Some(&3));
  assert_eq!(iter.next(), None);
  ```

- Iterators have built-in methods that allow you to consume them in different ways.

  ```rust
  let list = vec![1, 2, 3];
  let sum: i32 = list.iter().sum();
  ```

- If you need to take ownership of the values, you can use the `into_iter` method instead.

  ```rust
  let list = vec![1, 2, 3];
  let sum: i32 = list.into_iter().sum();
  ```

- Some methods return iterators after consuming them, like the `map` method.

  ```rust
  let list = vec![1, 2, 3];
  let sum: i32 = list.iter().map(|x| x * x).sum();
  ```

- For complex operations, you can use iterators and closures together.

  ```rust
  let list = vec![1, 2, 3];
  let sum: i32 = list.iter().filter(|&x| x % 2 == 0).map(|x| x * x).sum();
  // The filter method takes a closure that returns a boolean value to determine if the value should be included in the iterator.
  ```

- In rust iterators are zero-cost abstractions, meaning that they are as fast as if you wrote the code by hand, which is why they are preferred over loops.
