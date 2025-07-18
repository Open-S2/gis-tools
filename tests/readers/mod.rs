mod csv;
mod geotiff;
mod gpx;
mod grib2;
mod gtfs;
mod json;
mod las;
mod nadgrid;
mod netcdf;
mod osm;
mod pmtiles;
mod shapefile;
mod tile;
mod wkt;

#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::readers::ReaderType;

    #[test]
    fn test_reader_type() {
        assert_eq!(ReaderType::from("csv"), ReaderType::CSV);
        assert_eq!(ReaderType::from("geotiff"), ReaderType::GeoTIFF);
        assert_eq!(ReaderType::from("gpx"), ReaderType::GPX);
        assert_eq!(ReaderType::from("grib2"), ReaderType::GRIB2);
        assert_eq!(ReaderType::from("gtfs"), ReaderType::GTFS);
        assert_eq!(ReaderType::from("json"), ReaderType::JSON);
        assert_eq!(ReaderType::from("jsonld"), ReaderType::JSONLD);
        assert_eq!(ReaderType::from("jsonsq"), ReaderType::JSONSQ);
        assert_eq!(ReaderType::from("las"), ReaderType::LAS);
        assert_eq!(ReaderType::from("laz"), ReaderType::LAZ);
        assert_eq!(ReaderType::from("nadgrid"), ReaderType::NADGrid);
        assert_eq!(ReaderType::from("netcdf"), ReaderType::NetCDF);
        assert_eq!(ReaderType::from("osm"), ReaderType::OSM);
        assert_eq!(ReaderType::from("shapefile"), ReaderType::Shapefile);
        assert_eq!(ReaderType::from("tile"), ReaderType::Tile);
        assert_eq!(ReaderType::from("wkt"), ReaderType::WKT);
    }
}
