// https://docs.ogc.org/is/19-008r4/19-008r4.html#_requirements_class_tiff
use super::constants::GeoTIFFTypes;
use crate::{
    parsers::Reader,
    readers::{GeoKeyDirectoryKeys, GeoStore},
};
use alloc::vec::Vec;

/// A key value pair
#[derive(Debug, Clone, Default, PartialEq)]
struct KeyValue {
    key: u16,
    value: Vec<u8>,
    field_type: GeoTIFFTypes,
}

/// A tiepoint structured for decoding images
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct GeoTiePoint {
    /// The i index
    pub i: f64,
    /// The j index
    pub j: f64,
    /// The k index
    pub k: f64,
    /// The x coordinate
    pub x: f64,
    /// The y coordinate
    pub y: f64,
    /// The z coordinate
    pub z: f64,
}

/// The pixel scale
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct GeoPixelScale {
    /// The pixel x scale
    pub x: f64,
    /// The pixel y scale
    pub y: f64,
    /// The pixel z scale
    pub z: f64,
}

/// The image directory
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ImageDirectory {
    /// the geo key directory
    pub geo_key_directory: GeoStore,
    /// The pixel scale
    pub pixel_scale: GeoPixelScale,
    /// The tie point
    pub tie_point: GeoTiePoint,
    /// Variables
    pub variables: GeoStore,
}
impl ImageDirectory {
    /// Insert a variable into the image directory
    pub fn insert(&mut self, key: u16, value: Vec<u8>, field_type: GeoTIFFTypes) {
        self.variables.insert(key, value, field_type);
    }
    /// Get the length of variables
    pub fn len(&self) -> usize {
        self.variables.len() + self.geo_key_directory.len()
    }
    /// Check if no variables are set
    pub fn is_empty(&self) -> bool {
        self.geo_key_directory.is_empty() && self.variables.is_empty()
    }
}

/// GeoTIFF Header Reader
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GeoTIFFHeaderReader {
    /// true if reading in the data is little endian
    pub little_endian: bool,
    /// true if reading in the data is big endian
    pub big_tiff: bool,
    /// Key-Value pairs (value is an index pointing to where in the data the value exists)
    pub image_directories: Vec<ImageDirectory>,
}
impl GeoTIFFHeaderReader {
    /// Create a new GeoTIFFHeaderReader
    pub fn new<T: Reader>(reader: &T) -> GeoTIFFHeaderReader {
        let mut tiff_reader = GeoTIFFHeaderReader {
            little_endian: true,
            big_tiff: false,
            image_directories: Vec::new(),
        };
        tiff_reader.parse_header(reader);

        tiff_reader
    }

    /// parses the header data to begin parsing the GeoTIFF
    fn parse_header<T: Reader>(&mut self, reader: &T) {
        // pull the endianess from the header
        let bom = reader.uint16_be(Some(0));
        if bom == 0x4949 {
            self.little_endian = true;
        } else if bom == 0x4d4d {
            self.little_endian = false;
        } else {
            panic!("Invalid byte order value.");
        }
        let le = self.little_endian;

        let magic_number = reader.uint16(Some(2), Some(le));
        if magic_number == 42 {
            self.big_tiff = false;
        } else if magic_number == 43 {
            self.big_tiff = true;
            let offset_byte_size = reader.uint16(Some(4), Some(le));
            if offset_byte_size != 8 {
                panic!("Unsupported offset byte-size.");
            }
        } else {
            panic!("Invalid magic number.");
        }

        let first_ifd_offset = if self.big_tiff {
            reader.uint64(Some(8), Some(le))
        } else {
            reader.uint32(Some(4), Some(le)) as u64
        };

        self.get_image_metadata(first_ifd_offset, reader);
    }

