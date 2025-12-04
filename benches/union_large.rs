use std::{fs, vec};
// use criterion::{black_box, criterion_group, criterion_main, Criterion};
use criterion::{Criterion, criterion_group, criterion_main};
use gistools::geometry::{clean_polygons, convert, polygons_union};
use s2json::{JSONCollection, Point, Projection, VectorGeometry};

fn setup_data() -> Vec<Vec<Vec<Point>>> {
    let data =
        fs::read_to_string("tests/geometry/tools/fixtures/chunks-water/args.geojson").unwrap();
    let fc: JSONCollection = serde_json::from_str(&data).unwrap();
    let vector_features = convert(Projection::WG, &fc, Some(true), Some(false));

    let mut res: Vec<Vec<Vec<Point>>> = vec![];

    for feature in vector_features {
        match &feature.geometry {
            VectorGeometry::Polygon(p) => res.push(
                p.coordinates
                    .iter()
                    .map(|l| l.iter().map(Into::into).collect::<Vec<Point>>())
                    .collect::<Vec<Vec<Point>>>(),
            ),
            VectorGeometry::MultiPolygon(p) => res.push(
                p.coordinates
                    .iter()
                    .flat_map(|l| {
                        l.iter()
                            .map(|ll| ll.iter().map(Into::into).collect::<Vec<Point>>())
                            .collect::<Vec<Vec<Point>>>()
                    })
                    .collect::<Vec<Vec<Point>>>(),
            ),
            _ => {}
        }
    }

    let (cleaned_data, _) = clean_polygons(&res, false, false).unwrap();

    cleaned_data
}

fn polygons_union_large_test(input: &Vec<Vec<Vec<Point>>>) {
    polygons_union(input);
}

fn criterion_benchmark(c: &mut Criterion) {
    // Run async setup once, outside the measurement
    let input = setup_data();

    let mut group = c.benchmark_group("polygons_union_large_test");
    group.sample_size(10);

    group.bench_with_input("union_large", &input, |b, data| {
        b.iter(|| {
            polygons_union_large_test(data);
        })
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
