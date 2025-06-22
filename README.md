# enum_to_string

![MSRV](https://img.shields.io/badge/Rust%20MSRV-1.78.0-brightgreen)
[![crates.io](https://img.shields.io/crates/v/enum_name.svg)](https://crates.io/crates/enum_name)
[![Downloads](https://img.shields.io/crates/d/enum_name.svg)](https://crates.io/crates/enum_name)

No import Enum Verient to string conversion

## Overview

The `enum_name` crate provides a simple and efficient way to extract the name of an enum variant as a string or `&str`. This can be useful for logging, debugging, serialization, or any context where you want to display enum variant names without relying on custom formatting or derives.

No dependencies, no macros, no `Debug` or `Display` traits required — just pure `std::any::type_name`.

---

## How to use

Add this to your `Cargo.toml`:

```toml
[dependencies]
enum_name = "0.1"
```

```Rust
use enum_name::{variant_to_str, variant_to_string};

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

[![GitHub](https://img.shields.io/badge/github-had2020%2Fenum__name-blue?logo=github)](https://github.com/had2020/enum_name)


PRs welcome!

Leave a star to till me you want more development on this framework.
