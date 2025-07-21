use crate::parsers::{Buffer, RGBA, Reader};
use alloc::{string::String, vec, vec::Vec};
use s2json::{BBox3D, MValue, ValuePrimitive, VectorPoint};
use serde::{Deserialize, Serialize};

/// Extended VARIABLE LENGTH RECORDS:
/// The Extended Variable Length Records are used to add custom data to the LAZ Header Block.
/// This record type allows data to be much larger in size.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LASExtendedVariableLengthRecord {
    /// Reserved unsigned short 2 bytes
    pub reserved: u16,
    /// User ID char[16] 16 bytes
    pub user_id: String,
    /// Record ID unsigned short 2 bytes
    pub record_id: u16,
    /// Record Length After Header unsigned short 2 bytes (8 bytes for EVLR)
    pub record_length: u64,
    /// Description char[32] 32 bytes
    pub description: String,
    /// The data of the record
    pub data: Option<Vec<u8>>,
}
impl LASExtendedVariableLengthRecord {
    /// Create an LASVariableLengthRecord from reader
    pub fn from_reader<T: Reader>(reader: &T, offset: u64) -> Self {
        let record_length = reader.uint16_le(Some(offset + 20)) as u64;
        LASExtendedVariableLengthRecord {
            reserved: reader.uint16_le(Some(offset)),
            user_id: reader.parse_string(Some(offset + 2), Some(16)),
            record_id: reader.uint16_le(Some(offset + 18)),
            record_length,
            description: reader.parse_string(Some(offset + 22), Some(32)),
            data: if record_length > 0 {
                Some(reader.slice(Some(offset + 54), Some(offset + 54 + record_length)))
            } else {
                None
            },
        }
    }
    /// Create an LASExtendedVariableLengthRecord from reader
    pub fn from_reader_extended<T: Reader>(reader: &T, offset: u64) -> Self {
        let record_length = reader.uint64_le(Some(offset + 20));
        LASExtendedVariableLengthRecord {
            reserved: reader.uint16_le(Some(offset)),
            user_id: reader.parse_string(Some(offset + 2), Some(16)),
            record_id: reader.uint16_le(Some(offset + 18)),
            record_length,
            description: reader.parse_string(Some(offset + 28), Some(32)),
            data: if record_length > 0 {
                Some(reader.slice(Some(offset + 60), Some(offset + 60 + record_length)))
            } else {
                None
            },
        }
    }
}

