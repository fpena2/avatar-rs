# Avatar-rs

Generate semi-random avatars.

Inspired by GitHub's default avatars.

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
