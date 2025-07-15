#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    extern crate alloc;

    use alloc::string::String;
    use gistools::{
        parsers::{FeatureReader, FileReader},
        readers::{DataBaseFile, SHPHeader, ShapeFileReader},
    };
    use s2json::{
        BBox3D, MValue, MValueCompatible, VectorFeature, VectorFeatureType, VectorGeometry,
        VectorPoint,
    };
    use std::path::PathBuf;

    #[test]
    fn test_shapefile() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/shapefile/fixtures/utf.shp");

        let shp: ShapeFileReader<FileReader, MValue> =
            ShapeFileReader::new(FileReader::from(path.clone()), None, None);

        let header = shp.get_header();
        assert_eq!(
            header,
            &SHPHeader {
                bbox: BBox3D::new(
                    -108.97956848144531,
                    41.244772343082076,
                    -108.6328125,
                    41.253032440653186,
                    0.,
                    0.
                ),
                length: 156,
                shp_code: 1,
                version: 1000
            }
        );

        let features: Vec<_> = shp.iter().collect();
        assert_eq!(features.len(), 2);

        assert_eq!(
            features,
            vec![
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: Some(1),
                    geometry: VectorGeometry::new_point(
                        VectorPoint::new_xy(-108.6328125, 41.244772343082076, Some(())),
                        None
                    ),
                    ..Default::default()
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: Some(2),
                    geometry: VectorGeometry::new_point(
                        VectorPoint::new_xy(-108.97956848144531, 41.253032440653186, Some(())),
                        None
                    ),
                    ..Default::default()
                }
            ]
        )
    }

    #[test]
    fn test_shapefile_with_utf() {
        #[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
        struct FieldStruct {
            field: String,
        }
        impl FieldStruct {
            fn new(field: String) -> Self {
                Self { field }
            }
        }

        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let pbf_path = path.join("tests/readers/shapefile/fixtures/utf.shp");
        let dbf_path = path.join("tests/readers/shapefile/fixtures/utf.dbf");

        let dbf: DataBaseFile<FileReader, FieldStruct> =
            DataBaseFile::new(FileReader::from(dbf_path.clone()), Some("utf-8".into()));
        let shp: ShapeFileReader<FileReader, FieldStruct> =
            ShapeFileReader::new(FileReader::from(pbf_path.clone()), Some(dbf), None);

        let header = shp.get_header();
        assert_eq!(
            header,
            &SHPHeader {
                bbox: BBox3D::new(
                    -108.97956848144531,
                    41.244772343082076,
                    -108.6328125,
                    41.253032440653186,
                    0.,
                    0.
                ),
                length: 156,
                shp_code: 1,
                version: 1000
            }
        );

        let features: Vec<_> = shp.iter().collect();
        assert_eq!(features.len(), 2);

        assert_eq!(
            features,
            vec![
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: Some(1),
                    properties: FieldStruct::new("💩".into()),
                    geometry: VectorGeometry::new_point(
                        VectorPoint::new_xy(-108.6328125, 41.244772343082076, Some(())),
                        None
                    ),
                    ..Default::default()
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: Some(2),
                    properties: FieldStruct::new("Hněvošický háj".into()),
                    geometry: VectorGeometry::new_point(
                        VectorPoint::new_xy(-108.97956848144531, 41.253032440653186, Some(())),
                        None
                    ),
                    ..Default::default()
                }
            ]
        );
    }

    #[test]
    fn test_shapefile_polylines() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/shapefile/fixtures/export_polylinez.shp");

        let shp: ShapeFileReader<FileReader, MValue> =
            ShapeFileReader::new(FileReader::from(path.clone()), None, None);

        let header = shp.get_header();
        assert_eq!(
            header,
            &SHPHeader {
                bbox: BBox3D::new(-120., 38., -113., 45., 0., 0.),
                length: 384,
                shp_code: 13,
                version: 1000
            }
        );

        let features: Vec<_> = shp.iter().collect();
        assert_eq!(features.len(), 1);

        assert_eq!(
            features,
            vec![VectorFeature {
                _type: VectorFeatureType::VectorFeature,
                id: Some(1),
                geometry: VectorGeometry::new_multilinestring(
                    vec![
                        vec![
                            VectorPoint::new_xyz(-120., 45., 800., Some(())),
                            VectorPoint::new_xyz(-119., 44., 1100., Some(())),
                            VectorPoint::new_xyz(-118., 43., 2300., Some(())),
                        ],
                        vec![
                            VectorPoint::new_xyz(-115., 40., 0., Some(())),
                            VectorPoint::new_xyz(-114., 39., 0., Some(())),
                            VectorPoint::new_xyz(-113., 38., 0., Some(())),
                        ],
                    ],
                    Some(BBox3D::new(-120., 38., -113., 45., 0., 2300.)),
                ),
                ..Default::default()
            }]
        )
    }

    #[test]
    fn test_shapefile_multipointz() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/shapefile/fixtures/export_multipointz.shp");

        let shp: ShapeFileReader<FileReader, MValue> =
            ShapeFileReader::new(FileReader::from(path.clone()), None, None);

        let header = shp.get_header();
        assert_eq!(
            header,
            &SHPHeader {
                bbox: BBox3D::new(-123., 46., -121., 48., 0., 0.),
                length: 276,
                shp_code: 18,
                version: 1000
            }
        );

        let features: Vec<_> = shp.iter().collect();
        assert_eq!(features.len(), 1);

        assert_eq!(
            features,
            vec![VectorFeature {
                _type: VectorFeatureType::VectorFeature,
                id: Some(0),
                geometry: VectorGeometry::new_multipoint(
                    vec![
                        VectorPoint::new_xyz(-123., 48., 1200., None),
                        VectorPoint::new_xyz(-122., 47., 2500., None),
                        VectorPoint::new_xyz(-121., 46., 3600., None),
                    ],
                    Some(BBox3D::new(-123., 46., -121., 48., 1200., 3600.)),
                ),
                ..Default::default()
            }]
        )
    }
}