/// LAS Header Block
/// Any field in the Public Header Block that is not required and is not used must be zero filled.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct LASHeader {
    /// File Signature ("LASF") char[4] 4 bytes
    /// The file signature must contain the four characters "LASF", and it is required by the LAS
    /// specification. These four characters can be checked by user software as a quick look initial
    /// determination of file type.
    pub signature: String,
    /// File Source ID unsigned short 2 bytes
    ///
    /// File Source ID (Flight Line Number if this file was derived from an original flight line):
    /// This field should be set to a value between 1 and 65,535, inclusive. A value of zero (0)
    /// is interpreted to mean that an ID has not been assigned. In this case, processing software is
    /// free to assign any LAS 1.2 3 valid number. Note that this scheme allows a LIDAR project to
    /// contain up to 65,535 unique sources. A source can
    pub source_id: u16,
    /// Global Encoding unsigned short 2 bytes.
    ///
    /// The meaning of GPS Time in the Point Records
    /// - 0 (not set) -> GPS time in the point record fields is GPS Week Time (the same as previous
    ///   versions of LAS).
    /// - 1 (set) -> GPS Time is standard GPS Time (satellite GPS Time) minus 1 x 109. The offset
    ///   moves the time back to near zero to improve floating point resolution.
    pub encoding: u16,
    /// Project ID - GUID data 1 unsigned long 4 bytes. 0 means no project ID
    pub project_id1: u32,
    /// Project ID - GUID data 2 unsigned short 2 byte. 0 means no project ID
    pub project_id2: u16,
    /// Project ID - GUID data 3 unsigned short 2 byte. 0 means no project ID
    pub project_id3: u16,
    /// Project ID - GUID data 4 unsigned char[8] 8 bytes. 0 means no project ID
    pub project_id4: String,
    /// Version Major unsigned char 1 byte
    pub major_version: u8,
    /// Version Minor unsigned char 1 byte
    pub minor_version: u8,
    /// System Identifier char[32] 32 bytes
    pub system_identifier: String,
    /// Generating Software char[32] 32 bytes
    pub generating_software: String,
    /// File Creation Day Year unsigned short 2 bytes. 0 means no creation date
    ///
    /// Day, expressed as an unsigned short, on which this file was created. Day is computed as the
    /// Greenwich Mean Time (GMT) day. January 1 is considered day 1.
    pub file_creation_day: u16,
    /// File Creation Day Year unsigned short 2 bytes. 0 means no creation date
    ///
    /// The year, expressed as a four digit number, in which the file was created.
    pub file_creation_year: u16,
    /// Header Size unsigned short 2 bytes
    ///
    /// The size, in bytes, of the Public Header Block itself. In the event that the header is extended
    /// by a software application through the addition of data at the end of the header, the Header
    /// Size field must be updated with the new header size. Extension of the Public Header Block is
    /// discouraged; the Variable Length Records should be used whenever possible to add custom header
    /// data. In the event a generating software package adds data to the Public Header Block, this
    /// data must be placed at the end of the structure and the Header Size must be updated to reflect
    /// the new size.
    pub header_size: u16,
    /// Offset to Point Data unsigned int 4 bytes
    ///
    /// The actual number of bytes from the beginning of the file to the first field of the first point
    /// record data field. This data offset must be updated if any software adds data from the Public
    /// Header Block or adds/removes data to/from the Variable Length Records.
    pub offset_to_points: u32,
    /// Number of Variable Length Records unsigned int 4 bytes
    /// This field contains the current number of Variable Length Records. This number must be updated
    /// if the number of Variable Length Records changes at any time.
    pub num_variable_length_records: u32,
    /// Point Data Format ID unsigned short 1 byte
    ///
    /// The point data format ID corresponds to the point data record format type.
    /// LAS 1.4 defines types 0-10.
    pub point_data_format_id: u8,
    /// Point Data Record Length unsigned short 2 bytes
    pub point_data_record_length: u16,
    /// Number of point records unsigned long 4 bytes
    pub num_points: u32,
    /// Number of points by return unsigned long[5] 20 bytes
    pub num_points_by_return: [u32; 5],
    /// X scale factor double 8 bytes
    pub x_scale_factor: f64,
    /// Y scale factor double 8 bytes
    pub y_scale_factor: f64,
    /// Z scale factor double 8 bytes
    pub z_scale_factor: f64,
    /// X offset double 8 bytes
    pub x_offset: f64,
    /// Y offset double 8 bytes
    pub y_offset: f64,
    /// Z offset double 8 bytes
    pub z_offset: f64,
    /// Max X double 8 bytes
    pub max_x: f64,
    /// Min X double 8 bytes
    pub min_x: f64,
    /// Max Y double 8 bytes
    pub max_y: f64,
    /// Min Y double 8 bytes
    pub min_y: f64,
    /// Max Z double 8 bytes
    pub max_z: f64,
    /// Min Z double 8 bytes
    pub min_z: f64,
    /// Start of Waveform Data Packet Record - Unsigned long long 8 bytes
    pub waveform_data_packet_offset: u64,
    /// Start of first Extended Variable Length Record - unsigned long long 8 bytes
    pub extended_variable_length_record_offset: u64,
    /// Number of Extended Variable Length Records - unsigned long 4 bytes
    pub extended_variable_length_size: u32,
    /// Number of points by return unsigned long long [15] 120 bytes *
    pub num_points_by_return_ll: [u64; 15],
}
impl LASHeader {
    /// Get the bounding box
    pub fn bbox(&self) -> BBox3D {
        BBox3D::new(self.min_x, self.min_y, self.min_z, self.max_x, self.max_y, self.max_z)
    }
    /// Create from a reader
    pub fn from_reader<T: Reader>(reader: &T) -> Self {
        let mut header = LASHeader {
            // Main components
            signature: reader.parse_string(Some(0), Some(4)),
            source_id: reader.uint16_le(Some(4)),
            encoding: reader.uint16_le(Some(6)),
            project_id1: reader.uint32_le(Some(8)),
            project_id2: reader.uint16_le(Some(12)),
            project_id3: reader.uint16_le(Some(14)),
            project_id4: reader.parse_string(Some(16), Some(8)),
            major_version: reader.uint8(Some(24)),
            minor_version: reader.uint8(Some(25)),
            system_identifier: reader.parse_string(Some(26), Some(32)),
            generating_software: reader.parse_string(Some(58), Some(32)),
            file_creation_day: reader.uint16_le(Some(90)),
            file_creation_year: reader.uint16_le(Some(92)),
            header_size: reader.uint16_le(Some(94)),
            offset_to_points: reader.uint32_le(Some(96)),
            num_variable_length_records: reader.uint32_le(Some(100)),
            point_data_format_id: reader.uint8(Some(104)),
            point_data_record_length: reader.uint16_le(Some(105)),
            num_points: reader.uint32_le(Some(107)),
            num_points_by_return: [
                reader.uint32_le(Some(111)),
                reader.uint32_le(Some(115)),
                reader.uint32_le(Some(119)),
                reader.uint32_le(Some(123)),
                reader.uint32_le(Some(127)),
            ],
            x_scale_factor: reader.f64_le(Some(131)),
            y_scale_factor: reader.f64_le(Some(139)),
            z_scale_factor: reader.f64_le(Some(147)),
            x_offset: reader.f64_le(Some(155)),
            y_offset: reader.f64_le(Some(163)),
            z_offset: reader.f64_le(Some(171)),
            max_x: reader.f64_le(Some(179)),
            min_x: reader.f64_le(Some(187)),
            max_y: reader.f64_le(Some(195)),
            min_y: reader.f64_le(Some(203)),
            max_z: reader.f64_le(Some(211)),
            min_z: reader.f64_le(Some(219)),
            ..Default::default()
        };
        // 1.4 header components
        if header.header_size > 227 {
            header.waveform_data_packet_offset = reader.uint64_le(Some(227));
        }
        if header.header_size > 235 {
            header.extended_variable_length_record_offset = reader.uint64_le(Some(235));
        }
        if header.header_size > 243 {
            header.extended_variable_length_size = reader.uint32_le(Some(243));
        }
        // re-adjust numPoints and numPointsByReturn if header includes modern numPoints variable
        if header.header_size > 247 {
            header.num_points = reader.uint32_le(Some(247));
        }
        // set new numPointsByReturn if header includes
        if header.header_size > 251 {
            let mut cur_offset = 251;
            for i in 0..15 {
                header.num_points_by_return_ll[i] = reader.uint64_le(Some(cur_offset));
                cur_offset += 8;
            }
        }

        header
    }
}

