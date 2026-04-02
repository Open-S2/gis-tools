#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::geometry::{LonLat, S2CellId, S2Point, normalize_ll};
    use s2json::{MValue, VectorPoint};

    #[test]
    fn from_points() {
        let ll: LonLat = LonLat::from(&S2Point { x: 0.0, y: 0.0, z: 0.0 });
        assert_eq!(ll, LonLat::new(0.0, 0.0, None));
        let ll: LonLat = LonLat::from(&S2Point { x: 1.0, y: 0.0, z: 0.0 });
        assert_eq!(ll, LonLat::new(0.0, 0.0, None));
        let ll: LonLat = LonLat::from(&S2Point { x: 0.0, y: 1.0, z: 0.0 });
        assert_eq!(ll, LonLat::new(90.0, 0.0, None));
        let ll: LonLat = LonLat::from(&S2Point { x: 0.0, y: 0.0, z: -1.0 });
        assert_eq!(ll, LonLat::new(0.0, -90.0, None));
        let ll: LonLat = LonLat::from(&S2Point { x: 0.0, y: 0.0, z: 1.0 });
        assert_eq!(ll, LonLat::new(0.0, 90.0, None));
    }

    #[test]
    fn from_s2cell_id() {
        let ll: LonLat = LonLat::from(S2CellId::new(1152921504606846977));
        assert_eq!(ll, LonLat::new(0.0, 0.0, None));
    }

    #[test]
    fn coords() {
        let ll: LonLat = LonLat::new(20.0, 50.0, None);
        assert_eq!(ll.coords(), (20.0, 50.0));
    }

    #[test]
    fn take() {
        let mut ll: LonLat = LonLat::new(20.0, 50.0, None);
        let vp = ll.take();
        assert_eq!(vp, VectorPoint::new(20.0, 50.0, None, None));
    }

    #[test]
    fn get_distance() {
        let ll: LonLat = LonLat::new(0.0, 0.0, None);
        assert_eq!(ll.get_distance(&LonLat::<MValue>::new(0.0, 0.0, None)), 0.0);
        assert_eq!(
            ll.get_distance(&LonLat::<MValue>::new(0.017453292519943295, 0.0, None)),
            0.00030461741978670857
        );
    }

    #[test]
    fn normalize() {
        let mut ll: LonLat = LonLat::new(0.0, 0.0, None);
        ll.normalize();
        assert_eq!(ll, LonLat::new(0.0, 0.0, None));
        let mut ll: LonLat = LonLat::new(0.01745329251994, 0.111111, None);
        ll.normalize();
        assert_eq!(ll, LonLat::new(0.017453292519945762, 0.111111, None));
        let mut ll: LonLat = LonLat::new(640.0, 100.0, None);
        ll.normalize();
        assert_eq!(ll, LonLat::new(-80.0, 90.0, None));
        let mut ll: LonLat = LonLat::new(-640.0, -100.0, None);
        ll.normalize();
        assert_eq!(ll, LonLat::new(80.0, -90.0, None));
        let mut ll: LonLat = LonLat::new(-180.0, 0.0, None);
        ll.normalize();
        assert_eq!(ll, LonLat::new(180.0, 0.0, None));
        let mut ll: LonLat = LonLat::new(180.0, 0.0, None);
        ll.normalize();
        assert_eq!(ll, LonLat::new(180.0, 0.0, None));
        let mut ll: LonLat = LonLat::new(-179.99, 0.0, None);
        ll.normalize();
        assert_eq!(ll, LonLat::new(-179.99, 0.0, None));
    }

    #[test]
    fn normalize_ll_points() {
        let mut ll: VectorPoint<MValue> = VectorPoint::new_xy(0.0, 0.0, None);
        normalize_ll(&mut ll);
        assert_eq!(ll, VectorPoint::new_xy(0.0, 0.0, None));
        let mut ll: VectorPoint<MValue> = VectorPoint::new_xy(0.01745329251994, 0.111111, None);
        normalize_ll(&mut ll);
        assert_eq!(ll, VectorPoint::new_xy(0.017453292519945762, 0.111111, None));
        let mut ll: VectorPoint<MValue> = VectorPoint::new_xy(640.0, 100.0, None);
        normalize_ll(&mut ll);
        assert_eq!(ll, VectorPoint::new_xy(-80.0, 90.0, None));
        let mut ll: VectorPoint<MValue> = VectorPoint::new_xy(-640.0, -100.0, None);
        normalize_ll(&mut ll);
        assert_eq!(ll, VectorPoint::new_xy(80.0, -90.0, None));
        let mut ll: VectorPoint<MValue> = VectorPoint::new_xy(-180.0, 0.0, None);
        normalize_ll(&mut ll);
        assert_eq!(ll, VectorPoint::new_xy(180.0, 0.0, None));
        let mut ll: VectorPoint<MValue> = VectorPoint::new_xy(180.0, 0.0, None);
        normalize_ll(&mut ll);
        assert_eq!(ll, VectorPoint::new_xy(180.0, 0.0, None));
        let mut ll: VectorPoint<MValue> = VectorPoint::new_xy(-179.99, 0.0, None);
        normalize_ll(&mut ll);
        assert_eq!(ll, VectorPoint::new_xy(-179.99, 0.0, None));
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn to_angles() {
        let ll: LonLat = LonLat::new(0.0, 0.0, None);
        assert_eq!(ll.to_angles(), (0.0.into(), 0.0.into()));
        let ll: LonLat = LonLat::new(0.01745329251994, 0.111111, None);
        assert_eq!(ll.to_angles(), (0.00030461741978665105.into(), 0.0019392527851834196.into()));
        let ll: LonLat = LonLat::new(90.0, 180.0, None);
        assert_eq!(ll.to_angles(), (1.5707963267948966.into(), 3.141592653589793.into()));
    }

    #[test]
    fn to_point_and_gl() {
        let ll: LonLat = LonLat::new(0.0, 0.0, None);
        assert_eq!(S2Point::from(&ll), S2Point { x: 1.0, y: 0.0, z: 0.0 });
        let ll: LonLat = LonLat::new(90.0, 0.0, None);
        assert_eq!(ll.to_point(), S2Point { x: 6.123233995736766e-17, y: 1.0, z: 0.0 });
        let ll: LonLat = LonLat::new(0.0, 90.0, None);
        assert_eq!(ll.to_point(), S2Point { x: 6.123233995736766e-17, y: 0.0, z: 1.0 });
        let ll: LonLat = LonLat::new(180.0, 0., None);
        assert_eq!(ll.to_point(), S2Point { x: -1.0, y: 1.2246467991473532e-16, z: 0.0 });
        assert_eq!(ll.to_point_gl(), S2Point { x: 1.2246467991473532e-16, y: 0.0, z: -1.0 });
    }

    #[test]
    fn bearing() {
        let ll: LonLat = LonLat::new(0.0, 0.0, None);
        assert_eq!(ll.get_bearing(&LonLat::<MValue>::new(0.0, 0.0, None)), 0.0);
        assert_eq!(ll.get_bearing(&LonLat::<MValue>::new(90.0, 0.0, None)), 90.0);
        assert_eq!(ll.get_bearing(&LonLat::<MValue>::new(180.0, 0.0, None)), 90.0);
        assert_eq!(ll.get_bearing(&LonLat::<MValue>::new(0.0, 90.0, None)), 0.0);
        assert_eq!(ll.get_bearing(&LonLat::<MValue>::new(-89.9, 0.0, None)), 270.0);
        assert_eq!(ll.get_bearing(&LonLat::<MValue>::new(0.0, -90.0, None)), 180.0);
        assert_eq!(ll.get_bearing(&LonLat::<MValue>::new(-180.0, 0.0, None)), 270.0);
        let ll = LonLat::<MValue>::new(-60.0, -40.0, None);
        assert_eq!(ll.get_bearing(&LonLat::<MValue>::new(20.0, 10.0, None)), 75.936859467864);
    }

    #[test]
    fn maths() {
        // ADD
        let ll1: LonLat = LonLat::new(15.0, -20.0, None);
        let ll2: LonLat = LonLat::new(30.0, 40.0, None);
        let ll3: LonLat = ll1 + ll2;
        assert_eq!(ll3, LonLat::new(45.0, 20.0, None));
        // SUB
        let ll1: LonLat = LonLat::new(15.0, -20.0, None);
        let ll2: LonLat = LonLat::new(30.0, 40.0, None);
        let ll3: LonLat = ll1 - ll2;
        assert_eq!(ll3, LonLat::new(-15.0, -60.0, None));
        // MUL
        let ll1: LonLat = LonLat::new(15.0, -20.0, None);
        let ll2: LonLat = LonLat::new(30.0, 40.0, None);
        let ll3: LonLat = ll1 * ll2;
        assert_eq!(ll3, LonLat::new(450.0, -800.0, None));
        // DIV
        let ll1: LonLat = LonLat::new(15.0, -20.0, None);
        let ll2: LonLat = LonLat::new(30.0, 40.0, None);
        let ll3 = ll1 / ll2;
        assert_eq!(ll3, LonLat::new(0.5, -0.5, None));
        // NEG
        let ll1 = LonLat::new(15.0, -20.0, None);
        let ll2 = -ll1;
        assert_eq!(ll2, LonLat::new(-15.0, 20.0, None));
        // CMP
        let ll1 = LonLat::new(15.0, -20.0, None);
        let ll2 = LonLat::new(30.0, 40.0, None);
        assert!(ll1 < ll2);
        assert!(ll1 <= ll2);
        assert!(ll2 > ll1);
        assert!(ll2 >= ll1);

        let ll1 = LonLat::new(15.0, -20.0, None);
        let ll2 = LonLat::new(15.0, 40.0, None);
        assert!(ll1 < ll2);
        assert!(ll1 <= ll2);
        assert!(ll2 > ll1);
        assert!(ll2 >= ll1);

        let ll1: LonLat = LonLat::new(15.0, -20.0, None);
        let ll2 = LonLat::new(15.0, -20.0, None);
        assert!(ll1 == ll2);

        let ll1: LonLat = LonLat::new(15.0, f64::NAN, None);
        let ll2 = LonLat::new(15.0, f64::NAN, None);
        assert!(ll1 != ll2);
    }
}
