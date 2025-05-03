#[cfg(test)]
// #[coverage(off)]
mod tests {
    use gistools::geometry::{S1Angle, S1ChordAngle, S2Cap, S2CellId, S2Point};

    #[test]
    fn new() {
        let cap = S2Cap::new(S2Point::new(1.0, 0.0, 0.0), S1ChordAngle::zero(), ());
        assert_eq!(cap.center, S2Point::new(1.0, 0.0, 0.0));
        assert_eq!(cap.radius, S1ChordAngle::zero());
        assert_eq!(cap.radius(), S1Angle::new(0.0));
        assert_eq!(cap.area(), 0.0);
    }

    #[test]
    fn empty() {
        let cap = S2Cap::empty(());
        assert_eq!(cap.center, S2Point::new(1.0, 0.0, 0.0));
        assert_eq!(cap.radius, S1ChordAngle::negative_angle());
        assert_eq!(cap.area(), 0.0);
        assert!(cap.is_empty());
        assert!(!cap.is_full());
    }

    #[test]
    fn full() {
        let cap = S2Cap::full(());
        assert_eq!(cap.center, S2Point::new(1.0, 0.0, 0.0));
        assert_eq!(cap.radius, S1ChordAngle::straight_angle());
        assert_eq!(cap.area(), 12.566370614359172);
        assert!(!cap.is_empty());
        assert!(cap.is_full());
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn area() {
        let face = S2CellId::from_face(0);
        let cap = S2Cap::from_s1_chord_angle(face.into(), S1ChordAngle::new(1.), Some(1));
        assert_eq!(cap.area(), 3.141592653589793);
        assert_eq!(cap.data, Some(1));
    }

    #[test]
    fn height() {
        let face = S2CellId::from_face(0);
        let cap = S2Cap::from_s1_chord_angle(face.into(), S1ChordAngle::new(1.), Some(1));
        assert_eq!(cap.height(), 0.5);
    }

    #[test]
    fn from_s1_angle() {
        let face = S2CellId::from_face(0);
        let cap = S2Cap::from_s1_angle(face.into(), S1Angle::new(1.), Some(1));
        assert_eq!(cap.radius, S1ChordAngle::new(0.9193953882637206));
    }

    #[test]
    fn from_s1_chord_angle() {
        let face = S2CellId::from_face(0);
        let cap = S2Cap::from_s1_chord_angle(face.into(), S1ChordAngle::new(1.), Some(1));
        assert_eq!(cap.radius, S1ChordAngle::new(1.));
    }

    #[test]
    fn from_s2_point() {
        let cap = S2Cap::from_s2_point(S2Point { x: 1., y: 0., z: 0. }, Some(1));
        assert_eq!(cap.radius, S1ChordAngle::zero());
    }

    #[test]
    fn contains_s2_cell() {
        let face = S2CellId::from_face(0);
        let sub_point = S2CellId::from_face_ij(0, 10, 10, Some(5));
        let sub_point2 = S2CellId::from_face_ij(3, 10, 10, Some(6));
        let cap = S2Cap::from_s1_chord_angle(face.into(), S1ChordAngle::new(1.), Some(1));
        assert!(cap.contains_s2_cell(sub_point));
        assert!(!cap.contains_s2_cell(sub_point2));

        let empty = S2Cap::empty(Some(1));
        assert!(!empty.contains_s2_cell(sub_point));

        let full = S2Cap::full(Some(1));
        assert!(full.contains_s2_cell(sub_point));
    }

    #[test]
    fn contains_s2_point() {
        let face = S2CellId::from_face(0);
        let sub_point = S2CellId::from_face_ij(0, 10, 10, Some(5));
        let sub_point2 = S2CellId::from_face_ij(3, 10, 10, Some(6));
        let cap = S2Cap::from_s1_chord_angle(face.into(), S1ChordAngle::new(1.), Some(1));

        assert!(cap.contains_s2_point(&sub_point.into()));
        assert!(!cap.contains_s2_point(&sub_point2.into()));
    }

    #[test]
    fn complement() {
        let cap = S2Cap::from_s1_chord_angle(
            S2Point { x: 1., y: 0., z: 0. },
            S1ChordAngle::new(1.),
            Some(1),
        );
        let comp = cap.complement();
        assert_eq!(comp.center, S2Point { x: -1., y: 0., z: 0. });
        assert_eq!(comp.radius, S1ChordAngle::new(3.));

        let cap = S2Cap::full(Some(()));
        let comp = cap.complement();
        assert!(comp.is_empty());
        let cap = S2Cap::empty(Some(()));
        let comp = cap.complement();
        assert!(comp.is_full());
    }

    #[test]
    fn intersects_s2_cell_fast() {
        let face = S2CellId::from_face(0);
        let cap = S2Cap::from_s1_chord_angle(face.into(), S1ChordAngle::new(0.95), Some(1));

        assert!(cap.intersects_s2_cell_fast(13546827679130451968.into()));
        assert!(cap.intersects_s2_cell_fast(12970366926827028480.into()));
        assert!(cap.intersects_s2_cell_fast(10664523917613334528.into()));
        assert!(cap.intersects_s2_cell_fast(10088063165309911040.into()));
        assert!(cap.intersects_s2_cell_fast(5476377146882523136.into()));
        assert!(cap.intersects_s2_cell_fast(4899916394579099648.into()));
        assert!(cap.intersects_s2_cell_fast(4323455642275676160.into()));
        assert!(cap.intersects_s2_cell_fast(2594073385365405696.into()));
        assert!(!cap.intersects_s2_cell_fast(3746994889972252672.into()));
    }

    #[test]
    fn get_intersecting_cells() {
        let face = S2CellId::from_face(0);
        let cap = S2Cap::from_s1_chord_angle(face.into(), S1ChordAngle::new(1.), Some(1));

        let mut expected: Vec<S2CellId> = vec![
            13546827679130451968.into(),
            12970366926827028480.into(),
            10664523917613334528.into(),
            10088063165309911040.into(),
            5476377146882523136.into(),
            4899916394579099648.into(),
            4323455642275676160.into(),
            2594073385365405696.into(),
            1152921504606846976.into(),
        ];
        let mut result = cap.get_intersecting_cells();

        expected.sort_unstable();
        result.sort_unstable();

        assert_eq!(result, expected);
    }
}
