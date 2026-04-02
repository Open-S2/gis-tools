#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    extern crate alloc;
    extern crate std;

    use alloc::format;
    use gistools::{
        // geometry::{ConvertVectorFeatureWM, TileID},
        geometry::TileID,
        parsers::Buffer,
        tools::{build_contours, build_terrain_mesh, generate_hillshade, vectorize_hillshade},
    };
    use image::GenericImageView;
    use s2json::{FeatureCollection, Features, VectorGeometry};
    use std::{fs, path::PathBuf};

    #[test]
    fn contours_lines() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(format!("tests/tools/elevation/fixtures/13_1556_3084.webp"));
        let elevation_image = fs::read(path).unwrap();
        let image_buffer = Buffer::from(elevation_image);

        let isolines = build_contours(&image_buffer, None, None, None, None, None);

        // uses the gistools::geometry::ConvertVectorFeatureWM trait. uncomment this
        // if wanting to see the data in geojson.io
        // isolines.features = isolines
        //     .features
        //     .iter()
        //     .map(|feature| {
        //         let Features::VectorFeature(vf) = feature else {
        //             panic!("Expected VectorFeature");
        //         };
        //         Features::Feature(vf.to_feature(true))
        //     })
        //     .collect();

        // write isolines to a file
        // let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        // path.push(format!("tests/tools/elevation/fixtures/tmp.json"));
        // fs::write(path, serde_json::to_string_pretty(&isolines).unwrap()).unwrap();

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(format!("tests/tools/elevation/fixtures/13_1556_3084_lines.json"));
        let file_as_str = fs::read_to_string(path).unwrap();
        let expect_data: FeatureCollection = serde_json::from_str(&file_as_str).unwrap();

        assert_eq!(isolines.features.len(), expect_data.features.len());

        for i in 0..isolines.features.len() {
            let feature = isolines.features.get(i).unwrap();
            let expected_feature = expect_data.features.get(i).unwrap();
            let Features::VectorFeature(vf) = &feature else {
                panic!("Expected VectorFeature");
            };
            let VectorGeometry::MultiPolygon(lines) = &vf.geometry else {
                panic!("Expected MultiPolygon");
            };
            let Features::VectorFeature(expected_vf) = &expected_feature else {
                panic!("Expected VectorFeature");
            };
            let VectorGeometry::MultiPolygon(expected_lines) = &expected_vf.geometry else {
                panic!("Expected MultiPolygon");
            };
            assert_eq!(lines, expected_lines);
        }
    }

    #[test]
    fn terrain_mesh() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(format!("tests/tools/elevation/fixtures/fuji.png"));
        let elevation_image = fs::read(path).unwrap();
        let image_buffer = Buffer::from(elevation_image);

        let res_mesh = build_terrain_mesh(&image_buffer, Some(500.0), None, None);

        assert_eq!(
            res_mesh.vertices,
            vec![
                320, 64, 256, 128, 320, 128, 384, 128, 256, 0, 288, 160, 256, 192, 288, 192, 320,
                192, 304, 176, 256, 256, 288, 224, 352, 160, 320, 160, 512, 0, 384, 0, 128, 128,
                128, 0, 64, 64, 64, 0, 0, 0, 32, 32, 192, 192, 384, 384, 512, 256, 384, 256, 320,
                320, 320, 256, 512, 512, 512, 128, 448, 192, 384, 192, 128, 384, 256, 512, 256,
                384, 0, 512, 128, 256, 64, 192, 0, 256, 64, 128, 32, 96, 0, 128, 32, 64, 16, 48, 0,
                64, 0, 32,
            ]
        );

        assert_eq!(
            res_mesh.triangles,
            vec![
                0, 1, 2, 3, 0, 2, 4, 1, 0, 5, 6, 7, 7, 8, 9, 5, 7, 9, 1, 6, 5, 6, 10, 11, 11, 8, 7,
                6, 11, 7, 12, 2, 13, 8, 12, 13, 3, 2, 12, 2, 1, 5, 13, 5, 9, 8, 13, 9, 2, 5, 13, 3,
                14, 15, 15, 4, 0, 3, 15, 0, 16, 4, 17, 18, 17, 19, 19, 20, 21, 18, 19, 21, 16, 17,
                18, 1, 16, 22, 22, 10, 6, 1, 22, 6, 4, 16, 1, 23, 24, 25, 26, 25, 27, 10, 26, 27,
                23, 25, 26, 28, 24, 23, 29, 3, 30, 24, 29, 30, 14, 3, 29, 8, 25, 31, 31, 3, 12, 8,
                31, 12, 27, 8, 11, 10, 27, 11, 25, 8, 27, 25, 24, 30, 30, 3, 31, 25, 30, 31, 32,
                33, 34, 10, 32, 34, 35, 33, 32, 33, 28, 23, 34, 23, 26, 10, 34, 26, 33, 23, 34, 36,
                16, 37, 38, 36, 37, 36, 10, 22, 16, 36, 22, 39, 18, 40, 41, 39, 40, 16, 18, 39, 42,
                21, 43, 44, 42, 43, 18, 21, 42, 21, 20, 45, 45, 44, 43, 21, 45, 43, 44, 41, 40, 40,
                18, 42, 44, 40, 42, 41, 38, 37, 37, 16, 39, 41, 37, 39, 38, 35, 32, 32, 10, 36, 38,
                32, 36,
            ]
        );
    }

    #[test]
    fn test_generate_hillshade() {
        let elevation_image = fs::read("tests/tools/elevation/fixtures/13_1544_3085.webp").unwrap();
        let res = generate_hillshade(
            &Buffer::new(elevation_image),
            TileID::new_wm(13, 1544, 3085),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        assert_eq!(res.width, 514);
        assert_eq!(res.height, 514);
        assert_eq!(res.hillshade.len(), 514 * 514);

        // pull in raw data to compare:
        let expected =
            image::open("tests/tools/elevation/fixtures/13_1544_3085_hillshade.png").unwrap();
        let expected_raw: Vec<u8> = expected
            .pixels()
            .map(|p| p.2[0]) // Just take the R component
            .collect();
        assert_eq!(
            res.hillshade.iter().map(|x| (*x).round() as u8).collect::<Vec<u8>>(),
            expected_raw
        );
    }

    #[test]
    fn test_vector_hillshade() {
        let elevation_image = fs::read("tests/tools/elevation/fixtures/13_1544_3085.webp").unwrap();
        let vectors = vectorize_hillshade(
            &Buffer::new(elevation_image),
            TileID::new_wm(13, 1544, 3085),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        // uses the gistools::geometry::ConvertVectorFeatureWM trait. uncomment this
        // if wanting to see the data in geojson.io
        // vectors.features = vectors
        //     .features
        //     .iter()
        //     .map(|feature| {
        //         let Features::VectorFeature(vf) = feature else {
        //             panic!("Expected VectorFeature");
        //         };
        //         Features::Feature(vf.to_feature(true))
        //     })
        //     .collect();

        // write elevation_image to a file
        // let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        // path.push(format!("tests/tools/elevation/fixtures/tmp.json"));
        // fs::write(path, serde_json::to_string_pretty(&vectors).unwrap()).unwrap();

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(format!(
            "tests/tools/elevation/fixtures/13_1544_3085_vector_hillshade_rust.json"
        ));
        let file_as_str = fs::read_to_string(path).unwrap();
        let expect_data: FeatureCollection = serde_json::from_str(&file_as_str).unwrap();

        assert_eq!(vectors.features.len(), expect_data.features.len());

        for i in 0..vectors.features.len() {
            let feature = vectors.features.get(i).unwrap();
            let expected_feature = expect_data.features.get(i).unwrap();
            let Features::VectorFeature(vf) = &feature else {
                panic!("Expected VectorFeature");
            };
            let VectorGeometry::MultiPolygon(lines) = &vf.geometry else {
                panic!("Expected MultiPolygon");
            };
            let Features::VectorFeature(expected_vf) = &expected_feature else {
                panic!("Expected VectorFeature");
            };
            let VectorGeometry::MultiPolygon(expected_lines) = &expected_vf.geometry else {
                panic!("Expected MultiPolygon");
            };
            assert_eq!(lines, expected_lines);
        }
    }
}
