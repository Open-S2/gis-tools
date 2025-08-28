#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::proj::{
        CoordinateStep, Coords, HOTINE_OBLIQUE_MERCATOR_VARIANT_A,
        HotineObliqueMercatorVariantAProjection, LATITUDE_OF_FIRST_POINT, LATITUDE_OF_SECOND_POINT,
        LONGITUDE_OF_FIRST_POINT, LONGITUDE_OF_SECOND_POINT, Proj, ProjectCoordinates,
        derive_sphere,
    };
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn test_omerc() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.set_f64(LONGITUDE_OF_FIRST_POINT, -1.);
            proj.set_f64(LATITUDE_OF_FIRST_POINT, -1.);
            proj.set_f64(LONGITUDE_OF_SECOND_POINT, 1.5);
            proj.set_f64(LATITUDE_OF_SECOND_POINT, 1.5);
        }
        let projection = HotineObliqueMercatorVariantAProjection::new(proj);

        assert_eq!(projection.code(), HOTINE_OBLIQUE_MERCATOR_VARIANT_A);
        assert_eq!(projection.name(), "Oblique Mercator");
        assert_eq!(
            HotineObliqueMercatorVariantAProjection::names(),
            &[
                "Hotine_Oblique_Mercator",
                "Hotine Oblique Mercator",
                "Hotine_Oblique_Mercator_Azimuth_Natural_Origin",
                "Hotine Oblique Mercator Azimuth Natural Origin",
                "Hotine_Oblique_Mercator_Two_Point_Natural_Origin",
                "Hotine Oblique Mercator Two Point Natural Origin",
                "Hotine_Oblique_Mercator_Azimuth_Center",
                "Hotine Oblique Mercator Azimuth Center",
                "Hotine Oblique Mercator (variant A)",
                "Hotine Oblique Mercator (variant B)",
                "Oblique_Mercator",
                "Oblique Mercator",
                "omerc",
            ]
        );

        // let mut coords = Coords::new_xy(0., 0.);
        // projection.forward(&mut coords);
        // assert_eq!(coords, Coords::new_xy(0.0, 0.));

        // let mut coords = Coords::new_xy(0., 0.);
        // projection.inverse(&mut coords);
        // assert_eq!(coords, Coords::new_xy(0.0, 0.));
    }

    #[test]
    fn test_omerc2() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.set_f64(LONGITUDE_OF_FIRST_POINT, -1.);
            proj.set_f64(LATITUDE_OF_FIRST_POINT, -1.);
            proj.set_f64(LONGITUDE_OF_SECOND_POINT, 1.5);
            proj.set_f64(LATITUDE_OF_SECOND_POINT, 1.5);
            proj.ellps = "GRS80".to_string();
            derive_sphere(proj);
        }
        let projection = HotineObliqueMercatorVariantAProjection::new(proj.clone());

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));
    }

    #[test]
    fn test_omerc3() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.set_f64(LONGITUDE_OF_FIRST_POINT, -1.);
            proj.set_f64(LATITUDE_OF_FIRST_POINT, -1.);
            proj.set_f64(LONGITUDE_OF_SECOND_POINT, 1.5);
            proj.set_f64(LATITUDE_OF_SECOND_POINT, 1.5);
            proj.ellps = "SPHERE".to_string();
            derive_sphere(proj);
        }
        let projection = HotineObliqueMercatorVariantAProjection::new(proj.clone());

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.));
    }
}
