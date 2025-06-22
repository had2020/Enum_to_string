# enum_to_string

A lightweight, no-macro Rust utility for converting enum variant references to their string name.

## Overview

The `enum_to_string` crate provides a simple and efficient way to extract the name of an enum variant as a string or `&str`. This can be useful for logging, debugging, serialization, or any context where you want to display enum variant names without relying on custom formatting or derives.

No dependencies, no macros, no `Debug` or `Display` traits required — just pure `std::any::type_name`.

---

## 🔧 Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
enum_to_string = "0.1"
```

```Rust
use enum_to_string::{variant_to_str, variant_to_string};

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let dir = Direction::East;

    println!("Variant (str): {}", variant_to_str(&dir));     // Output: "East"
    println!("Variant (String): {}", variant_to_string(&dir)); // Output: "East"
}
```

## Notes

These functions work best with `enum variant references`. Passing full types or manually constructed types may not yield expected results.

Does `not support` `tuple or struct variants` (Enum::TupleVariant(x) will just return "TupleVariant").

This is a zero-cost abstraction at runtime, as type_name::<T>() is evaluated statically.

Feel free to use, modify, and distribute, or contribute to the github.

https://github.com/had2020/Enum_to_string

PRs welcome!

Leave a star to till me you want more development on this framework.
