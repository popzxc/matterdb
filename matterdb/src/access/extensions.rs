//! Extension traits to simplify index instantiation.

use crate::{
    access::{Access, FromAccess},
    views::IndexType,
    BinaryKey, BinaryValue, Entry, Group, IndexAddress, KeySetIndex, ListIndex, MapIndex,
    SparseListIndex,
};

/// Extension trait allowing for easy access to indexes from any type implementing
/// [`Access`] + `Copy`.
///
/// # Implementation details
///
/// This trait is essentially a thin wrapper around [`FromAccess`]. Where `FromAccess` returns
/// an access error, the methods of this trait will `unwrap()` the error and panic.
/// This trait is helpful for references implementing [`Access`], such as `&Fork` or `&dyn Snapshot`
/// because Rust method resolution does not apply [`AccessExt`] to variables of corresponding types.
/// For example, if fork has type Fork, then `fork.get_list("foo")` is not resolved
/// as `AccessExt::get_list(..)`, only `(&fork).get_list("foo")` is.
/// [`Access`]: trait.Access.html
/// [`AccessExt`]: trait.AccessExt.html
/// [`FromAccess`]: trait.FromAccess.html
///
/// # Examples
///
/// ```
/// use matterdb::{access::CopyAccessExt, Database, ListIndex, TemporaryDB};
///
/// let db = TemporaryDB::new();
/// let fork = db.fork();
/// // Extension methods can be used on `Fork`s:
/// {
///     let mut list: ListIndex<_, String> = fork.get_list("list");
///     list.push("foo".to_owned());
/// }
///
/// // ...and on `Snapshot`s:
/// let snapshot = db.snapshot();
/// assert!(snapshot
///     .get_map::<_, u64, String>("map")
///     .get(&0)
///     .is_none());
///
/// // ...and on `ReadonlyFork`s:
/// {
///     let list = fork.readonly().get_list::<_, String>("list");
///     assert_eq!(list.len(), 1);
/// }
///
/// // ...and on `Patch`es:
/// let patch = fork.into_patch();
/// let list = patch.get_list::<_, String>("list");
/// assert_eq!(list.len(), 1);
/// ```
pub trait CopyAccessExt: Access + Copy {
    /// Returns a group of indexes. All indexes in the group have the same type.
    /// Indexes are initialized lazily; i.e., no initialization is performed when the group
    /// is created.
    ///
    /// Note that unlike other methods, this one requires address to be a string.
    /// This is to prevent collisions among groups.
    fn get_group<K, I>(self, name: impl Into<String>) -> Group<Self, K, I>
    where
        K: BinaryKey + ?Sized,
        I: FromAccess<Self>,
    {
        Group::from_access(self, IndexAddress::from_root(name))
            .unwrap_or_else(|e| panic!("MerkleDB error: {}", e))
    }

    /// Gets an entry index with the specified address.
    ///
    /// # Panics
    ///
    /// If the index exists, but is not an entry.
    fn get_entry<I, V>(self, addr: I) -> Entry<Self::Base, V>
    where
        I: Into<IndexAddress>,
        V: BinaryValue,
    {
        Entry::from_access(self, addr.into()).unwrap_or_else(|e| panic!("MerkleDB error: {}", e))
    }

    /// Gets a list index with the specified address.
    ///
    /// # Panics
    ///
    /// If the index exists, but is not a list.
    fn get_list<I, V>(self, addr: I) -> ListIndex<Self::Base, V>
    where
        I: Into<IndexAddress>,
        V: BinaryValue,
    {
        ListIndex::from_access(self, addr.into())
            .unwrap_or_else(|e| panic!("MerkleDB error: {}", e))
    }

    /// Gets a map index with the specified address.
    ///
    /// # Panics
    ///
    /// If the index exists, but is not a map.
    fn get_map<I, K, V>(self, addr: I) -> MapIndex<Self::Base, K, V>
    where
        I: Into<IndexAddress>,
        K: BinaryKey + ?Sized,
        V: BinaryValue,
    {
        MapIndex::from_access(self, addr.into()).unwrap_or_else(|e| panic!("MerkleDB error: {}", e))
    }

