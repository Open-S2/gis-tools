use std::path::PathBuf;

use gistools::{parsers::BufferReader, readers::GeoTIFFReader};

#[test]
fn test_geotiff_rgba() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/readers/geotiff/fixtures/RGBA.tiff");
    let bytes = std::fs::read(path.clone()).unwrap();
    let geotiff = GeoTIFFReader::new(BufferReader::from(bytes), None);

    let mut image = geotiff.get_image(None).unwrap();
    // let raster = image.raster_data(None);
    let _rgb = image.get_rgba();
}
