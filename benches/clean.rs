use std::vec;
// use criterion::{black_box, criterion_group, criterion_main, Criterion};
use criterion::{Criterion, criterion_group, criterion_main};
use gistools::geometry::clean_polygon;
use s2json::{NewXY, Point};

const COUNT: usize = 1_000_000;

fn clean_polygon_test() {
    let polygon = vec![vec![
        Point::new_xy(8.094854051549703, 44.067038922182604),
        Point::new_xy(27.45169791493106, 34.31013538862004),
        Point::new_xy(31.238906496896703, 25.572928139998595),
        Point::new_xy(26.610096007827508, 22.88716015007573),
        Point::new_xy(25.978894577499233, 18.957601207155236),
        Point::new_xy(32.08050840400031, 17.157354229920827),
        Point::new_xy(38.8133236608289, 20.541732106259843),
        Point::new_xy(40.496527475035236, 28.199781765371043),
        Point::new_xy(7.463652621221485, 25.00221485407819),
        Point::new_xy(25.347693147171753, 4.999693002409302),
        Point::new_xy(-7.4747812298659255, -36.777396059815665),
        Point::new_xy(27.662098391706394, -40.233822107102995),
        Point::new_xy(28.92450125236215, -14.406933337995738),
        Point::new_xy(4.097244992807987, -34.38206769619466),
        Point::new_xy(62.79897801327945, -31.19907851930298),
        Point::new_xy(86.57423188895399, 16.55327251195662),
        Point::new_xy(54.38295894224376, 12.685928855764459),
        Point::new_xy(73.73980280562509, -3.197906810124664),
        Point::new_xy(81.52462044633336, 36.369487623534425),
        Point::new_xy(54.80375989579596, 56.70904723358515),
        Point::new_xy(8.094854051549703, 44.067038922182604),
    ]];

    for _ in 0..COUNT {
        clean_polygon(&polygon, false);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("clean_polygon_test");
    group.sample_size(10);
    group.bench_function("clean", |b| b.iter(clean_polygon_test));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
