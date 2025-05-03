#[cfg(test)]
// #[coverage(off)]
mod tests {
    extern crate std;

    use gistools::geometry::{S1Angle, S1ChordAngle};
    use std::ops::Neg;

    #[test]
    fn new() {
        let angle = S1ChordAngle::new(1.0);
        assert_eq!(angle.length2, 1.0);
        let inf_angle = S1ChordAngle::infinity();
        assert_eq!(inf_angle.length2, f64::INFINITY);

        let neg_val = S1ChordAngle::from_angle(S1Angle::new(-2.));
        assert_eq!(neg_val.length2, -1.0);

        let neg_inf_val = S1ChordAngle::from_angle(S1Angle::infinity());
        assert_eq!(neg_inf_val.length2, f64::INFINITY);
    }

    #[test]
    fn fast_upper_bound_from() {
        assert_eq!(S1ChordAngle::fast_upper_bound_from(0.0.into()), 0.0);
        assert_eq!(S1ChordAngle::fast_upper_bound_from(1.0.into()), 1.0);
        assert_eq!(S1ChordAngle::fast_upper_bound_from(2.0.into()), 4.0);
        assert_eq!(S1ChordAngle::fast_upper_bound_from(3.0.into()), 4.0);
    }

    #[test]
    fn is_special() {
        assert!(!S1ChordAngle::new(0.0).is_special());
        assert!(S1ChordAngle::new(-2.0).is_special());
        assert!(S1ChordAngle::infinity().is_special());
    }

    #[test]
    fn to_angle() {
        assert_eq!(S1ChordAngle::new(0.0).to_angle(), S1Angle::new(0.0));
        assert_eq!(S1ChordAngle::new(0.9193953882637206).to_angle(), S1Angle::new(1.0));
        assert_eq!(S1ChordAngle::new(2.8322936730942847).to_angle(), S1Angle::new(2.0));
        assert_eq!(
            S1ChordAngle::new(3.979984993200891).to_angle(),
            S1Angle::new(3.0000000000000004)
        );
        assert_eq!(S1ChordAngle::new(4.0).to_angle(), S1Angle::new(std::f64::consts::PI));

        assert_eq!(S1ChordAngle::new(-20.0).to_angle(), S1Angle::from(-1.0));
        assert_eq!(S1ChordAngle::infinity().to_angle(), S1Angle::infinity());

        let ca = S1ChordAngle::new(4.0);
        let an: S1Angle = ca.into();
        assert_eq!(an, S1Angle::new(std::f64::consts::PI));
    }

    #[test]
    fn to_meters() {
        assert_eq!(S1ChordAngle::new(0.0).to_meters(None), 0.0);
        assert_eq!(S1ChordAngle::new(0.9193953882637206).to_meters(None), 6371008.8);
    }

    #[test]
    fn to_km() {
        assert_eq!(S1ChordAngle::new(0.0).to_km(None), 0.0);
        assert_eq!(S1ChordAngle::new(0.9193953882637206).to_km(None), 6371.0088);
    }

    #[test]
    fn from_meters() {
        assert_eq!(S1ChordAngle::from_meters(0.0, None).length2, 0.0);
        assert_eq!(S1ChordAngle::from_meters(6371008.8, None).length2, 0.9193953882637206);
    }

    #[test]
    fn from_km() {
        assert_eq!(S1ChordAngle::from_km(0.0, None).length2, 0.0);
        assert_eq!(S1ChordAngle::from_km(6371.0088, None).length2, 0.9193953882637206);
    }

    #[test]
    fn chord_angle_tan() {
        assert_eq!(S1ChordAngle::new(0.0).chord_angle_tan(), 0.0);
        assert_eq!(S1ChordAngle::new(1.0).chord_angle_tan(), 1.7320508075688772);
        assert_eq!(S1ChordAngle::new(2.0).chord_angle_tan(), f64::INFINITY);
        assert_eq!(S1ChordAngle::new(3.0).chord_angle_tan(), -1.7320508075688772);
        assert_eq!(S1ChordAngle::new(4.0).chord_angle_tan(), -0.0);
        assert!(S1ChordAngle::new(5.0).chord_angle_tan().is_nan());
    }

    #[test]
    fn chord_angle_sin2() {
        assert_eq!(S1ChordAngle::new(0.0).chord_angle_sin2(), 0.0);
        assert_eq!(S1ChordAngle::new(1.0).chord_angle_sin2(), 0.75);
        assert_eq!(S1ChordAngle::new(2.0).chord_angle_sin2(), 1.0);
        assert_eq!(S1ChordAngle::new(3.0).chord_angle_sin2(), 0.75);
        assert_eq!(S1ChordAngle::new(4.0).chord_angle_sin2(), 0.0);
        assert_eq!(S1ChordAngle::new(5.0).chord_angle_sin2(), -1.25);
    }

    #[test]
    #[allow(clippy::approx_constant)]
    pub fn maths() {
        let s1 = S1ChordAngle::from_degrees(180.);
        let s2 = S1ChordAngle::from_degrees(90.);
        assert_eq!(s1 + s2, S1ChordAngle::new(6.0));
        assert_eq!(s1 + (90.0_f64).to_radians(), S1ChordAngle::new(5.570796326794897));
        assert_eq!(s1 - s2, S1ChordAngle::new(2.0000000000000004));
        assert_eq!(s1 - (90.0_f64).to_radians(), S1ChordAngle::new(2.4292036732051034));
        assert_eq!((s1 * s2), S1ChordAngle::new(7.999999999999998));
        assert_eq!(s1 * 2., S1ChordAngle::new(8.0));
        assert_eq!((s1 / s2), S1ChordAngle::new(2.0000000000000004));
        assert_eq!(s1 / 2., S1ChordAngle::new(2.0));
        assert_eq!((s1 % *s2), S1ChordAngle::new(8.881784197001252e-16));
        assert_eq!(s1 % 180., S1ChordAngle::new(4.0));
        assert_eq!(s1 % 90., S1ChordAngle::new(4.0));
        let mut s1 = S1ChordAngle::new(180.);
        s1 %= 180.;
        assert_eq!(s1, S1ChordAngle::new(0.));

        assert_eq!(s1.neg(), S1ChordAngle::new(-0.));
    }
}
