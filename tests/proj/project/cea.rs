#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::proj::{
        CoordinateStep, Coords, EqualAreaCylindricalProjection, LATITUDE_STD_PARALLEL, Proj,
        ProjectCoordinates, derive_sphere,
    };
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn test_cea() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.set_f64(LATITUDE_STD_PARALLEL, 0.25);
        }
        let projection = EqualAreaCylindricalProjection::new(proj);

        assert_eq!(projection.code(), -1);
        assert_eq!(projection.name(), "Equal Area Cylindrical");
        assert_eq!(EqualAreaCylindricalProjection::names(), &["Equal Area Cylindrical", "cea"]);

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));
    }

    #[test]
    fn test_cea2() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.set_f64(LATITUDE_STD_PARALLEL, 0.25);
            proj.ellps = "GRS80".to_string();
            derive_sphere(proj);
        }
        let projection = EqualAreaCylindricalProjection::new(proj.clone());

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));
    }

    #[test]
    fn test_cea3() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.set_f64(LATITUDE_STD_PARALLEL, 0.25);
            proj.ellps = "SPHERE".to_string();
            derive_sphere(proj);
        }
        let projection = EqualAreaCylindricalProjection::new(proj.clone());

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));
    }
}
