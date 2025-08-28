#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::proj::{
        CoordinateStep, Coords, Proj, ProjectCoordinates, TRANSVERSE_MERCATOR,
        TransverseMercatorProjection, derive_sphere,
    };
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn test_tmerc() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        // {
        //     let proj = &mut proj.borrow_mut();
        //     proj.set_f64(LATITUDE_STD_PARALLEL, 0.25);
        // }
        let projection = TransverseMercatorProjection::new(proj);

        assert_eq!(projection.code(), TRANSVERSE_MERCATOR);
        assert_eq!(projection.name(), "Transverse Mercator");
        assert_eq!(
            TransverseMercatorProjection::names(),
            &[
                "Transverse Mercator",
                "Transverse_Mercator",
                "Transverse Mercator (South Oriented)",
                "tmerc",
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
    fn test_tmerc2() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.ellps = "GRS80".to_string();
            derive_sphere(proj);
        }
        let projection = TransverseMercatorProjection::new(proj.clone());

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));
    }

    #[test]
    fn test_tmerc3() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.ellps = "SPHERE".to_string();
            derive_sphere(proj);
        }
        let projection = TransverseMercatorProjection::new(proj.clone());

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));
    }
}
