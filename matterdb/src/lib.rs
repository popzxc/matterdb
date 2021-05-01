//! Interfaces to work with persisted data.
//!
//! # Database
//!
//! A [`Database`] is a container for data persistence. Internally, a `Database` is
//! a collection of named key-value stores (aka column families)
//! with reading isolation and atomic writes. The database is assumed to be embedded,
//! that is, the application process has exclusive access to the DB during operation.
//! You can interact with the `Database` from multiple threads by cloning its instance.
//!
//! This crate provides two database types: [`RocksDB`] and [`TemporaryDB`].
//!
//! # Snapshot and Fork
//!
//! Snapshots and forks facilitate access to the database.
//!
//! If you need to read the data, you can create a [`Snapshot`] using the [`snapshot`][1] method
//! of the `Database` instance. Snapshots provide read isolation, so you are guaranteed to work
//! with consistent values even if the data in the database changes between reads. `Snapshot`
//! provides all the necessary methods for reading data from the database, so `&Snapshot`
//! is used as a storage view for creating a read-only representation of the [indexes](#indexes).
//!
//! If you need to make changes to the database, you need to create a [`Fork`] using
//! the [`fork`][2] method of the `Database`. Like `Snapshot`, `Fork` provides read isolation,
//! but also allows creating a sequence of changes to the database that are specified
//! as a [`Patch`]. A patch can be atomically [`merge`]d into a database. Different threads
//! may call `merge` concurrently.
//!
//! # `BinaryKey` and `BinaryValue` traits
//!
//! If you need to use your own data types as keys or values in the storage, you need to implement
//! the [`BinaryKey`] or [`BinaryValue`] traits respectively. These traits have already been
//! implemented for most standard types.
//!
//! # Indexes
//!
//! Indexes are structures representing data collections stored in the database.
//! This concept is similar to tables in relational databases. The interfaces
//! of the indexes are similar to ordinary collections (like arrays, maps and sets).
//!
//! Each index occupies a certain set of keys in a single column family of the [`Database`].
//! On the other hand, multiple indexes can be stored in the same column family, provided
//! that their key spaces do not intersect. Isolation is commonly achieved with the help
//! of [`Group`]s or keyed [`IndexAddress`]es.
//!
//! This crate provides the following index types:
//!
//! - [`Entry`] is a specific index that stores only one value. Useful for global values, such as
//!   configuration. Similar to a combination of [`Box`] and [`Option`].
//! - [`ListIndex`] is a list of items stored in a sequential order. Similar to [`Vec`].
//! - [`SparseListIndex`] is a list of items stored in a sequential order. Similar to `ListIndex`,
//!   but may contain indexes without elements.
//! - [`MapIndex`] is a map of keys and values. Similar to [`BTreeMap`].
//! - [`KeySetIndex`] and [`ValueSetIndex`] are sets of items, similar to [`BTreeSet`] and
//!   [`HashSet`] accordingly.
//!
//! # Migrations
//!
//! The database [provides tooling](migration/index.html) for data migrations. With the help
//! of migration, it is possible to gradually accumulate changes to a set of indexes (including
//! across process restarts) and then atomically apply or discard these changes.
//!
//! [`Database`]: trait.Database.html
//! [`RocksDB`]: struct.RocksDB.html
//! [`TemporaryDB`]: struct.TemporaryDB.html
//! [`Snapshot`]: trait.Snapshot.html
//! [`Fork`]: struct.Fork.html
//! [`Patch`]: struct.Patch.html
//! [1]: trait.Database.html#tymethod.snapshot
//! [2]: trait.Database.html#method.fork
//! [`merge`]: trait.Database.html#tymethod.merge
//! [`BinaryKey`]: trait.BinaryKey.html
//! [`BinaryValue`]: trait.BinaryValue.html
//! [`Entry`]: indexes/struct.Entry.html
//! [`ListIndex`]: indexes/struct.ListIndex.html
//! [`SparseListIndex`]: indexes/struct.SparseListIndex.html
//! [`MapIndex`]: indexes/struct.MapIndex.html
//! [`KeySetIndex`]: indexes/struct.KeySetIndex.html
//! [`ValueSetIndex`]: indexes/struct.ValueSetIndex.html
//! [`ObjectHash`]: trait.ObjectHash.html
//! [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
//! [`Box`]: https://doc.rust-lang.org/std/boxed/struct.Box.html
//! [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
//! [`BTreeMap`]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
//! [`BTreeSet`]: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html
//! [`HashSet`]: https://doc.rust-lang.org/std/collections/struct.HashSet.html
//! [`Group`]: indexes/group/struct.Group.html

#![warn(
    missing_debug_implementations,
    unsafe_code,
    bare_trait_objects,
    missing_docs
)]
#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
    // Next `cast_*` lints don't give alternatives.
    clippy::cast_possible_wrap, clippy::cast_possible_truncation, clippy::cast_sign_loss,
    // Next lints produce too much noise/false positives.
    clippy::module_name_repetitions, clippy::similar_names, clippy::must_use_candidate,
    clippy::pub_enum_variant_names, clippy::upper_case_acronyms,
    // '... may panic' lints.
    clippy::indexing_slicing,
    // Too much work to fix.
    clippy::missing_errors_doc, clippy::missing_const_for_fn, clippy::missing_panics_doc,
    // Seems should be fixed in `thiserror` crate.
    clippy::reversed_empty_ranges,
)]

#[allow(unused_imports)]
#[macro_use] // Code generated by Protobuf requires `serde_derive` macros to be globally available.
extern crate serde_derive;

// Re-exports for use in the derive macros.
#[doc(hidden)]
pub mod _reexports {
    pub use anyhow::Error;
}

pub use self::{
    backends::{
        rocksdb::{self, RocksDB},
        temporarydb::TemporaryDB,
    },
    db::{
        Database, DatabaseExt, Fork, Iter, Iterator, OwnedReadonlyFork, Patch, ReadonlyFork,
        Snapshot,
    },
    error::Error,
    keys::BinaryKey,
    lazy::Lazy,
    options::DBOptions,
    values::BinaryValue,
    views::{AsReadonly, IndexAddress, IndexType, ResolvedAddress},
};
// Workaround for 'Linked file at path {matterdb_path}/struct.MapIndex.html
// does not exist!'
#[doc(no_inline)]
pub use self::indexes::{Entry, Group, KeySetIndex, ListIndex, MapIndex, SparseListIndex};

#[macro_use]
mod macros;
pub mod access;
mod backends;
mod db;
mod error;
pub mod generic;
pub mod indexes;
mod keys;
mod lazy;
pub mod migration;
mod options;
pub mod validation;
mod values;
mod views;

/// A specialized `Result` type for I/O operations with storage.
pub type Result<T> = std::result::Result<T, Error>;
