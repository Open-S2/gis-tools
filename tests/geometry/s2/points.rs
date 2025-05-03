#[cfg(test)]
// #[coverage(off)]
mod tests {
    use gistools::geometry::{S2CellId, S2Point};
    use s2json::VectorPoint;

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
}