/// Enum representing the LAZ header item type.
/// NOTE: The number in the name, for example in “Point10”, refers to the LAS and LAZ
/// version where that type got added.
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum LAZHeaderItemType {
    /// `BYTE` (extra bytes that are appended to a LAS Point Data Record Format 0 to 5)
    #[default]
    Byte = 0,
    /// `SHORT` (reserved, unsupported)
    Short = 1,
    /// `INT` (reserved, unsupported)
    Int = 2,
    /// `LONG` (reserved, unsupported)
    Long = 3,
    /// `FLOAT` (reserved, unsupported)
    Float = 4,
    /// `DOUBLE` (reserved, unsupported)
    Double = 5,
    /// `POINT10` (LAS Point Data Record Format 0, containing the core fields that are shared
    /// between LAS Point Data Record Formats 0 to 5)
    Point10 = 6,
    /// `GPSTIME11` (the GPS Time field that is added for LAS Point Data Record Formats 1, 3,
    /// 4 and 5)
    GpsTime11 = 7,
    /// `RGB12` (the R, G and B fields that are added for LAS Point Data Record Formats 2, 3 and 5)
    Rgb12 = 8,
    /// `WAVEPACKET13` (the 7 fields for the Waveform packet that are added for LAS Point Data
    /// Record Formats 4 and 5)
    WavePacket13 = 9,
    /// `POINT14` (LAS Point Data Record Format 6, containing the core fields that are shared
    /// between LAS Point Data Record Formats 6 to 10)
    Point14 = 10,
    /// `RGB14` (the R, G and B fields that are added for LAS Point Data Record Format 7)
    Rgb14 = 11,
    /// `RGBNIR14` (the R, G, B and NIR (near infrared) fields that are added for LAS Point
    /// Data Record Formats 8 and 10)
    RgbNir14 = 12,
    /// `WAVEPACKET14` (the 7 fields for the Waveform packet that are added for LAS Point Data
    /// Record Formast 9 and 10)
    WavePacket14 = 13,
    /// `BYTE14` (extra bytes that are appended to a LAS Point Data Record Format 6 to 10)
    Byte14 = 14,
}
impl From<u16> for LAZHeaderItemType {
    fn from(value: u16) -> Self {
        match value {
            1 => Self::Short,
            2 => Self::Int,
            3 => Self::Long,
            4 => Self::Float,
            5 => Self::Double,
            6 => Self::Point10,
            7 => Self::GpsTime11,
            8 => Self::Rgb12,
            9 => Self::WavePacket13,
            10 => Self::Point14,
            11 => Self::Rgb14,
            12 => Self::RgbNir14,
            13 => Self::WavePacket14,
            14 => Self::Byte14,
            _ => Self::Byte,
        }
    }
}

/// Enum representing the LAZ Item type
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum LAZCompressor {
    /// No Compression (Uncompressed Standard LAS file)
    #[default]
    None = 0,
    /// Pointwise compression (only for point types 0 to 5)
    Pointwise = 1,
    /// Pointwise and chunked compression (only for point types 0 to 5)
    PointwiseAndChunked = 2,
    /// Layered and chunked compression (only for point types 6 to 10)
    LayeredAndChunked = 3,
}
impl From<u16> for LAZCompressor {
    fn from(value: u16) -> Self {
        match value {
            1 => Self::Pointwise,
            2 => Self::PointwiseAndChunked,
            3 => Self::LayeredAndChunked,
            _ => Self::None,
        }
    }
}

