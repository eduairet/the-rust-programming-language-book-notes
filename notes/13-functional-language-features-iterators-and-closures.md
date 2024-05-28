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
