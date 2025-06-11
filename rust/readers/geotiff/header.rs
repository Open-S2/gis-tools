// https://docs.ogc.org/is/19-008r4/19-008r4.html#_requirements_class_tiff
use super::{
    TiePoint,
    constants::{ARRAY_FIELDS, FieldTypes, field_tag_name, geo_key_name},
};
use crate::parsers::Reader;
use alloc::{collections::BTreeMap, string::String};
use s2json::PrimitiveValue;

/// Container to store the parsed file directory, geo key directory and
/// offset to the next IFD
pub type ImageFileDirectory = BTreeMap<String, PrimitiveValue>;

/// A key value pair
#[derive(Debug, Clone, Default, PartialEq)]
struct KeyValue {
    key: String,
    value: PrimitiveValue,
}

/// The pixel scale
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PixelScale {
    /// The pixel x scale
    pub x: f64,
    /// The pixel y scale
    pub y: f64,
    /// The pixel z scale
    pub z: f64,
}

///GeoTIFF Header Reader
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GeoTIFFHeaderReader<T: Reader> {
    reader: T,
    /// true if reading in the data is little endian
    pub little_endian: bool,
    /// true if reading in the data is big endian
    pub big_tiff: bool,
    /// the image file directory
    pub image_directories: ImageFileDirectory,
    /// the geo key directory
    pub geo_key_directory: ImageFileDirectory,
    /// The pixel scale
    pub pixel_scale: PixelScale,
    /// The tie point
    pub tie_point: TiePoint,
}
impl<T: Reader> GeoTIFFHeaderReader<T> {
    /// Create a new GeoTIFFHeaderReader
    pub fn new(reader: T) -> GeoTIFFHeaderReader<T> {
        let mut tiff_reader = GeoTIFFHeaderReader {
            reader,
            little_endian: true,
            big_tiff: false,
            image_directories: BTreeMap::new(),
            geo_key_directory: BTreeMap::new(),
            pixel_scale: PixelScale::default(),
            tie_point: TiePoint::default(),
        };
        tiff_reader.parse_header();

        tiff_reader
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.image_directories.is_empty()
    }

    /** @returns - the number of images in the GeoTIFF */
    pub fn len(&self) -> usize {
        self.image_directories.len()
    }

