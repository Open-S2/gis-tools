#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use gistools::proj::{
        AZIMUTHAL_EQUIDISTANT, AzimuthalEquidistantProjection, CoordinateStep, Coords, GUAM, Proj,
        ProjectCoordinates, derive_sphere,
    };

    #[test]
    fn test_aeqd() {
        let proj = Rc::new(RefCell::new(Proj::default()));

        let projection = AzimuthalEquidistantProjection::new(proj);

        assert_eq!(projection.code(), AZIMUTHAL_EQUIDISTANT);
        assert_eq!(projection.name(), "Azimuthal Equidistant");
        assert_eq!(
            AzimuthalEquidistantProjection::names(),
            &["Azimuthal Equidistant", "Azimuthal_Equidistant", "aeqd", "guam"]
        );

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0., 0.));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0., 0.));
    }

    #[test]
    fn test_aeqd2() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.set_f64(GUAM, 1.);
            proj.ellps = "GRS80".to_string();
            derive_sphere(proj);
        }
        let projection = AzimuthalEquidistantProjection::new(proj.clone());

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0., 0.));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0., 0.));
    }
}
