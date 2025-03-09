use core::marker::PhantomData;

use alloc::{string::String, vec, vec::Vec};

use s2json::{MValue, MValueCompatible, PrimitiveValue, Properties, ValueType};

use crate::readers::Reader;
use crate::util::Date;

/// The Header data explaining the contents of the DBF file
#[derive(Debug, PartialEq)]
pub struct DBFHeader {
    /// The last updated date
    #[allow(dead_code)]
    last_updated: Date,
    /// The number of records
    records: usize,
    /// The length of the header data
    header_len: usize,
    /// The length of each row
    rec_len: usize,
}

/// Each row is a key definition to build the properties for each column
#[derive(Debug, Default, PartialEq, Clone)]
pub struct DBFRow {
    /// The name of the row
    name: String,
    /// The data type of the row
    data_type: char,
    /// The length of the row
    len: usize,
    /// The decimal places of the row
    #[allow(dead_code)]
    decimal: usize,
}

/// A DBF data class to parse the data from a DBF
#[derive(Debug)]
pub struct DataBaseFile<T: Reader, M: MValueCompatible = MValue> {
    /// The input reader
    reader: T,
    header: DBFHeader,
    rows: Vec<DBFRow>,
    #[allow(dead_code)]
    /// The encoding of the raw data to string; defaults to 'utf-8'. Others not supported.
    encoding: Option<String>,
    _phantom: PhantomData<M>,
}
impl<T: Reader, M: MValueCompatible> DataBaseFile<T, M> {
    /// Create a new DBF data class given an input reader
    pub fn new(mut reader: T, encoding: Option<String>) -> DataBaseFile<T, M> {
        let header = DataBaseFile::<T, M>::parse_header(&mut reader);
        let rows = DataBaseFile::<T, M>::parse_row_header(&mut reader, &header);
        DataBaseFile::<T, M> { reader, header, rows, encoding, _phantom: PhantomData }
    }

    /// Create a copy of the header data
    pub fn get_header(&self) -> &DBFHeader {
        &self.header
    }

    /// Get the properties for the given index
    pub fn get_properties(&mut self, index: usize) -> Option<M> {
        let DBFHeader { records, rec_len, .. } = self.header;
        if index > records - 1 {
            return None;
        }
        let offset = ((self.rows.len() + 1) << 5) + 2 + index * rec_len;

        Some(self.parse_properties(offset))
    }

    /// Get all the properties in the DBF
    pub fn get_all_properties(&mut self) -> Vec<M> {
        let DBFHeader { records, .. } = self.header;
        let mut res: Vec<M> = vec![];
        for i in 0..records {
            if let Some(properties) = self.get_properties(i) {
                res.push(properties);
            }
        }

        res
    }

    /// Parse the header and store it in the class
    fn parse_header(reader: &mut T) -> DBFHeader {
        DBFHeader {
            last_updated: Date::new(
                reader.uint8(Some(1)) as u16 + 1_900,
                reader.uint8(Some(2)),
                reader.uint8(Some(3)),
            ),
            records: reader.uint32_le(Some(4)) as usize,
            header_len: reader.uint16_le(Some(8)) as usize,
            rec_len: reader.uint16_le(Some(10)) as usize,
        }
    }

    /// Parses the row header and builds an array of keys that each property may have
    fn parse_row_header(reader: &mut T, header: &DBFHeader) -> Vec<DBFRow> {
        let header_len = header.header_len;
        let len = header_len - 1;
        let mut res: Vec<DBFRow> = vec![];

        let mut offset = 32;
        while offset < len {
            res.push(DBFRow {
                name: reader.parse_string(Some(offset), Some(11)),
                data_type: char::from(reader.uint8(Some(offset + 11))),
                len: reader.uint8(Some(offset + 16)) as usize,
                decimal: reader.uint8(Some(offset + 17)) as usize,
            });
            if reader.uint8(Some(offset + 32)) == 13 {
                break;
            } else {
                offset += 32;
            }
        }

        res
    }

    /// Parse the properties starting from the given offset
    fn parse_properties(&mut self, mut offset: usize) -> M
    where
        M: MValueCompatible,
    {
        let mut properties: Properties = Properties::new();
        for row in self.rows.clone().into_iter() {
            let value = self.parse_value(offset, row.len, row.data_type);
            offset += row.len;
            properties.insert(row.name.clone(), ValueType::Primitive(value));
        }

        properties.into()
    }

    /// Parse the value at the given offset
    fn parse_value(&mut self, offset: usize, len: usize, v_type: char) -> PrimitiveValue {
        let text_data: String = self.reader.parse_string(Some(offset), Some(len)).trim().into();

        match v_type {
            'N' | 'F' | 'O' => (text_data.parse::<f64>().expect("Failed to parse float")).into(),
            'D' => Date::new(
                text_data[0..4].parse::<u16>().expect("Failed to parse year"),
                text_data[4..6].parse::<u8>().expect("Failed to parse month"),
                text_data[6..8].parse::<u8>().expect("Failed to parse day"),
            )
            .get_time()
            .into(),
            'L' => (text_data.to_lowercase() == "y" || text_data.to_lowercase() == "t").into(),
            _ => {
                if text_data == "undefined" {
                    PrimitiveValue::Null
                } else {
                    text_data.into()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use s2json::MValue;

    use super::*;

    use crate::readers::FileReader;

    use std::path::PathBuf;

    #[test]
    fn test_empty_dbf() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/shapefile/fixtures/empty.dbf");

        let reader = FileReader::new(path).unwrap();
        let mut dbf = DataBaseFile::new(reader, Some("utf-8".into()));

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
        let mut dbf = DataBaseFile::new(reader, Some("utf-8".into()));

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
        let mut dbf = DataBaseFile::new(reader, Some("utf-8".into()));

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
        let mut dbf = DataBaseFile::new(reader, Some("utf-8".into()));

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
        let mut dbf = DataBaseFile::new(reader, Some("utf-8".into()));

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
