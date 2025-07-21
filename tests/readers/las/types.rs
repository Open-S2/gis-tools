#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::readers::{
        ClassFlag, LASClassification, LASClassification14, LASHeader, LASPoint, LAZCompressor,
        LAZHeaderItemType, WavePacket,
    };
    use s2json::BBox3D;

    #[test]
    fn test_wave_packet() {
        let default = WavePacket::default();
        assert_eq!(
            default.to_bytes(),
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0
            ]
        );
    }

    #[test]
    fn test_las_header() {
        let default = LASHeader::default();
        assert_eq!(default.bbox(), BBox3D::new(0., 0., 0., 0., 0., 0.));
    }

    #[test]
    fn test_las_point() {
        let default = LASPoint::default();
        assert_eq!(default.class_flag(), ClassFlag::Synthetic);
        assert_eq!(default.class_type(false), LASClassification::CreatedNeverClassified);
        assert_eq!(default.class_type(true), LASClassification::CreatedNeverClassified);
        assert_eq!(default.class_type14(), LASClassification14::CreatedNeverClassified);
    }

    #[test]
    fn tess_lazheader_item_type() {
        assert_eq!(LAZHeaderItemType::Byte, LAZHeaderItemType::from(0));
        assert_eq!(LAZHeaderItemType::Short, LAZHeaderItemType::from(1));
        assert_eq!(LAZHeaderItemType::Int, LAZHeaderItemType::from(2));
        assert_eq!(LAZHeaderItemType::Long, LAZHeaderItemType::from(3));
        assert_eq!(LAZHeaderItemType::Float, LAZHeaderItemType::from(4));
        assert_eq!(LAZHeaderItemType::Double, LAZHeaderItemType::from(5));
        assert_eq!(LAZHeaderItemType::Point10, LAZHeaderItemType::from(6));
        assert_eq!(LAZHeaderItemType::GpsTime11, LAZHeaderItemType::from(7));
        assert_eq!(LAZHeaderItemType::Rgb12, LAZHeaderItemType::from(8));
        assert_eq!(LAZHeaderItemType::WavePacket13, LAZHeaderItemType::from(9));
        assert_eq!(LAZHeaderItemType::Point14, LAZHeaderItemType::from(10));
        assert_eq!(LAZHeaderItemType::Rgb14, LAZHeaderItemType::from(11));
        assert_eq!(LAZHeaderItemType::RgbNir14, LAZHeaderItemType::from(12));
        assert_eq!(LAZHeaderItemType::WavePacket14, LAZHeaderItemType::from(13));
        assert_eq!(LAZHeaderItemType::Byte14, LAZHeaderItemType::from(14));
    }

    #[test]
    fn tess_laz_compressor() {
        assert_eq!(LAZCompressor::None, LAZCompressor::from(0));
        assert_eq!(LAZCompressor::Pointwise, LAZCompressor::from(1));
        assert_eq!(LAZCompressor::PointwiseAndChunked, LAZCompressor::from(2));
        assert_eq!(LAZCompressor::LayeredAndChunked, LAZCompressor::from(3));
    }

    #[test]
    fn tess_laz_class_flags() {
        assert_eq!(ClassFlag::Synthetic, ClassFlag::from(0));
        assert_eq!(ClassFlag::KeyPoint, ClassFlag::from(1));
        assert_eq!(ClassFlag::Withheld, ClassFlag::from(2));
        assert_eq!(ClassFlag::Overlap, ClassFlag::from(3));
        assert_eq!(ClassFlag::Unknown, ClassFlag::from(4));
    }

    #[test]
    fn tess_laz_classification_flags() {
        use LASClassification::*;

        assert_eq!(CreatedNeverClassified, LASClassification::from(0));
        assert_eq!(Unclassified, LASClassification::from(1));
        assert_eq!(Ground, LASClassification::from(2));
        assert_eq!(LowVegetation, LASClassification::from(3));
        assert_eq!(MediumVegetation, LASClassification::from(4));
        assert_eq!(HighVegetation, LASClassification::from(5));
        assert_eq!(Building, LASClassification::from(6));
        assert_eq!(LowPointNoise, LASClassification::from(7));
        assert_eq!(ModelKeyPointMassPoint, LASClassification::from(8));
        assert_eq!(Water, LASClassification::from(9));
        assert_eq!(OverlapPoints, LASClassification::from(12));

        // Reserved cases
        assert_eq!(Reserved, LASClassification::from(10));
        assert_eq!(Reserved, LASClassification::from(11));
        assert_eq!(Reserved, LASClassification::from(13));
        assert_eq!(Reserved, LASClassification::from(255));
    }

    #[test]
    fn tess_laz_classification_14_flags() {
        assert_eq!(LASClassification14::CreatedNeverClassified, LASClassification14::from(0));
        assert_eq!(LASClassification14::Unclassified, LASClassification14::from(1));
        assert_eq!(LASClassification14::Ground, LASClassification14::from(2));
        assert_eq!(LASClassification14::LowVegetation, LASClassification14::from(3));
        assert_eq!(LASClassification14::MediumVegetation, LASClassification14::from(4));
        assert_eq!(LASClassification14::HighVegetation, LASClassification14::from(5));
        assert_eq!(LASClassification14::Building, LASClassification14::from(6));
        assert_eq!(LASClassification14::LowPointNoise, LASClassification14::from(7));
        assert_eq!(LASClassification14::ModelKeyPointMassPoint, LASClassification14::from(8));
        assert_eq!(LASClassification14::Water, LASClassification14::from(9));
        assert_eq!(LASClassification14::Rail, LASClassification14::from(10));
        assert_eq!(LASClassification14::RoadSurface, LASClassification14::from(11));
        assert_eq!(LASClassification14::WireGuardShield, LASClassification14::from(13));
        assert_eq!(LASClassification14::WireConductorPhase, LASClassification14::from(14));
        assert_eq!(LASClassification14::TransmissionTower, LASClassification14::from(15));
        assert_eq!(LASClassification14::WireStructureConnector, LASClassification14::from(16));
        assert_eq!(LASClassification14::BridgeDeck, LASClassification14::from(17));
        assert_eq!(LASClassification14::HighNoise, LASClassification14::from(18));
        assert_eq!(LASClassification14::OverheadSructure, LASClassification14::from(19));
        assert_eq!(LASClassification14::IgnoredGround, LASClassification14::from(20));
        assert_eq!(LASClassification14::Snow, LASClassification14::from(21));
        assert_eq!(LASClassification14::TemporalExclusion, LASClassification14::from(22));

        // Reserved (unassigned codes 12, 23..=63)
        assert_eq!(LASClassification14::Reserved, LASClassification14::from(12));
        assert_eq!(LASClassification14::Reserved, LASClassification14::from(23));
        assert_eq!(LASClassification14::Reserved, LASClassification14::from(63));

        // User definable
        assert_eq!(LASClassification14::UserDefinable, LASClassification14::from(64));
        assert_eq!(LASClassification14::UserDefinable, LASClassification14::from(128));
        assert_eq!(LASClassification14::UserDefinable, LASClassification14::from(255));
    }
}
