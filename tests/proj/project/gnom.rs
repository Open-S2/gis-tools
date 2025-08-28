#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::proj::{
        CoordinateStep, Coords, GnomonicProjection, Proj, ProjectCoordinates, derive_sphere,
    };
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn test_gnom() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        // {
        //     let proj = &mut proj.borrow_mut();
        //     proj.set_f64(LATITUDE_OF_FIRST_STANDARD_PARALLEL, 0.25);
        //     proj.set_f64(LATITUDE_OF_SECOND_STANDARD_PARALLEL, 0.5);
        // }
        let projection = GnomonicProjection::new(proj);

        assert_eq!(projection.code(), -1);
        assert_eq!(projection.name(), "Gnomonic");
        assert_eq!(GnomonicProjection::names(), &["Gnomonic", "gnom"]);

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));
    }

    // #[test]
    // fn test_gnom2() {
    //     let proj = Rc::new(RefCell::new(Proj::default()));
    //     {
    //         let proj = &mut proj.borrow_mut();
    //         proj.ellps = "GRS80".to_string();
    //         derive_sphere(proj);
    //     }
    //     let projection = GnomonicProjection::new(proj.clone());

    //     let mut coords = Coords::new_xy(80., 80.);
    //     projection.forward(&mut coords);
    //     assert_eq!(coords, Coords::new_xy(0.0, 0.));

    //     // let mut coords = Coords::new_xy(0., 0.);
    //     // projection.inverse(&mut coords);
    //     // assert_eq!(coords, Coords::new_xy(0.0, 0.));
    // }

    #[test]
    fn test_gnom3() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.ellps = "SPHERE".to_string();
            derive_sphere(proj);
        }
        let projection = GnomonicProjection::new(proj.clone());

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));
    }
}
