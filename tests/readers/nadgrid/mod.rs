#[cfg(test)]
// #[coverage(off)]
mod tests {
    extern crate alloc;

    use gistools::{
        parsers::{FeatureReader, FileReader},
        readers::{NadGridHeader, NadGridReader},
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

        assert_eq!(
            first_mp[0],
            VectorPoint::new(3.47407399194824e-5, -1.3331145212039613e-5, None, None)
        );

        let features = nadgrid_reader.iter().collect::<Vec<_>>();
        assert_eq!(features.len(), 1);
    }
}
