#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    extern crate alloc;
    extern crate std;

    use alloc::vec;
    use gistools::tools::{PolyLabelMetadata, polylabel, polylabels};
    use s2json::{Polygon, VectorLineString, VectorPoint, VectorPolygon};
    use std::{fs, path::PathBuf};

    #[test]
    fn empty() {
        let data: Vec<Vec<VectorPoint<()>>> = vec![vec![]];
        let emp = polylabel(&data, None);

        assert_eq!(emp, VectorPoint::new_xy(0., 0., Some(PolyLabelMetadata::new(0.))));
    }

    #[test]
    fn works_on_degenerate_polygons() {
        let p1 = polylabel(
            &vec![vec![
                VectorPoint::<()>::new_xy(0., 0., None),
                VectorPoint::new_xy(1., 0., None),
                VectorPoint::new_xy(2., 0., None),
                VectorPoint::new_xy(0., 0., None),
            ]],
            None,
        );
        assert_eq!(p1, VectorPoint::new_xy(0., 0., Some(PolyLabelMetadata::new(0.))));

        let p2 = polylabel(
            &vec![vec![
                VectorPoint::<()>::new_xy(0., 0., None),
                VectorPoint::new_xy(1., 0., None),
                VectorPoint::new_xy(1., 1., None),
                VectorPoint::new_xy(1., 0., None),
                VectorPoint::new_xy(0., 0., None),
            ]],
            None,
        );
        assert_eq!(p2, VectorPoint::new_xy(0., 0., Some(PolyLabelMetadata::new(0.))));

        let p3 = polylabel(
            &vec![vec![
                VectorPoint::<()>::new_xy(0., 0., None),
                VectorPoint::new_xy(0., 0., None),
                VectorPoint::new_xy(0., 0., None),
                VectorPoint::new_xy(0., 0., None),
                VectorPoint::new_xy(0., 0., None),
            ]],
            None,
        );
        assert_eq!(p3, VectorPoint::new_xy(0., 0., Some(PolyLabelMetadata::new(0.))));
    }

    #[test]
    fn water1_pole_of_inaccess_precision_1() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/tools/fixtures/water1.json");
        let file_as_str = fs::read_to_string(path).unwrap();
        let water_1: Polygon = serde_json::from_str(&file_as_str).unwrap();

        let vector_water_1: VectorPolygon = convert_poly(&water_1);
        let polylabel_high_precision = polylabel(&vector_water_1, Some(1.));
        assert_eq!(
            polylabel_high_precision,
            VectorPoint::new_xy(
                3865.85009765625,
                2124.87841796875,
                Some(PolyLabelMetadata::new(288.8493574779127)),
            )
        );

        let polylabel_low_precision = polylabel(&vector_water_1, Some(50.));
        assert_eq!(
            polylabel_low_precision,
            VectorPoint::new_xy(
                3854.296875,
                2123.828125,
                Some(PolyLabelMetadata::new(278.5795872381558)),
            )
        );
    }

    #[test]
    fn water1_pole_of_inaccess_precision_1_multi() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/tools/fixtures/water1.json");
        let file_as_str = fs::read_to_string(path).unwrap();
        let water_1: Polygon = serde_json::from_str(&file_as_str).unwrap();

        let vector_water_1: VectorPolygon = convert_poly(&water_1);
        let polylabel_high_precision = polylabels(&vec![vector_water_1.clone()], Some(1.));
        assert_eq!(
            polylabel_high_precision,
            vec![VectorPoint::new_xy(
                3865.85009765625,
                2124.87841796875,
                Some(PolyLabelMetadata::new(288.8493574779127)),
            )]
        );

        let polylabel_low_precision = polylabels(&vec![vector_water_1], Some(50.));
        assert_eq!(
            polylabel_low_precision,
            vec![VectorPoint::new_xy(
                3854.296875,
                2123.828125,
                Some(PolyLabelMetadata::new(278.5795872381558)),
            )]
        );
    }

    #[test]
    fn water2_pole_of_inaccess_precision() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/tools/fixtures/water2.json");
        let file_as_str = fs::read_to_string(path).unwrap();
        let water_2: Polygon = serde_json::from_str(&file_as_str).unwrap();

        let vector_water_2: VectorPolygon = convert_poly(&water_2);
        let polylabel_high_precision = polylabel(&vector_water_2, Some(1.));
        assert_eq!(
            polylabel_high_precision,
            VectorPoint::new_xy(3263.5, 3263.5, Some(PolyLabelMetadata::new(960.5)),)
        );
    }

    fn convert_poly<M: Clone>(input: &Polygon) -> VectorPolygon<M> {
        let mut res: VectorPolygon<M> = vec![];
        for ring in input {
            let mut new_ring: VectorLineString<M> = vec![];
            for point in ring {
                new_ring.push(VectorPoint::new_xy(point.0, point.1, None));
            }
            res.push(new_ring);
        }

        res
    }
}
