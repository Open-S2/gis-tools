#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    extern crate alloc;

    use gistools::{
        parsers::{FeatureReader, FileReader},
        readers::{GISReader, NadGridHeader, NadGridReader, ReaderType},
    };
    use s2json::VectorPoint;
    use std::path::PathBuf;

    #[test]
    fn test_nadgrid_reader() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/proj4/fixtures/BETA2007.gsb");

        let nadgrid_reader = NadGridReader::new("test".into(), FileReader::from(path.clone()));

        assert!(!nadgrid_reader.is_empty());
        assert_eq!(nadgrid_reader.len(), 1);

        assert_eq!(
            *nadgrid_reader.header(),
            NadGridHeader {
                n_fields: 11,
                n_subgrid_fields: 11,
                n_subgrids: 1,
                shift_type: "SECONDS ".into(),
                from_semi_major_axis: 6377397.155,
                from_semi_minor_axis: 6356078.963,
                to_semi_major_axis: 6378137.0,
                to_semi_minor_axis: 6356752.314,
            }
        );

        let first_mp = nadgrid_reader.get_points(0).unwrap();
        assert_eq!(first_mp.len(), 5_208);

        let doesnt_exist = nadgrid_reader.get_points(10_000);
        assert!(doesnt_exist.is_none());

        assert_eq!(
            first_mp[0],
            VectorPoint::new(3.47407399194824e-5, -1.3331145212039613e-5, None, None)
        );

        let features = nadgrid_reader.iter().collect::<Vec<_>>();
        assert_eq!(features.len(), 1);

        let features = nadgrid_reader.par_iter(1, 0).collect::<Vec<_>>();
        assert_eq!(features.len(), 1);
    }

    #[test]
    fn test_nadgrid_gis_reader() {
        // file
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/proj4/fixtures/BETA2007.gsb");
        let gis_reader = GISReader::from_path(path.clone(), None, None);
        assert_eq!(gis_reader.get_type(), ReaderType::NADGrid);
        let features: Vec<_> = gis_reader.iter().collect();
        assert_eq!(features.len(), 1);

        // buffer
        let bytes = std::fs::read(path.clone()).unwrap();
        let gis_reader = GISReader::from_buffer(bytes, ReaderType::NADGrid, None);
        let features: Vec<_> = gis_reader.par_iter(1, 0).collect();
        assert_eq!(features.len(), 1);
    }
}
