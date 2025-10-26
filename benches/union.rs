use std::vec;
// use criterion::{black_box, criterion_group, criterion_main, Criterion};
use criterion::{Criterion, criterion_group, criterion_main};
use gistools::geometry::polygons_union;
use s2json::{NewXY, Point};

const COUNT: usize = 1_000_000;

fn polygons_union_test() {
    let polygon_a = vec![vec![
        Point::new_xy(-36.843736697711705, 26.902507073493283),
        Point::new_xy(-38.77733044771159, -10.660574687279677),
        Point::new_xy(-9.246080447711194, 4.565507293900282),
        Point::new_xy(-24.890611697711705, 10.141965007796856),
        Point::new_xy(-8.36717419771125, 17.476464485265197),
        Point::new_xy(-36.843736697711705, 26.902507073493283),
    ]];
    let polygon_b = vec![vec![
        Point::new_xy(25.91016955228889, 25.48298173273801),
        Point::new_xy(-19.617174197711336, 17.81148831664035),
        Point::new_xy(-8.191392947711279, 12.382961401589753),
        Point::new_xy(-17.33201794771142, 4.3902626777368),
        Point::new_xy(30.304700802288608, -11.523054338551972),
        Point::new_xy(25.91016955228889, 25.48298173273801),
    ]];
    let both = vec![polygon_a, polygon_b];

    for _ in 0..COUNT {
        polygons_union(&both);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("polygons_union_test");
    group.sample_size(10);
    group.bench_function("union", |b| b.iter(polygons_union_test));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
