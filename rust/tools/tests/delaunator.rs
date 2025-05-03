#[cfg(test)]
// #[coverage(off)]
mod tests {
    extern crate alloc;
    extern crate std;

    use alloc::format;
    use libm::{fabs, pow};
    use s2json::{Point, VectorPoint};
    use std::{fs, path::PathBuf};
    use tools::{Delaunator, NO_REF};

    #[test]
    fn ukraine() {
        let mut points = get_points("ukraine");
        let mut del = Delaunator::from_points(&points);
        validate(&points, &del);

        assert_eq!(del.triangles.len(), 5133);

        // reusing the Delaunator
        del.coords[0] = 80.;
        del.coords[1] = 220.;
        points[0] = Point(80., 220.);
        del.update();
        validate(&points, &del);
        assert_eq!(del.triangles.len(), 5139);
    }

    #[test]
    fn issue_11() {
        let points = vec![
            Point(516., 661.),
            Point(369., 793.),
            Point(426., 539.),
            Point(273., 525.),
            Point(204., 694.),
            Point(747., 750.),
            Point(454., 390.),
        ];
        let del = Delaunator::from_points(&points);
        validate(&points, &del);
    }

    #[test]
    fn issue_13() {
        let points = get_points("issue13");
        let del = Delaunator::from_points(&points);
        validate(&points, &del);
    }

    #[test]
    fn issue_24() {
        let points = vec![
            Point(382., 302.),
            Point(382., 328.),
            Point(382., 205.),
            Point(623., 175.),
            Point(382., 188.),
            Point(382., 284.),
            Point(623., 87.),
            Point(623., 341.),
            Point(141., 227.),
        ];
        let del = Delaunator::from_points(&points);
        validate(&points, &del);
    }

    #[test]
    fn issue_43() {
        let points = get_points("issue43");
        let del = Delaunator::from_points(&points);
        validate(&points, &del);
    }

    #[test]
    fn issue_44() {
        let points = get_points("issue44");
        let del = Delaunator::from_points(&points);
        validate(&points, &del);
    }

    #[test]
    fn robust_1() {
        let points = get_points("robustness1");
        let del = Delaunator::from_points(&points);
        validate(&points, &del);
    }

    #[test]
    fn robust_1_e9() {
        let mut points = get_points("robustness1");
        points = points.into_iter().map(|p| Point(p.0 / 1e9, p.1 / 1e9)).collect();
        let del = Delaunator::from_points(&points);
        validate(&points, &del);
    }

    #[test]
    fn robust_1_e9_mult() {
        let mut points = get_points("robustness1");
        points = points.into_iter().map(|p| Point(p.0 * 1e9, p.1 * 1e9)).collect();
        let del = Delaunator::from_points(&points);
        validate(&points, &del);
    }

    #[test]
    fn robust_1_100() {
        let mut points = get_points("robustness1");
        points = points.into_iter().map(|p| Point(p.0 / 100., p.1 / 100.)).collect();
        let del = Delaunator::from_points(&points);
        validate(&points, &del);
    }

    #[test]
    fn robust_1_100_mult() {
        let mut points = get_points("robustness1");
        points = points.into_iter().map(|p| Point(p.0 * 100., p.1 * 100.)).collect();
        let del = Delaunator::from_points(&points);
        validate(&points, &del);
    }

    #[test]
    fn robust_2() {
        let points = get_points("robustness2");
        let del = Delaunator::from_points(&points);
        validate(&points, &del);
    }

    #[test]
    fn robust_3() {
        let points = get_points("robustness3");
        let del = Delaunator::from_points(&points);
        validate(&points, &del);
    }

    #[test]
    fn robust_4() {
        let points = get_points("robustness4");
        let del = Delaunator::from_points(&points);
        validate(&points, &del);
    }

    #[test]
    fn returns_empty_triangulation_for_small_number_of_points() {
        let d = Delaunator::from_points(&[]);
        assert_eq!(d.triangles, vec![] as Vec<usize>);
        assert_eq!(d.hull, vec![] as Vec<usize>);
        let d = Delaunator::from_points(&[Point(168., 180.)]);
        assert_eq!(d.triangles, vec![] as Vec<usize>);
        assert_eq!(d.hull, vec![0]);
        // TODO:
        // let d = Delaunator::from_points(&[Point(168., 180.), Point(168., 178.)]);
        // assert_eq!(d.triangles, vec![] as Vec<usize>);
        // assert_eq!(d.hull, vec![1, 0]); // [0, 1] is also correct
    }