    /// Instructs to parse an image file directory at the given file offset.
    /// As there is no way to ensure that a location is indeed the start of an IFD,
    /// this function must be called with caution (e.g only using the IFD offsets from
    /// the headers or other IFDs).
    ///
    /// @param first_offset - the offset to begin parsing the IFDs (Image File Directory) at.
    fn get_image_metadata<T: Reader>(&mut self, first_offset: u64, reader: &T) {
        let GeoTIFFHeaderReader { big_tiff, little_endian, .. } = *self;
        let entry_size = if big_tiff { 20 } else { 12 };
        let offset_size = if big_tiff { 8 } else { 2 };
        let mut offset = first_offset;

        let mut ifd_offset = first_offset;
        while ifd_offset != 0 {
            let mut ifd = ImageDirectory::default();
            let num_dir_entries = self.read_tag(offset, reader);

            let mut i = offset + offset_size;
            let mut geokey_dir_offset: Option<u64> = None;
            let mut prev_tag = 0;
            for _ in 0..num_dir_entries {
                let field_tag = if little_endian {
                    reader.uint16_le(Some(i))
                } else {
                    reader.uint16_be(Some(i))
                };
                if field_tag < prev_tag {
                    panic!("Invalid IFD, {} < {}", field_tag, prev_tag);
                }
                prev_tag = field_tag;
                if field_tag == 33550 {
                    // GeoPixelScaleTag
                    ifd.pixel_scale = self.get_pixel_scale(i, reader);
                } else if field_tag == 33922 {
                    // TiepointTag
                    ifd.tie_point = self.get_tiepoint(i, reader);
                } else if field_tag == 34735 {
                    // GeoKeyDirectory - map to use after all keys are cached.
                    geokey_dir_offset = Some(i);
                }
                // TIFFTAG_GEODOUBLEPARAMS can just be placed inside the variables section
                // else if field_tag == 34736 { ... }
                // TIFFTAG_GEODOUBLEPARAMS can just be placed inside the variables section
                // else if field_tag == 34737 { ... }
                else {
                    let KeyValue { key, value, field_type } =
                        self.get_key_value(field_tag as u64, i, reader);
                    ifd.insert(key, value, field_type);
                }
                i += entry_size;
            }
            // Validate it has a TransformationTag or a TiepointTag before storing
            if let Some(geokey_dir_offset) = geokey_dir_offset {
                self.get_geo_key_directory(&mut ifd, geokey_dir_offset, reader);
            } else {
                // panic!("No GeoKeyDirectory found. May contain errors");
            }
            if !ifd.is_empty() {
                self.image_directories.push(ifd);
            } else {
                break;
            }
            // increment offset and check for the next IFD
            offset += offset_size + entry_size * num_dir_entries;
            ifd_offset = self.read_tag(offset, reader);
            offset += offset_size;
        }
    }

    /// Reads the value of the tag at the given offset (16 bits if not big_tIFF)
    /// @param offset - the offset to read the tag from
    /// @returns - the value of the tag
    fn read_tag<T: Reader>(&mut self, offset: u64, reader: &T) -> u64 {
        let Self { big_tiff, little_endian, .. } = self;
        if *big_tiff {
            reader.uint64(Some(offset), Some(*little_endian))
        } else {
            reader.uint16(Some(offset), Some(*little_endian)) as u64
        }
    }

    /// Reads the value of the tag at the given offset (32 bits if not big_tIFF)
    /// @param offset - the offset to read the tag from
    /// @returns - the value of the tag
    fn read_offset<T: Reader>(&mut self, offset: u64, reader: &T) -> u64 {
        let Self { big_tiff, little_endian, .. } = self;
        if *big_tiff {
            reader.uint64(Some(offset), Some(*little_endian))
        } else {
            reader.uint32(Some(offset), Some(*little_endian)) as u64
        }
    }

    /// Get the pixel scale from the GeoKeyDirectory
    /// @param offset - the offset to begin parsing the IFDs (GeoKeyDirectory) at.
    /// @returns the parsed GeoKeyDirectory
    fn get_pixel_scale<T: Reader>(&mut self, offset: u64, reader: &T) -> GeoPixelScale {
        let Self { little_endian, big_tiff, .. } = *self;
        let field_type = reader.uint16(Some(offset + 2), Some(little_endian));
        if field_type != 12 {
            panic!("Invalid GeoKeyDirectory type {}", field_type);
        }
        let num_keys = self.read_offset(offset + 4, reader);
        if num_keys != 3 {
            panic!("Invalid GeoKeyDirectory num_keys {}", num_keys);
        }
        let value_offset = self.read_offset(offset + if big_tiff { 12 } else { 8 }, reader);

        GeoPixelScale {
            x: reader.f64(Some(value_offset), Some(little_endian)),
            y: reader.f64(Some(value_offset + 8), Some(little_endian)),
            z: reader.f64(Some(value_offset + 16), Some(little_endian)),
        }
    }

    /// https://docs.ogc.org/is/19-008r4/19-008r4.html#_geokey_directory_test
    /// @param offset - the offset to begin parsing the IFDs (GeoKeyDirectory) at.
    /// @param file_dir - the parsed ImageFileDirectory thus far
    /// @returns the parsed GeoKeyDirectory
    fn get_geo_key_directory<T: Reader>(
        &mut self,
        ifd: &mut ImageDirectory,
        offset: u64,
        reader: &T,
    ) {
        let file_dir = &mut ifd.variables;
        let num_keys = self.read_offset(offset + 4, reader);
        let value_offset = self.read_offset(offset + (if self.big_tiff { 12 } else { 8 }), reader);
        let raw_geokeys = reader.slice(Some(value_offset), Some(value_offset + num_keys * 2));
        let raw_geokeys: Vec<u16> = raw_geokeys
            .chunks_exact(2)
            .map(|chunk| u16::from_le_bytes(chunk.try_into().unwrap()))
            .collect();
        let geo_key_directory = parse_geotiff_raw_geokeys(&raw_geokeys, file_dir);
        // Validate that there is a GTModelType GeoKey in the GeoKey Directory
        if !geo_key_directory.has(GeoKeyDirectoryKeys::GTModelTypeGeoKey as u16) {
            panic!("Missing \"GTModelTypeGeoKey\" in GeoKeyDirectory");
        }

        ifd.geo_key_directory = geo_key_directory;
    }

