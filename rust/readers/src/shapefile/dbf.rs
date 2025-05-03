use alloc::{string::String, vec, vec::Vec};
use core::marker::PhantomData;
use parsers::Reader;
use s2json::{MValue, MValueCompatible, PrimitiveValue, Properties, ValueType};
use util::Date;

/// The Header data explaining the contents of the DBF file
#[derive(Debug, PartialEq)]
pub struct DBFHeader {
    /// The last updated date
    #[allow(dead_code)]
    pub last_updated: Date,
    /// The number of records
    pub records: u64,
    /// The length of the header data
    pub header_len: u64,
    /// The length of each row
    pub rec_len: u64,
}

/// Each row is a key definition to build the properties for each column
#[derive(Debug, Default, PartialEq, Clone)]
pub struct DBFRow {
    /// The name of the row
    name: String,
    /// The data type of the row
    data_type: char,
    /// The length of the row
    len: u64,
    /// The decimal places of the row
    #[allow(dead_code)]
    decimal: u64,
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
    pub fn get_properties(&self, index: u64) -> Option<M> {
        let DBFHeader { records, rec_len, .. } = self.header;
        if index > records - 1 {
            return None;
        }
        let offset = ((self.rows.len() as u64 + 1) << 5) + 2 + index * rec_len;

        Some(self.parse_properties(offset))
    }

    /// Get all the properties in the DBF
    pub fn get_all_properties(&self) -> Vec<M> {
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
            records: reader.uint32_le(Some(4)) as u64,
            header_len: reader.uint16_le(Some(8)) as u64,
            rec_len: reader.uint16_le(Some(10)) as u64,
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
                len: reader.uint8(Some(offset + 16)) as u64,
                decimal: reader.uint8(Some(offset + 17)) as u64,
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
    fn parse_properties(&self, mut offset: u64) -> M
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
    fn parse_value(&self, offset: u64, len: u64, v_type: char) -> PrimitiveValue {
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
