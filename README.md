# getch-rs

[![Actions Status](https://github.com/kumavale/getch-rs/workflows/CI/badge.svg)](https://github.com/kumavale/getch-rs/actions)
[![Crates.io](https://img.shields.io/crates/v/getch-rs.svg)](https://crates.io/crates/getch-rs)
[![Documentation](https://docs.rs/getch-rs/badge.svg)](https://docs.rs/getch-rs/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE)

`getch` for Windows and Unix.

## Usage

Cargo.toml

```toml
[dependencies]
getch-rs = "0.2"
```

main.rs

```rust
use getch_rs::Getch;

fn main() {
    let g = Getch::new();

    if let Ok(key) = g.getch() {
        println!("{:?}", key);
    }
}
```

## Examples

```
$ cargo run --example getch
```

## Contributing

This project welcomes your PR and issues. For example, fixing bugs, adding features, refactoring, etc.
