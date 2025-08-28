#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::proj::{
        CoordinateStep, Coords, GaussSchreiberTransverseMercatorProjection, Proj,
        ProjectCoordinates, derive_sphere,
    };
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn test_gstmerc() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        // {
        //     let proj = &mut proj.borrow_mut();
        //     proj.set_f64(LATITUDE_OF_FIRST_STANDARD_PARALLEL, 0.25);
        //     proj.set_f64(LATITUDE_OF_SECOND_STANDARD_PARALLEL, 0.5);
        // }
        let projection = GaussSchreiberTransverseMercatorProjection::new(proj);

        assert_eq!(projection.code(), -1);
        assert_eq!(projection.name(), "Gauss-Schreiber Transverse Mercator");
        assert_eq!(
            GaussSchreiberTransverseMercatorProjection::names(),
            &[
                "Gauss-Schreiber Transverse Mercator (aka Gauss-Laborde Reunion)",
                "Gauss-Schreiber Transverse Mercator",
                "gstmerc",
            ]
        );

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));

        // let mut coords = Coords::new_xy(0.4, 0.5);
        // projection.inverse(&mut coords);
        // assert_eq!(coords, Coords::new_xy(0.0, 0.));
    }

    #[test]
    fn test_gstmerc2() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.ellps = "GRS80".to_string();
            derive_sphere(proj);
        }
        let projection = GaussSchreiberTransverseMercatorProjection::new(proj.clone());

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));
    }

    #[test]
    fn test_gstmerc3() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.ellps = "SPHERE".to_string();
            derive_sphere(proj);
        }
        let projection = GaussSchreiberTransverseMercatorProjection::new(proj.clone());

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));
    }
}