    /// Gets a sparse list index with the specified address.
    ///
    /// # Panics
    ///
    /// If the index exists, but is not a sparse list.
    fn get_sparse_list<I, V>(self, addr: I) -> SparseListIndex<Self::Base, V>
    where
        I: Into<IndexAddress>,
        V: BinaryValue,
    {
        SparseListIndex::from_access(self, addr.into())
            .unwrap_or_else(|e| panic!("MerkleDB error: {}", e))
    }

    /// Gets a key set index with the specified address.
    ///
    /// # Panics
    ///
    /// If the index exists, but is not a key set.
    fn get_key_set<I, K>(self, addr: I) -> KeySetIndex<Self::Base, K>
    where
        I: Into<IndexAddress>,
        K: BinaryKey + ?Sized,
    {
        KeySetIndex::from_access(self, addr.into())
            .unwrap_or_else(|e| panic!("MerkleDB error: {}", e))
    }

    /// Gets index type at the specified address, or `None` if there is no index.
    fn index_type<I>(self, addr: I) -> Option<IndexType>
    where
        I: Into<IndexAddress>,
    {
        self.get_index_metadata(addr.into())
            .unwrap_or_else(|e| panic!("MerkleDB error: {}", e))
            .map(|metadata| metadata.index_type())
    }
}

impl<T: Access + Copy> CopyAccessExt for T {}

/// Extension trait allowing for easy access to indexes from any type implementing
/// [`Access`].
///
/// # Implementation details
///
/// This trait is essentially a thin wrapper around [`FromAccess`]. Where [`FromAccess`] returns
/// an access error, the methods of this trait will `unwrap()` the error and panic.
/// For a version on [`AccessExt`] traits designed for `Copy` types (e.g. `&Fork` and
/// `&dyn Snapshot`) see [`CopyAccessExt`] trait.
///
/// [`Access`]: trait.Access.html
/// [`FromAccess`]: trait.FromAccess.html
/// [`AccessExt`]: trait.AccessExt.html
/// [`CopyAccessExt`]: trait.CopyAccessExt.html
pub trait AccessExt: Access {
    /// Returns a group of indexes. All indexes in the group have the same type.
    /// Indexes are initialized lazily; i.e., no initialization is performed when the group
    /// is created.
    ///
    /// Note that unlike other methods, this one requires address to be a string.
    /// This is to prevent collisions among groups.
    fn get_group<K, I>(&self, name: impl Into<String>) -> Group<Self, K, I>
    where
        K: BinaryKey + ?Sized,
        I: FromAccess<Self>,
    {
        Group::from_access(self.clone(), IndexAddress::from_root(name))
            .unwrap_or_else(|e| panic!("MerkleDB error: {}", e))
    }

    /// Gets an entry index with the specified address.
    ///
    /// # Panics
    ///
    /// If the index exists, but is not an entry.
    fn get_entry<I, V>(&self, addr: I) -> Entry<Self::Base, V>
    where
        I: Into<IndexAddress>,
        V: BinaryValue,
    {
        Entry::from_access(self.clone(), addr.into())
            .unwrap_or_else(|e| panic!("MerkleDB error: {}", e))
    }

    /// Gets a list index with the specified address.
    ///
    /// # Panics
    ///
    /// If the index exists, but is not a list.
    fn get_list<I, V>(&self, addr: I) -> ListIndex<Self::Base, V>
    where
        I: Into<IndexAddress>,
        V: BinaryValue,
    {
        ListIndex::from_access(self.clone(), addr.into())
            .unwrap_or_else(|e| panic!("MerkleDB error: {}", e))
    }

    /// Gets a map index with the specified address.
    ///
    /// # Panics
    ///
    /// If the index exists, but is not a map.
    fn get_map<I, K, V>(&self, addr: I) -> MapIndex<Self::Base, K, V>
    where
        I: Into<IndexAddress>,
        K: BinaryKey + ?Sized,
        V: BinaryValue,
    {
        MapIndex::from_access(self.clone(), addr.into())
            .unwrap_or_else(|e| panic!("MerkleDB error: {}", e))
    }

