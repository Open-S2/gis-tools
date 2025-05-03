#[cfg(test)]
// #[coverage(off)]
mod tests {
    extern crate alloc;

    use alloc::{string::ToString, vec};
    use gistools::geometry::{LonLat, S2CellId, S2Point};
    use s2json::{BBox, MValue, VectorPoint};

    #[test]
    fn new() {
        let id = 1152921504606846977_u64;
        let cid = S2CellId::new(id);
        assert_eq!(cid.id, id);
        let face = S2CellId::from_face(0);
        assert!(face.is_valid());
        assert!(face.is_face());
        let face_child = face.child(0);
        assert!(!face_child.is_face());

        assert_eq!(cid.low_bits(), 1);
        assert_eq!(cid.high_bits(), 268435456);

        let none = S2CellId::none();
        assert_eq!(none.id, 0);

        let sentinel = S2CellId::sentinel();
        assert_eq!(sentinel.id, 18446744073709551615);
    }

    #[test]
    fn to_string() {
        let id = S2CellId::from_face(3).child(0).child(2);
        assert_eq!(id.to_string(), "3/02");

        let string = "3/02";
        let str_id: S2CellId = string.into();
        assert_eq!(str_id, id);

        let string_type = string.to_string();
        let str_type_id: S2CellId = string_type.into();
        assert_eq!(str_type_id, id);

        let invalid = S2CellId::sentinel();
        assert_eq!(invalid.to_string(), "Invalid");
    }

    #[test]
    fn from_face() {
        let id = S2CellId::from_face(0);
        assert_eq!(id.id, 1152921504606846976);
        let id = S2CellId::from_face(1);
        assert_eq!(id.id, 3458764513820540928);
        let id = S2CellId::from_face(2);
        assert_eq!(id.id, 5764607523034234880);
        let id = S2CellId::from_face(3);
        assert_eq!(id.id, 8070450532247928832);
        let id = S2CellId::from_face(4);
        assert_eq!(id.id, 10376293541461622784);
        let id = S2CellId::from_face(5);
        assert_eq!(id.id, 12682136550675316736);
    }

    #[test]
    fn from_lon_lat() {
        let ll: LonLat = LonLat::new(0.0, 0.0, None);
        let id = S2CellId::from_lon_lat(&ll);
        assert_eq!(id.id, 1152921504606846977);
        let ll: LonLat = LonLat::new(90.0, 0.0, None);
        let id = S2CellId::from_lon_lat(&ll);
        assert_eq!(id.id, 3458764513820540929);
        let ll: LonLat = LonLat::new(0.0, 90.0, None);
        let id = S2CellId::from_lon_lat(&ll);
        assert_eq!(id.id, 5764607523034234881);
        let ll: LonLat = LonLat::new(-90.0, 0.0, None);
        let id = S2CellId::from_lon_lat(&ll);
        assert_eq!(id.id, 10376293541461622785);
        let ll: LonLat = LonLat::new(0.0, -90.0, None);
        let id: S2CellId = (&ll).into();
        assert_eq!(id.id, 12682136550675316737);
    }

    #[test]
    fn from_s2point() {
        let p = S2Point { x: 1.0, y: 0.0, z: 0.0 };
        let id = S2CellId::from_s2_point(&p);
        assert_eq!(id.id, 1152921504606846977);
        let p = S2Point { x: 0.0, y: 1.0, z: 0.0 };
        let id = S2CellId::from_s2_point(&p);
        assert_eq!(id.id, 3458764513820540929);
        let p = S2Point { x: 0.0, y: 0.0, z: 1.0 };
        let id = S2CellId::from_s2_point(&p);
        assert_eq!(id.id, 5764607523034234881);
        let p = S2Point { x: -1.0, y: 0.0, z: 0.0 };
        let id: S2CellId = (&p).into();
        assert_eq!(id.id, 8070450532247928833);
    }

    #[test]
    fn from_vector_point() {
        let vp: VectorPoint = VectorPoint::new(1., 0., None, None);
        let id = S2CellId::from(&vp);
        assert_eq!(id.id, 1152921504606846977);
        let vp: VectorPoint = VectorPoint::new(0., 1., Some(0.), None);
        let id = S2CellId::from(&vp);
        assert_eq!(id.id, 3458764513820540929);
    }

