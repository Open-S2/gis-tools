#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::proj::{
        CoordinateStep, Coords, KROVAK, KROVAK_MODIFIED, KrovakModifiedProjection,
        KrovakProjection, Proj, ProjectCoordinates, derive_sphere,
    };
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn test_krovak() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        // {
        //     let proj = &mut proj.borrow_mut();
        //     proj.set_f64(LATITUDE_OF_FIRST_STANDARD_PARALLEL, 0.25);
        //     proj.set_f64(LATITUDE_OF_SECOND_STANDARD_PARALLEL, 0.5);
        // }
        let projection = KrovakProjection::new(proj);

        assert_eq!(projection.code(), KROVAK);
        assert_eq!(projection.name(), "Krovak");
        assert_eq!(KrovakProjection::names(), &["Krovak", "Modified Krovak"]);

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, -1.1160724884446869));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 1.0429668480103937));
    }

    #[test]
    fn test_krovak2() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.ellps = "GRS80".to_string();
            derive_sphere(proj);
        }
        let projection = KrovakProjection::new(proj.clone());

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, -1.1160724884446869));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 1.0429668480103937));
    }

    #[test]
    fn test_krovak3() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.ellps = "SPHERE".to_string();
            derive_sphere(proj);
        }
        let projection = KrovakProjection::new(proj.clone());

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, -1.1160724884446869));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 1.0429668480103937));
    }

    #[test]
    fn test_krovak_mod() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        // {
        //     let proj = &mut proj.borrow_mut();
        //     proj.set_f64(LATITUDE_OF_FIRST_STANDARD_PARALLEL, 0.25);
        //     proj.set_f64(LATITUDE_OF_SECOND_STANDARD_PARALLEL, 0.5);
        // }
        let projection = KrovakModifiedProjection::new(proj);

        assert_eq!(projection.code(), KROVAK_MODIFIED);
        assert_eq!(projection.name(), "Krovak");
        assert_eq!(KrovakModifiedProjection::names(), &["Krovak", "Modified Krovak"]);

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(-0.0018077470217246264, -1.1158835577808552));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(-7.879609514593877e-6, 1.0429668487648347));
    }

    #[test]
    fn test_krovak_mod2() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.ellps = "GRS80".to_string();
            derive_sphere(proj);
        }
        let projection = KrovakModifiedProjection::new(proj.clone());

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(-0.0018077470217246264, -1.1158835577808552));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(-7.879609514593877e-6, 1.0429668487648347));
    }

    #[test]
    fn test_krovak_mod3() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.ellps = "SPHERE".to_string();
            derive_sphere(proj);
        }
        let projection = KrovakModifiedProjection::new(proj.clone());

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(-0.0018077470217246264, -1.1158835577808552));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(-7.879609514593877e-6, 1.0429668487648347));
    }
}
