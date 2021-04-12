# Procedural macros for matterdb

![rust 1.45.0+ required](https://img.shields.io/badge/rust-1.45.0+-blue.svg?label=Required%20Rust)

This crate provides several procedural macros for MatterDB.

Overview of presented macros:

- `BinaryValue`: derive macro for `BinaryValue` trait of MerkleDB.
  The implementation uses `serde` traits using `bincode`.
- `FromAccess`: derive macro for `FromAccess` trait for schemas of
  MerkleDB indexes.

Consult [the crate docs](https://docs.rs/matterdb-derive) for more details.

## Usage

Include `matterdb-derive` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
matterdb-derive = "1.0.0"
```

## License

`matterdb-derive` is licensed under the Apache License (Version 2.0).
See [LICENSE](LICENSE) for details.
