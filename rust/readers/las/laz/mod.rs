/// Arithmetic Decoder tools
pub mod arithmetic_decoder;
/// LAZ Constants
pub mod constants;
/// Integer Compressor
pub mod integer_compressor;
/// LAZ Reader
pub mod reader;
/// LAZ Version 1 Reader
pub mod v1;
/// LAZ Version 2 Reader
pub mod v2;
/// LAZ Version 3 Reader
pub mod v3;

use super::{
    LASPoint,
    util::{i8_clamp, i16_quantize},
};
use crate::parsers::{BufferReader, Reader};
use alloc::boxed::Box;
pub use reader::*;

/// LAZ Item Reader
#[derive(Debug)]
pub enum ItemReaders<T: Reader> {
    /// LAZ Version 10.1 Point10
    V1Point10(Box<v1::LAZPoint10v1Reader<T>>),
    /// LAZ Version 10.2 Point10
    V2Point10(Box<v2::LAZPoint10v2Reader<T>>),
    /// LAZ Version 11.1 GpsTime
    V1GpsTime11(Box<v1::LAZGpsTime11v1Reader<T>>),
    /// LAZ Version 11.2 GpsTime
    V2GpsTime11(Box<v2::LAZGpsTime11v2Reader<T>>),
    /// LAZ Version 12.1 Rgb
    V1Rgb12(Box<v1::LAZrgb12v1Reader<T>>),
    /// LAZ Version 12.2 Rgb
    V2Rgb12(Box<v2::LAZrgb12v2Reader<T>>),
    /// LAZ Version 13.1 Wavepacket
    V1Wavepacket13(Box<v1::LAZwavepacket13v1Reader<T>>),
    /// LAZ Version 10.1 Byte
    V1Byte10(Box<v1::LAZbyte10v1Reader<T>>),
    /// LAZ Version 10.2 Byte
    V2Byte10(Box<v2::LAZbyte10v2Reader<T>>),
    /// LAZ Version 14.3 Point
    V3Point14(Box<v3::LAZPoint14v3Reader<T>>),
    /// LAZ Version 14.3 Rgb
    V3Rgb14(Box<v3::LAZrgb14v3Reader<T>>),
    /// LAZ Version 14.3 Rgb Nir
    V3RgbNir14(Box<v3::LAZrgbNir14v3Reader<T>>),
    /// LAZ Version 14.3 Wavepacket
    V3Wavepacket14(Box<v3::LAZwavepacket14v3Reader<T>>),
    /// LAZ Version 14.3 Byte
    V3Byte14(Box<v3::LAZbyte14v3Reader<T>>),
}
impl<T: Reader> ItemReader for ItemReaders<T> {
    fn init<R: Reader>(&mut self, item: &R, point: &mut LASPoint, context: &mut u32) {
        match self {
            ItemReaders::V1Point10(v1) => v1.init(item, point, context),
            ItemReaders::V2Point10(v2) => v2.init(item, point, context),
            ItemReaders::V1GpsTime11(v1) => v1.init(item, point, context),
            ItemReaders::V2GpsTime11(v2) => v2.init(item, point, context),
            ItemReaders::V1Rgb12(v1) => v1.init(item, point, context),
            ItemReaders::V2Rgb12(v2) => v2.init(item, point, context),
            ItemReaders::V1Wavepacket13(v1) => v1.init(item, point, context),
            ItemReaders::V1Byte10(v1) => v1.init(item, point, context),
            ItemReaders::V2Byte10(v2) => v2.init(item, point, context),
            ItemReaders::V3Point14(v3) => v3.init(item, point, context),
            ItemReaders::V3Rgb14(v3) => v3.init(item, point, context),
            ItemReaders::V3RgbNir14(v3) => v3.init(item, point, context),
            ItemReaders::V3Wavepacket14(v3) => v3.init(item, point, context),
            ItemReaders::V3Byte14(v3) => v3.init(item, point, context),
        }
    }
    fn read(&mut self, item: &mut LASPoint, context: &mut u32) {
        match self {
            ItemReaders::V1Point10(v1) => v1.read(item, context),
            ItemReaders::V2Point10(v2) => v2.read(item, context),
            ItemReaders::V1GpsTime11(v1) => v1.read(item, context),
            ItemReaders::V2GpsTime11(v2) => v2.read(item, context),
            ItemReaders::V1Rgb12(v1) => v1.read(item, context),
            ItemReaders::V2Rgb12(v2) => v2.read(item, context),
            ItemReaders::V1Wavepacket13(v1) => v1.read(item, context),
            ItemReaders::V1Byte10(v1) => v1.read(item, context),
            ItemReaders::V2Byte10(v2) => v2.read(item, context),
            ItemReaders::V3Point14(v3) => v3.read(item, context),
            ItemReaders::V3Rgb14(v3) => v3.read(item, context),
            ItemReaders::V3RgbNir14(v3) => v3.read(item, context),
            ItemReaders::V3Wavepacket14(v3) => v3.read(item, context),
            ItemReaders::V3Byte14(v3) => v3.read(item, context),
        }
    }
    fn chunk_sizes<R: Reader>(&mut self, reader: &R) {
        match self {
            ItemReaders::V1Point10(v1) => v1.chunk_sizes(reader),
            ItemReaders::V2Point10(v2) => v2.chunk_sizes(reader),
            ItemReaders::V1GpsTime11(v1) => v1.chunk_sizes(reader),
            ItemReaders::V2GpsTime11(v2) => v2.chunk_sizes(reader),
            ItemReaders::V1Rgb12(v1) => v1.chunk_sizes(reader),
            ItemReaders::V2Rgb12(v2) => v2.chunk_sizes(reader),
            ItemReaders::V1Wavepacket13(v1) => v1.chunk_sizes(reader),
            ItemReaders::V1Byte10(v1) => v1.chunk_sizes(reader),
            ItemReaders::V2Byte10(v2) => v2.chunk_sizes(reader),
            ItemReaders::V3Point14(v3) => v3.chunk_sizes(reader),
            ItemReaders::V3Rgb14(v3) => v3.chunk_sizes(reader),
            ItemReaders::V3RgbNir14(v3) => v3.chunk_sizes(reader),
            ItemReaders::V3Wavepacket14(v3) => v3.chunk_sizes(reader),
            ItemReaders::V3Byte14(v3) => v3.chunk_sizes(reader),
        }
    }
}
impl<T: Reader> From<v1::LAZPoint10v1Reader<T>> for ItemReaders<T> {
    fn from(reader: v1::LAZPoint10v1Reader<T>) -> Self {
        ItemReaders::V1Point10(reader.into())
    }
}
impl<T: Reader> From<v2::LAZPoint10v2Reader<T>> for ItemReaders<T> {
    fn from(reader: v2::LAZPoint10v2Reader<T>) -> Self {
        ItemReaders::V2Point10(reader.into())
    }
}
impl<T: Reader> From<v1::LAZGpsTime11v1Reader<T>> for ItemReaders<T> {
    fn from(reader: v1::LAZGpsTime11v1Reader<T>) -> Self {
        ItemReaders::V1GpsTime11(reader.into())
    }
}
impl<T: Reader> From<v2::LAZGpsTime11v2Reader<T>> for ItemReaders<T> {
    fn from(reader: v2::LAZGpsTime11v2Reader<T>) -> Self {
        ItemReaders::V2GpsTime11(reader.into())
    }
}
impl<T: Reader> From<v1::LAZrgb12v1Reader<T>> for ItemReaders<T> {
    fn from(reader: v1::LAZrgb12v1Reader<T>) -> Self {
        ItemReaders::V1Rgb12(reader.into())
    }
}
impl<T: Reader> From<v2::LAZrgb12v2Reader<T>> for ItemReaders<T> {
    fn from(reader: v2::LAZrgb12v2Reader<T>) -> Self {
        ItemReaders::V2Rgb12(reader.into())
    }
}
impl<T: Reader> From<v1::LAZwavepacket13v1Reader<T>> for ItemReaders<T> {
    fn from(reader: v1::LAZwavepacket13v1Reader<T>) -> Self {
        ItemReaders::V1Wavepacket13(reader.into())
    }
}
impl<T: Reader> From<v1::LAZbyte10v1Reader<T>> for ItemReaders<T> {
    fn from(reader: v1::LAZbyte10v1Reader<T>) -> Self {
        ItemReaders::V1Byte10(reader.into())
    }
}
impl<T: Reader> From<v2::LAZbyte10v2Reader<T>> for ItemReaders<T> {
    fn from(reader: v2::LAZbyte10v2Reader<T>) -> Self {
        ItemReaders::V2Byte10(reader.into())
    }
}
impl<T: Reader> From<v3::LAZPoint14v3Reader<T>> for ItemReaders<T> {
    fn from(reader: v3::LAZPoint14v3Reader<T>) -> Self {
        ItemReaders::V3Point14(reader.into())
    }
}
impl<T: Reader> From<v3::LAZrgb14v3Reader<T>> for ItemReaders<T> {
    fn from(reader: v3::LAZrgb14v3Reader<T>) -> Self {
        ItemReaders::V3Rgb14(reader.into())
    }
}
impl<T: Reader> From<v3::LAZrgbNir14v3Reader<T>> for ItemReaders<T> {
    fn from(reader: v3::LAZrgbNir14v3Reader<T>) -> Self {
        ItemReaders::V3RgbNir14(reader.into())
    }
}
impl<T: Reader> From<v3::LAZwavepacket14v3Reader<T>> for ItemReaders<T> {
    fn from(reader: v3::LAZwavepacket14v3Reader<T>) -> Self {
        ItemReaders::V3Wavepacket14(reader.into())
    }
}
impl<T: Reader> From<v3::LAZbyte14v3Reader<T>> for ItemReaders<T> {
    fn from(reader: v3::LAZbyte14v3Reader<T>) -> Self {
        ItemReaders::V3Byte14(reader.into())
    }
}