    /// Gets a sparse list index with the specified address.
    ///
    /// # Panics
    ///
    /// If the index exists, but is not a sparse list.
    fn get_sparse_list<I, V>(&self, addr: I) -> SparseListIndex<Self::Base, V>
    where
        I: Into<IndexAddress>,
        V: BinaryValue,
    {
        SparseListIndex::from_access(self.clone(), addr.into())
            .unwrap_or_else(|e| panic!("MerkleDB error: {}", e))
    }

    /// Gets a key set index with the specified address.
    ///
    /// # Panics
    ///
    /// If the index exists, but is not a key set.
    fn get_key_set<I, K>(&self, addr: I) -> KeySetIndex<Self::Base, K>
    where
        I: Into<IndexAddress>,
        K: BinaryKey + ?Sized,
    {
        KeySetIndex::from_access(self.clone(), addr.into())
            .unwrap_or_else(|e| panic!("MerkleDB error: {}", e))
    }

    /// Gets index type at the specified address, or `None` if there is no index.
    fn index_type<I>(&self, addr: I) -> Option<IndexType>
    where
        I: Into<IndexAddress>,
    {
        self.clone()
            .get_index_metadata(addr.into())
            .unwrap_or_else(|e| panic!("MerkleDB error: {}", e))
            .map(|metadata| metadata.index_type())
    }
}

impl<T: Access> AccessExt for T {}

#[cfg(test)]
mod tests {
    use super::{AccessExt, CopyAccessExt, IndexType};
    use crate::{access::Prefixed, migration::Migration, Database, TemporaryDB};

    #[test]
    fn index_type_works() {
        let db = TemporaryDB::new();
        let fork = db.fork();
        fork.get_list("list").extend(vec![1, 2, 3]);
        assert_eq!(fork.index_type("list"), Some(IndexType::List));
        fork.get_map(("fam", &0_u8)).put(&1_u8, 2_u8);
        assert_eq!(fork.index_type(("fam", &0_u8)), Some(IndexType::Map));
        assert_eq!(fork.index_type(("fam", &1_u8)), None);

        let patch = fork.into_patch();
        assert_eq!(patch.index_type("list"), Some(IndexType::List));
        assert_eq!(patch.index_type(("fam", &0_u8)), Some(IndexType::Map));
        assert_eq!(patch.index_type(("fam", &1_u8)), None);

        db.merge(patch).unwrap();
        let snapshot = db.snapshot();
        assert_eq!(snapshot.index_type("list"), Some(IndexType::List));
        assert_eq!(snapshot.index_type(("fam", &0_u8)), Some(IndexType::Map));
        assert_eq!(snapshot.index_type(("fam", &1_u8)), None);
    }

    #[test]
    fn index_type_in_migration() {
        let db = TemporaryDB::new();
        let mut fork = db.fork();
        fork.get_list("some.list").extend(vec![1, 2, 3]);
        fork.get_entry(("some.entry", &0_u8)).set("!".to_owned());
        fork.get_entry(("some.entry", &1_u8)).set("!!".to_owned());

        {
            let migration = Migration::new("some", &fork);
            migration.get_list("list").extend(vec![4, 5, 6]);
            migration.create_tombstone(("entry", &0_u8));
            assert_eq!(migration.index_type("list"), Some(IndexType::List));
            assert_eq!(
                migration.index_type(("entry", &0_u8)),
                Some(IndexType::Tombstone)
            );
            assert_eq!(migration.index_type(("entry", &1_u8)), None);
        }
        fork.flush_migration("some");

        let patch = fork.into_patch();
        let ns = Prefixed::new("some", &patch);
        assert_eq!(ns.clone().index_type("list"), Some(IndexType::List));
        assert_eq!(ns.clone().index_type(("entry", &0_u8)), None);
        assert_eq!(
            ns.clone().index_type(("entry", &1_u8)),
            Some(IndexType::Entry)
        );

        db.merge(patch).unwrap();
        let snapshot = db.snapshot();
        assert_eq!(snapshot.index_type("some.list"), Some(IndexType::List));
        assert_eq!(snapshot.index_type(("some.entry", &0_u8)), None);
        assert_eq!(
            snapshot.index_type(("some.entry", &1_u8)),
            Some(IndexType::Entry)
        );
    }
}
