#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use gistools::proj::{
        CoordinateStep, Coords, GoodeHomolosineProjection, Proj, ProjectCoordinates,
    };

    #[test]
    fn test_goode() {
        let proj = Rc::new(RefCell::new(Proj::default()));

        let projection = GoodeHomolosineProjection::new(proj);

        assert_eq!(projection.code(), -1);
        assert_eq!(projection.name(), "Goode Homolosine");

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0., 0.));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0., 0.));

        let mut coords = Coords::new_xy(1., 1.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(5.5128474740096825e-17, 1.3614135623730952));

        let mut coords = Coords::new_xy(1., 1.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(1.111495672708268, 0.04754356460271263));
    }
}
