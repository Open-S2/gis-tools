#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::{
        parsers::FeatureReader,
        readers::{SHPHeader, mmap::shapefile_from_path},
    };
    use s2json::{BBox3D, MValue, VectorFeature, VectorFeatureType, VectorGeometry, VectorPoint};
    use std::{collections::BTreeMap, path::PathBuf};

    #[test]
    fn test_shapefile_path() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/shapefile/fixtures/utf.shp");
        let path_str = path.to_str().unwrap();

        #[derive(Default, Debug, Clone, MValue, PartialEq)]
        struct Props {
            field: String,
        }

        let shp = shapefile_from_path(path_str, BTreeMap::from([("a".into(), "b".into())]));

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

        let features: Vec<VectorFeature<(), Props, MValue>> = shp.iter().collect();
        assert_eq!(features.len(), 2);

        assert_eq!(
            features,
            vec![
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: Some(1),
                    properties: Props { field: "💩".into() },
                    geometry: VectorGeometry::new_point(
                        VectorPoint::new_xy(
                            -108.6328125,
                            41.244772343082076,
                            Some(MValue::default())
                        ),
                        None
                    ),
                    ..Default::default()
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: Some(2),
                    properties: Props { field: "Hněvošický háj".into() },
                    geometry: VectorGeometry::new_point(
                        VectorPoint::new_xy(
                            -108.97956848144531,
                            41.253032440653186,
                            Some(MValue::default())
                        ),
                        None
                    ),
                    ..Default::default()
                }
            ]
        )
    }
}