    #[test]
    fn returns_empty_triangulation_for_all_collinear_input() {
        let d =
            Delaunator::from_points(&[Point(0., 0.), Point(1., 0.), Point(3., 0.), Point(2., 0.)]);
        assert_eq!(d.triangles, vec![] as Vec<usize>);
        assert_eq!(d.hull, vec![0, 1, 3, 2]); // [2, 3, 0, 1] is also correct
    }

    #[test]
    fn supports_custom_point_format() {
        let d = Delaunator::from_vector_points(&[
            VectorPoint::<()>::new_xy(5., 5., None),
            VectorPoint::new_xy(7., 5., None),
            VectorPoint::new_xy(7., 6., None),
        ]);
        assert_eq!(d.triangles, vec![0, 2, 1]);
    }

    fn orient(p: Point, r: Point, q: Point) -> f64 {
        let Point(px, py) = p;
        let Point(rx, ry) = r;
        let Point(qx, qy) = q;
        let l = (ry - py) * (qx - px);
        let r = (rx - px) * (qy - py);
        if fabs(l - r) >= 3.3306690738754716e-16 * fabs(l + r) { l - r } else { 0. }
    }

    fn convex(r: Point, q: Point, p: Point) -> bool {
        orient(p, r, q) >= 0. || orient(r, q, p) >= 0. || orient(q, p, r) >= 0.
    }

    /// validate the Delaunay triangulation
    fn validate(points: &[Point], d: &Delaunator) {
        // validate halfedges
        for i in 0..d.halfedges.len() {
            assert!(
                d.halfedges[i] == NO_REF || d.halfedges[d.halfedges[i]] == i,
                "valid halfedge connection"
            );
        }

        // validate triangulation
        let mut hull_areas: Vec<f64> = vec![];
        let len = d.hull.len();
        let mut i = 0;
        let mut j = len.saturating_sub(1); // Prevent underflow if len == 0

        while i < len {
            let Point(x0, y0) = points[d.hull[j]];
            let Point(x, y) = points[d.hull[i]];
            hull_areas.push((x - x0) * (y + y0));
            assert!(
                convex(
                    points[d.hull[j]],
                    points[d.hull[(j + 1) % len]],
                    points[d.hull[(j + 3) % len]],
                ),
                "{}",
                format!("hull should be convex at {j}"),
            );

            j = i;
            i += 1;
        }
        let hull_area = sum(&hull_areas);

        let mut triangle_areas: Vec<f64> = vec![];
        for i in (0..d.triangles.len()).step_by(3) {
            let Point(ax, ay) = points[d.triangles[i]];
            let Point(bx, by) = points[d.triangles[i + 1]];
            let Point(cx, cy) = points[d.triangles[i + 2]];
            triangle_areas.push(fabs((by - ay) * (cx - bx) - (bx - ax) * (cy - by)));
        }
        let triangles_area = sum(&triangle_areas);

        let err = fabs((hull_area - triangles_area) / hull_area);
        assert!(err <= pow(2., -51.), "{}", format!("triangulation should be valid; {err} error"));
    }

    /// Kahan and Babuska summation, Neumaier variant; accumulates less FP error
    fn sum(x: &[f64]) -> f64 {
        let mut sum = x[0];
        let mut err = 0.;
        for i in x.iter().skip(1) {
            let k = *i;
            let m = sum + k;
            err += if fabs(sum) >= fabs(k) { sum - m + k } else { k - m + sum };
            sum = m;
        }
        sum + err
    }

    fn get_points(name: &str) -> Vec<Point> {
        let mut path = PathBuf::from(env!("CARGO_WORKSPACE_DIR"));
        path.push(format!("tests/tools/fixtures/{name}.json"));
        let file_as_str = fs::read_to_string(path).unwrap();
        serde_json::from_str(&file_as_str).unwrap()
    }
}
