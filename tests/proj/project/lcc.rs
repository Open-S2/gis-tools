#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::proj::{
        CoordinateStep, Coords, LAMBERT_CONFORMAL_CONIC_1SP, LATITUDE_OF_NATURAL_ORIGIN,
        LONGITUDE_OF_NATURAL_ORIGIN, LambertConformalConic1SPProjection, Proj, ProjectCoordinates,
        derive_sphere,
    };
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn test_lcc() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.set_f64(LATITUDE_OF_NATURAL_ORIGIN, 0.25);
            proj.set_f64(LONGITUDE_OF_NATURAL_ORIGIN, 0.25);
        }
        let projection = LambertConformalConic1SPProjection::new(proj);

        assert_eq!(projection.code(), LAMBERT_CONFORMAL_CONIC_1SP);
        assert_eq!(projection.name(), "Lambert Conformal Conic");
        assert_eq!(
            LambertConformalConic1SPProjection::names(),
            &[
                "Lambert Conic Conformal (1SP)",
                "Lambert_Conformal_Conic_1SP",
                "Lambert Conic Conformal (2SP)",
                "Lambert_Conformal_Conic_2SP",
                "Lambert Conic Conformal (LCC)",
                "Lambert_Conformal_Conic",
                "Lambert Conformal Conic",
                "LambertConformalConic",
                "lcc",
            ]
        );

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, -0.2526037996527273));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.24999999999999933));
    }

    #[test]
    fn test_lcc2() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.set_f64(LATITUDE_OF_NATURAL_ORIGIN, 0.25);
            proj.set_f64(LONGITUDE_OF_NATURAL_ORIGIN, 0.25);
            proj.ellps = "GRS80".to_string();
            derive_sphere(proj);
        }
        let projection = LambertConformalConic1SPProjection::new(proj.clone());

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, -0.25094710101403184));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.25));
    }

    #[test]
    fn test_lcc3() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.set_f64(LATITUDE_OF_NATURAL_ORIGIN, 0.25);
            proj.set_f64(LONGITUDE_OF_NATURAL_ORIGIN, 0.25);
            proj.ellps = "SPHERE".to_string();
            derive_sphere(proj);
        }
        let projection = LambertConformalConic1SPProjection::new(proj.clone());

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, -0.2526037996527273));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.24999999999999933));
    }
}