/// A LAZ Header Item
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct LAZHeaderItem {
    /// U16 type: 2 bytes * num_items
    pub r#type: LAZHeaderItemType,
    /// U16 size: 2 bytes * num_items
    pub size: u16,
    /// U16 version: 2 bytes * num_items
    pub version: u16,
}

/// A LAZ Header
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LAZHeader {
    /// Compressor unsigned short 2 bytes *
    pub compressor: LAZCompressor,
    /// Coder unsigned short 2 bytes *
    pub coder: u16,
    /// Version Major unsigned char 1 byte *
    pub version_major: u8,
    /// Version Minor unsigned char 1 byte *
    pub version_minor: u8,
    /// Version Revision unsigned short 2 bytes *
    pub version_revision: u16,
    /// Options unsigned long 4 bytes *
    pub options: u32,
    /// Chunk Size unsigned long 4 bytes *
    pub chunk_size: u32,
    /// Number of special EVLRs signed long long 8 bytes *
    pub num_special_evlrs: i64,
    /// Offset of special EVLRs signed long long 8 bytes *
    pub offset_special_evlrs: i64,
    /// Number of Items unsigned short 2 bytes *
    pub num_items: u16,
    /// Item records Array of “Item record” 6 bytes * Number of Items *
    pub items: Vec<LAZHeaderItem>,
}
impl LAZHeader {
    /// Build LAZ Header
    pub fn from_bytes(data: Vec<u8>) -> Self {
        let mut raw_header = Buffer::from(data);
        let mut header = LAZHeader {
            compressor: LAZCompressor::from(raw_header.get_u16_at(0)),
            coder: raw_header.get_u16_at(2),
            version_major: raw_header.get_u8_at(4),
            version_minor: raw_header.get_u8_at(5),
            version_revision: raw_header.get_u16_at(6),
            options: raw_header.get_u32_at(8),
            chunk_size: raw_header.get_u32_at(12),
            num_special_evlrs: raw_header.get_i64_at(16),
            offset_special_evlrs: raw_header.get_i64_at(24),
            num_items: raw_header.get_u16_at(32),
            items: vec![],
        };
        // Parse items
        for i in 0..header.num_items as usize {
            header.items.push(LAZHeaderItem {
                r#type: (raw_header.get_u16_at(34 + i * 6)).into(),
                size: raw_header.get_u16_at(36 + i * 6),
                version: raw_header.get_u16_at(38 + i * 6),
            });
        }

        header
    }
}

/// A Waveform Packet of type 13 or 14
#[derive(Debug, Default, Clone, PartialEq, MValue, ValuePrimitive, Serialize, Deserialize)]
pub struct WavePacket {
    /// Wave Packet Descriptor Index
    pub descriptor_index: u8,
    /// Byte offset to Waveform Packet Data
    pub offset: u64,
    /// Waveform packet size in bytes
    pub length: u32,
    /// Return Point location
    pub return_point: f32,
    /// X
    pub x_t: f32,
    /// Y
    pub y_t: f32,
    /// Z
    pub z_t: f32,
}
impl WavePacket {
    /// Build LAS Point from Format4 or Format5
    pub fn from_reader<T: Reader>(reader: &T, offset: u64) -> Self {
        WavePacket {
            descriptor_index: reader.uint8(Some(offset)),
            offset: reader.uint64_le(Some(offset + 1)),
            length: reader.uint32_le(Some(offset + 9)),
            return_point: reader.f32_le(Some(offset + 13)),
            x_t: reader.f32_le(Some(offset + 17)),
            y_t: reader.f32_le(Some(offset + 21)),
            z_t: reader.f32_le(Some(offset + 25)),
        }
    }

    /// Convert to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Buffer::default();
        buf.set_u8_at(0, self.descriptor_index);
        buf.set_u64_at(1, self.offset);
        buf.set_u32_at(9, self.length);
        buf.set_f32_at(13, self.return_point);
        buf.set_f32_at(17, self.x_t);
        buf.set_f32_at(21, self.y_t);
        buf.set_f32_at(25, self.z_t);
        buf.take()
    }
}

/// A Classification Flag as an enum
#[derive(Debug, Default, Clone, PartialEq)]
pub enum ClassFlag {
    /// Synthetic
    Synthetic,
    /// Key-point
    KeyPoint,
    /// Withheld
    Withheld,
    /// Overlap
    Overlap,
    /// Unknown
    #[default]
    Unknown,
}
impl From<u8> for ClassFlag {
    fn from(class: u8) -> Self {
        match class {
            0 => ClassFlag::Synthetic,
            1 => ClassFlag::KeyPoint,
            2 => ClassFlag::Withheld,
            3 => ClassFlag::Overlap,
            _ => ClassFlag::Unknown,
        }
    }
}