    #[test]
    fn to_point() {
        let id = S2CellId::from_face(0);
        let p = id.to_point();
        assert_eq!(p, S2Point { x: 1.0, y: 0.0, z: 0.0 });
        let id = S2CellId::from_face(1);
        let p = id.to_point();
        assert_eq!(p, S2Point { x: 0.0, y: 1.0, z: 0.0 });
        let id = S2CellId::from_face(2);
        let p = id.to_point();
        assert_eq!(p, S2Point { x: 0.0, y: 0.0, z: 1.0 });
        let id = S2CellId::from_face(3);
        let p = id.to_point();
        assert_eq!(p, S2Point { x: -1.0, y: 0.0, z: 0.0 });
    }

    #[test]
    fn get_bound_uv() {
        assert_eq!(S2CellId::from_face(0).get_bound_uv(), BBox::new(-1.0, -1.0, 1.0, 1.0));
        assert_eq!(S2CellId::from_face(0).child(0).get_bound_uv(), BBox::new(-1.0, -1.0, 0.0, 0.0));
        assert_eq!(S2CellId::from_face(1).get_bound_uv(), BBox::new(-1.0, -1.0, 1.0, 1.0));
    }

    #[test]
    fn vertex_neighbors() {
        let id = S2CellId::from_face(0);
        assert_eq!(
            id.vertex_neighbors(None),
            vec![
                S2CellId::new(1152921504606846976),
                S2CellId::new(3458764513820540928),
                S2CellId::new(5764607523034234880)
            ]
        );
        let id: S2CellId = 123974589433424.into();
        assert_eq!(
            id.vertex_neighbors(None),
            vec![
                S2CellId::new(123974589433424),
                S2CellId::new(123974589433584),
                S2CellId::new(123974589433776),
                S2CellId::new(123974589433616)
            ]
        );
    }

    #[test]
    fn neighbors() {
        let id = S2CellId::from_face(0);
        assert_eq!(
            id.neighbors(),
            [
                S2CellId::new(12682136550675316736),
                S2CellId::new(3458764513820540928),
                S2CellId::new(5764607523034234880),
                S2CellId::new(10376293541461622784)
            ]
        );
    }

    #[test]
    fn neighbors_ij() {
        assert_eq!(
            S2CellId::neighbors_ij(0, 0, 0, 0),
            [
                S2CellId::new(12682136550675316736),
                S2CellId::new(3458764513820540928),
                S2CellId::new(5764607523034234880),
                S2CellId::new(10376293541461622784)
            ]
        );
    }

    #[test]
    fn bounds_st() {
        assert_eq!(S2CellId::from_face(0).bounds_st(Some(0)), BBox::new(0., 0., 1., 1.));
        assert_eq!(S2CellId::from_face(0).bounds_st(Some(1)), BBox::new(0.25, 0.25, 0.75, 0.75));
        assert_eq!(
            S2CellId::from_face(0).bounds_st(Some(2)),
            BBox::new(0.375, 0.375, 0.625, 0.625)
        );
        assert_eq!(S2CellId::from_face(1).bounds_st(Some(0)), BBox::new(0., 0., 1., 1.));
    }

    #[test]
    fn next() {
        assert_eq!(S2CellId::from_face(0).next(), S2CellId::new(3458764513820540928));
        assert_eq!(S2CellId::from_face(1).next(), S2CellId::new(5764607523034234880));
        assert_eq!(S2CellId::from_face(2).next(), S2CellId::new(8070450532247928832));
        assert_eq!(S2CellId::from_face(3).next(), S2CellId::new(10376293541461622784));

        let wrap = S2CellId::from_face(5).next().next().next();
        assert_eq!(wrap, S2CellId::from_face(2));
    }

    #[test]
    fn parent() {
        let id = S2CellId::from_face(0);
        let child = id.child(0).child(2).child(3);
        assert_eq!(child.parent(None), id.child(0).child(2));
        assert_eq!(child.parent(Some(0)), id);
    }

    #[test]
    fn range() {
        assert_eq!(
            S2CellId::from_face(0).range(),
            (S2CellId::new(1), S2CellId::new(2305843009213693951))
        );
    }

    #[test]
    fn contains_s2point() {
        let face_0 = S2CellId::from_face(0);
        let point: S2Point = (&LonLat::<MValue>::new(0., 0., None)).into();
        let point2: S2Point = (&LonLat::<MValue>::new(-160., 70., None)).into();
        assert!(face_0.contains_s2point(&point));
        assert!(!face_0.contains_s2point(&point2));
    }

