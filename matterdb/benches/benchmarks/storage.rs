use criterion::{
    black_box, AxisScale, BatchSize, Bencher, BenchmarkId, Criterion, PlotConfiguration, Throughput,
};
use rand::{rngs::StdRng, Rng, SeedableRng};

use matterdb::{access::CopyAccessExt, Fork, ListIndex, MapIndex};

use super::BenchDB;

const NAME: &str = "name";
const FAMILY: &str = "index_family";
const SAMPLE_SIZE: usize = 10;
const CHUNK_SIZE: usize = 64;
const SEED: [u8; 32] = [100; 32];

#[cfg(all(test, not(feature = "long_benchmarks")))]
const ITEM_COUNTS: [usize; 3] = [1_000, 10_000, 100_000];

#[cfg(all(test, feature = "long_benchmarks"))]
const ITEM_COUNTS: [usize; 4] = [1_000, 10_000, 100_000, 1_000_000];

fn generate_random_kv(len: usize) -> Vec<(u32, Vec<u8>)> {
    let mut key = 0;
    let kv_generator = |_| {
        let v = vec![0; CHUNK_SIZE];
        // Generate only unique keys.
        let k = key;
        key += 1;
        (k, v)
    };

    (0..len).map(kv_generator).collect()
}

fn plain_map_index_insert(b: &mut Bencher<'_>, len: usize) {
    let data = generate_random_kv(len);
    b.iter_with_setup(
        || (BenchDB::default(), data.clone()),
        |(db, data)| {
            let fork = db.fork();
            {
                let mut table = fork.get_map(NAME);
                for item in data {
                    table.put(&item.0, item.1);
                }
            }
            db.merge_sync(fork.into_patch()).unwrap();
        },
    );
}

fn plain_map_index_with_family_insert(b: &mut Bencher<'_>, len: usize) {
    let data = generate_random_kv(len);
    b.iter_with_setup(
        || (BenchDB::default(), data.clone()),
        |(db, data)| {
            let fork = db.fork();
            {
                let mut table = fork.get_map((NAME, FAMILY));
                for item in data {
                    table.put(&item.0, item.1);
                }
            }
            db.merge_sync(fork.into_patch()).unwrap();
        },
    );
}

fn plain_map_index_iter(b: &mut Bencher<'_>, len: usize) {
    let data = generate_random_kv(len);
    let db = BenchDB::default();
    let fork = db.fork();

    {
        let mut table = fork.get_map(NAME);
        assert!(table.keys().next().is_none());
        for item in data {
            table.put(&item.0, item.1);
        }
    }
    db.merge_sync(fork.into_patch()).unwrap();

    b.iter_with_setup(
        || db.snapshot(),
        |snapshot| {
            let index: MapIndex<_, u32, Vec<u8>> = snapshot.get_map(NAME);
            for (key, value) in &index {
                black_box(key);
                black_box(value);
            }
        },
    );
}

fn plain_map_index_with_family_iter(b: &mut Bencher<'_>, len: usize) {
    let data = generate_random_kv(len);
    let db = BenchDB::default();
    let fork = db.fork();

    {
        let mut table = fork.get_map((NAME, FAMILY));
        assert!(table.keys().next().is_none());
        for item in data {
            table.put(&item.0, item.1);
        }
    }
    db.merge(fork.into_patch()).unwrap();

    b.iter_with_setup(
        || db.snapshot(),
        |snapshot| {
            let index: MapIndex<_, u32, Vec<u8>> = snapshot.get_map((NAME, FAMILY));
            for (key, value) in &index {
                black_box(key);
                black_box(value);
            }
        },
    );
}