/// A LAS Point Data Record. Compatible with Point Data Record Format 0 to 10
#[derive(Debug, Default, Clone, PartialEq, MValue, Serialize, Deserialize)]
pub struct LASPoint {
    // POINT10 components inherited
    /// X coordinate
    pub x: i32,
    /// Y coordinate
    pub y: i32,
    /// Z coordinate
    pub z: i32,
    /// Intensity
    pub intensity: u16,
    /// flags
    pub flags: u8,
    //? flags start
    /// Return Number
    pub return_number: u8,
    /// Number of Returns
    pub number_of_returns: u8,
    /// Scan Direction Flag
    pub scan_direction_flag: bool,
    /// Edge of Flight Line
    pub edge_of_flight_line: bool,
    //? flags end
    /// Classification
    pub classification: u8,
    //? flags2 start
    /// True if it's synthetic
    pub is_synthetic: bool,
    /// True if it's key point
    pub is_key_point: bool,
    /// True if it's withheld
    pub is_withheld: bool,
    //? flags2 end
    /// Overlap
    pub scan_angle_rank: i8,
    /// User Data
    pub user_data: u8,
    /// Point Source ID
    pub point_source_id: u16,

    // POINT14 EXTENSION
    /// Legacy Point Type
    pub legacy_point_type: u8,
    /// Legacy Classification
    pub legacy_classification: u8,
    /// Legacy Return Number
    pub legacy_return_number: u8,
    /// Legacy Number of Returns
    pub legacy_number_of_returns: u8,
    /// Legacy Scan Direction Flag
    pub legacy_scan_angle_rank: i8,
    /// Scanner Channel is used to indicate the channel (scanner head) of a multichannel system
    pub scanner_channel: u8,
    /// Classification flags are used to indicate special characteristics associated with the point.
    pub class_flag: u8,
    /// The Scan Angle is a signed short that represents the rotational position of the
    /// emitted laser pulse with respect to the vertical of the coordinate system of the data. Down in the
    /// data coordinate system is the 0.0 position. Each increment represents 0.006 degrees.
    pub scan_angle: i16,

    // GPSTIME11
    /// GPS Time Change
    pub gps_time_change: Option<u8>,
    /// GPS Time
    pub gps_time: Option<f64>,

    // RGB12 & RGB14
    /// RGB Color
    pub rgba: Option<RGBA>,

    // WAVEPACKET13 & WAVEPACKET14
    /// Wave Packet Data
    pub wave_packet: Option<WavePacket>,

