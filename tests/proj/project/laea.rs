#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::proj::{
        CoordinateStep, Coords, LAMBERT_AZIMUTHAL_EQUAL_AREA, LambertAzimuthalEqualAreaProjection,
        Proj, ProjectCoordinates, derive_sphere,
    };
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn test_laea() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        // {
        //     let proj = &mut proj.borrow_mut();
        //     proj.set_f64(LATITUDE_STD_PARALLEL, 0.25);
        // }
        let projection = LambertAzimuthalEqualAreaProjection::new(proj);

        assert_eq!(projection.code(), LAMBERT_AZIMUTHAL_EQUAL_AREA);
        assert_eq!(projection.name(), "Lambert Azimuthal Equal Area");
        assert_eq!(
            LambertAzimuthalEqualAreaProjection::names(),
            &[
                "Lambert Azimuthal Equal Area",
                "Lambert_Azimuthal_Equal_Area",
                "Lambert Azimuthal Equal Area (Spherical)",
                "laea",
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
    fn test_laea2() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.ellps = "GRS80".to_string();
            derive_sphere(proj);
        }
        let projection = LambertAzimuthalEqualAreaProjection::new(proj.clone());

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));
    }

    #[test]
    fn test_laea3() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.ellps = "SPHERE".to_string();
            derive_sphere(proj);
        }
        let projection = LambertAzimuthalEqualAreaProjection::new(proj.clone());

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));
    }
}
