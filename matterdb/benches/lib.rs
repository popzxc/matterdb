use criterion::{criterion_group, criterion_main};

use crate::benchmarks::{
    encoding::bench_encoding, schema_patterns::bench_schema_patterns, storage::bench_storage,
};

mod benchmarks;

criterion_group!(
    benches,
    bench_storage,
    bench_encoding,
    bench_schema_patterns
);
criterion_main!(benches);