    /// @param offset - the offset to begin parsing the IFDs (TiepointTag) at.
    /// @returns the parsed Tiepoint
    fn get_tiepoint<T: Reader>(&mut self, offset: u64, reader: &T) -> GeoTiePoint {
        let Self { big_tiff, little_endian, .. } = *self;
        // Validate that Bytes 2-3 = 12 (Double)
        let field_type = reader.uint16(Some(offset + 2), Some(little_endian));
        if field_type != 12 {
            panic!("Invalid TiepointTag type ${field_type}");
        }
        let mut tie_point = GeoTiePoint::default();
        // get size to the value in Bytes 4-7
        let count = self.read_offset(offset + 4, reader);
        // Set TagValue to the value in Bytes 8-11
        let value_offset = self.read_offset(offset + if big_tiff { 12 } else { 8 }, reader);
        for i in 0..count {
            let val = reader.f64(Some(value_offset + i * 8), Some(little_endian));
            match i {
                0 => tie_point.i = val,
                1 => tie_point.j = val,
                2 => tie_point.k = val,
                3 => tie_point.x = val,
                4 => tie_point.y = val,
                5 => tie_point.z = val,
                _ => panic!("Invalid TiepointTag index {}", i),
            }
        }

        tie_point
    }

    /// @param field_tag - the tag to read
    /// @param offset - the current offset in the IFD header data
    /// @returns the parsed key value
    fn get_key_value<T: Reader>(&mut self, field_tag: u64, offset: u64, reader: &T) -> KeyValue {
        let field_type = reader.uint16(Some(offset + 2), Some(self.little_endian));
        let type_count = self.read_offset(offset + 4, reader);
        let field_type_length = GeoTIFFTypes::from(field_type).to_size();
        let value_offset = offset + (if self.big_tiff { 12 } else { 8 });
        let actual_offset =
            if (field_type_length as u64) * type_count <= (if self.big_tiff { 8 } else { 4 }) {
                value_offset
            } else {
                self.read_offset(value_offset, reader)
            };
        let value = self.get_value(
            field_tag as usize,
            field_type.into(),
            type_count,
            actual_offset,
            reader,
        );

        // write the tags value to the file directly
        KeyValue { key: field_tag as u16, value, field_type: field_type.into() }
    }

    /// @param field_tag - the tag to read
    /// @param field_type - the field type
    /// @param type_count - the number of values
    /// @param value_offset - the value offset
    /// @returns - the parsed value
    fn get_value<T: Reader>(
        &self,
        _field_tag: usize,
        field_type: GeoTIFFTypes,
        type_count: u64,
        value_offset: u64,
        reader: &T,
    ) -> Vec<u8> {
        match field_type {
            GeoTIFFTypes::ASCII
            | GeoTIFFTypes::BYTE
            | GeoTIFFTypes::UNDEFINED
            | GeoTIFFTypes::SBYTE => {
                reader.slice(Some(value_offset), Some(value_offset + type_count))
            }
            GeoTIFFTypes::SHORT | GeoTIFFTypes::SSHORT => {
                reader.slice(Some(value_offset), Some(value_offset + type_count * 2))
            }
            GeoTIFFTypes::LONG | GeoTIFFTypes::SLONG | GeoTIFFTypes::FLOAT | GeoTIFFTypes::IFD => {
                reader.slice(Some(value_offset), Some(value_offset + type_count * 4))
            }
            GeoTIFFTypes::RATIONAL | GeoTIFFTypes::SRATIONAL | GeoTIFFTypes::DOUBLE => {
                reader.slice(Some(value_offset), Some(value_offset + type_count * 8))
            }
            GeoTIFFTypes::LONG8 | GeoTIFFTypes::SLONG8 | GeoTIFFTypes::IFD8 => {
                reader.slice(Some(value_offset), Some(value_offset + type_count * 8))
            }
        }
    }
}

/// Parse the raw geo keys
///
/// @param raw_geokeys - the raw geo keys
/// @param file_dir - the image file directory
/// @returns - the parsed geo keys
pub fn parse_geotiff_raw_geokeys(raw_geokeys: &[u16], file_dir: &GeoStore) -> GeoStore {
    let mut geo_key_directory = GeoStore::default();
    let mut i = 4;
    let geo_key_count = raw_geokeys[3] as usize * 4;
    while i <= geo_key_count {
        let key = raw_geokeys[i];
        let location = raw_geokeys[i + 1];
        let count = raw_geokeys[i + 2] as usize;
        let offset = raw_geokeys[i + 3];

        if location == 0 {
            geo_key_directory.set_short(key, offset as i16);
        } else if let Some((value, _)) = file_dir.get(location) {
            let offset = offset as usize;
            geo_key_directory.set(
                key,
                value[offset..(offset + count)].to_vec(),
                GeoTIFFTypes::BYTE,
            );
        }

        i += 4;
    }

    geo_key_directory
}
