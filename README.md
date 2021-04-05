# Rust Iterators (impl. of `std::iter::Iterator::flatten`)

This is a simple implementation of the rust standard library's iterator's flatten method, taken from [this](https://www.youtube.com/watch?v=yozQ9C69pNs&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=4) stream by @jonhoo.

## Usage (if you publish to crates.io)
```rust
fn main() {
    // `flatten2` is the method added by this library
    assert_eq!(vec![vec![0, 1]].into_iter().flatten2().count(), 2);
}
```