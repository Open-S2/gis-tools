// use criterion::{black_box, criterion_group, criterion_main, Criterion};
use criterion::{Criterion, criterion_group, criterion_main};
use gistools::data_store::{FileOptions, S2FileStore};
use libm::{ceil, floor};
use rand::random;
use serde::{Deserialize, Serialize};
// use tempfile::NamedTempFile;

const COUNT: usize = 1_000_000;

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
struct Value {
    val: u64,
}

fn test_store() {
    // let temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    // let file_path = temp_file.path().to_string_lossy().into_owned();
    let mut store = S2FileStore::<u64, Value>::new(
        None,
        Some(FileOptions { thread_count: Some(6), ..Default::default() }),
    );

    for _ in 0..COUNT {
        let x = get_random_int(0., COUNT as f64) as u64;
        store.set(x, Value { val: x });
    }
    // ensure at least ONE 22
    store.set(22, Value { val: 22 });

    // try getting the value (force a sort)
    let _ = store.get(22, None);
    store.cleanup();
}

fn get_random_int(a: f64, b: f64) -> f64 {
    if a > b {
        panic!("The first argument must be less than or equal to the second argument.");
    }

    let min = ceil(a);
    let max = floor(b);
    floor(random::<f64>() * (max - min + 1.)) + min
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("test_store");
    group.sample_size(10);
    group.bench_function("test store", |b| b.iter(test_store));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
