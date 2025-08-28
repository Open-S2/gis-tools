#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::proj::{
        CoordinateStep, Coords, NewZealandMapGridProjection, Proj, ProjectCoordinates,
        derive_sphere,
    };
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn test_nzmg() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        // {
        //     let proj = &mut proj.borrow_mut();
        //     proj.set_f64(LATITUDE_STD_PARALLEL, 0.25);
        // }
        let projection = NewZealandMapGridProjection::new(proj);

        assert_eq!(projection.code(), -1);
        assert_eq!(projection.name(), "New Zealand Map Grid");
        assert_eq!(
            NewZealandMapGridProjection::names(),
            &["New Zealand Map Grid", "NewZealandMapGrid", "New_Zealand_Map_Grid", "nzmg"]
        );

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.565550771980446));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, -0.7155849933176751));
    }

    #[test]
    fn test_nzmg2() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.ellps = "GRS80".to_string();
            derive_sphere(proj);
        }
        let projection = NewZealandMapGridProjection::new(proj.clone());

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.565550771980446));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, -0.7155849933176751));
    }

    #[test]
    fn test_nzmg3() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        {
            let proj = &mut proj.borrow_mut();
            proj.ellps = "SPHERE".to_string();
            derive_sphere(proj);
        }
        let projection = NewZealandMapGridProjection::new(proj.clone());

        let mut coords = Coords::new_xy(0., 0.);
        projection.forward(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, 0.565550771980446));

        let mut coords = Coords::new_xy(0., 0.);
        projection.inverse(&mut coords);
        assert_eq!(coords, Coords::new_xy(0.0, -0.7155849933176751));
    }
}
