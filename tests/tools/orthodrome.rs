#[cfg(test)]
// #[coverage(off)]
mod tests {
    use gistools::{geometry::LonLat, tools::Orthodrome};
    use s2json::{MValue, VectorPoint};

    #[test]
    fn orthodrome() {
        let ortho = Orthodrome::new(0., 0., 0., 0.);
        assert_eq!(ortho.a, 0.);
        assert_eq!(ortho.dist, 0.);

        let ortho = Orthodrome::default();
        assert_eq!(ortho.a, 0.);
        assert_eq!(ortho.dist, 0.);
    }

    #[test]
    fn from_points() {
        let ortho = Orthodrome::from_points(&LonLat::new(0., 0., None), &LonLat::new(0., 0., None));
        assert_eq!(ortho.a, 0.);
        assert_eq!(ortho.dist, 0.);
    }

    #[test]
    fn from_vector_points() {
        let ortho = Orthodrome::from_vector_points(
            &VectorPoint::<()>::new_xy(0., 0., None),
            &VectorPoint::<MValue>::new_xy(0., 0., None),
        );
        assert_eq!(ortho.a, 0.);
        assert_eq!(ortho.dist, 0.);
    }

    #[test]
    fn intermediate_point_same() {
        let ortho = Orthodrome::new(0., 0., 0., 0.);
        assert_eq!(ortho.intermediate_point(0.5), LonLat::new(0., 0., None));
    }

    #[test]
    fn intermediate_point_far() {
        let ortho = Orthodrome::new(-60., -40., 20., 10.);
        assert_eq!(ortho.intermediate_point(0.), LonLat::new(-59.99999999999999, -40., None));
        assert_eq!(
            ortho.intermediate_point(0.2),
            LonLat::new(-39.13793657428956, -33.728521975616516, None)
        );
        assert_eq!(
            ortho.intermediate_point(0.4),
            LonLat::new(-21.692497560895635, -24.50037918247324, None)
        );
        assert_eq!(
            ortho.intermediate_point(0.6),
            LonLat::new(-6.830669211476937, -13.564157442008685, None)
        );
        assert_eq!(
            ortho.intermediate_point(0.8),
            LonLat::new(6.673353815433631, -1.8320330896428323, None)
        );
        assert_eq!(ortho.intermediate_point(1.), LonLat::new(20., 10., None));
    }

    #[test]
    fn distance_to_same() {
        let ortho = Orthodrome::new(0., 0., 0., 0.);
        assert_eq!(ortho.distance_to(), 0.);
    }

    #[test]
    fn distance_to_far() {
        let ortho = Orthodrome::new(-60., -40., 20., 10.);
        assert_eq!(ortho.distance_to(), 1.5514126949321814);
    }

    #[test]
    fn bearing() {
        let ortho = Orthodrome::new(-60., -40., 20., 10.);
        assert_eq!(ortho.bearing(), 75.936859467864);
    }
}
