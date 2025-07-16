#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use std::{
        io::Cursor,
        path::{Path, PathBuf},
    };

    use gistools::{
        data_structures::{GridOptions, LocalPointGrid},
        geometry::{LonLat, S2CellId},
        parsers::RGBA,
        readers::{RasterTileFetcher, TileFetcher},
        util::InterpolationMethod,
    };
    use image::{ImageBuffer, ImageReader, RgbaImage};
    use s2json::Projection;

    // TODO: insert_feature

    #[test]
    fn basic_wm_test() {
        // prep expected
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path = path.join("tests/data_structures/fixtures/wm/wmTileFromWM_rust.png");
        let expected_image_data = std::fs::read(path).unwrap();
        let image_reader = ImageReader::new(Cursor::new(expected_image_data));
        let expected_image = image_reader.with_guessed_format().unwrap().decode().unwrap();
        let image_data = expected_image.to_rgba8().into_vec();

        // setup grid
        let mut grid = LocalPointGrid::<RGBA>::new(Some(GridOptions {
            projection: Some(Projection::WG),
            minzoom: Some(0),
            maxzoom: Some(1),
            buffer_size: Some(0),
            maxzoom_interpolation: Some(InterpolationMethod::IDW),
            ..Default::default()
        }));

        // grab reader
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path = path.join("tests/readers/tile/fixtures/wm/satellite");
        let reader = RasterTileFetcher::<RGBA>::new(path, Some(1));

        // insert data
        grid.insert_reader(&reader);
        grid.insert_lon_lat(LonLat::new(0.0, 0.0, Some(RGBA::from_u8s(255, 0, 0, 255))));
        grid.insert_point(LonLat::new(0.0, 0.0, None), Some(RGBA::from_u8s(255, 0, 0, 255)));
        grid.insert_face_st(0.into(), 0., 0., RGBA::from_u8s(0, 255, 0, 255));
        grid.build_clusters();

        // get face
        let tile0 = grid.get_tile(S2CellId::from_face(0));
        if let Some(tile0) = tile0 {
            let data = tile0.data;
            let image: Vec<u8> = data
                .iter()
                .flat_map(|rgba| {
                    let (r, g, b, a) = rgba.to_u8s();
                    [r, g, b, a]
                })
                .collect();

            // // save the image
            // let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            // path = path.join("tests/data_structures/fixtures/wm/wmTileFromWM_rust.png");
            // write_png(&path, image.clone(), tile0.size, tile0.size);

            assert_eq!(image, image_data);
        } else {
            panic!("Tile is undefined");
        }
    }

    #[allow(dead_code)]
    fn write_png(path: &Path, image: Vec<u8>, width: u32, height: u32) {
        assert_eq!(image.len(), (width * height * 4) as usize);
        let buffer: RgbaImage = ImageBuffer::from_raw(width, height, image)
            .expect("Invalid image dimensions or buffer size");
        buffer.save(path).expect("Failed to write PNG");
    }
}
