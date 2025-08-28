#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::proj::{
        CoordinateStep, Coords, M_VAL, N_VAL, OblatedEqualAreaProjection, Proj, ProjectCoordinates,
        derive_sphere,
    };
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn test_oea() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.set_f64(N_VAL, 0.25);
            proj.set_f64(M_VAL, 0.25);
        }
        let projection = OblatedEqualAreaProjection::new(proj);

        assert_eq!(projection.code(), -1);
        assert_eq!(projection.name(), "Oblated Equal Area");
        assert_eq!(OblatedEqualAreaProjection::names(), &["Oblated Equal Area", "oea"]);

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));
    }

    #[test]
    fn test_oea2() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.set_f64(N_VAL, 0.25);
            proj.set_f64(M_VAL, 0.25);
            proj.ellps = "GRS80".to_string();
            derive_sphere(proj);
        }
        let projection = OblatedEqualAreaProjection::new(proj.clone());

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));
    }

    #[test]
    fn test_oea3() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.set_f64(N_VAL, 0.25);
            proj.set_f64(M_VAL, 0.25);
            proj.ellps = "SPHERE".to_string();
            derive_sphere(proj);
        }
        let projection = OblatedEqualAreaProjection::new(proj.clone());

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));
    }
}
