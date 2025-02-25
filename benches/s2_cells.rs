// use criterion::{black_box, criterion_group, criterion_main, Criterion};
use criterion::{criterion_group, criterion_main, Criterion};

use gistools::geometry::{LonLat, S2CellId};

use libm::{ceil, floor};
use rand::random;

const COUNT: usize = 1_000_000;

fn build_cells() {
    let mut cells: Vec<S2CellId> = vec![];

    for _ in 0..COUNT {
        let mut ll = LonLat::new(get_random_int(-180.0, 180.0), get_random_int(-90.0, 90.0), None);
        ll.normalize();
        cells.push(ll.into());
    }
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
    let mut group = c.benchmark_group("build_cells");
    group.sample_size(10);
    group.bench_function("build cells", |b| b.iter(build_cells));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
