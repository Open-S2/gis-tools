#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::proj::{Coords, DatumParams, DatumType, Proj, datum_transform};

    #[test]
    fn test_datum_type_is_params() {
        assert!(DatumType::Param3.is_params());
        assert!(DatumType::Param7.is_params());
        assert!(!DatumType::GridShift.is_params());
        assert!(!DatumType::WGS84.is_params());
        assert!(!DatumType::NoDatum.is_params());
    }

    #[test]
    fn test_datum_type_is_wgs84() {
        assert!(DatumType::WGS84.is_wgs84(&DatumParams::default()));
        assert!(DatumType::Param3.is_wgs84(&DatumParams::Param3(0.0, 0.0, 0.0)));
        assert!(!DatumType::Param3.is_wgs84(&DatumParams::Param3(1.0, 0.0, 0.0)));
    }

    #[test]
    fn test_datum_params_vec_conversion() {
        let p3 = DatumParams::Param3(1.0, 2.0, 3.0);
        assert_eq!(DatumParams::from_vec(p3.to_vec()), p3);

        let p7 = DatumParams::Param7(1.0, 2.0, 3.0, 0.1, 0.2, 0.3, 1.0);
        assert_eq!(DatumParams::from_vec(p7.to_vec()), p7);
    }

    #[test]
    fn test_datum_params_is_wgs84() {
        assert!(DatumParams::Param3(0.0, 0.0, 0.0).is_wgs84());
        assert!(DatumParams::Param7(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0).is_wgs84());
        assert!(!DatumParams::Param3(1.0, 0.0, 0.0).is_wgs84());
    }

    #[test]
    fn test_to_wgs84_param3() {
        let mut c = Coords::new(1.0, 2.0, 3.0, 0.0);
        let d = DatumParams::Param3(10.0, 20.0, 30.0);
        d.to_wgs84(&mut c);
        assert_eq!(c, Coords::new(11.0, 22.0, -27.0, 0.0));
    }

    #[test]
    fn test_from_wgs84_param3() {
        let mut c = Coords::new(1.0, 2.0, 3.0, 0.0);
        let d = DatumParams::Param3(10.0, 20.0, 30.0);
        d.from_wgs84(&mut c);
        assert_eq!(c, Coords::new(-9.0, -18.0, -27.0, 0.0));
    }

    #[test]
    fn test_to_from_wgs84_param7_inverse() {
        let params = DatumParams::Param7(1.0, 2.0, 3.0, 0.001, 0.002, 0.003, 1.0);
        let orig = Coords::new(10.0, 20.0, 30.0, 0.0);
        let mut c = orig;

        params.to_wgs84(&mut c);
        params.from_wgs84(&mut c);

        assert!((c.0 - orig.0).abs() < 1e-1);
        assert!((c.1 - orig.1).abs() < 1e-1);
        assert!((c.2 - orig.2).abs() < 1e-1);
    }

    #[test]
    fn test_datum_transform_noop_if_same_type() {
        let p = Coords::new(10.0, 20.0, 30.0, 0.0);
        let mut c = p;

        let proj = Proj { datum_type: DatumType::NoDatum, ..Default::default() };
        datum_transform(&mut c, &proj, &proj);
        assert_eq!(c, p);
    }

    #[test]
    fn test_datum_transform_applies_param3() {
        let source = Proj {
            a: 6378137.0,
            b: 6356752.3,
            es: 0.00669437999014,
            datum_type: DatumType::Param3,
            datum_params: DatumParams::Param3(1.0, 2.0, 3.0),
            ..Default::default()
        };

        let dest = Proj {
            a: 6378137.0,
            b: 6356752.3,
            es: 0.00669437999014,
            datum_type: DatumType::Param3,
            datum_params: DatumParams::Param3(-1.0, -2.0, -3.0),
            ..Default::default()
        };

        let mut pt = Coords::new(1.0, 0.5, 3.0, 0.0);

        datum_transform(&mut pt, &source, &dest);
        assert_eq!(
            pt,
            Coords::new(1.0000000853795554, 0.49999966429485654, 6.902161192148924, 0.0)
        );
    }
}
