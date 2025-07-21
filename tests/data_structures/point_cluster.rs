#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    extern crate alloc;
    extern crate std;

    use alloc::{vec, vec::Vec};
    use gistools::{
        data_structures::{ClusterData, ClusterOptions, LocalClusterStore},
        geometry::{LonLat, S2CellId},
        parsers::FileReader,
        readers::json::JSONReader,
    };
    use s2json::{
        MValueCompatible, Projection, VectorFeature, VectorFeatureType, VectorGeometry, VectorPoint,
    };
    use serde::{Deserialize, Serialize};
    use std::path::PathBuf;

    #[test]
    fn basic_test() {
        #[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
        struct Test {
            a: i32,
        }

        let mut cluster_store = LocalClusterStore::new(
            Some(ClusterOptions { maxzoom: Some(4), ..Default::default() }),
            None,
        );
        cluster_store.insert_lon_lat(LonLat::new(0., 0., Some(Test { a: 22 })));
        cluster_store.insert_lon_lat(LonLat::new(2., -2., Some(Test { a: 0 })));
        cluster_store.insert_lon_lat(LonLat::new(1., -1., Some(Test { a: 1 })));
        cluster_store.insert_point(LonLat::new(-160., 60., None), Some(Test { a: 2 }));
        cluster_store.insert_face_st(0.into(), 0.25, 0.25, Test { a: 3 });

        cluster_store.build_clusters(None);

        let tile_0 = cluster_store.get_tile(S2CellId::from_face(0));
        let default_layer = tile_0.layers.get("default").unwrap();

        let expected: Vec<VectorFeature<(), Test, ClusterData>> = vec![
            VectorFeature {
                _type: VectorFeatureType::VectorFeature,
                face: 0.into(),
                properties: Test { a: 3 },
                geometry: VectorGeometry::new_point(
                    VectorPoint {
                        x: 0.25,
                        y: 0.25,
                        z: None,
                        m: Some(ClusterData { count: 1 }),
                        t: None,
                    },
                    None,
                ),
                ..Default::default()
            },
            VectorFeature {
                _type: VectorFeatureType::VectorFeature,
                face: 0.into(),
                properties: Test { a: 22 },
                geometry: VectorGeometry::new_point(
                    VectorPoint {
                        x: 0.5129216695148089,
                        y: 0.4870724987162167,
                        z: None,
                        m: Some(ClusterData { count: 3 }),
                        t: None,
                    },
                    None,
                ),
                ..Default::default()
            },
        ];
        assert_eq!(&default_layer.features, &expected);
    }

    #[test]
    fn test_from_reader() {
        #[derive(Debug, Default, Clone, MValueCompatible, PartialEq, Serialize, Deserialize)]
        #[serde(default)]
        struct Props {
            name: String,
        }

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path = path.join("tests/readers/json/fixtures/multipoint.geojson");

        let reader: JSONReader<FileReader, (), Props, Props> =
            JSONReader::new(FileReader::from(path));

        let mut cluster_store = LocalClusterStore::new(
            Some(ClusterOptions {
                maxzoom: Some(4),
                projection: Some(Projection::WG),
                ..Default::default()
            }),
            None,
        );
        cluster_store.insert_reader(&reader);
    }
}