    #[test]
    fn contains() {
        assert!(S2CellId::from_face(0).contains(S2CellId::from_face(0)));
        assert!(!S2CellId::from_face(0).contains(S2CellId::from_face(1)));
        assert!(S2CellId::from_face(0).contains(S2CellId::from_face(0).child(1)));
    }

    #[test]
    fn intersects() {
        assert!(S2CellId::from_face(0).intersects(S2CellId::from_face(0)));
        assert!(!S2CellId::from_face(0).intersects(S2CellId::from_face(1)));
        assert!(S2CellId::from_face(0).intersects(S2CellId::from_face(0).child(1)));
    }

    #[test]
    fn prev() {
        assert_eq!(S2CellId::from_face(1).prev(), S2CellId::new(1152921504606846976));
        assert_eq!(S2CellId::from_face(2).prev(), S2CellId::new(3458764513820540928));
        assert_eq!(S2CellId::from_face(3).prev(), S2CellId::new(5764607523034234880));
        assert_eq!(S2CellId::from_face(4).prev(), S2CellId::new(8070450532247928832));
        assert_eq!(S2CellId::from_face(5).prev(), S2CellId::new(10376293541461622784));

        let id = S2CellId::from_face(2);
        let next2 = id.next().next();
        let prev2 = next2.prev().prev();
        assert_eq!(id, prev2);

        let wrap = S2CellId::from_face(0).prev().prev().prev();
        assert_eq!(wrap, S2CellId::from_face(3));
    }

    #[test]
    fn children() {
        assert_eq!(
            S2CellId::from_face(0).children(None),
            [
                S2CellId::new(288230376151711744),
                S2CellId::new(2017612633061982208),
                S2CellId::new(1441151880758558720),
                S2CellId::new(864691128455135232),
            ]
        );
        assert_eq!(
            S2CellId::from_face(0).children(Some(0)),
            [
                S2CellId::new(288230376151711744),
                S2CellId::new(864691128455135232),
                S2CellId::new(1441151880758558720),
                S2CellId::new(2017612633061982208),
            ]
        );
    }

    #[test]
    fn children_ij() {
        assert_eq!(
            S2CellId::children_ij(0, 0, 0, 0),
            [
                S2CellId::new(288230376151711744),
                S2CellId::new(2017612633061982208),
                S2CellId::new(864691128455135232),
                S2CellId::new(1441151880758558720),
            ]
        )
    }

    #[test]
    fn pos() {
        assert_eq!(S2CellId::from_face(0).pos(), 1152921504606846976);
        assert_eq!(S2CellId::from_face(1).pos(), 1152921504606846976);
        assert_eq!(S2CellId::from_face(2).pos(), 1152921504606846976);
        assert_eq!(S2CellId::from_face(3).pos(), 1152921504606846976);
    }

    #[test]
    fn distance() {
        assert_eq!(S2CellId::from_face(0).distance(Some(0)), 0.);
        assert_eq!(S2CellId::from_face(0).distance(Some(1)), 2.);
        assert_eq!(S2CellId::from_face(0).distance(Some(2)), 8.);
        assert_eq!(S2CellId::from_face(0).distance(Some(3)), 32.);
    }

    #[test]
    fn from_distance() {
        assert_eq!(S2CellId::from_distance(0, None), S2CellId::new(1));
        assert_eq!(S2CellId::from_distance(1, None), S2CellId::new(3));
        assert_eq!(S2CellId::from_distance(2, None), S2CellId::new(5));
        assert_eq!(S2CellId::from_distance(3, None), S2CellId::new(7));
        assert_eq!(S2CellId::from_distance(4, None), S2CellId::new(9));
        assert_eq!(S2CellId::from_distance(5, None), S2CellId::new(11));
    }

    #[test]
    fn from_face_st() {
        assert_eq!(S2CellId::from_face_st(0, 0., 0.), S2CellId::new(1));
    }

    #[test]
    fn from_face_uv() {
        assert_eq!(S2CellId::from_face_uv(0, 0., 0.), S2CellId::new(1152921504606846977));
    }

    #[test]
    fn get_size_ij() {
        let id = S2CellId::from_face(0);
        let child = id.child(0).child(2).child(1).child(2).child(3);
        assert_eq!(id.get_size_ij(), 1073741824);
        assert_eq!(child.get_size_ij(), 33554432);
    }

