//! Abstract settings for databases.

use rocksdb::DBCompressionType;
use serde::{Deserialize, Serialize};

/// Options for the database.
///
/// These parameters apply to the underlying database, currently `RocksDB`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub struct DBOptions {
    /// Number of open files that can be used by the database.
    ///
    /// The underlying database opens multiple files during operation. If your system has a
    /// limit on the number of files which can be open simultaneously, you can
    /// adjust this option to match the limit. Note, that limiting the number
    /// of simultaneously open files might slow down the speed of database operation.
    ///
    /// Defaults to `None`, meaning that the number of open files is unlimited.
    pub max_open_files: Option<i32>,
    /// An option to indicate whether the system should create a database or not,
    /// if it's missing.
    ///
    /// This option applies to the cases when a node was
    /// switched off and is on again. If the database cannot be found at the
    /// indicated path and this option is switched on, a new database will be
    /// created at that path and blocks will be included therein.
    ///
    /// Defaults to `true`.
    pub create_if_missing: bool,
    /// An algorithm used for database compression.
    ///
    /// Defaults to `CompressionType::None`, meaning there is no compression.
    pub compression_type: CompressionType,
    /// Max total size of the WAL journal in bytes.
    ///
    /// Defaults to `None`, meaning that the size of WAL journal will be adjusted
    /// by the rocksdb.
    pub max_total_wal_size: Option<u64>,
    /// Max `LRU` in-memory cache size in bytes.
    ///
    /// Defaults to `None`, meaning that there will be no cache used.
    pub max_cache_size: Option<usize>,
}

impl DBOptions {
    /// Creates a new `DBOptions` object.
    pub fn new(
        max_open_files: Option<i32>,
        create_if_missing: bool,
        compression_type: CompressionType,
        max_total_wal_size: Option<u64>,
        max_cache_size: Option<usize>,
    ) -> Self {
        Self {
            max_open_files,
            create_if_missing,
            compression_type,
            max_total_wal_size,
            max_cache_size,
        }
    }
}

/// Algorithms of compression for the database.
///
/// Database contents are stored in a set of blocks, each of which holds a
/// sequence of key-value pairs. Each block may be compressed before
/// being stored in a file. The following enum describes which
/// compression algorithm (if any) is used to compress a block.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CompressionType {
    Bz2,
    Lz4,
    Lz4hc,
    Snappy,
    Zlib,
    Zstd,
    None,
}

impl From<CompressionType> for DBCompressionType {
    fn from(compression_type: CompressionType) -> Self {
        match compression_type {
            CompressionType::Bz2 => Self::Bz2,
            CompressionType::Lz4 => Self::Lz4,
            CompressionType::Lz4hc => Self::Lz4hc,
            CompressionType::Snappy => Self::Snappy,
            CompressionType::Zlib => Self::Zlib,
            CompressionType::Zstd => Self::Zstd,
            CompressionType::None => Self::None,
        }
    }
}

impl Default for DBOptions {
    fn default() -> Self {
        Self::new(None, true, CompressionType::None, None, None)
    }
}
