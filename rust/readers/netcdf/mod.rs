use crate::parsers::{FeatureReader, Reader};
use alloc::{boxed::Box, collections::BTreeMap, string::String, vec, vec::Vec};
use core::cell::RefCell;
use s2json::{MValue, Properties, ValueType, VectorFeature, VectorGeometry, VectorPoint};

// TODO: I don't know why anymore but the FileReader is SLOOOOOW, I think it re-reads over and over.
// But the BufferReader is fast. 99% of nc files are small so this is probably not worth the effort
// to fix anytime soon

/// The kind of data that can be stored in a NetCDF file
#[derive(Debug, Clone, PartialEq)]
pub enum CDFValue {
    /// String case
    String(String),
    /// Number case
    Number(f64),
    /// Array of numbers case
    Array(Vec<f64>),
}
impl Default for CDFValue {
    fn default() -> Self {
        CDFValue::Number(0.0)
    }
}
impl CDFValue {
    /// Converts a CDFValue to a number
    pub fn to_num(&self) -> f64 {
        match self {
            CDFValue::Number(n) => *n,
            _ => 0.0,
        }
    }
    /// Get the number in the array at a given index
    pub fn get_index(&self, index: u64) -> f64 {
        match self {
            CDFValue::Array(v) => v[index as usize],
            _ => 0.0,
        }
    }
}
impl From<&CDFValue> for ValueType {
    fn from(value: &CDFValue) -> Self {
        match value {
            CDFValue::String(s) => s.into(),
            CDFValue::Number(n) => (*n).into(),
            CDFValue::Array(v) => v.clone().into(),
        }
    }
}
impl From<&str> for CDFValue {
    fn from(value: &str) -> Self {
        CDFValue::String(value.into())
    }
}
impl From<f64> for CDFValue {
    fn from(value: f64) -> Self {
        CDFValue::Number(value)
    }
}
impl From<Vec<f64>> for CDFValue {
    fn from(value: Vec<f64>) -> Self {
        CDFValue::Array(value)
    }
}

/// An NetCDF Shaped Vector Feature
pub type CDFVectorFeature = VectorFeature<(), Properties, MValue>;

/// The kind of attributes that can be stored in a NetCDF file. Similar to a GeoJSON Properties object
pub type CDFAttributes = BTreeMap<String, CDFValue>;

/// Track the dimension and its max value (can be infinity)
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CDFDimension {
    /// index of the dimension
    pub index: u64,
    /// name of the dimension
    pub name: String,
    /// size of the dimension
    pub size: u64,
}

/// Track information about the dimensions, which is "unlimited" dimension, and variable sizes
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CDFRecordDimension {
    /// Length of the record dimension sum of the var_size's of all the record variables
    pub size: u64,
    /// ID of the record dimension
    pub id: Option<u64>,
    /// Name of the record dimension
    pub name: Option<String>,
    /// Step of the record dimension
    pub record_step: Option<u64>,
}

/// A NetCDF variable
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CDFVariable {
    /// name of the variable
    pub name: String,
    /// Array with the dimension IDs of the variable
    pub dimensions: Vec<CDFDimension>,
    /// Array with the attributes of the variable
    pub attributes: CDFAttributes,
    /// type of the variable
    pub r#type: CDFDataType,
    /// size of the variable
    pub size: u64,
    /// offset where of the variable begins
    pub offset: u64,
    /// True if is a record variable, false otherwise (unlimited size)
    pub record: bool,
}

// Grammar constants
const NC_UNLIMITED: u64 = 0;
const NC_DIMENSION: u64 = 10;
const NC_VARIABLE: u64 = 11;
const NC_ATTRIBUTE: u64 = 12;

/// Enum of the NetCDF data types available
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum CDFDataType {
    /// Byte size (1 byte)
    #[default]
    BYTE = 1,
    /// Char size (1 byte)
    CHAR = 2,
    /// Short size (2 bytes)
    SHORT = 3,
    /// Integer size (4 bytes)
    INT = 4,
    /// Float size (4 bytes)
    FLOAT = 5,
    /// Double size (8 bytes)
    DOUBLE = 6,
}
impl From<u64> for CDFDataType {
    fn from(value: u64) -> Self {
        match value {
            2 => CDFDataType::CHAR,
            3 => CDFDataType::SHORT,
            4 => CDFDataType::INT,
            5 => CDFDataType::FLOAT,
            6 => CDFDataType::DOUBLE,
            _ => CDFDataType::BYTE,
        }
    }
}

