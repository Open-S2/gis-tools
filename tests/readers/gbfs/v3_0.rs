#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use crate::spawn_test_server;
    use gistools::{parsers::FeatureReader, readers::GBFSReader};
    use s2json::VectorFeature;
    use std::{format, path::PathBuf};

    #[test]
    fn test_gbfs_reader_v3() {
        // `http://localhost:${server.port}/readers/gbfs/fixtures/v1.1/gbfs.json`,
        smol::block_on(async {
            let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let path_str: String = path.to_str().unwrap().into();
            let server = spawn_test_server(&path_str);

            let url = format!("{}/tests/readers/gbfs/fixtures/v3.0/gbfs.json", server);
            let reader_v3 = GBFSReader::from_url(&url, None).await;

            let features = reader_v3.iter().collect::<Vec<VectorFeature>>();
            assert_eq!(features.len(), 106);

            let features: Vec<_> = (0..3usize)
                .into_iter()
                .flat_map(|thread_id| {
                    let reader = reader_v3.clone();
                    let res: Vec<_> = reader.par_iter(3, thread_id).collect();
                    res
                })
                .collect();
            assert_eq!(features.len(), 106);

            if let GBFSReader::V3(v3) = reader_v3 {
                let features = v3.iter().collect::<Vec<VectorFeature>>();
                assert_eq!(features.len(), 106);

                let features: Vec<_> = (0..3usize)
                    .into_iter()
                    .flat_map(|thread_id| {
                        let reader = v3.clone();
                        let res: Vec<_> = reader.par_iter(3, thread_id).collect();
                        res
                    })
                    .collect();
                assert_eq!(features.len(), 106);
            } else {
                panic!("Expected GBFSReader::V3");
            }
        });
    }
}