    #[test]
    fn get_vertices_raw() {
        let id = S2CellId::from_face(0);
        assert_eq!(
            id.get_vertices_raw(),
            [
                S2Point { x: 1., y: -1., z: -1. },
                S2Point { x: 1., y: 1., z: -1. },
                S2Point { x: 1., y: 1., z: 1. },
                S2Point { x: 1., y: -1., z: 1. },
            ]
        );

        let level_10 = S2CellId::from_face_ij(0, 10, 20, Some(10));
        assert_eq!(
            level_10.get_vertices_raw(),
            [
                S2Point { x: 1., y: -0.9740854899088541, z: -0.94842529296875 },
                S2Point { x: 1., y: -0.9715080261230469, z: -0.94842529296875 },
                S2Point { x: 1., y: -0.9715080261230469, z: -0.9458732604980469 },
                S2Point { x: 1., y: -0.9740854899088541, z: -0.9458732604980469 },
            ]
        );
    }

    #[test]
    fn get_vertices() {
        let id = S2CellId::from_face(0);
        assert_eq!(
            id.get_vertices(),
            [
                S2Point { x: 0.5773502691896258, y: -0.5773502691896258, z: -0.5773502691896258 },
                S2Point { x: 0.5773502691896258, y: 0.5773502691896258, z: -0.5773502691896258 },
                S2Point { x: 0.5773502691896258, y: 0.5773502691896258, z: 0.5773502691896258 },
                S2Point { x: 0.5773502691896258, y: -0.5773502691896258, z: 0.5773502691896258 }
            ]
        );

        let level_10 = S2CellId::from_face_ij(0, 10, 20, Some(10));
        assert_eq!(
            level_10.get_vertices(),
            [
                S2Point { x: 0.5925201015153633, y: -0.5771652333654366, z: -0.5619610508695819 },
                S2Point { x: 0.5930423748666049, y: -0.5761454270139794, z: -0.5624563881257431 },
                S2Point { x: 0.593547171020095, y: -0.576635840528651, z: -0.5614203979121691 },
                S2Point { x: 0.5930235640377648, y: -0.5776556489032209, z: -0.5609251320685729 }
            ]
        );
    }

    #[test]
    fn get_edges_raw() {
        let id = S2CellId::from_face(0);
        assert_eq!(
            id.get_edges_raw(),
            [
                S2Point { x: 1.0, y: 0.0, z: 1.0 },
                S2Point { x: 1.0, y: -1.0, z: 0.0 },
                S2Point { x: 1.0, y: -0.0, z: -1.0 },
                S2Point { x: 1.0, y: 1.0, z: -0.0 }
            ]
        );

        let level_10 = S2CellId::from_face_ij(0, 10, 20, Some(10));
        assert_eq!(
            level_10.get_edges_raw(),
            [
                S2Point { x: 0.94842529296875, y: 0.0, z: 1.0 },
                S2Point { x: -0.9715080261230469, y: -1.0, z: 0.0 },
                S2Point { x: -0.9458732604980469, y: -0.0, z: -1.0 },
                S2Point { x: 0.9740854899088541, y: 1.0, z: -0.0 }
            ]
        );
    }

    #[test]
    fn get_edges() {
        let id = S2CellId::from_face(0);
        assert_eq!(
            id.get_edges(),
            [
                S2Point { x: 0.7071067811865475, y: 0.0, z: 0.7071067811865475 },
                S2Point { x: 0.7071067811865475, y: -0.7071067811865475, z: 0.0 },
                S2Point { x: 0.7071067811865475, y: -0.0, z: -0.7071067811865475 },
                S2Point { x: 0.7071067811865475, y: 0.7071067811865475, z: -0.0 }
            ]
        );

        let level_10 = S2CellId::from_face_ij(0, 10, 20, Some(10));
        assert_eq!(
            level_10.get_edges(),
            [
                S2Point { x: 0.6881486685943737, y: 0.0, z: 0.7255697140260132 },
                S2Point { x: -0.696815003909965, y: -0.7172508977519342, z: 0.0 },
                S2Point { x: -0.6871719848800082, y: -0.0, z: -0.7264947785057162 },
                S2Point { x: 0.6977642240401561, y: 0.7163275002745871, z: -0.0 }
            ]
        );
    }
}
