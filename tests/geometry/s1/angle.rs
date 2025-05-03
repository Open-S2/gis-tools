#[cfg(test)]
// #[coverage(off)]
mod tests {
    use std::{cmp::Ordering, ops::Neg};

    use gistools::geometry::{LonLat, S1Angle};
    use s2json::MValue;

    #[test]
    fn new() {
        let angle = S1Angle::new(1.0);
        assert_eq!(angle.to_degrees(), 57.29577951308232);
        assert_eq!(angle, S1Angle::from_degrees(57.29577951308232));
        assert!(!angle.is_sign_negative());
        assert!(angle.is_sign_positive());

        let angle_inf = S1Angle::infinity();
        assert!(angle_inf.is_infinite());
        assert!(!angle_inf.is_nan());
        assert!(!angle_inf.is_sign_negative());
        assert!(angle_inf.is_sign_positive());
    }

    #[test]
    fn e5() {
        assert_eq!(S1Angle::new(0.).e5(), 0.0);
        assert_eq!(S1Angle::new(1.).e5(), 5729577.951308233);
        assert_eq!(S1Angle::new(-1.).e5(), -5729577.951308233);

        assert_eq!(S1Angle::to_e5(0.), 0.0);
    }
    #[test]
    fn e6() {
        assert_eq!(S1Angle::new(0.).e6(), 0.0);
        assert_eq!(S1Angle::new(1.).e6(), 57295779.513082325);
        assert_eq!(S1Angle::new(-1.).e6(), -57295779.513082325);

        assert_eq!(S1Angle::to_e6(0.), 0.0);
    }
    #[test]
    fn e7() {
        assert_eq!(S1Angle::new(0.).e7(), 0.0);
        assert_eq!(S1Angle::new(1.).e7(), 572957795.1308233);
        assert_eq!(S1Angle::new(-1.).e7(), -572957795.1308233);

        assert_eq!(S1Angle::to_e7(0.), 0.0);
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn from_s2points() {
        assert_eq!(
            S1Angle::new(0.),
            S1Angle::from_s2points(
                &(&LonLat::<MValue>::new(0., 0., None)).into(),
                &(&LonLat::<MValue>::new(0., 0., None)).into()
            )
        );
        assert_eq!(
            S1Angle::new(0.017453292519943295),
            S1Angle::from_s2points(
                &(&LonLat::<MValue>::new(1., 0., None)).into(),
                &(&LonLat::<MValue>::new(0., 0., None)).into()
            )
        );
        assert_eq!(
            S1Angle::new(1.5707963267948966),
            S1Angle::from_s2points(
                &(&LonLat::<MValue>::new(90., 0., None)).into(),
                &(&LonLat::<MValue>::new(0., 0., None)).into()
            )
        );
    }

    #[test]
    #[allow(clippy::approx_constant)]
    pub fn from_lon_lat() {
        assert_eq!(
            S1Angle::new(0.),
            S1Angle::from_lon_lat(
                &LonLat::<MValue>::new(0., 0., None),
                &LonLat::<MValue>::new(0., 0., None)
            )
        );
        assert_eq!(
            S1Angle::new(0.017453292519943295),
            S1Angle::from_lon_lat(
                &LonLat::<MValue>::new(1., 0., None),
                &LonLat::<MValue>::new(0., 0., None)
            )
        );
        assert_eq!(
            S1Angle::new(1.5707963267948963),
            S1Angle::from_lon_lat(
                &LonLat::<MValue>::new(90., 0., None),
                &LonLat::<MValue>::new(0., 0., None)
            )
        );
    }

    #[test]
    pub fn to_meters() {
        assert_eq!(S1Angle::new(0.).to_meters(None), 0.0);
        assert_eq!(S1Angle::from_degrees(180.).to_meters(None), 20015114.442035925);
    }

    #[test]
    pub fn from_meters() {
        assert_eq!(S1Angle::from_meters(0., None).to_degrees(), 0.);
        assert_eq!(S1Angle::from_meters(20015114.442035925, None).to_degrees(), 180.00000000000003);
    }

    #[test]
    pub fn to_km() {
        assert_eq!(S1Angle::new(0.).to_km(None), 0.0);
        assert_eq!(S1Angle::from_degrees(180.).to_km(None), 20015.114442035925);
    }

    #[test]
    pub fn from_km() {
        assert_eq!(S1Angle::from_km(0., None).to_degrees(), 0.);
        assert_eq!(S1Angle::from_km(20015.114442035925, None).to_degrees(), 180.00000000000003);
    }

    #[test]
    #[allow(clippy::approx_constant)]
    pub fn normalize() {
        assert_eq!(S1Angle::new(0.).normalize().radians, 0.0);
        assert_eq!(S1Angle::from_degrees(1.).normalize().radians, 0.017453292519943295);
        assert_eq!(S1Angle::from_degrees(180.).normalize().radians, 3.141592653589793);
        assert_eq!(S1Angle::from_degrees(360.).normalize().radians, 0.0);
    }

    #[test]
    #[allow(clippy::approx_constant)]
    pub fn maths() {
        let s1 = S1Angle::from_degrees(180.);
        let s2 = S1Angle::from_degrees(90.);
        assert_eq!(s1 + s2, S1Angle::from_degrees(270.));
        assert_eq!(s1 + (90.0_f64).to_radians(), S1Angle::from_degrees(270.));
        assert_eq!(s1 - s2, S1Angle::from_degrees(90.));
        assert_eq!(s1 - (90.0_f64).to_radians(), S1Angle::from_degrees(90.));
        assert_eq!((s1 * s2).normalize(), S1Angle::new(4.934802200544679));
        assert_eq!(s1 * 2., S1Angle::from_degrees(360.));
        assert_eq!((s1 / s2).normalize(), S1Angle::new(2.0));
        assert_eq!(s1 / 2., S1Angle::from_degrees(90.));
        assert_eq!((s1 % *s2).normalize(), S1Angle::new(0.0));
        assert_eq!(s1 % 180., S1Angle::from_degrees(180.));
        assert_eq!(s1 % 90., S1Angle::from_degrees(180.));
        let mut s1 = S1Angle::from_degrees(180.);
        s1 %= 180.;
        assert_eq!(s1, S1Angle::from_degrees(180.));

        assert_eq!(s1.neg(), S1Angle::from_degrees(-180.));
    }

    #[test]
    pub fn cmp() {
        let s1 = S1Angle::from_degrees(180.);
        let s2 = S1Angle::from_degrees(90.);
        assert!(s1 > s2);
        assert!(s1 >= s2);
        assert!(s2 < s1);
        assert!(s2 <= s1);
        assert!(s1 != s2);
        assert!(s1 == s1);
        assert!(s1 == (180.0_f64).to_radians());
        let rad = (180.0_f64).to_radians();
        assert_eq!(s1.partial_cmp(&rad), Some(Ordering::Equal));
    }
}