    /// parses the header data to begin parsing the GeoTIFF
    fn parse_header(&mut self) {
        let GeoTIFFHeaderReader { reader, little_endian, .. } = self;
        let le = *little_endian;
        // pull the endianess from the header
        let bom = reader.uint16_be(Some(0));
        if bom == 0x4949 {
            self.little_endian = true;
        } else if bom == 0x4d4d {
            self.little_endian = false;
        } else {
            panic!("Invalid byte order value.");
        }

        let magic_number = if le { reader.uint16_le(Some(2)) } else { reader.uint16_be(Some(2)) };
        if magic_number == 42 {
            self.big_tiff = false;
        } else if magic_number == 43 {
            self.big_tiff = true;
            let offset_byte_size =
                if le { reader.uint16_le(Some(4)) } else { reader.uint16_be(Some(4)) };
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

        self.get_image_metadata(first_ifd_offset);
    }

    /**
     * Instructs to parse an image file directory at the given file offset.
     * As there is no way to ensure that a location is indeed the start of an IFD,
     * this function must be called with caution (e.g only using the IFD offsets from
     * the headers or other IFDs).
     * @param first_offset - the offset to begin parsing the IFDs (Image File Directory) at.
     */
    fn get_image_metadata(&mut self, first_offset: u64) {
        let GeoTIFFHeaderReader { big_tiff, little_endian, .. } = *self;
        let entry_size = if big_tiff { 20 } else { 12 };
        let offset_size = if big_tiff { 8 } else { 2 };
        let mut offset = first_offset;

        let mut ifd_offset = first_offset;
        while ifd_offset != 0 {
            let mut ifd: ImageFileDirectory = BTreeMap::new();
            let num_dir_entries = self.read_tag(offset);

            let mut i = offset + offset_size;
            let mut geokey_dir_offset: Option<u64> = None;
            let mut prev_tag = 0;
            for _ in 0..num_dir_entries {
                let field_tag = if little_endian {
                    self.reader.uint16_le(Some(i))
                } else {
                    self.reader.uint16_be(Some(i))
                };
                if field_tag < prev_tag {
                    panic!("Invalid IFD, {} < {}", field_tag, prev_tag);
                }
                prev_tag = field_tag;
                if field_tag == 33550 {
                    // PixelScaleTag
                    self.get_pixel_scale(i);
                } else if field_tag == 33922 {
                    // TiepointTag
                    self.get_tiepoint(i)
                } else if field_tag == 34735 {
                    // GeoKeyDirectory - map to use after all keys are cached.
                    geokey_dir_offset = Some(i);
                } else {
                    let KeyValue { key, value } = self.get_key_value(field_tag as u64, i);
                    ifd.insert(key, value);
                }
                // NOTE: Technically geotiffs support column encoding of double and ascii values. Seems like it's not common enough to use though
                // else if (field_tag == 34736) {
                //   // location of DoubleValues
                // } else if (field_tag == 34737) {
                //   // location of ASCIIValues
                // }
                i += entry_size;
            }
            // Validate it has a TransformationTag or a TiepointTag before storing
            //   if (geokey_dir_offset == undefined)
            // console.info('No GeoKeyDirectory found. May contain errors');
            if let Some(geokey_dir_offset) = geokey_dir_offset {
                self.get_geo_key_directory(geokey_dir_offset);
                //   ifd.GeoKeyDirectory = self.get_geo_key_directory(geokey_dir_offset, ifd);
                // ifd.insert(
                //     "GeoKeyDirectory".into(),
                //     PrimitiveValue::Object(self.get_geo_key_directory(geokey_dir_offset, ifd)),
                // );
            }
            //   else ifd.GeoKeyDirectory = self.get_geo_key_directory(geokey_dir_offset, ifd);
            //   if (ifd.tiepoint == undefined && ifd.ModelTransformation == undefined)
            // console.info('No ModelTiepoint or ModelTransformation found. May contain errors');
            if !ifd.is_empty() {
                // self.image_directories.push(ifd);
                self.image_directories = ifd;
            } else {
                break;
            }
            // increment offset and check for the next IFD
            // 814
            offset += offset_size + entry_size * num_dir_entries;
            ifd_offset = self.read_tag(offset);
            offset += offset_size;
        }
    }

    /**
     * Reads the value of the tag at the given offset (16 bits if not big_tIFF)
     * @param offset - the offset to read the tag from
     * @returns - the value of the tag
     */
    fn read_tag(&mut self, offset: u64) -> u64 {
        let Self { reader, big_tiff, little_endian, .. } = self;
        if *big_tiff {
            reader.uint64(Some(offset), Some(*little_endian))
        } else {
            reader.uint16(Some(offset), Some(*little_endian)) as u64
        }
    }

    /**
     * Reads the value of the tag at the given offset (32 bits if not big_tIFF)
     * @param offset - the offset to read the tag from
     * @returns - the value of the tag
     */
    fn read_offset(&mut self, offset: u64) -> u64 {
        let Self { reader, big_tiff, little_endian, .. } = self;
        if *big_tiff {
            reader.uint64(Some(offset), Some(*little_endian))
        } else {
            reader.uint32(Some(offset), Some(*little_endian)) as u64
        }
    }

    /**
     * Get the pixel scale from the GeoKeyDirectory
     * @param offset - the offset to begin parsing the IFDs (GeoKeyDirectory) at.
     * @returns the parsed GeoKeyDirectory
     */
    fn get_pixel_scale(&mut self, offset: u64) {
        let Self { little_endian, big_tiff, .. } = *self;
        let field_type = self.reader.uint16(Some(offset + 2), Some(little_endian));
        if field_type != 12 {
            panic!("Invalid GeoKeyDirectory type {}", field_type);
        }
        let num_keys = self.read_offset(offset + 4);
        if num_keys != 3 {
            panic!("Invalid GeoKeyDirectory num_keys {}", num_keys);
        }
        let value_offset = self.read_offset(offset + if big_tiff { 12 } else { 8 });

        self.pixel_scale.x = self.reader.f64(Some(value_offset), Some(little_endian));
        self.pixel_scale.y = self.reader.f64(Some(value_offset + 8), Some(little_endian));
        self.pixel_scale.z = self.reader.f64(Some(value_offset + 16), Some(little_endian));
    }

    /**
     * https://docs.ogc.org/is/19-008r4/19-008r4.html#_geokey_directory_test
     * @param offset - the offset to begin parsing the IFDs (GeoKeyDirectory) at.
     * @param file_dir - the parsed ImageFileDirectory thus far
     * @returns the parsed GeoKeyDirectory
     */
    fn get_geo_key_directory(&mut self, offset: u64) {
        todo!()
        // let { reader, big_tiff } = this;

        // let num_keys = self.read_offset(offset + 4);
        // let value_offset = self.read_offset(offset + (big_tiff ? 12 : 8));
        // let raw_geokeys = new Uint16Array(reader.slice(value_offset, value_offset + num_keys * 2).buffer);
        // let GeoKeyDirectory = parse_geotiff_raw_geokeys(raw_geokeys, file_dir);
        // // Validate that there is a GTModelType GeoKey in the GeoKey Directory
        // if (GeoKeyDirectory.GTModelTypeGeoKey == undefined) {
        //   panic!(`Missing "GTModelTypeGeoKey" in GeoKeyDirectory`);
        // }

        // GeoKeyDirectory
    }

    /**
     * @param offset - the offset to begin parsing the IFDs (TiepointTag) at.
     * @returns the parsed Tiepoint
     */
    fn get_tiepoint(&mut self, offset: u64) {
        let Self { big_tiff, little_endian, .. } = *self;
        // Validate that Bytes 2-3 = 12 (Double)
        let field_type = self.reader.uint16(Some(offset + 2), Some(little_endian));
        if field_type != 12 {
            panic!("Invalid TiepointTag type ${field_type}");
        }
        // get size to the value in Bytes 4-7
        let count = self.read_offset(offset + 4);
        // Set TagValue to the value in Bytes 8-11
        let value_offset = self.read_offset(offset + if big_tiff { 12 } else { 8 });
        for i in 0..count {
            let val = self.reader.f64(Some(value_offset + i * 8), Some(little_endian));
            match i {
                0 => self.tie_point.i = val,
                1 => self.tie_point.j = val,
                2 => self.tie_point.k = val,
                3 => self.tie_point.x = val,
                4 => self.tie_point.y = val,
                5 => self.tie_point.z = val,
                _ => panic!("Invalid TiepointTag index {}", i),
            }
        }
    }

    /**
     * @param field_tag - the tag to read
     * @param offset - the current offset in the IFD header data
     * @returns the parsed key value
     */
    fn get_key_value(&mut self, field_tag: u64, offset: u64) -> KeyValue {
        todo!()
        // let { reader, little_endian } = this;
        // let field_type = reader.uint16_be(offset + 2, little_endian);
        // let typeCount = self.read_offset(offset + 4);

        // let field_typeLength = get_field_type_length(field_type as keyof typeof FIELD_TAG_NAMES);
        // let value_offset = offset + (self.big_tiff ? 12 : 8);
        // let actualOffset =
        //   field_typeLength * typeCount <= (self.big_tiff ? 8 : 4)
        //     ? value_offset
        //     : self.read_offset(value_offset);
        // let value = self.get_value(field_tag, field_type, typeCount, actualOffset);

        // // write the tags value to the file directly
        // return {
        //   key: FIELD_TAG_NAMES[field_tag as keyof typeof FIELD_TAG_NAMES] as keyof ImageFileDirectory,
        //   value,
        // };
    }
}

//   /**
//    * @param field_tag - the tag to read
//    * @param field_type - the field type
//    * @param typeCount - the number of values
//    * @param value_offset - the value offset
//    * @returns - the parsed value
//    */
//   fn get_value(
//     field_tag: number,
//     field_type: number,
//     typeCount: number,
//     value_offset: number,
//   ): undefined | number | number[] | string {
//     const { reader, little_endian } = this;
//     const res: number[] = [];
//     if (field_type == FieldTypes.ASCII) {
//       return reader.parseString(value_offset, typeCount);
//     } else if (field_type == FieldTypes.BYTE || field_type == FieldTypes.UNDEFINED) {
//       for (let i = 0; i < typeCount; i++) res.push(reader.getUint8(value_offset + i));
//     } else if (field_type == FieldTypes.SBYTE) {
//       for (let i = 0; i < typeCount; i++) res.push(reader.getInt8(value_offset + i));
//     } else if (field_type == FieldTypes.SHORT) {
//       for (let i = 0; i < typeCount; i++)
//         res.push(reader.uint16_be(value_offset + i * 2, little_endian));
//     } else if (field_type == FieldTypes.SSHORT) {
//       for (let i = 0; i < typeCount; i++)
//         res.push(reader.getInt16(value_offset + i * 2, little_endian));
//     } else if (field_type == FieldTypes.LONG) {
//       for (let i = 0; i < typeCount; i++)
//         res.push(reader.getUint32(value_offset + i * 4, little_endian));
//     } else if (field_type == FieldTypes.SLONG) {
//       for (let i = 0; i < typeCount; i++)
//         res.push(reader.getInt32(value_offset + i * 4, little_endian));
//     } else if (field_type == FieldTypes.FLOAT) {
//       for (let i = 0; i < typeCount; i++)
//         res.push(reader.getFloat32(value_offset + i * 4, little_endian));
//     } else if (field_type == FieldTypes.RATIONAL) {
//       typeCount *= 2;
//       for (let i = 0; i < typeCount; i += 2) {
//         res.push(reader.getUint32(value_offset + i * 4, little_endian));
//         res.push(reader.getUint32(value_offset + i * 4 + 4, little_endian));
//       }
//     } else if (field_type == FieldTypes.SRATIONAL) {
//       typeCount *= 2;
//       for (let i = 0; i < typeCount; i += 2) {
//         res.push(reader.getInt32(value_offset + i * 4, little_endian));
//         res.push(reader.getInt32(value_offset + i * 4 + 4, little_endian));
//       }
//     } else if (field_type == FieldTypes.DOUBLE) {
//       for (let i = 0; i < typeCount; i++)
//         res.push(reader.getFloat64(value_offset + i * 8, little_endian));
//     } else if (field_type == FieldTypes.LONG8) {
//       for (let i = 0; i < typeCount; i++)
//         res.push(Number(reader.getBigUint64(value_offset + i * 8, little_endian)));
//     } else if (field_type == FieldTypes.SLONG8) {
//       for (let i = 0; i < typeCount; i++)
//         res.push(Number(reader.getBigInt64(value_offset + i * 8, little_endian)));
//     }

//     // unpack single values from the array
//     if (
//       typeCount == 1 &&
//       ARRAY_FIELDS.indexOf(field_tag) == -1 &&
//       !(field_type == FieldTypes.RATIONAL || field_type == FieldTypes.SRATIONAL)
//     ) {
//       return res[0];
//     } else {
//       return res;
//     }
//   }
// }

/**
 * Get the field type length
 * @param field_type - the field type
 * @returns - the field type length
 */
fn get_field_type_length(field_type: &str) -> usize {
    match field_type.into() {
        FieldTypes::BYTE | FieldTypes::ASCII | FieldTypes::SBYTE | FieldTypes::UNDEFINED => 1,
        FieldTypes::SHORT | FieldTypes::SSHORT => 2,
        FieldTypes::LONG | FieldTypes::SLONG | FieldTypes::FLOAT | FieldTypes::IFD => 4,
        FieldTypes::RATIONAL
        | FieldTypes::SRATIONAL
        | FieldTypes::DOUBLE
        | FieldTypes::LONG8
        | FieldTypes::SLONG8
        | FieldTypes::IFD8 => 8,
    }
}

// /**
//  * Parse the raw geo keys
//  * @param raw_geokeys - the raw geo keys
//  * @param file_dir - the image file directory
//  * @returns - the parsed geo keys
//  */
// export function parse_geotiff_raw_geokeys(
//   raw_geokeys: Uint16Array,
//   file_dir: ImageFileDirectory,
// ): GeoKeyDirectory {
//   const GeoKeyDirectory: GeoKeyDirectory = {};
//   for (let i = 4; i <= raw_geokeys[3] * 4; i += 4) {
//     const geoKey = raw_geokeys[i];
//     const key = GEO_KEY_NAMES[geoKey as keyof typeof GEO_KEY_NAMES];
//     const location =
//       raw_geokeys[i + 1] != 0
//         ? FIELD_TAG_NAMES[raw_geokeys[i + 1] as keyof typeof FIELD_TAG_NAMES]
//         : null;
//     const count = raw_geokeys[i + 2];
//     const offset = raw_geokeys[i + 3];

//     let value: null | string | number | number[] = null;
//     if (location == null) {
//       value = offset;
//     } else {
//       value = file_dir[location as keyof ImageFileDirectory] as string | number | number[];
//       if (typeof value == 'undefined' || value == null) {
//         panic!(`Could not get value of geoKey '${key}' at location '${location}'.`);
//       } else if (typeof value == 'string') {
//         value = value.substring(offset, offset + count - 1);
//       } else if (Array.isArray(value)) {
//         value = value.slice(offset, offset + count);
//         if (count == 1) value = value[0];
//       }
//     }
//     // @ts-expect-error - value assignment is ok here
//     GeoKeyDirectory[key] = value;
//   }
//   return GeoKeyDirectory;
// }