/// Template for reading data
pub trait ItemReader {
    /// Initialize the reader
    fn init<R: Reader>(&mut self, item: &R, point: &mut LASPoint, context: &mut u32);
    /// Read the next item
    fn read(&mut self, item: &mut LASPoint, context: &mut u32);
    /// Read in chunk sizes
    fn chunk_sizes<R: Reader>(&mut self, reader: &R);
}

/// Compression leaves the raw input for point14 a bit messy. This fixes the problem
pub fn modify_point14_raw_input(reader: &BufferReader) -> BufferReader {
    let mut tmp = LASPoint::default();
    tmp.inject_point14_temp(reader, 0);
    let LASPoint { x, y, z, intensity, flags, scan_direction_flag, edge_of_flight_line, .. } = tmp;
    let mut res = LASPoint {
        x,
        y,
        z,
        intensity,
        flags,
        scan_direction_flag,
        edge_of_flight_line,
        ..Default::default()
    };

    if tmp.number_of_returns > 7 {
        if tmp.return_number > 6 {
            if tmp.return_number >= tmp.number_of_returns {
                res.legacy_return_number = 7;
            } else {
                res.legacy_return_number = 6;
            }
        } else {
            res.legacy_return_number = tmp.return_number;
        }
        res.legacy_number_of_returns = 7;
    } else {
        res.legacy_return_number = tmp.return_number;
        res.legacy_number_of_returns = tmp.number_of_returns;
    }
    // res.legacy_flags = (tmp.classification << 5) & 0xe0;
    if tmp.classification < 32 {
        res.legacy_classification |= tmp.classification;
    }
    res.legacy_scan_angle_rank = i8_clamp(i16_quantize(0.006 * tmp.scan_angle as f64) as i32);
    res.user_data = tmp.user_data;
    res.point_source_id = tmp.point_source_id;
    res.scanner_channel = tmp.scanner_channel;
    res.class_flag = tmp.class_flag;
    res.classification = tmp.classification;
    res.return_number = tmp.return_number;
    res.number_of_returns = tmp.number_of_returns;
    res.scan_angle = tmp.scan_angle;
    res.gps_time = Some(reader.f64_le(Some(22)));

    res.to_buffer_14(true).into()
}