/// Given a type, get the number of bytes it represents
///
/// ## Parameters
/// - `type`: the NetCDF data type
///
/// ## Returns
/// The number of bytes for the data type
pub fn netcdf_type_to_bytes(r#type: CDFDataType) -> u64 {
    match r#type {
        CDFDataType::BYTE | CDFDataType::CHAR => 1,
        CDFDataType::SHORT => 2,
        CDFDataType::INT | CDFDataType::FLOAT => 4,
        CDFDataType::DOUBLE => 8,
    }
}

/// User defined options on how to parse the NetCDF file
#[derive(Debug, Default, Clone, PartialEq)]
pub struct NetCDFReaderOptions {
    /// If provided the lookup of the longitude [Default='lon']
    pub lon_key: Option<String>,
    /// If provided the lookup of the latitude [Default='lat']
    pub lat_key: Option<String>,
    /// If provided the lookup for the height value [Default=undefined]
    pub height_key: Option<String>,
    /// List of fields to include in the feature properties
    pub prop_fields: Option<Vec<String>>,
}

/// # NetCDF v3.x Reader
///
/// ## Description
/// Read the NetCDF v3.x file format
///
/// [See specification](https://www.unidata.ucar.edu/software/netcdf/docs/file_format_specifications.html)
///
/// Implements the [`FeatureReader`] trait
///
/// ## Usage
///
/// The methods you have access to:
/// - [`NetCDFReader::new`]: Create a new NetCDFReader
/// - [`NetCDFReader::len`]: Returns the number of records
/// - [`NetCDFReader::is_empty`]: Returns true if the reader is empty
/// - [`NetCDFReader::get_properties`]: Returns the properties for a given index
/// - [`NetCDFReader::get_point`]: Get the point at a given index
/// - [`NetCDFReader::get_feature`]: Reads a point in at index as a feature
/// - [`NetCDFReader::get_data_variable`]: Retrieves the data for a given variable
/// - [`NetCDFReader::iter`]: Create an iterator over the features
/// - [`NetCDFReader::par_iter`]: Create a parallel iterator over the features
///
/// ### Buffer Reader
/// ```rust
/// use gistools::{
///     parsers::{FeatureReader, BufferReader},
///     readers::{NetCDFReader, NetCDFReaderOptions},
/// };
///
/// // Ignore this, used to setup example
/// use std::path::PathBuf;
/// let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
/// path.push("tests/readers/netcdf/fixtures/ichthyop.nc");
/// let data: Vec<u8> = std::fs::read(path).unwrap();
///
/// let netcdf_reader = NetCDFReader::new(
///     BufferReader::from(data),
///     Some(NetCDFReaderOptions {
///         lon_key: Some("lon".into()),
///         lat_key: Some("lat".into()),
///         height_key: Some("depth".into()),
///         prop_fields: Some(vec!["depth".into()]),
///     }),
/// );
/// assert_eq!(netcdf_reader.len(), 49);
/// let features: Vec<_> = netcdf_reader.iter().collect();
/// ```
///
/// ## Links
/// - <https://www.unidata.ucar.edu/software/netcdf/docs/file_format_specifications.html>
#[derive(Debug)]
pub struct NetCDFReader<T: Reader> {
    reader: T,
    /// Record dimension
    pub record_dimension: CDFRecordDimension,
    /// List of dimensions
    pub dimensions: Vec<CDFDimension>,
    /// List of global attributes
    pub global_attributes: CDFAttributes,
    /// List of variables
    pub variables: Vec<CDFVariable>,
    /// Describes if offsets are 32 or 64 bits
    pub is64: bool,
    /// Track the cursor for parsing the header
    cursor: RefCell<u64>,
    lon_key: String,
    lat_key: String,
    height_key: Option<String>,
    prop_fields: Vec<String>,
}
impl<T: Reader> NetCDFReader<T> {
    /// Creates a new NetCDF reader
    pub fn new(reader: T, options: Option<NetCDFReaderOptions>) -> Self {
        // Validate that it's a NetCDF file
        let magic = reader.parse_string(Some(0), Some(3));
        if &magic != "CDF" {
            panic!("Not a valid NetCDF file: should start with CDF");
        }
        // Check the NetCDF format
        let is64 = reader.uint8(Some(3)) != 1;
        let options = options.unwrap_or_default();
        let mut cdf_reader = NetCDFReader {
            reader,
            record_dimension: CDFRecordDimension {
                size: 0,
                id: None,
                name: None,
                record_step: None,
            },
            dimensions: vec![],
            global_attributes: BTreeMap::new(),
            variables: vec![],
            is64,
            cursor: 4.into(),
            lon_key: options.lon_key.unwrap_or("lon".into()),
            lat_key: options.lat_key.unwrap_or("lat".into()),
            height_key: options.height_key,
            prop_fields: options.prop_fields.unwrap_or_default(),
        };
        // Read the header
        cdf_reader.parse_header();

        cdf_reader
    }

