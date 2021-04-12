#![allow(clippy::upper_case_acronyms)]

use matterdb::{Database, DbOptions, Fork, Patch, Result, RocksDB, Snapshot};
use tempfile::{tempdir, TempDir};

pub mod encoding;
pub mod schema_patterns;
pub mod storage;

pub(super) struct BenchDB {
    _dir: TempDir,
    db: RocksDB,
}

impl BenchDB {
    pub(crate) fn new() -> Self {
        let dir = tempdir().expect("Couldn't create tempdir");
        let db =
            RocksDB::open(dir.path(), &DbOptions::default()).expect("Couldn't create database");
        Self { _dir: dir, db }
    }

    pub(crate) fn fork(&self) -> Fork {
        self.db.fork()
    }

    pub(crate) fn snapshot(&self) -> Box<dyn Snapshot> {
        self.db.snapshot()
    }

    pub(crate) fn merge(&self, patch: Patch) -> Result<()> {
        self.db.merge(patch)
    }

    pub(crate) fn merge_sync(&self, patch: Patch) -> Result<()> {
        self.db.merge_sync(patch)
    }
}

impl Default for BenchDB {
    fn default() -> Self {
        Self::new()
    }
}
