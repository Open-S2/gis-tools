#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::{
        parsers::FileReader,
        readers::{DBFHeader, DataBaseFile},
        util::Date,
    };
    use s2json::{MValue, Properties, ValueType};
    use std::path::PathBuf;

    #[test]
    fn test_empty_dbf() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/shapefile/fixtures/empty.dbf");

        let reader = FileReader::new(path).unwrap();
        let dbf = DataBaseFile::new(reader, Some("utf-8".into()));

        assert_eq!(
            dbf.get_header(),
            &DBFHeader {
                last_updated: Date::new(2016, 2, 21),
                records: 2,
                header_len: 33,
                rec_len: 1,
            }
        );

        let properties_0: MValue = dbf.get_properties(0).unwrap();
        assert_eq!(properties_0, Properties::new());

        let properties_1: MValue = dbf.get_properties(1).unwrap();
        assert_eq!(properties_1, Properties::new());

        let properties_2: Option<MValue> = dbf.get_properties(2);
        assert!(properties_2.is_none());
    }

    #[test]
    fn test_codepage_dbf() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/shapefile/fixtures/codepage.dbf");

        let reader = FileReader::new(path).unwrap();
        let dbf = DataBaseFile::new(reader, Some("utf-8".into()));

        assert_eq!(
            dbf.get_header(),
            &DBFHeader {
                last_updated: Date::new(1995, 7, 26),
                records: 2,
                header_len: 65,
                rec_len: 255,
            }
        );

        let properties_0: MValue = dbf.get_properties(0).unwrap();
        assert_eq!(
            properties_0,
            Properties::from([("field".into(), ValueType::Primitive("??".into()))])
        );

        let properties_1: MValue = dbf.get_properties(1).unwrap();
        assert_eq!(
            properties_1,
            Properties::from([("field".into(), ValueType::Primitive("Hn�vo�ick� h�j".into()))])
        );

        let properties_2: Option<MValue> = dbf.get_properties(2);
        assert!(properties_2.is_none());

        let all_props: Vec<MValue> = dbf.get_all_properties();
        assert_eq!(all_props.len(), 2);

        assert_eq!(all_props[0], properties_0);
        assert_eq!(all_props[1], properties_1);
    }

    #[test]
    fn test_utf_dbf() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/shapefile/fixtures/utf.dbf");

        let reader = FileReader::new(path).unwrap();
        let dbf = DataBaseFile::new(reader, Some("utf-8".into()));

        assert_eq!(
            dbf.get_header(),
            &DBFHeader {
                last_updated: Date::new(1995, 7, 26),
                records: 2,
                header_len: 65,
                rec_len: 255,
            }
        );

        let properties_0: MValue = dbf.get_properties(0).unwrap();
        assert_eq!(
            properties_0,
            Properties::from([("field".into(), ValueType::Primitive("💩".into()))])
        );

        let properties_1 = dbf.get_properties(1).unwrap();
        assert_eq!(
            properties_1,
            Properties::from([("field".into(), ValueType::Primitive("Hněvošický háj".into()))])
        );

        let properties_2 = dbf.get_properties(2);
        assert!(properties_2.is_none());

        let all_props = dbf.get_all_properties();
        assert_eq!(all_props.len(), 2);

        assert_eq!(all_props[0], properties_0);
        assert_eq!(all_props[1], properties_1);
    }

    #[test]
    fn test_watershed_dbf() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/shapefile/fixtures/watershed.dbf");

        let reader = FileReader::new(path).unwrap();
        let dbf = DataBaseFile::new(reader, Some("utf-8".into()));

        assert_eq!(
            dbf.get_header(),
            &DBFHeader {
                last_updated: Date::new(2013, 9, 20),
                records: 33,
                header_len: 193,
                rec_len: 104,
            }
        );

        let all_props = dbf.get_all_properties();
        assert_eq!(all_props.len(), 33);

        let first: &MValue = all_props.first().unwrap();
        assert_eq!(
            first,
            &Properties::from([
                ("DWM_NAME".into(), ValueType::Primitive("BUZZARDS BAY".into())),
                ("DWM_CODE".into(), ValueType::Primitive("95".into())),
                ("DRAINAGE".into(), ValueType::Primitive("coastal".into())),
                ("SHAPE_AREA".into(), ValueType::Primitive(1100426424.93_f64.into())),
                ("SHAPE_LEN".into(), ValueType::Primitive(680071.913919_f64.into())),
            ])
        );

        let last = all_props.last().unwrap();
        assert_eq!(
            last,
            &Properties::from([
                ("DWM_NAME".into(), ValueType::Primitive("HUDSON: Kinderhook".into())),
                ("DWM_CODE".into(), ValueType::Primitive("12".into())),
                ("DRAINAGE".into(), ValueType::Primitive("river".into())),
                ("SHAPE_AREA".into(), ValueType::Primitive(56596528.9263_f64.into())),
                ("SHAPE_LEN".into(), ValueType::Primitive(55533.0776528_f64.into())),
            ])
        );
    }

    #[test]
    fn test_watershed_special_dbf() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/shapefile/fixtures/watershed-specialCharacters.dbf");

        let reader = FileReader::new(path).unwrap();
        let dbf = DataBaseFile::new(reader, Some("utf-8".into()));

        assert_eq!(
            dbf.get_header(),
            &DBFHeader {
                last_updated: Date::new(2013, 9, 20),
                records: 33,
                header_len: 193,
                rec_len: 104,
            }
        );

        let all_props = dbf.get_all_properties();
        assert_eq!(all_props.len(), 33);

        let first: &MValue = all_props.first().unwrap();
        assert_eq!(
            first,
            &Properties::from([
                ("DWM_NAME".into(), ValueType::Primitive("BUZZARDS BAY".into())),
                ("DWM_CODE".into(), ValueType::Primitive("95".into())),
                ("TEST.\"-:!".into(), ValueType::Primitive("coastal".into())),
                ("SHAPE_AREA".into(), ValueType::Primitive(1100426424.93_f64.into())),
                ("SHAPE_LEN".into(), ValueType::Primitive(680071.913919_f64.into())),
            ])
        );

        let last = all_props.last().unwrap();
        assert_eq!(
            last,
            &Properties::from([
                ("DWM_NAME".into(), ValueType::Primitive("HUDSON: Kinderhook".into())),
                ("DWM_CODE".into(), ValueType::Primitive("12".into())),
                ("TEST.\"-:!".into(), ValueType::Primitive("river".into())),
                ("SHAPE_AREA".into(), ValueType::Primitive(56596528.9263_f64.into())),
                ("SHAPE_LEN".into(), ValueType::Primitive(55533.0776528_f64.into())),
            ])
        );
    }
}