    // NIR
    /// NIR: The NIR (near infrared) channel value associated with this point.
    pub nir: Option<u16>,
}
impl LASPoint {
    /// Build LAS Point from Format0
    pub fn format0<T: Reader>(reader: &T, offset: u64) -> Self {
        let mut res = LASPoint::default();
        res.inject_point10(reader, offset);
        res
    }
    /// Build LAS Point from Format1
    pub fn format1<T: Reader>(reader: &T, offset: u64) -> Self {
        let mut res = LASPoint::default();
        res.inject_point10(reader, offset);
        res.inject_gps_time(reader, offset + 20);
        res
    }
    /// Build LAS Point from Format2
    pub fn format2<T: Reader>(reader: &T, offset: u64) -> Self {
        let mut res = LASPoint::default();
        res.inject_point10(reader, offset);
        res.inject_rgb(reader, offset + 20);
        res
    }
    /// Build LAS Point from Format3
    pub fn format3<T: Reader>(reader: &T, offset: u64) -> Self {
        let mut res = LASPoint::default();
        res.inject_point10(reader, offset);
        res.inject_gps_time(reader, offset + 20);
        res.inject_rgb(reader, offset + 28);
        res
    }
    /// Build LAS Point from Format4
    pub fn format4<T: Reader>(reader: &T, offset: u64) -> Self {
        let mut res = LASPoint::default();
        res.inject_point10(reader, offset);
        res.inject_gps_time(reader, offset + 20);
        res.inject_wave_packet(reader, offset + 28);
        res
    }
    /// Build LAS Point from Format5
    pub fn format5<T: Reader>(reader: &T, offset: u64) -> Self {
        let mut res = LASPoint::default();
        res.inject_point10(reader, offset);
        res.inject_gps_time(reader, offset + 20);
        res.inject_rgb(reader, offset + 28);
        res.inject_wave_packet(reader, offset + 34);
        res
    }
    /// Build LAS Point from Format6
    pub fn format6<T: Reader>(reader: &T, offset: u64) -> Self {
        let mut res = LASPoint::default();
        res.inject_point14(reader, offset, false);
        res
    }
    /// Build LAS Point from Format7
    pub fn format7<T: Reader>(reader: &T, offset: u64) -> Self {
        let mut res = LASPoint::default();
        res.inject_point14(reader, offset, false);
        res.inject_rgb(reader, offset + 30);
        res
    }
    /// Build LAS Point from Format8
    pub fn format8<T: Reader>(reader: &T, offset: u64) -> Self {
        let mut res = LASPoint::default();
        res.inject_point14(reader, offset, false);
        res.inject_rgb_nir(reader, offset + 30);
        res
    }
    /// Build LAS Point from Format9
    pub fn format9<T: Reader>(reader: &T, offset: u64) -> Self {
        let mut res = LASPoint::default();
        res.inject_point14(reader, offset, false);
        res.inject_wave_packet(reader, offset + 30);
        res
    }
    /// Build LAS Point from Format10
    pub fn format10<T: Reader>(reader: &T, offset: u64) -> Self {
        let mut res = LASPoint::default();
        res.inject_point14(reader, offset, false);
        res.inject_rgb_nir(reader, offset + 30);
        res.inject_wave_packet(reader, offset + 38);
        res
    }
    /// Inject POINT10
    pub fn inject_point10<T: Reader>(&mut self, reader: &T, offset: u64) {
        let flags1 = reader.uint8(Some(offset + 14));
        let flags2 = reader.uint8(Some(offset + 15));
        self.x = reader.int32_le(Some(offset));
        self.y = reader.int32_le(Some(offset + 4));
        self.z = reader.int32_le(Some(offset + 8));
        self.intensity = reader.uint16_le(Some(offset + 12));
        self.set_flags(flags1, false);
        self.set_flags2(flags2);
        self.scan_angle_rank = reader.int8(Some(offset + 16));
        self.user_data = reader.uint8(Some(offset + 17));
        self.point_source_id = reader.uint16_le(Some(offset + 18));
    }
    /// Inject temporary POINT14
    pub fn inject_point14_temp<T: Reader>(&mut self, reader: &T, offset: u64) {
        let flags1 = reader.uint8(Some(offset + 14));
        let flags2 = reader.uint8(Some(offset + 15));
        let flags3 = reader.uint8(Some(offset + 16));
        self.x = reader.int32_le(Some(offset));
        self.y = reader.int32_le(Some(offset + 4));
        self.z = reader.int32_le(Some(offset + 8));
        self.intensity = reader.uint16_le(Some(offset + 12));
        self.flags = flags1;
        self.return_number = flags1 & 0b0000_1111;
        self.number_of_returns = (flags1 & 0b1111_0000) >> 4;
        self.class_flag = flags2 & 0b0000_1111;
        self.scanner_channel = (flags2 & 0b0011_0000) >> 4;
        self.scan_direction_flag = (flags2 & 0b0100_0000) != 0;
        self.edge_of_flight_line = (flags2 & 0b1000_0000) != 0;
        self.classification = flags3;
        self.user_data = reader.uint8(Some(offset + 17));
        self.scan_angle = reader.int16_le(Some(offset + 18));
        self.point_source_id = reader.uint16_le(Some(offset + 20));
    }
    /// Inject POINT14
    pub fn inject_point14<T: Reader>(&mut self, reader: &T, offset: u64, compressed: bool) {
        let flags1 = reader.uint8(Some(offset + 14));
        let flags2 = reader.uint8(Some(offset + 15));
        let flags3 = reader.uint8(Some(offset + 22));
        self.x = reader.int32_le(Some(offset));
        self.y = reader.int32_le(Some(offset + 4));
        self.z = reader.int32_le(Some(offset + 8));
        self.intensity = reader.uint16_le(Some(offset + 12));
        self.set_flags(flags1, true);
        self.set_classification(flags2);
        self.set_flags3(flags3);
        self.legacy_scan_angle_rank = reader.int8(Some(offset + 16));
        self.user_data = reader.uint8(Some(offset + 17));
        self.scan_angle = reader.int16_le(Some(offset + 18));
        self.point_source_id = reader.uint16_le(Some(offset + 20));
        // Compressed LASzip 1.4 points only
        if compressed {
            self.classification = reader.uint8(Some(offset + 23));
            let flags4 = reader.uint8(Some(offset + 24));
            self.return_number = flags4 & 0b0000_1111;
            self.number_of_returns = (flags4 & 0b1111_0000) >> 4;
            self.gps_time_change = Some(reader.uint8(Some(offset + 28)));
            self.gps_time = Some(reader.f64_le(Some(offset + 29)));
            self.inject_rgb(reader, offset + 37);
        }
    }
    /// Set Flags
    pub fn set_flags(&mut self, flags: u8, point14: bool) {
        self.flags = flags;
        if point14 {
            self.legacy_return_number = flags & 0b0000_0111; // 3 bits (bits 0 – 2)
            self.legacy_number_of_returns = (flags & 0b0011_1000) >> 2; // 3 bits (bits 3 – 5)
        } else {
            self.return_number = flags & 0b0000_0111; // 4 bits (bits 0 – 3)
            self.number_of_returns = (flags & 0b0011_1000) >> 3; // 4 bits (bits 4 – 7)
        }
        self.scan_direction_flag = (flags & 0b0100_0000) != 0; // 1 bit (bit 6)
        self.edge_of_flight_line = (flags & 0b1000_0000) != 0; // 1 bit (bit 7)
    }
    /// Set Flags 2
    pub fn set_flags2(&mut self, class: u8) {
        self.classification = class;
        self.class_flag = class & 0b1111; // 4 bis (bit 0 - 3)
        self.scanner_channel = (class & 0b0011_0000) >> 4; // 2 bits (bit 4 - 5)
        self.is_synthetic = (class & 0b0010_0000) != 0;
        self.is_key_point = (class & 0b0100_0000) != 0;
        self.is_withheld = (class & 0b1000_0000) != 0;
    }
    /// Set Flags 2 14
    pub fn set_classification(&mut self, class: u8) {
        self.legacy_classification = class & 0b1_1111;
        self.is_synthetic = (class & 0b0010_0000) != 0;
        self.is_key_point = (class & 0b0100_0000) != 0;
        self.is_withheld = (class & 0b1000_0000) != 0;
    }
    /// get the class flag as an enum
    pub fn class_flag(&self) -> ClassFlag {
        self.class_flag.into()
    }
    /// class type
    pub fn class_type(&self, point14: bool) -> LASClassification {
        if point14 { self.legacy_classification.into() } else { self.classification.into() }
    }
    /// class type 14
    pub fn class_type14(&self) -> LASClassification14 {
        self.classification.into()
    }
    /// Set Classification14
    pub fn set_flags3(&mut self, class: u8) {
        self.legacy_point_type = class & 0b11;
        self.scanner_channel = (class & 0b1100) >> 2;
        self.class_flag = (class & 0b1111_0000) >> 4;
    }
    /// Inject GPSTIME11
    pub fn inject_gps_time<T: Reader>(&mut self, reader: &T, offset: u64) {
        self.gps_time = Some(reader.f64_le(Some(offset)));
    }
    /// Inject RGB12 & RGB14
    pub fn inject_rgb<T: Reader>(&mut self, reader: &T, offset: u64) {
        self.rgba = Some(RGBA::from_reader(reader, Some(offset)));
    }
    /// Inject NIR
    pub fn inject_nir<T: Reader>(&mut self, reader: &T, offset: u64) {
        self.nir = Some(reader.uint16_le(Some(offset)));
    }
    /// Inject 8 bytes (2 bytes each for R, G, B, and NIR)
    pub fn inject_rgb_nir<T: Reader>(&mut self, reader: &T, offset: u64) {
        self.inject_rgb(reader, offset);
        self.inject_nir(reader, offset + 6);
    }
    /// Inject WAVEPACKET13 & WAVEPACKET14
    pub fn inject_wave_packet<T: Reader>(&mut self, reader: &T, offset: u64) {
        self.wave_packet = Some(WavePacket::from_reader(reader, offset));
    }
    /// To Vector Point
    pub fn to_vector_point(&self, header: &LASHeader) -> VectorPoint<LASPoint> {
        let LASHeader {
            x_offset,
            y_offset,
            z_offset,
            x_scale_factor,
            y_scale_factor,
            z_scale_factor,
            ..
        } = header;
        VectorPoint::new_xyz(
            self.x as f64 * x_scale_factor + x_offset,
            self.y as f64 * y_scale_factor + y_offset,
            self.z as f64 * z_scale_factor + z_offset,
            Some(self.clone()),
        )
    }
    /// To Buffer
    pub fn to_buffer_14(&self, compressed: bool) -> Vec<u8> {
        let mut buf = Buffer::new(vec![0u8; 48]);
        buf.set_i32_at(0, self.x);
        buf.set_i32_at(4, self.y);
        buf.set_i32_at(8, self.z);
        buf.set_u16_at(12, self.intensity);
        buf.set_u8_at(
            14,
            (self.return_number & 0b0000_0111)
                | ((self.number_of_returns & 0b0000_0111) << 3)
                | ((self.scan_direction_flag as u8) << 6)
                | ((self.edge_of_flight_line as u8) << 7),
        );
        buf.set_u8_at(
            15,
            self.legacy_classification
                | (self.is_synthetic as u8) << 5
                | (self.is_key_point as u8) << 6
                | (self.is_withheld as u8) << 7,
        );
        buf.set_i8_at(16, self.legacy_scan_angle_rank);
        buf.set_u8_at(17, self.user_data);
        // FIX FROM HERE
        buf.set_i16_at(18, self.scan_angle);
        buf.set_u16_at(20, self.point_source_id);
        if compressed {
            buf.set_u8_at(
                22,
                self.legacy_point_type | (self.scanner_channel << 2) | (self.class_flag << 4),
            );
            buf.set_u8_at(23, self.classification);
            buf.set_u8_at(24, self.return_number + (self.number_of_returns << 4));
            buf.set_u8_at(28, self.gps_time_change.unwrap_or(0));
            buf.set_f64_at(29, self.gps_time.unwrap_or(0.));
            let (r, g, b, _) = self.rgba.unwrap_or_default().to_u16s();
            buf.set_u16_at(37, r);
            buf.set_u16_at(39, g);
            buf.set_u16_at(41, b);
        }
        buf.take()
    }
}

