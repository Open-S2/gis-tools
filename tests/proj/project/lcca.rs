// #[cfg(test)]
// // #[coverage(off)]
// #[cfg_attr(feature = "nightly", coverage(off))]
// mod tests {
//     use gistools::proj::{
//         CoordinateStep, Coords, LATITUDE_OF_PROJECTION_CENTRE,
//         LambertConformalConicAlternativeProjection, Proj, ProjectCoordinates, derive_sphere,
//     };
//     use std::{cell::RefCell, rc::Rc};

//     #[test]
//     fn test_lcca() {
//         let proj = Rc::new(RefCell::new(Proj::default()));
//         {
//             let proj = &mut proj.borrow_mut();
//             proj.set_f64(LATITUDE_OF_PROJECTION_CENTRE, 1.570796);
//         }
//         let projection = LambertConformalConicAlternativeProjection::new(proj);

//         assert_eq!(projection.code(), -1);
//         assert_eq!(projection.name(), "Lambert Conformal Conic Alternative");
//         assert_eq!(
//             LambertConformalConicAlternativeProjection::names(),
//             &["Lambert Conformal Conic Alternative", "lcca"]
//         );

//         let mut coords = Coords::new_xy(2.0, 2.0);
//         projection.forward(&mut coords);
//         assert_eq!(coords, Coords::new_xy(0., 0.));

//         let mut coords = Coords::new_xy(0., 0.);
//         projection.inverse(&mut coords);
//         assert_eq!(coords, Coords::new_xy(0.0, 0.));
//     }

//     #[test]
//     fn test_lcca2() {
//         let proj = Rc::new(RefCell::new(Proj::default()));
//         {
//             let proj = &mut proj.borrow_mut();
//             proj.set_f64(LATITUDE_OF_PROJECTION_CENTRE, 1.570796);
//             proj.ellps = "GRS80".to_string();
//             derive_sphere(proj);
//         }
//         let projection = LambertConformalConicAlternativeProjection::new(proj.clone());

//         let mut coords = Coords::new_xy(0., 0.);
//         projection.forward(&mut coords);
//         assert_eq!(coords, Coords::new_xy(0.0, 0.));

//         let mut coords = Coords::new_xy(0., 0.);
//         projection.inverse(&mut coords);
//         assert_eq!(coords, Coords::new_xy(0.0, 0.));
//     }

//     #[test]
//     fn test_lcca3() {
//         let proj = Rc::new(RefCell::new(Proj::default()));
//         {
//             let proj = &mut proj.borrow_mut();
//             proj.set_f64(LATITUDE_OF_PROJECTION_CENTRE, 1.570796);
//             proj.ellps = "SPHERE".to_string();
//             derive_sphere(proj);
//         }
//         let projection = LambertConformalConicAlternativeProjection::new(proj.clone());

//         let mut coords = Coords::new_xy(0., 0.);
//         projection.forward(&mut coords);
//         assert_eq!(coords, Coords::new_xy(0.0, 0.));

//         let mut coords = Coords::new_xy(0., 0.);
//         projection.inverse(&mut coords);
//         assert_eq!(coords, Coords::new_xy(0.0, 0.));
//     }
// }
