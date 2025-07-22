#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use gistools::proj::{
        CoordinateStep, Coords, EQUAL_EARTH, EqualEarthProjection, Proj, ProjectCoordinates,
        derive_eccentricity, derive_sphere,
    };

    #[test]
    fn test_eqearth() {
        let proj = Rc::new(RefCell::new(Proj::default()));

        let projection = EqualEarthProjection::new(proj);

        assert_eq!(projection.code(), EQUAL_EARTH);
        assert_eq!(projection.name(), "Equal Earth");
        assert_eq!(EqualEarthProjection::names(), &["Equal Earth", "EqualEarth", "eqearth"]);

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0., 0.));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0., 0.));
    }

    #[test]
    #[should_panic]
    fn test_eqearth2() {
        let proj = Rc::new(RefCell::new(Proj::default()));

        let projection = EqualEarthProjection::new(proj.clone());

        {
            let proj = &mut proj.borrow_mut();
            proj.ellps = "GRS80".to_string();
            derive_sphere(proj);
            derive_eccentricity(proj);
        }

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        // assert_eq!(coords, Coords::new_xy(0., 0.));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0., 0.));
    }
}