    /// Returns the number of records
    pub fn len(&self) -> u64 {
        let lat = self.get_data_variable(self.lat_key.clone());
        if let Some(lat) = lat {
            return lat.len() as u64;
        }
        0
    }

    /// Check if the reader is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get the properties at a given index
    pub fn get_properties(&self, index: u64) -> Option<MValue> {
        let mut m = MValue::new();
        for field in self.prop_fields.clone().into_iter() {
            let value = self.get_data_variable(field.clone());
            if let Some(value) = value {
                let field: String = field.clone();
                let value: ValueType = (&value[index as usize]).into();
                m.insert(field, value);
            }
        }
        Some(m)
    }

    /// Get the point at a given index
    pub fn get_point(&self, index: u64) -> Option<VectorPoint<MValue>> {
        if index >= self.len() {
            return None;
        }
        let lat = self.get_data_variable(self.lat_key.clone());
        let lon = self.get_data_variable(self.lon_key.clone());
        let height = self.get_data_variable(self.height_key.clone().unwrap_or_default());
        if let (Some(lat), Some(lon)) = (lat, lon) {
            let lat = lat[0].get_index(index);
            let lon = lon[0].get_index(index);
            let m = self.get_properties(index);
            return Some(VectorPoint::new(lon, lat, height.map(|h| h[index as usize].to_num()), m));
        }
        None
    }

    /// Reads a point in at index as a feature
    pub fn get_feature(&self, index: u64) -> Option<CDFVectorFeature> {
        self.get_point(index).map(|point| {
            VectorFeature::new_wm(
                None,
                Properties::default(),
                VectorGeometry::new_point(point, None),
                None,
            )
        })
    }

    /// Retrieves the data for a given variable
    ///
    /// ## Parameters
    /// - `variable_name`: Name of the variable to search or variable object
    ///
    /// ## Returns
    /// The variable values
    pub fn get_data_variable(&self, variable_name: String) -> Option<Vec<CDFValue>> {
        let variable = self.variables.iter().find(|val| val.name == variable_name).cloned();
        // return nothing if not found
        if let Some(variable) = variable {
            // go to the offset position
            *self.cursor.borrow_mut() = variable.offset;
            // return the data
            return if variable.record {
                Some(self.get_record(variable))
            } else {
                Some(self.get_non_record(variable))
            };
        }
        None
    }

    // INTERNAL

    /// Internal method to get the current offset
    ///
    /// ## Returns
    /// The current offset
    fn get_offset(&self) -> u64 {
        if self.is64 { self.get_u64() } else { self.get_u32() }
    }

    /// Internal method to get a 32 but value under the cursor
    ///
    /// ## Returns
    /// A 32 bit value
    fn get_u32(&self) -> u64 {
        let data = self.reader.uint32_be(Some(*self.cursor.borrow()));
        *self.cursor.borrow_mut() += 4;
        data as u64
    }

    /// Internal method to get a 64 but value under the cursor
    ///
    /// ## Returns
    /// A 64 bit value
    fn get_u64(&self) -> u64 {
        let data = self.reader.uint64_be(Some(*self.cursor.borrow()));
        *self.cursor.borrow_mut() += 8;
        data
    }

    /// Internal method to read a string under the cursor
    ///
    /// ## Returns
    /// A string of the name
    fn get_name(&self) -> String {
        let name_length = self.get_u32();
        let name = self.reader.parse_string(Some(*self.cursor.borrow()), Some(name_length));
        *self.cursor.borrow_mut() += name_length;
        self.padding();

        name.trim().into()
    }

    /// Internal method to Parse the header
    fn parse_header(&mut self) {
        // build dimension list
        self.record_dimension.size = self.get_u32();
        self.build_dimension_list();
        // build global attributes
        self.global_attributes = self.build_attributes();
        // build the variable list
        self.build_variables_list();
    }

