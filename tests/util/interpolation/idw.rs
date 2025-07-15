#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    extern crate alloc;

    use alloc::{vec, vec::Vec};
    use gistools::{
        parsers::RGBA,
        util::{VectorPointRGBA, default_get_interpolate_current_value, idw_interpolation},
    };
    use s2json::{MValue, VectorPoint};

    #[test]
    fn test_idw_interpolation() {
        let ref_data = vec![
            VectorPoint::<MValue>::new(0., 0., Some(1.), None),
            VectorPoint::new(1., 0., Some(2.), None),
            VectorPoint::new(0., 1., Some(3.), None),
            VectorPoint::new(1., 1., Some(4.), None),
        ];

        // test 1
        let point: VectorPoint = VectorPoint::new(0.5, 0.5, None, None);
        let result = idw_interpolation(&point, &ref_data, default_get_interpolate_current_value);
        assert_eq!(result, 1.5826612903225805);

        // test 2
        let point: VectorPoint = VectorPoint::new(0.65, 0.15, None, None);
        let result = idw_interpolation(&point, &ref_data, default_get_interpolate_current_value);
        assert_eq!(result, 1.5649491648500804);

        // test 3
        let ref_data: Vec<VectorPoint> = vec![];
        let result = idw_interpolation(&point, &ref_data, default_get_interpolate_current_value);
        assert_eq!(result, 0.);
    }

    #[test]
    fn test_idw_interpolation_rgba() {
        let ref_data = vec![
            VectorPointRGBA::new(0., 0., None, Some(RGBA::from_u8s(20, 20, 60, 255))),
            VectorPoint::new(1., 0., None, Some(RGBA::from_u8s(30, 100, 60, 255))),
            VectorPoint::new(0., 1., None, Some(RGBA::from_u8s(127, 127, 60, 255))),
            VectorPoint::new(1., 1., None, Some(RGBA::from_u8s(255, 255, 60, 255))),
        ];

        // test 1
        let point: VectorPoint = VectorPoint::new(0.5, 0.5, None, None);
        let result = idw_interpolation(&point, &ref_data, |p| p.m.unwrap());
        assert_eq!(result.to_u8s(), (84, 107, 60, 255));

        // test 2
        let point: VectorPoint = VectorPoint::new(0.65, 0.15, None, None);
        let result = idw_interpolation(&point, &ref_data, |p| p.m.unwrap());
        assert_eq!(result.to_u8s(), (46, 92, 60, 255));

        // test 3
        let ref_data: Vec<VectorPoint<RGBA>> = vec![];
        let result = idw_interpolation(&point, &ref_data, |p| p.m.unwrap());
        assert_eq!(result.to_u8s(), (0, 0, 0, 255));
    }

    #[test]
    fn test_idw_interpolation_m_rgba() {
        let ref_data: vec::Vec<VectorPoint> = vec![
            VectorPoint::new(
                0.,
                0.,
                None,
                Some(MValue::from([
                    ("r".into(), 20_u64.into()),
                    ("g".into(), 20_u64.into()),
                    ("b".into(), 60_u64.into()),
                    ("a".into(), 255_u64.into()),
                ])),
            ),
            VectorPoint::new(
                1.,
                0.,
                None,
                Some(MValue::from([
                    ("r".into(), 30_u64.into()),
                    ("g".into(), 100_u64.into()),
                    ("b".into(), 60_u64.into()),
                    ("a".into(), 255_u64.into()),
                ])),
            ),
            VectorPoint::new(
                0.,
                1.,
                None,
                Some(MValue::from([
                    ("r".into(), 127_u64.into()),
                    ("g".into(), 127_u64.into()),
                    ("b".into(), 60_u64.into()),
                    ("a".into(), 255_u64.into()),
                ])),
            ),
            VectorPoint::new(
                1.,
                1.,
                None,
                Some(MValue::from([
                    ("r".into(), 255_u64.into()),
                    ("g".into(), 255_u64.into()),
                    ("b".into(), 60_u64.into()),
                    ("a".into(), 255_u64.into()),
                ])),
            ),
        ];

        // test 1
        let point: VectorPoint = VectorPoint::new(0.5, 0.5, None, None);
        let result = idw_interpolation(&point, &ref_data, |p| RGBA::from(p.m.clone().unwrap()));
        assert_eq!(result.to_u8s(), (84, 107, 60, 255));

        // test 2
        let point: VectorPoint = VectorPoint::new(0.65, 0.15, None, None);
        let result = idw_interpolation(&point, &ref_data, |p| RGBA::from(p.m.clone().unwrap()));
        assert_eq!(result.to_u8s(), (46, 92, 60, 255));

        // test 3
        let ref_data: Vec<VectorPoint<RGBA>> = vec![];
        let result = idw_interpolation(&point, &ref_data, |p| p.m.unwrap());
        assert_eq!(result.to_u8s(), (0, 0, 0, 255));
    }
}
