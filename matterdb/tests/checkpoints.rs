use matterdb::{access::CopyAccessExt, DBOptions, Database, RocksDB};
use tempfile::TempDir;

#[test]
fn checkpoints() {
    let src_temp_dir = TempDir::new().unwrap();
    let dst_temp_dir = TempDir::new().unwrap();

    let src_path = src_temp_dir.path().join("src");
    let dst_path = dst_temp_dir.path().join("dst");

    // Convert into `dyn Database` to test downcast.
    let db = RocksDB::open(&*src_path, &DBOptions::default()).unwrap();

    // Write some data to the source database.
    {
        let fork = db.fork();
        fork.get_entry("first").set(vec![1_u8; 1024]);
        db.merge_sync(fork.into_patch()).unwrap();
    }

    // Create checkpoint
    {
        db.create_checkpoint(&*dst_path).unwrap();
    }

    // Add more data to the source database
    {
        let fork = db.fork();
        fork.get_entry("second").set(vec![2_u8; 1024]);
        db.merge_sync(fork.into_patch()).unwrap();
    }

    // Close source database.
    drop(db);

    // Open checkpoint and Assert that it's not affected
    // by the data added after create_checkpoint call.
    {
        let checkpoint = RocksDB::open(&*dst_path, &DBOptions::default()).unwrap();
        let fork = checkpoint.fork();

        assert_eq!(fork.get_entry("first").get(), Some(vec![1_u8; 1024]));
        assert_eq!(fork.get_entry("second").get(), None::<Vec<u8>>);

        // Add more data to the checkpoint
        fork.get_entry("third").set(vec![3_u8; 1024]);
        checkpoint.merge_sync(fork.into_patch()).unwrap();
    }

    // Assert that source database is not affected by the data added to checkpoint.
    {
        let db = RocksDB::open(&*src_path, &DBOptions::default()).unwrap();
        let fork = db.fork();

        assert_eq!(fork.get_entry("first").get(), Some(vec![1_u8; 1024]));
        assert_eq!(fork.get_entry("second").get(), Some(vec![2_u8; 1024]));
        assert_eq!(fork.get_entry("third").get(), None::<Vec<u8>>);
    }

    // Delete source database's directory.
    drop(src_temp_dir);

    // Assert that checkpoint is not affected if source database is deleted.
    {
        let checkpoint = RocksDB::open(&*dst_path, &DBOptions::default()).unwrap();
        let fork = checkpoint.fork();

        assert_eq!(fork.get_entry("first").get(), Some(vec![1_u8; 1024]));
        assert_eq!(fork.get_entry("second").get(), None::<Vec<u8>>);

        // Add more data to the checkpoint
        fork.get_entry("third").set(vec![3_u8; 1024]);
        checkpoint.merge_sync(fork.into_patch()).unwrap();
    }
}