    /// Get the data type
    ///
    /// ## Parameters
    /// - `type`: the data type
    /// - `size`: the data size
    ///
    /// ## Returns
    /// The data type
    fn get_type(&self, r#type: CDFDataType, size: u64) -> CDFValue {
        let data = if r#type == CDFDataType::BYTE {
            let mut res = vec![];
            let mut i = 0;
            while i < size {
                res.push(self.reader.uint8(Some(*self.cursor.borrow())) as f64);
                *self.cursor.borrow_mut() += 1;
                i += 1;
            }
            CDFValue::Array(res)
        } else if r#type == CDFDataType::CHAR {
            let res = self.reader.parse_string(Some(*self.cursor.borrow()), Some(size));
            *self.cursor.borrow_mut() += size;
            CDFValue::String(res.trim().into())
        } else if r#type == CDFDataType::SHORT
            || r#type == CDFDataType::INT
            || r#type == CDFDataType::FLOAT
            || r#type == CDFDataType::DOUBLE
        {
            let step = if r#type == CDFDataType::DOUBLE {
                8
            } else if r#type == CDFDataType::SHORT {
                2
            } else {
                4
            };
            let read_number: Box<dyn Fn(u64) -> f64> = if r#type == CDFDataType::SHORT {
                Box::new(|offset: u64| self.reader.int16_be(Some(offset)) as f64)
            } else if r#type == CDFDataType::INT {
                Box::new(|offset: u64| self.reader.int32_be(Some(offset)) as f64)
            } else if r#type == CDFDataType::FLOAT {
                Box::new(|offset: u64| self.reader.f32_be(Some(offset)) as f64)
            } else {
                Box::new(|offset: u64| self.reader.f64_be(Some(offset)))
            };
            let mut res = vec![];
            let mut i = 0;
            while i < size {
                res.push(read_number(*self.cursor.borrow()));
                *self.cursor.borrow_mut() += step;
                i += 1;
            }
            if res.len() == 1 { CDFValue::Number(res[0]) } else { CDFValue::Array(res) }
        } else {
            panic!("non valid type {:?}", r#type);
        };

        self.padding();

        data
    }

    /// Internal method to build the dimension list
    fn build_dimension_list(&mut self) {
        let dim_list_tag = self.get_u32();

        if dim_list_tag == 0 {
            let ensure_empty = self.get_u32();
            if ensure_empty != 0 {
                panic!("wrong empty tag for list of dimensions");
            }
        } else {
            if dim_list_tag != NC_DIMENSION {
                panic!("wrong tag for list of dimensions");
            }

            // Length of dimensions
            let dimension_size = self.get_u32();
            // populate `name` and `size` for each dimension
            let mut index = 0;
            while index < dimension_size {
                // Read name
                let name = self.get_name();
                // Read dimension size
                let size = self.get_u32();
                if size == NC_UNLIMITED {
                    // in netcdf 3 one field can be of size unlimited
                    self.record_dimension.id = Some(index);
                    self.record_dimension.name = Some(name.clone());
                }
                // store the dimension
                self.dimensions.push(CDFDimension { index, name, size });

                index += 1;
            }
        }
    }

    /// Internal method to build attributes including global attributes
    ///
    /// ## Returns
    /// Attributes from a block of data at a given offset
    fn build_attributes(&mut self) -> CDFAttributes {
        let mut atrributes = CDFAttributes::default();
        let g_att_tag = self.get_u32();
        if g_att_tag == 0 {
            let ensure_empty = self.get_u32();
            if ensure_empty != 0 {
                panic!("wrong empty tag for list of attributes");
            }
        } else {
            if g_att_tag != NC_ATTRIBUTE {
                panic!("wrong tag for list of attributes");
            }
            // Length of attributes
            let attribute_size = self.get_u32();
            // Populate `name`, `type` and `value` for each attribute
            let mut ga_idx = 0;
            while ga_idx < attribute_size {
                // Read name, type, and size of data block
                let name = self.get_name();
                let r#type: CDFDataType = self.get_u32().into();
                let size = self.get_u32();
                // store the attribute key-value
                let data = self.get_type(r#type, size);
                atrributes.insert(name, data);
                ga_idx += 1;
            }
        }

        atrributes
    }

    /// Internal method to build a variable list from a block of data at a given offset
    fn build_variables_list(&mut self) {
        let var_tag = self.get_u32();
        let mut record_step = 0;
        if var_tag == 0 {
            let ensure_empty = self.get_u32();
            if ensure_empty != 0 {
                panic!("wrong empty tag for list of variables");
            }
        } else {
            if var_tag != NC_VARIABLE {
                panic!("wrong tag for list of variables");
            }
            // Length of variables
            let var_size = self.get_u32();
            let mut v_idx = 0;
            while v_idx < var_size {
                // Read name, dimensionality, and index into the list of dimensions
                let name = self.get_name();
                let dimensionality = self.get_u32();
                let mut dimensions_ids = vec![];
                let mut dim = 0;
                while dim < dimensionality {
                    dimensions_ids.push(self.get_u32());
                    dim += 1;
                }
                // Read variables size
                let attributes = self.build_attributes();
                // Read type
                let r#type: CDFDataType = self.get_u32().into();
                // Read variable size
                // The 32-bit var_size field is not large enough to contain the size of variables that require
                // more than 2^32 - 4 bytes, so 2^32 - 1 is used in the var_size field for such variables.
                let var_size = self.get_u32();
                // Read offset
                let offset = self.get_offset();
                let mut record = false;
                // Count amount of record variables
                if !dimensions_ids.is_empty()
                    && dimensions_ids[0] == self.record_dimension.id.unwrap_or_default()
                {
                    record_step += var_size;
                    record = true;
                }
                self.variables.push(CDFVariable {
                    name,
                    dimensions: dimensions_ids
                        .iter()
                        .map(|id| self.dimensions[*id as usize].clone())
                        .collect(),
                    attributes,
                    r#type,
                    size: var_size,
                    offset,
                    record,
                });
                v_idx += 1;
            }
        }
        self.record_dimension.record_step = Some(record_step);
    }

    /// Read data for the given non-record variable
    ///
    /// ## Parameters
    /// - `variable`: Variable metadata
    ///
    /// ## Returns
    ///  Data of the element
    fn get_non_record(&self, variable: CDFVariable) -> Vec<CDFValue> {
        // variable type
        let CDFVariable { size, r#type, .. } = variable;
        // size of the data
        let total_size = size / netcdf_type_to_bytes(r#type);
        // iterates over the data
        let mut data = vec![];
        let mut i = 0;
        while i < total_size {
            data.push(self.get_type(r#type, 1));
            i += 1;
        }

        data
    }

    /// Read data for the given record variable
    ///
    /// ## Parameters
    /// - `variable`: Variable metadata
    ///
    /// ## Returns
    /// Data of the element
    fn get_record(&self, variable: CDFVariable) -> Vec<CDFValue> {
        // prep variables
        let CDFRecordDimension { record_step, size: total_size, .. } = self.record_dimension;
        let CDFVariable { size, r#type, .. } = variable;
        let width = if size != 0 { size / netcdf_type_to_bytes(r#type) } else { 1 };

        if record_step.is_none() {
            panic!("record_dimension.record_step is undefined");
        }
        let record_step = record_step.unwrap();

        // iterates over the data
        let mut data = vec![];
        let mut i = 0;
        while i < total_size {
            let current_offset = *self.cursor.borrow();
            data.push(self.get_type(r#type, width));
            *self.cursor.borrow_mut() = current_offset + record_step;
            i += 1;
        }

        data
    }

    /// Apply padding as data is mapped to 4-byte alignment
    fn padding(&self) {
        let cursor = *self.cursor.borrow();
        if !cursor.is_multiple_of(4) {
            *self.cursor.borrow_mut() += 4 - (cursor % 4);
        }
    }
}

/// The NetCDF Iterator tool
#[derive(Debug)]
pub struct CDFIterator<'a, T: Reader> {
    reader: &'a NetCDFReader<T>,
    index: u64,
}
impl<T: Reader> Iterator for CDFIterator<'_, T> {
    type Item = CDFVectorFeature;

    fn next(&mut self) -> Option<Self::Item> {
        let cdf_reader = &self.reader;
        if let Some(point) = cdf_reader.get_feature(self.index) {
            self.index += 1;
            Some(point)
        } else {
            None
        }
    }
}
/// A feature reader trait with a callback-based approach
impl<T: Reader> FeatureReader<(), Properties, MValue> for NetCDFReader<T> {
    type FeatureIterator<'a>
        = CDFIterator<'a, T>
    where
        T: 'a;

    fn iter(&self) -> Self::FeatureIterator<'_> {
        CDFIterator { reader: self, index: 0 }
    }

    #[cfg(feature = "std")]
    fn par_iter(&self, _pool_size: usize, _thread_id: usize) -> Self::FeatureIterator<'_> {
        CDFIterator { reader: self, index: 0 }
    }
}
