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
  - Taking ownership: `let x = vec![1, 2, 3]; let equal_to_x = move |z| z == x;`
  - Borrowing: `let x = vec![1, 2, 3]; let equal_to_x = |z| z == &x;`
  - Mutably borrowing: `let mut x = vec![1, 2, 3]; let equal_to_x = |z| z == &mut x;`
