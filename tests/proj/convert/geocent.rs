#[cfg(test)]
mod tests {
    use core::cell::RefCell;
    use gistools::proj::{
        CoordinateStep, Coords, GeocentricConverter, IoUnits, Proj, ProjectCoordinates,
    };
    use std::rc::Rc;

    fn make_proj() -> Rc<RefCell<Proj>> {
        Rc::new(RefCell::new(Proj::default()))
    }

    #[test]
    fn converter_sets_proj_fields() {
        let proj = make_proj();
        let conv = GeocentricConverter::new(proj.clone());
        let p = proj.borrow();

        assert_eq!(p.left, IoUnits::RADIANS);
        assert_eq!(p.right, IoUnits::CARTESIAN);
        assert_eq!(p.x0, 0.0);
        assert_eq!(p.y0, 0.0);
        assert!(p.is_geocent);

        assert_eq!(conv.name(), "geocentric latitude");
        assert!(GeocentricConverter::names().contains(&"geocent"));
        assert!(GeocentricConverter::names().contains(&"geocentric latitude"));
        assert_eq!(conv.code(), -1);
    }

    #[test]
    fn forward_and_inverse_are_noops() {
        let proj = make_proj();
        let conv = GeocentricConverter::new(proj);

        let mut coords = Coords::new(1.0, 2.0, 3.0, 4.0);
        let orig = coords.clone();

        conv.forward(&mut coords);
        assert_eq!(coords, orig);

        conv.inverse(&mut coords);
        assert_eq!(coords, orig);
    }
}
