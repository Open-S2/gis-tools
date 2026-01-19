#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::geometry::{S2CellId, S2Point};
    use s2json::{NewXY, Point, Point3D, SetXY, SetZ, VectorPoint};

    #[test]
    fn is_empty() {
        let point = S2Point { x: 1.0, y: 2.0, z: 3.0 };
        assert!(!point.is_empty());
        assert!(S2Point::new(0.0, 0.0, 0.0).is_empty());
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn angle() {
        let point1 = S2Point { x: 1.0, y: 0.0, z: 0.0 };
        let point2 = S2Point { x: 0.0, y: 1.0, z: 0.0 };
        assert_eq!(point1.angle(&point2), 1.5707963267948966);
    }

    #[test]
    fn cross() {
        let point1 = S2Point { x: 1.0, y: 0.0, z: 0.0 };
        let point2 = S2Point { x: 0.0, y: 1.0, z: 0.0 };
        assert_eq!(point1.cross(&point2), S2Point { x: 0.0, y: 0.0, z: 1.0 });
    }

    #[test]
    fn to_face_st() {
        let point = S2Point { x: 0.0, y: 1.0, z: 0.0 };
        assert_eq!(point.to_face_st(), (1, 0.5, 0.5));

        let point =
            S2Point { x: 0.5773502691896258, y: -0.5773502691896258, z: -0.5773502691896258 };
        assert_eq!(point.to_face_st(), (5, 0., 1.));
        let point =
            S2Point { x: 0.5773502691896258, y: 0.5773502691896258, z: -0.5773502691896258 };
        assert_eq!(point.to_face_st(), (5, 1., 1.));
        let point = S2Point { x: 0.5773502691896258, y: 0.5773502691896258, z: 0.5773502691896258 };
        assert_eq!(point.to_face_st(), (2, 0., 0.));
        let point =
            S2Point { x: -0.5773502691896258, y: 0.5773502691896258, z: 0.5773502691896258 };
        assert_eq!(point.to_face_st(), (2, 1., 0.));
        let point =
            S2Point { x: -0.5773502691896258, y: -0.5773502691896258, z: 0.5773502691896258 };
        assert_eq!(point.to_face_st(), (2, 1., 1.));
        let point =
            S2Point { x: -0.5773502691896258, y: -0.5773502691896258, z: -0.5773502691896258 };
        assert_eq!(point.to_face_st(), (5, 0., 0.));
    }

    #[test]
    fn to_face_uv() {
        let point = S2Point { x: 1.0, y: 0.0, z: 0.0 };
        assert_eq!(point.to_face_uv(), (0, 0., 0.));
        let point = S2Point { x: 0.0, y: 1.0, z: 0.0 };
        assert_eq!(point.to_face_uv(), (1, 0., 0.));
        let point = S2Point { x: 0.0, y: 0.0, z: 1.0 };
        assert_eq!(point.to_face_uv(), (2, 0., 0.));
        let point = S2Point { x: -1.0, y: 0.0, z: 0.0 };
        assert_eq!(point.to_face_uv(), (3, 0., 0.));
        let point = S2Point { x: 0.0, y: -1.0, z: 0.0 };
        assert_eq!(point.to_face_uv(), (4, 0., 0.));
        let point = S2Point { x: 0.0, y: 0.0, z: -1.0 };
        assert_eq!(point.to_face_uv(), (5, 0., 0.));
    }

    #[test]
    fn to_face_ij() {
        let point =
            S2Point { x: 0.5773502691896258, y: -0.5773502691896258, z: -0.5773502691896258 };
        assert_eq!(point.to_face_ij(None), (5, 0, 1073741823));
        let point =
            S2Point { x: 0.5773502691896258, y: 0.5773502691896258, z: -0.5773502691896258 };
        assert_eq!(point.to_face_ij(None), (5, 1073741823, 1073741823));
        let point = S2Point { x: 0.9999999503294631, y: 0.9999997516473249, z: 1.0 };
        assert_eq!(point.to_face_ij(None), (2, 20, 100));
        let point =
            S2Point { x: 0.5773502691896258, y: -0.5773502691896258, z: -0.5773502691896258 };
        assert_eq!(point.to_face_ij(Some(10)), (5, 0, 1023));
    }

    #[test]
    fn to_lon_lat() {
        let point = S2Point { x: 1.0, y: 0.0, z: 0.0 };
        assert_eq!(point.to_lon_lat::<Point>(), Point(0., 0.));
        let point = S2Point { x: 0.00000000000000006123233995736766, y: 1., z: 0. };
        assert_eq!(point.to_lon_lat::<Point>(), Point(90., 0.));
        let point = S2Point { x: 0.00000000000000006123233995736766, y: 0., z: 1. };
        assert_eq!(point.to_lon_lat::<Point>(), Point(0., 90.));
    }

    #[test]
    fn get_face() {
        let point = S2Point { x: 0.0, y: 1.0, z: 0.0 };
        assert_eq!(point.get_face(), 1);
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn distance() {
        let point1 = S2Point { x: 1.0, y: 0.0, z: 0.0 };
        let point2 = S2Point { x: 0.0, y: 1.0, z: 0.0 };
        assert_eq!(point1.distance(&point2), 1.4142135623730951);
    }

    #[test]
    fn intermediate() {
        let point1 = S2Point { x: 1.0, y: 0.0, z: 0.0 };
        let point2 = S2Point { x: 0.0, y: 1.0, z: 0.0 };
        assert_eq!(point1.intermediate(&point2, 0.5), S2Point { x: 0.5, y: 0.5, z: 0.0 });
    }

    #[test]
    fn add() {
        let point1 = S2Point { x: 1.0, y: 0.0, z: 0.0 };
        let point2 = S2Point { x: 0.0, y: 1.0, z: 0.0 };
        assert_eq!(point1 + point2, S2Point { x: 1.0, y: 1.0, z: 0.0 });
        let f: f64 = 0.5;
        assert_eq!(point1 + f, S2Point { x: 1.5, y: 0.5, z: 0.5 });
    }

    #[test]
    fn sub() {
        let point1 = S2Point { x: 1.0, y: 0.0, z: 0.0 };
        let point2 = S2Point { x: 0.0, y: 1.0, z: 0.0 };
        assert_eq!(point1 - point2, S2Point { x: 1.0, y: -1.0, z: 0.0 });
        let f: f64 = 0.5;
        assert_eq!(point1 - f, S2Point { x: 0.5, y: -0.5, z: -0.5 });
    }

    #[test]
    fn mul() {
        let point1 = S2Point { x: 1.0, y: 0.0, z: 0.0 };
        let point2 = S2Point { x: 0.0, y: 1.0, z: 0.0 };
        assert_eq!(point1 * point2, S2Point { x: 0.0, y: 0.0, z: 0.0 });
        let f: f64 = 0.5;
        assert_eq!(point1 * f, S2Point { x: 0.5, y: 0.0, z: 0.0 });
    }

    #[test]
    fn div() {
        let point1 = S2Point { x: 1.0, y: 0.0, z: 0.5 };
        let point2 = S2Point { x: 1.0, y: 1.0, z: 0.1 };
        assert_eq!(point1 / point2, S2Point { x: 1.0, y: 0.0, z: 5.0 });
        let f: f64 = 0.5;
        assert_eq!(point1 / f, S2Point { x: 2.0, y: 0.0, z: 1.0 });
    }

    #[test]
    fn neg() {
        let point1 = S2Point { x: 1.0, y: 0.0, z: 0.0 };
        assert_eq!(-point1, S2Point { x: -1.0, y: 0.0, z: 0.0 });
    }

    #[test]
    fn dot() {
        let point1 = S2Point { x: 1.0, y: 0.0, z: 0.0 };
        let point2 = S2Point { x: 0.0, y: 1.0, z: 0.0 };
        assert_eq!(point1.dot(&point2), 0.0);
    }

    #[test]
    fn rem() {
        let point1 = S2Point { x: 1.0, y: 0.0, z: 0.0 };
        let f: f64 = 0.5;
        assert_eq!(point1 % f, S2Point { x: 0.0, y: 0.0, z: 0.0 });
    }

    #[test]
    fn rem_assign() {
        let mut point1 = S2Point { x: 1.0, y: 0.0, z: 0.0 };
        let f: f64 = 0.5;
        point1 %= f;
        assert_eq!(point1, S2Point { x: 0.0, y: 0.0, z: 0.0 });
    }

    #[test]
    fn from_s2_cell_id() {
        let id: S2CellId = 1152921504606846977.into();
        assert_eq!(S2Point::from(id), S2Point { x: 1.0, y: 0.0, z: 0.0 });
    }

    #[test]
    fn from_vector_point() {
        let vp: VectorPoint = VectorPoint::new(1., 2., None, None);
        assert_eq!(S2Point::from(&vp), S2Point { x: 1.0, y: 2.0, z: 0.0 });

        let vp: VectorPoint = VectorPoint::new(1., 2., Some(3.), None);
        assert_eq!(S2Point::from(&vp), S2Point { x: 1.0, y: 2.0, z: 3.0 });
    }

    #[test]
    fn cmp() {
        let point1 = S2Point { x: 1.0, y: 0.0, z: 0.0 };
        let point2 = S2Point { x: 0.0, y: 1.0, z: 0.0 };
        assert!(point1 > point2);
        assert!(point2 < point1);
        assert!(point1 != point2);
        let point1 = S2Point { x: 0.0, y: 1.0, z: 0.0 };
        let point2 = S2Point { x: 0.0, y: 0.0, z: 1.0 };
        assert!(point1 > point2);
        assert!(point2 < point1);
        assert!(point1 != point2);
        let point1 = S2Point { x: 0.0, y: 0.0, z: 5.0 };
        let point2 = S2Point { x: 0.0, y: 0.0, z: 2.0 };
        assert!(point1 > point2);
        assert!(point2 < point1);
        assert!(point1 != point2);
        let point1 = S2Point { x: 0.0, y: 0.0, z: 2.0 };
        let point2 = S2Point { x: 0.0, y: 0.0, z: 2.0 };
        assert!(point1 == point2);

        let point1 = S2Point { x: f64::NAN, y: f64::NAN, z: f64::NAN };
        let point2 = S2Point { x: f64::NAN, y: f64::NAN, z: f64::NAN };
        assert!(point1 != point2);
    }

    #[test]
    fn from_face_st() {
        assert_eq!(
            S2Point::from_face_st(0, 0., 0.),
            S2Point { x: 0.5773502691896258, y: -0.5773502691896258, z: -0.5773502691896258 }
        );
        assert_eq!(
            S2Point::from_face_st(1, 0., 0.),
            S2Point { x: 0.5773502691896258, y: 0.5773502691896258, z: -0.5773502691896258 }
        );
        assert_eq!(
            S2Point::from_face_st(2, 0., 0.),
            S2Point { x: 0.5773502691896258, y: 0.5773502691896258, z: 0.5773502691896258 }
        );
        assert_eq!(
            S2Point::from_face_st(3, 0., 0.),
            S2Point { x: -0.5773502691896258, y: 0.5773502691896258, z: 0.5773502691896258 }
        );
        assert_eq!(
            S2Point::from_face_st(4, 0., 0.),
            S2Point { x: -0.5773502691896258, y: -0.5773502691896258, z: 0.5773502691896258 }
        );
        assert_eq!(
            S2Point::from_face_st(5, 0., 0.),
            S2Point { x: -0.5773502691896258, y: -0.5773502691896258, z: -0.5773502691896258 }
        );
    }

    #[test]
    fn from_face_st_gl() {
        assert_eq!(
            S2Point::from_face_st_gl(0, 0., 0.),
            S2Point { x: -0.5773502691896258, y: -0.5773502691896258, z: 0.5773502691896258 }
        );
        assert_eq!(
            S2Point::from_face_st_gl(1, 0., 0.),
            S2Point { x: 0.5773502691896258, y: -0.5773502691896258, z: 0.5773502691896258 }
        );
        assert_eq!(
            S2Point::from_face_st_gl(2, 0., 0.),
            S2Point { x: 0.5773502691896258, y: 0.5773502691896258, z: 0.5773502691896258 }
        );
        assert_eq!(
            S2Point::from_face_st_gl(3, 0., 0.),
            S2Point { x: 0.5773502691896258, y: 0.5773502691896258, z: -0.5773502691896258 }
        );
        assert_eq!(
            S2Point::from_face_st_gl(4, 0., 0.),
            S2Point { x: -0.5773502691896258, y: 0.5773502691896258, z: -0.5773502691896258 }
        );
        assert_eq!(
            S2Point::from_face_st_gl(5, 0., 0.),
            S2Point { x: -0.5773502691896258, y: -0.5773502691896258, z: -0.5773502691896258 }
        );
    }

    #[test]
    fn from_face_uv() {
        assert_eq!(S2Point::from_face_uv(0, 0., 0.), S2Point { x: 1., y: 0., z: 0. });
        assert_eq!(S2Point::from_face_uv(1, 0., 0.), S2Point { x: 0., y: 1., z: 0. });
        assert_eq!(S2Point::from_face_uv(2, 0., 0.), S2Point { x: 0., y: 0., z: 1. });
        assert_eq!(S2Point::from_face_uv(3, 0., 0.), S2Point { x: -1., y: 0., z: 0. });
        assert_eq!(S2Point::from_face_uv(4, 0., 0.), S2Point { x: 0., y: -1., z: 0. });
        assert_eq!(S2Point::from_face_uv(5, 0., 0.), S2Point { x: 0., y: 0., z: -1. });
    }

    #[test]
    fn from_face_uv_gl() {
        assert_eq!(S2Point::from_face_uv_gl(0, 0., 0.), S2Point { x: 0., y: 0., z: 1. });
        assert_eq!(S2Point::from_face_uv_gl(1, 0., 0.), S2Point { x: 1., y: 0., z: 0. });
        assert_eq!(S2Point::from_face_uv_gl(2, 0., 0.), S2Point { x: 0., y: 1., z: 0. });
        assert_eq!(S2Point::from_face_uv_gl(3, 0., 0.), S2Point { x: 0., y: 0., z: -1. });
        assert_eq!(S2Point::from_face_uv_gl(4, 0., 0.), S2Point { x: -1., y: 0., z: 0. });
        assert_eq!(S2Point::from_face_uv_gl(5, 0., 0.), S2Point { x: 0., y: -1., z: 0. });
    }

    #[test]
    fn new_xy() {
        let mut point: S2Point = NewXY::new_xy(1., 2.);
        assert_eq!(point, S2Point { x: 1., y: 2., z: 0. });
        point.set_x(3.);
        point.set_y(4.);
        point.set_z(5.);
        assert_eq!(point, S2Point { x: 3., y: 4., z: 5. });
    }

    #[test]
    fn test_from_lon_lat() {
        assert_eq!(S2Point::from_lon_lat(&Point3D(0., 0., 0.)), S2Point { x: 1., y: 0., z: 0. });
        assert_eq!(
            S2Point::from_lon_lat(&Point3D(90., 0., 0.)),
            S2Point { x: 0.00000000000000006123233995736766, y: 1., z: 0. }
        );
        assert_eq!(
            S2Point::from_lon_lat(&Point3D(0., 90., 0.)),
            S2Point { x: 0.00000000000000006123233995736766, y: 0., z: 1. }
        );
    }

    #[test]
    fn test_from_lon_lat_gl() {
        assert_eq!(S2Point::from_lon_lat_gl(&Point3D(0., 0., 0.)), S2Point { x: 0., y: 0., z: 1. });
        assert_eq!(
            S2Point::from_lon_lat_gl(&Point3D(90., 0., 0.)),
            S2Point { x: 1., y: 0., z: 0.00000000000000006123233995736766 }
        );
        assert_eq!(
            S2Point::from_lon_lat_gl(&Point3D(90., 0., 0.)),
            S2Point { x: 1., y: 0., z: 0.00000000000000006123233995736766 }
        );
        assert_eq!(
            S2Point::from_lon_lat_gl(&Point3D(0., 90., 0.)),
            S2Point { x: 0., y: 1., z: 0.00000000000000006123233995736766 }
        );
    }

    #[test]
    fn math_assignments() {
        // WITH POINTS
        // add
        let mut point = S2Point { x: 1., y: 2., z: 3. };
        point += S2Point { x: 4., y: 5., z: 6. };
        assert_eq!(point, S2Point { x: 5., y: 7., z: 9. });
        // sub
        let mut point = S2Point { x: 1., y: 2., z: 3. };
        point -= S2Point { x: 4., y: 5., z: 6. };
        assert_eq!(point, S2Point { x: -3., y: -3., z: -3. });
        // mul
        let mut point = S2Point { x: 1., y: 2., z: 3. };
        point *= S2Point { x: 4., y: 5., z: 6. };
        assert_eq!(point, S2Point { x: 4., y: 10., z: 18. });
        // div
        let mut point = S2Point { x: 1., y: 2., z: 3. };
        point /= S2Point { x: 4., y: 5., z: 6. };
        assert_eq!(point, S2Point { x: 0.25, y: 0.4, z: 0.5 });

        // WITH FLOATS
        // add
        let mut point = S2Point { x: 1., y: 2., z: 3. };
        point += 4.;
        assert_eq!(point, S2Point { x: 5., y: 6., z: 7. });
        // sub
        let mut point = S2Point { x: 1., y: 2., z: 3. };
        point -= 4.;
        assert_eq!(point, S2Point { x: -3., y: -2., z: -1. });
        // mul
        let mut point = S2Point { x: 1., y: 2., z: 3. };
        point *= 4.;
        assert_eq!(point, S2Point { x: 4., y: 8., z: 12. });
        // div
        let mut point = S2Point { x: 1., y: 2., z: 3. };
        point /= 4.;
        assert_eq!(point, S2Point { x: 0.25, y: 0.5, z: 0.75 });
    }
}
