#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::proj::{
        BONNE, BonneProjection, CoordinateStep, Coords, LATITUDE_OF_FIRST_STANDARD_PARALLEL, Proj,
        ProjectCoordinates, derive_sphere,
    };
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn test_bonne() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.set_f64(LATITUDE_OF_FIRST_STANDARD_PARALLEL, 0.25);
        }
        let projection = BonneProjection::new(proj);

        assert_eq!(projection.code(), BONNE);
        assert_eq!(projection.name(), "Bonne");
        assert_eq!(
            BonneProjection::names(),
            &["Bonne (Werner lat_1=90)", "bonne_werner", "Bonne", "bonne"]
        );

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, -0.24999999999999956));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.24999999999999956));
    }

    #[test]
    fn test_bonne2() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.set_f64(LATITUDE_OF_FIRST_STANDARD_PARALLEL, 0.25);
            proj.ellps = "GRS80".to_string();
            derive_sphere(proj);
        }
        let projection = BonneProjection::new(proj.clone());

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, -0.2495810746057603));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.2512107114836941));
    }
}
