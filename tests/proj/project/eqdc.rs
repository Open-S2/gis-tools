#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::proj::{
        CoordinateStep, Coords, EQUIDISTANT_CONIC, EquidistantConicProjection,
        LATITUDE_OF_FIRST_STANDARD_PARALLEL, LATITUDE_OF_SECOND_STANDARD_PARALLEL, Proj,
        ProjectCoordinates, derive_sphere,
    };
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn test_eqdc() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.set_f64(LATITUDE_OF_FIRST_STANDARD_PARALLEL, 0.25);
            proj.set_f64(LATITUDE_OF_SECOND_STANDARD_PARALLEL, 0.5);
        }
        let projection = EquidistantConicProjection::new(proj);

        assert_eq!(projection.code(), EQUIDISTANT_CONIC);
        assert_eq!(projection.name(), "Equidistant Conic");
        assert_eq!(
            EquidistantConicProjection::names(),
            &["Equidistant Conic", "Equidistant_Conic", "eqdc"]
        );

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));
    }

    #[test]
    fn test_eqdc2() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.set_f64(LATITUDE_OF_FIRST_STANDARD_PARALLEL, 0.25);
            proj.set_f64(LATITUDE_OF_SECOND_STANDARD_PARALLEL, 0.5);
            proj.ellps = "GRS80".to_string();
            derive_sphere(proj);
        }
        let projection = EquidistantConicProjection::new(proj.clone());

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));
    }

    #[test]
    fn test_eqdc3() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.set_f64(LATITUDE_OF_FIRST_STANDARD_PARALLEL, 0.25);
            proj.set_f64(LATITUDE_OF_SECOND_STANDARD_PARALLEL, 0.5);
            proj.ellps = "SPHERE".to_string();
            derive_sphere(proj);
        }
        let projection = EquidistantConicProjection::new(proj.clone());

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));
    }
}
