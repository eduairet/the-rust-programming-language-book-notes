# Functional Language Features: Iterators and Closures

## Closures

- Closures are anonymous functions that can capture their environment.

```rust
fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
    user_preference.unwrap_or_else(|| self.most_stocked())
} // The closure || captures the `self` parameter, if the closure would need parameters, they would be inside the pipes.
```
