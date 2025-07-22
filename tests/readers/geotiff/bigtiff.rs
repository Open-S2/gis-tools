#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::{parsers::BufferReader, readers::GeoTIFFReader};
    use std::path::PathBuf;

    #[test]
    fn test_geotiff_bigtiff() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/geotiff/fixtures/bigtiff.tiff");
        let bytes = std::fs::read(path.clone()).unwrap();
        let geotiff = GeoTIFFReader::new(BufferReader::from(bytes), None);

        let mut image = geotiff.get_image(None).unwrap();
        let raster = image.raster_data(None);

        assert_eq!(raster.width, 539);
        assert_eq!(raster.height, 448);
        assert_eq!(raster.alpha, false);
        assert_eq!(raster.min, 0.0);
        assert_eq!(raster.max, 65535.0);
    }

    #[test]
    fn test_geotiff_cog() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/geotiff/fixtures/cog.tiff");
        let bytes = std::fs::read(path.clone()).unwrap();
        let geotiff = GeoTIFFReader::new(BufferReader::from(bytes), None);

        let mut image = geotiff.get_image(None).unwrap();
        let raster = image.raster_data(None);

        assert_eq!(raster.width, 539);
        assert_eq!(raster.height, 448);
        assert_eq!(raster.alpha, false);
        assert_eq!(raster.min, 0.0);
        assert_eq!(raster.max, 65535.0);
    }
}
