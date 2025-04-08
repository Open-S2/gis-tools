use core::iter::repeat_with;
use criterion::{
    criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion, PlotConfiguration,
};
use gistools::tools::Delaunator;
use rand::{rngs::StdRng, Rng, SeedableRng};
use s2json::Point;

const COUNTS: &[usize] = &[100, 1000, 10_000, 100_000, 1_000_000];

fn bench(c: &mut Criterion) {
    let mut rng: StdRng = StdRng::seed_from_u64(123);

    let all_points: Vec<_> =
        repeat_with(|| rng.gen()).map(|(x, y)| Point(x, y)).take(*COUNTS.last().unwrap()).collect();

    let mut group = c.benchmark_group("delaunator");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for count in COUNTS {
        group.bench_with_input(BenchmarkId::from_parameter(count), count, |b, &count| {
            let points = &all_points[..count];
            b.iter(|| Delaunator::from_points(points))
        });
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
