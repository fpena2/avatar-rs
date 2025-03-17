# Avatar-rs

Rust library designed for generating semi-random avatars, inspired by GitHub's default avatars. It's simple to use and blazing fast, making it perfect for applications that require quick avatar generation.

## Features

- **Consistent Output** 🌱: Generate consistent avatars using a seed value.
- **High Performance** 🚀:
  - Icons are generated in less than 90 µs on an Intel Core i7-8650U.

  - Icon generation and saving to memory in less than 600 µs, though this can vary based on disk performance and system load.

## Usage

```rust
fn main() {
    let seed = 12345;
    let icon = Icon::new(seed);
    icon.save("test.png").unwrap();
}
```

## Example

![example](/static/example.png)
