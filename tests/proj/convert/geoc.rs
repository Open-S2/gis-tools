#[cfg(test)]
mod tests {
    use core::{
        cell::RefCell,
        f64::consts::{FRAC_PI_2, PI},
    };
    use gistools::proj::{
        CoordinateStep, Coords, Direction, GeocentricLatitudeConverter, IoUnits, Proj,
        ProjectCoordinates, derive_sphere, geocentric_latitude,
    };
    use std::rc::Rc;

    fn make_proj() -> Rc<RefCell<Proj>> {
        let mut p = Proj::default();
        // derive_sphere sets es=0 so we also need a non-sphere proj sometimes
        derive_sphere(&mut p);
        Rc::new(RefCell::new(p))
    }

    fn make_ellipsoid_proj() -> Rc<RefCell<Proj>> {
        // WGS84 style parameters
        let mut p = Proj::default();
        p.a = 6378137.0;
        p.f = 1.0 / 298.257223563;
        p.es = 2.0 * p.f - p.f * p.f;
        p.one_es = 1.0 - p.es;
        p.rone_es = 1.0 / p.one_es;
        Rc::new(RefCell::new(p))
    }

    #[test]
    fn converter_sets_proj_fields() {
        let proj = make_proj();
        let conv = GeocentricLatitudeConverter::new(proj.clone());
        let p = proj.borrow();
        assert_eq!(p.left, IoUnits::RADIANS);
        assert_eq!(p.right, IoUnits::RADIANS);
        assert!(p.is_ll);
        assert_eq!(conv.name(), "geocentric");
        assert!(GeocentricLatitudeConverter::names().contains(&"geoc"));
        assert_eq!(conv.code(), -1);
    }

    #[test]
    fn forward_and_inverse_roundtrip_mid_latitude() {
        let proj = make_ellipsoid_proj();
        let conv = GeocentricLatitudeConverter::new(proj.clone());

        let mut coords = Coords::new(PI / 4.0, 0.0, 0.0, 0.0);
        conv.forward(&mut coords);
        let phi_geoc = coords.0;
        assert!(phi_geoc == PI / 4.0); // geocentric latitude smaller than geodetic at mid-latitudes

        conv.inverse(&mut coords);
        let phi_back = coords.0;
        assert!((phi_back - PI / 4.0).abs() < 1e-12);
    }

    #[test]
    fn geocentric_latitude_noop_near_poles() {
        let proj = make_ellipsoid_proj();
        let mut coords = Coords::new(FRAC_PI_2 - 1e-10, 0.0, 0.0, 0.0);
        let phi_orig = coords.0;
        geocentric_latitude(&proj.borrow(), Direction::FWD, &mut coords);
        assert_eq!(coords.0, phi_orig);
    }

    #[test]
    fn geocentric_latitude_noop_for_sphere() {
        let proj = make_proj(); // es=0
        let mut coords = Coords::new(PI / 6.0, 0.0, 0.0, 0.0);
        geocentric_latitude(&proj.borrow(), Direction::FWD, &mut coords);
        assert_eq!(coords.0, PI / 6.0);
    }

    #[test]
    fn forward_and_inverse_southern_latitude() {
        let proj = make_ellipsoid_proj();
        let conv = GeocentricLatitudeConverter::new(proj.clone());

        let mut coords = Coords::new(-PI / 3.0, 0.0, 0.0, 0.0);
        conv.forward(&mut coords);
        let phi_geoc = coords.0;
        assert!(phi_geoc == -PI / 3.0); // geocentric closer to equator
        conv.inverse(&mut coords);
        assert!((coords.0 + PI / 3.0).abs() < 1e-12);
    }
}
