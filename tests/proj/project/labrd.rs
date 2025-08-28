#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::proj::{
        AZIMUTH_PROJECTION_CENTRE, CoordinateStep, Coords, LABORDE, LabordeProjection, Proj,
        ProjectCoordinates,
    };
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn test_labrd() {
        let proj = Rc::new(RefCell::new(Proj::default()));

        {
            let proj = &mut proj.borrow_mut();
            proj.set_f64(AZIMUTH_PROJECTION_CENTRE, 0.);
            proj.phi0 = 0.5;
        }

        let projection = LabordeProjection::new(proj);

        assert_eq!(projection.code(), LABORDE);
        assert_eq!(projection.name(), "Laborde");
        assert_eq!(LabordeProjection::names(), &["Laborde", "Laborde Oblique Mercator", "labrd"]);

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
    }

    #[test]
    #[should_panic(expected = "Invalid value for lat_0: lat_0 should be different from 0")]
    fn test_labrd_error() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        let _projection = LabordeProjection::new(proj);
    }
}