/// A Classification Type Flag as an enum
#[derive(Debug, Default, Clone, PartialEq)]
pub enum LASClassification {
    /// Created, Never Classified
    CreatedNeverClassified,
    /// Unclassified
    #[default]
    Unclassified,
    /// Ground
    Ground,
    /// Low Vegetation
    LowVegetation,
    /// Medium Vegetation
    MediumVegetation,
    /// High Vegetation
    HighVegetation,
    /// Building
    Building,
    /// Low Point Noise
    LowPointNoise,
    /// Model Key-point (mass point)
    ModelKeyPointMassPoint,
    /// Water
    Water,
    /// Overlap Points
    OverlapPoints,
    /// Reserved
    Reserved,
}
impl From<u8> for LASClassification {
    fn from(class: u8) -> Self {
        match class {
            0 => Self::CreatedNeverClassified,
            1 => Self::Unclassified,
            2 => Self::Ground,
            3 => Self::LowVegetation,
            4 => Self::MediumVegetation,
            5 => Self::HighVegetation,
            6 => Self::Building,
            7 => Self::LowPointNoise,
            8 => Self::ModelKeyPointMassPoint,
            9 => Self::Water,
            12 => Self::OverlapPoints,
            _ => Self::Reserved,
        }
    }
}

/// A Classification Type Flag as an enum
#[derive(Debug, Default, Clone, PartialEq)]
pub enum LASClassification14 {
    /// Created, Never Classified
    CreatedNeverClassified,
    /// Unclassified
    #[default]
    Unclassified,
    /// Ground
    Ground,
    /// Low Vegetation
    LowVegetation,
    /// Medium Vegetation
    MediumVegetation,
    /// High Vegetation
    HighVegetation,
    /// Building
    Building,
    /// Low Point Noise
    LowPointNoise,
    /// Model Key-point (mass point)
    ModelKeyPointMassPoint,
    /// Water
    Water,
    /// Rail
    Rail,
    /// Road Surface
    RoadSurface,
    /// Wire – Guard (Shield)
    WireGuardShield,
    /// Wire – Conductor (Phase)
    WireConductorPhase,
    /// Transmission Tower
    TransmissionTower,
    /// Wire-structure Connector (e.g. Insulator)
    WireStructureConnector,
    /// Bridge Deck
    BridgeDeck,
    /// High Noise
    HighNoise,
    /// Overhead Structure (e.g., conveyors, mining equipment, traffic lights)
    OverheadSructure,
    /// Ignored Ground (e.g., breakline proximity)
    IgnoredGround,
    /// Snow
    Snow,
    /// Temporal Exclusion (Features excluded due to changes over time between
    /// data sources, e.g., water levels, landslides, permafrost)
    TemporalExclusion,
    /// Reserved
    Reserved,
    /// User Definable
    UserDefinable,
}
impl From<u8> for LASClassification14 {
    fn from(class: u8) -> Self {
        match class {
            0 => Self::CreatedNeverClassified,
            1 => Self::Unclassified,
            2 => Self::Ground,
            3 => Self::LowVegetation,
            4 => Self::MediumVegetation,
            5 => Self::HighVegetation,
            6 => Self::Building,
            7 => Self::LowPointNoise,
            8 => Self::ModelKeyPointMassPoint,
            9 => Self::Water,
            10 => Self::Rail,
            11 => Self::RoadSurface,
            13 => Self::WireGuardShield,
            14 => Self::WireConductorPhase,
            15 => Self::TransmissionTower,
            16 => Self::WireStructureConnector,
            17 => Self::BridgeDeck,
            18 => Self::HighNoise,
            19 => Self::OverheadSructure,
            20 => Self::IgnoredGround,
            21 => Self::Snow,
            22 => Self::TemporalExclusion,
            // 23..=63 => Self::Reserved,
            64..=255 => Self::UserDefinable,
            _ => Self::Reserved,
        }
    }
}