fn plain_map_index_read(b: &mut Bencher<'_>, len: usize) {
    let data = generate_random_kv(len);
    let db = BenchDB::default();
    let fork = db.fork();

    {
        let mut table = fork.get_map(NAME);
        assert!(table.keys().next().is_none());
        for item in data.clone() {
            table.put(&item.0, item.1);
        }
    }
    db.merge_sync(fork.into_patch()).unwrap();

    b.iter_with_setup(
        || db.snapshot(),
        |snapshot| {
            let index: MapIndex<_, u32, Vec<u8>> = snapshot.get_map(NAME);
            for item in &data {
                let value = index.get(&item.0);
                black_box(value);
            }
        },
    );
}

fn plain_map_index_with_family_read(b: &mut Bencher<'_>, len: usize) {
    let data = generate_random_kv(len);
    let db = BenchDB::default();
    let fork = db.fork();

    {
        let mut table = fork.get_map((NAME, FAMILY));
        assert!(table.keys().next().is_none());
        for item in data.clone() {
            table.put(&item.0, item.1);
        }
    }
    db.merge_sync(fork.into_patch()).unwrap();

    b.iter_with_setup(
        || db.snapshot(),
        |snapshot| {
            let index: MapIndex<_, u32, Vec<u8>> = snapshot.get_map((NAME, FAMILY));
            for item in &data {
                let value = index.get(&item.0);
                black_box(value);
            }
        },
    );
}

fn bench_fn<F>(c: &mut Criterion, name: &str, benchmark: F)
where
    F: Fn(&mut Bencher<'_>, usize) + 'static,
{
    let mut group = c.benchmark_group(name);
    for item_counts in ITEM_COUNTS.iter() {
        group
            .bench_with_input(
                BenchmarkId::from_parameter(item_counts),
                item_counts,
                |b: &mut Bencher<'_>, len: &usize| benchmark(b, *len),
            )
            .throughput(Throughput::Elements(*item_counts as u64))
            .plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic))
            .sample_size(SAMPLE_SIZE);
    }
    group.finish();
}

fn fill_list(list: &mut ListIndex<&Fork, Vec<u8>>, rng: &mut impl Rng) {
    for _ in 0..500 {
        let mut buffer = vec![0_u8; 512];
        rng.fill(&mut buffer[..]);
        list.push(buffer);
    }
}

fn bench_index_clearing(bencher: &mut Bencher<'_>) {
    let mut rng = StdRng::from_seed(SEED);

    let db = BenchDB::default();
    // Surround the cleared index with the indexes in the same column family.
    let fork = db.fork();
    for key in &[0_u8, 2] {
        fill_list(&mut fork.get_list(("list", key)), &mut rng);
    }
    db.merge(fork.into_patch()).unwrap();

    bencher.iter_batched(
        || {
            let addr = ("list", &1_u8);
            let fork = db.fork();
            fill_list(&mut fork.get_list(addr), &mut rng);
            db.merge(fork.into_patch()).unwrap();

            let fork = db.fork();
            fork.get_list::<_, Vec<u8>>(addr).clear();
            fork.into_patch()
        },
        |patch| db.merge(patch).unwrap(),
        BatchSize::SmallInput,
    );

    let snapshot = db.snapshot();
    for key in &[0_u8, 2] {
        let list = snapshot.get_list::<_, Vec<u8>>(("list", key));
        assert_eq!(list.iter().count(), 500);
    }
}

pub fn bench_storage(c: &mut Criterion) {
    // MapIndex
    bench_fn(c, "storage/plain_map/insert", plain_map_index_insert);
    bench_fn(c, "storage/plain_map/iter", plain_map_index_iter);
    bench_fn(
        c,
        "storage/plain_map_with_family/insert",
        plain_map_index_with_family_insert,
    );
    bench_fn(
        c,
        "storage/plain_map_with_family/iter",
        plain_map_index_with_family_iter,
    );
    bench_fn(c, "storage/plain_map/read", plain_map_index_read);
    bench_fn(
        c,
        "storage/plain_map_with_family/read",
        plain_map_index_with_family_read,
    );

    // Index clearing
    c.bench_function("storage/clearing", bench_index_clearing);
}
