#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use gistools::proj::{
        ALBERS_EQUAL_AREA, AlbersConicEqualAreaProjection, CoordinateStep, Coords,
        LATITUDE_OF_FIRST_STANDARD_PARALLEL, LATITUDE_OF_SECOND_STANDARD_PARALLEL,
        LambertEqualAreaConicProjection, Proj, ProjectCoordinates,
    };

    #[test]
    fn test_aea() {
        let proj = Rc::new(RefCell::new(Proj::default()));

        {
            let proj = &mut proj.borrow_mut();
            proj.set_f64(LATITUDE_OF_FIRST_STANDARD_PARALLEL, 0.1);
            proj.set_f64(LATITUDE_OF_SECOND_STANDARD_PARALLEL, -0.5);
        }

        let projection = AlbersConicEqualAreaProjection::new(proj);

        assert_eq!(projection.code(), ALBERS_EQUAL_AREA);
        assert_eq!(projection.name(), "Albers Conic Equal Area");

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0., 0.));
    }

    #[test]
    #[should_panic(expected = "Invalid value for lat_1 and lat_2: |lat_1 + lat_2| should be > 0")]
    fn test_aea_error() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        let _projection = AlbersConicEqualAreaProjection::new(proj);
    }

    #[test]
    #[should_panic(expected = "Invalid value for lat_1: |lat_1| should be <= 90°")]
    fn test_aea_error_2() {
        let proj = Rc::new(RefCell::new(Proj::default()));

        {
            let proj = &mut proj.borrow_mut();
            proj.set_f64(LATITUDE_OF_FIRST_STANDARD_PARALLEL, 2.);
            // proj.set_f64(LATITUDE_OF_SECOND_STANDARD_PARALLEL, -0.5);
        }

        let _projection = AlbersConicEqualAreaProjection::new(proj);
    }

    #[test]
    #[should_panic(expected = "Invalid value for lat_2: |lat_2| should be <= 90°")]
    fn test_aea_error_3() {
        let proj = Rc::new(RefCell::new(Proj::default()));

        {
            let proj = &mut proj.borrow_mut();
            proj.set_f64(LATITUDE_OF_FIRST_STANDARD_PARALLEL, 0.1);
            proj.set_f64(LATITUDE_OF_SECOND_STANDARD_PARALLEL, 2.);
        }

        let _projection = AlbersConicEqualAreaProjection::new(proj);
    }

    #[test]
    #[should_panic(expected = "Invalid value for lat_1 and lat_2: |lat_1 + lat_2| should be > 0")]
    fn test_aea_error_4() {
        let proj = Rc::new(RefCell::new(Proj::default()));

        {
            let proj = &mut proj.borrow_mut();
            proj.set_f64(LATITUDE_OF_FIRST_STANDARD_PARALLEL, 1e-11);
            proj.set_f64(LATITUDE_OF_SECOND_STANDARD_PARALLEL, 1e-11);
        }

        let _projection = AlbersConicEqualAreaProjection::new(proj);
    }

    #[test]
    fn test_equal_area_conic() {
        let proj = Rc::new(RefCell::new(Proj::default()));

        {
            let proj = &mut proj.borrow_mut();
            proj.set_f64(LATITUDE_OF_FIRST_STANDARD_PARALLEL, 0.1);
            proj.set_f64(LATITUDE_OF_SECOND_STANDARD_PARALLEL, -0.5);
        }

        let projection = LambertEqualAreaConicProjection::new(proj);

        assert_eq!(projection.code(), -1);
        assert_eq!(projection.name(), "Lambert Equal Area Conic");

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0., 0.));
    }
}
