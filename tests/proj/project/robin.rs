#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::proj::{CoordinateStep, Coords, Proj, ProjectCoordinates, RobinsonProjection};
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn test_robin() {
        let proj = Rc::new(RefCell::new(Proj::default()));

        let projection = RobinsonProjection::new(proj);

        assert_eq!(projection.code(), -1);
        assert_eq!(projection.name(), "Robinson");

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0., -7.037599091e-18));

        let mut coords = Coords::new_xy(0., -7.037599091e-18);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0., -1.4649984086050534e-17));
    }

    #[test]
    #[should_panic(expected = "Coordinate outside projection domain")]
    fn test_robin_should_panic() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        let projection = RobinsonProjection::new(proj);

        let mut coords = Coords::new_xy(-2.0, -2.0);
        projection.inverse(&mut coords);
    }

    #[test]
    #[should_panic(expected = "Coordinate outside projection domain")]
    fn test_robin_should_panic_2() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        let projection = RobinsonProjection::new(proj);

        let mut coords = Coords::new_xy(0.0, f64::NAN);
        projection.inverse(&mut coords);
    }

    #[test]
    #[should_panic(expected = "Coordinate outside projection domain")]
    fn test_robin_should_panic_3() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        let projection = RobinsonProjection::new(proj);

        let mut coords = Coords::new_xy(f64::NAN, f64::NAN);
        projection.forward(&mut coords);
    }
}
