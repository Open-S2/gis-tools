#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::proj::{
        CoordinateStep, Coords, EQUIDISTANT_CYLINDRICAL, EquidistantCylindricalProjection,
        LATITUDE_STD_PARALLEL, Proj, ProjectCoordinates, derive_sphere,
    };
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn test_eqc() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.set_f64(LATITUDE_STD_PARALLEL, 0.25);
        }
        let projection = EquidistantCylindricalProjection::new(proj);

        assert_eq!(projection.code(), EQUIDISTANT_CYLINDRICAL);
        assert_eq!(projection.name(), "Equidistant Cylindrical");
        assert_eq!(
            EquidistantCylindricalProjection::names(),
            &[
                "Equidistant Cylindrical",
                "EquidistantCylindrical",
                "Equidistant Cylindrical (Plate Carree)",
                "eqc",
            ]
        );

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));
    }

    #[test]
    fn test_eqc2() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.set_f64(LATITUDE_STD_PARALLEL, 0.25);
            proj.ellps = "GRS80".to_string();
            derive_sphere(proj);
        }
        let projection = EquidistantCylindricalProjection::new(proj.clone());

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));
    }

    #[test]
    fn test_eqc3() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.set_f64(LATITUDE_STD_PARALLEL, 0.25);
            proj.ellps = "SPHERE".to_string();
            derive_sphere(proj);
        }
        let projection = EquidistantCylindricalProjection::new(proj.clone());

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));
    }
}
