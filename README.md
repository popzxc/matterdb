# MatterDB

[![Docs.rs](https://docs.rs/matterdb/badge.svg)](https://docs.rs/matterdb)
![rust 1.45.0+ required](https://img.shields.io/badge/rust-1.45.0+-blue.svg?label=Required%20Rust)

**MatterDB** is a document-oriented persistent storage.
Under the hood, MerkleDB uses RocksDB as a key-value storage.

## Features

- Supports list, map and set collections (aka *indexes*),
  as well as singular elements.
  Further, indexes can be organized into groups, allowing to create
  hierarchies of documents with arbitrary nesting.
- Ability to define data layouts in an intuitive, declarative format.
- Basic support of transactions: changes to the storage can be
  aggregated into a fork and then merged to the database atomically.
- Access control leveraging the Rust type system, allowing to precisely
  define access privileges for different actors.
- First-class support of long-running, fault-tolerant data migrations
  running concurrently with other I/O to the storage.

## Usage

Include `matterdb` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
matterdb = "1.0.0"
```

## History notice

MatterDB was initially created as [MerkleDB](https://github.com/exonum/exonum/tree/master/components/merkledb)
by [Exonum](https://exonum.com/index).

MerkleDB was initially created to support merklized collections atop of the persistent key-value storage.
This project does not have the same purpose: instead, it provides a generic convenient and (ideally) backend-agnostic
interface for the persistent NoSQL storage, without any bounds to the blockchain specifics.

## License

`matterdb` is licensed under the Apache License (Version 2.0).
See [LICENSE](LICENSE) for details.
