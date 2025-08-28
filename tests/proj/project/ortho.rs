#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::proj::{
        CoordinateStep, Coords, ORTHOGRAPHIC, OrthographicProjection, Proj, ProjectCoordinates,
        derive_sphere,
    };
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn test_ortho() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        // {
        //     let proj = &mut proj.borrow_mut();
        //     proj.set_f64(LATITUDE_STD_PARALLEL, 0.25);
        // }
        let projection = OrthographicProjection::new(proj);

        assert_eq!(projection.code(), ORTHOGRAPHIC);
        assert_eq!(projection.name(), "Orthographic");
        assert_eq!(OrthographicProjection::names(), &["Orthographic", "ortho"]);

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));
    }

    #[test]
    fn test_ortho2() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.ellps = "GRS80".to_string();
            derive_sphere(proj);
        }
        let projection = OrthographicProjection::new(proj.clone());

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));
    }

    #[test]
    fn test_ortho3() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.ellps = "SPHERE".to_string();
            derive_sphere(proj);
        }
        let projection = OrthographicProjection::new(proj.clone());

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));
    }
}
