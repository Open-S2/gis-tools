#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::proj::{
        CoordinateStep, Coords, MollweideProjection, Proj, ProjectCoordinates, WagnerIVProjection,
        WagnerVProjection, derive_sphere,
    };
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn test_moll() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        // {
        //     let proj = &mut proj.borrow_mut();
        //     proj.set_f64(LATITUDE_STD_PARALLEL, 0.25);
        // }
        let projection = MollweideProjection::new(proj);

        assert_eq!(projection.code(), -1);
        assert_eq!(projection.name(), "Mollweide");
        assert_eq!(MollweideProjection::names(), &["Mollweide", "moll"]);

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 1.4142135623730951));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));
    }

    #[test]
    fn test_moll2() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.ellps = "GRS80".to_string();
            derive_sphere(proj);
        }
        let projection = MollweideProjection::new(proj.clone());

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 1.4142135623730951));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));
    }

    #[test]
    fn test_moll3() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.ellps = "SPHERE".to_string();
            derive_sphere(proj);
        }
        let projection = MollweideProjection::new(proj.clone());

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 1.4142135623730951));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));
    }

    #[test]
    fn test_wag_4() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        // {
        //     let proj = &mut proj.borrow_mut();
        //     proj.set_f64(LATITUDE_STD_PARALLEL, 0.25);
        // }
        let projection = WagnerIVProjection::new(proj);

        assert_eq!(projection.code(), -1);
        assert_eq!(projection.name(), "Wagner IV");
        assert_eq!(WagnerIVProjection::names(), &["Wagner IV", "wag4"]);

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 1.5654814159993378));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));
    }

    #[test]
    fn test_wag_5() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        // {
        //     let proj = &mut proj.borrow_mut();
        //     proj.set_f64(LATITUDE_STD_PARALLEL, 0.25);
        // }
        let projection = WagnerVProjection::new(proj);

        assert_eq!(projection.code(), -1);
        assert_eq!(projection.name(), "Wagner V");
        assert_eq!(WagnerVProjection::names(), &["Wagner V", "wag5"]);

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 1.65014));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));
    }
}
