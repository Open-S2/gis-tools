use crate::parsers::Buffer;
use alloc::{vec, vec::Vec};
use core::iter::Sum;
use half::f16;

/// An array type
#[derive(Debug, Default, Clone, PartialEq)]
pub enum GTiffDataType {
    /// Unsigned 8-bit
    #[default]
    U8,
    /// Unsigned 16-bit
    U16,
    /// Unsigned 32-bit
    U32,
    /// Signed 8-bit
    I8,
    /// Signed 16-bit
    I16,
    /// Signed 32-bit
    I32,
    /// 16-bit float
    F16,
    /// 32-bit float
    F32,
    /// 64-bit float
    F64,
}
impl GTiffDataType {
    /// Convert the data format and bits per sample to an array type
    ///
    /// @param format - the data format
    /// @param bits_per_sample - the bits per sample
    /// @returns the array type constructor
    pub fn to_type(format: u16, bits_per_sample: u16) -> GTiffDataType {
        match format {
            1 => {
                // unsigned integer data
                if bits_per_sample <= 8 {
                    return GTiffDataType::U8;
                } else if bits_per_sample <= 16 {
                    return GTiffDataType::U16;
                } else if bits_per_sample <= 32 {
                    return GTiffDataType::U32;
                }
            }
            2 => {
                // twos complement signed integer data
                if bits_per_sample == 8 {
                    return GTiffDataType::I8;
                } else if bits_per_sample == 16 {
                    return GTiffDataType::I16;
                } else if bits_per_sample == 32 {
                    return GTiffDataType::I32;
                }
            }
            3 => {
                // floating point data
                match bits_per_sample {
                    16 => {
                        return GTiffDataType::F16;
                    }
                    32 => {
                        return GTiffDataType::F32;
                    }
                    64 => {
                        return GTiffDataType::F64;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        panic!("Unsupported data format/bits_per_sample");
    }
}

/// An array type
#[derive(Debug, Clone, PartialEq)]
pub enum RasterData {
    /// Unsigned 8-bit
    U8(Vec<u8>),
    /// Unsigned 16-bit
    U16(Vec<u16>),
    /// Unsigned 32-bit
    U32(Vec<u32>),
    /// Signed 8-bit
    I8(Vec<i8>),
    /// Signed 16-bit
    I16(Vec<i16>),
    /// Signed 32-bit
    I32(Vec<i32>),
    /// 16-bit float
    F16(Vec<f16>),
    /// 32-bit float
    F32(Vec<f32>),
    /// 64-bit float
    F64(Vec<f64>),
}
impl Default for RasterData {
    fn default() -> Self {
        Self::U8(vec![])
    }
}
impl From<Vec<u8>> for RasterData {
    fn from(data: Vec<u8>) -> Self {
        Self::U8(data)
    }
}
impl RasterData {
    /// Convert the data format and bits per sample to an array type
    ///
    /// @param format - the data format
    /// @param bits_per_sample - the bits per sample
    /// @returns the array type constructor
    pub fn to_type(data: &[u8], format: u16, bits_per_sample: u16) -> RasterData {
        match format {
            1 => {
                // unsigned integer data
                if bits_per_sample <= 8 {
                    return RasterData::U8(data.to_vec());
                } else if bits_per_sample <= 16 {
                    let data: Vec<u16> = data
                        .chunks(2)
                        .map(|chunk| u16::from_le_bytes(chunk.try_into().unwrap()))
                        .collect();
                    return RasterData::U16(data);
                } else if bits_per_sample <= 32 {
                    let data: Vec<u32> = data
                        .chunks(4)
                        .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap()))
                        .collect();
                    return RasterData::U32(data);
                }
            }
            2 => {
                // twos complement signed integer data
                if bits_per_sample == 8 {
                    let data: Vec<i8> = data.iter().map(|&x| x as i8).collect();
                    return RasterData::I8(data);
                } else if bits_per_sample == 16 {
                    let data: Vec<i16> = data
                        .chunks(2)
                        .map(|chunk| i16::from_le_bytes(chunk.try_into().unwrap()))
                        .collect();
                    return RasterData::I16(data);
                } else if bits_per_sample == 32 {
                    let data: Vec<i32> = data
                        .chunks(4)
                        .map(|chunk| i32::from_le_bytes(chunk.try_into().unwrap()))
                        .collect();
                    return RasterData::I32(data);
                }
            }
            3 => {
                // floating point data
                match bits_per_sample {
                    16 => {
                        let data: Vec<f16> = data
                            .chunks(2)
                            .map(|chunk| f16::from_le_bytes(chunk.try_into().unwrap()))
                            .collect();
                        return RasterData::F16(data);
                    }
                    32 => {
                        let data: Vec<f32> = data
                            .chunks(4)
                            .map(|chunk| f32::from_le_bytes(chunk.try_into().unwrap()))
                            .collect();
                        return RasterData::F32(data);
                    }
                    64 => {
                        let data: Vec<f64> = data
                            .chunks(8)
                            .map(|chunk| f64::from_le_bytes(chunk.try_into().unwrap()))
                            .collect();
                        return RasterData::F64(data);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        panic!("Unsupported data format/bits_per_sample");
    }

    /// Create an array of the right type and size
    ///
    /// @param format - the data format
    /// @param bits_per_sample - the bits per sample
    /// @param size - the size
    /// @returns - the array
    pub fn from_size(format: u16, bits_per_sample: u16, size: usize) -> RasterData {
        let mut raster_data = RasterData::to_type(&[0u8; 0], format, bits_per_sample);
        raster_data.resize(size);
        raster_data
    }

    /// Resize the array
    pub fn resize(&mut self, size: usize) {
        match self {
            RasterData::U8(data) => data.resize(size, 0),
            RasterData::U16(data) => data.resize(size, 0),
            RasterData::U32(data) => data.resize(size, 0),
            RasterData::I8(data) => data.resize(size, 0),
            RasterData::I16(data) => data.resize(size, 0),
            RasterData::I32(data) => data.resize(size, 0),
            RasterData::F16(data) => data.resize(size, f16::from_f32(0.0)),
            RasterData::F32(data) => data.resize(size, 0.0),
            RasterData::F64(data) => data.resize(size, 0.0),
        }
    }
}

/**
 * @param array - An array of numbers
 * @param start - Start index
 * @param end - End index
 * @returns The sum
 */
pub fn sample_sum<T>(array: &[T], start: usize, end: usize) -> T
where
    T: Copy + Sum<T>,
{
    array[start..end].iter().copied().sum()
}

/**
 * Check if the data needs normalization
 * @param format - the data format
 * @param bits_per_sample - the bits per sample
 * @returns - true if the data needs normalization
 */
pub fn needs_normalization(format: usize, bits_per_sample: usize) -> bool {
    if (format == 1 || format == 2) && bits_per_sample <= 32 && bits_per_sample % 8 == 0 {
        false
    } else {
        !(format == 3 && (bits_per_sample == 16 || bits_per_sample == 32 || bits_per_sample == 64))
    }
}

/**
 * Normalize the array
 *
 * @param in_buffer - the input buffer
 * @param format - the data format
 * @param planar_configuration - the planar configuration
 * @param samples_per_pixel - the number of samples per pixel
 * @param bits_per_sample - the bits per sample
 * @param tile_width - the tile width
 * @param tile_height - the tile height
 * @returns - the normalized array
 */
pub fn normalize_array(
    in_buffer: Vec<u8>,
    format: usize,
    planar_configuration: usize,
    samples_per_pixel: usize,
    bits_per_sample: usize,
    tile_width: usize,
    tile_height: usize,
) -> Vec<u8> {
    // const inByteArray = new Uint8Array(in_buffer);
    let mut view = Buffer::new(in_buffer);
    let out_size = if planar_configuration == 2 {
        tile_height * tile_width
    } else {
        tile_height * tile_width * samples_per_pixel
    };
    let samples_to_transfer = if planar_configuration == 2 { 1 } else { samples_per_pixel };
    // let out_array = RasterData::from_size(format, bits_per_sample, out_size); // (just create f64 of out_size)
    let mut out_array: Vec<u8> = vec![0; out_size];
    // let mut pixel = 0;

    let bit_mask = (1_usize << bits_per_sample) - 1;

    if format == 1 {
        // unsigned integer
        // translation of https://github.com/OSGeo/gdal/blob/master/gdal/frmts/gtiff/geotiff.cpp#L7337
        let pixel_bit_skip = if planar_configuration == 1 {
            samples_per_pixel * bits_per_sample
        } else {
            bits_per_sample
        };

        // Bits per line rounds up to next byte boundary.
        let mut bits_per_line = tile_width * pixel_bit_skip;
        if (bits_per_line & 7) != 0 {
            bits_per_line = (bits_per_line + 7) & !7;
        }

        for y in 0..tile_height {
            let line_bit_offset = y * bits_per_line;
            for x in 0..tile_width {
                let pixel_bit_offset = line_bit_offset + x * samples_to_transfer * bits_per_sample;
                for i in 0..samples_to_transfer {
                    let bit_offset = pixel_bit_offset + i * bits_per_sample;
                    let out_index = (y * tile_width + x) * samples_to_transfer + i;

                    // let byte_offset = Math.floor(bit_offset / 8);
                    let byte_offset = bit_offset / 8;
                    let inner_bit_offset = bit_offset % 8;
                    if inner_bit_offset + bits_per_sample <= 8 {
                        out_array[out_index] = ((view.get_u8_at(byte_offset) as usize
                            >> (8 - bits_per_sample - inner_bit_offset))
                            & bit_mask) as u8;
                    } else if inner_bit_offset + bits_per_sample <= 16 {
                        let val = ((view.get_u16_at(byte_offset) as usize
                            >> (16 - bits_per_sample - inner_bit_offset))
                            & bit_mask) as u16;
                        // add the two bytes
                        out_array[out_index * 2] = (val >> 8) as u8;
                        out_array[out_index * 2 + 1] = val as u8;
                    } else if inner_bit_offset + bits_per_sample <= 24 {
                        let raw = ((view.get_u16_at(byte_offset) as usize) << 8)
                            | view.get_u8_at(byte_offset + 2) as usize;
                        let val =
                            ((raw >> (24 - bits_per_sample - inner_bit_offset)) & bit_mask) as u32;
                        out_array[out_index * 3] = (val >> 16) as u8;
                        out_array[out_index * 3 + 1] = (val >> 8) as u8;
                        out_array[out_index * 3 + 2] = val as u8;
                    } else {
                        let val = ((view.get_u32_at(byte_offset) as usize
                            >> (32 - bits_per_sample - inner_bit_offset))
                            & bit_mask) as u32;
                        out_array[out_index * 4] = (val >> 24) as u8;
                        out_array[out_index * 4 + 1] = (val >> 16) as u8;
                        out_array[out_index * 4 + 2] = (val >> 8) as u8;
                        out_array[out_index * 4 + 3] = val as u8;
                    }

                    // let outWord = 0;
                    // for (let bit = 0; bit < bits_per_sample; ++bit) {
                    //   if (inByteArray[bit_offset >> 3]
                    //     & (0x80 >> (bit_offset & 7))) {
                    //     outWord |= (1 << (bits_per_sample - 1 - bit));
                    //   }
                    //   ++bit_offset;
                    // }

                    // out_array[out_index] = outWord;
                    // out_array[pixel] = outWord;
                    // pixel += 1;
                }
                // bit_offset = bit_offset + pixel_bit_skip - bits_per_sample;
            }
        }
    } else if format == 3 {
        // floating point - Float16 is handled elsewhere
        // normalize 16/24 bit floats to 32 bit floats in the array
        // if bits_per_sample == 16 {
        //     let mut byte = 0;
        //     let mut out_index = 0;
        //     while byte < in_buffer.len() {
        //         out_array[out_index] = view.get_f16(byte) as f64;
        //         byte += 2;
        //         out_index += 1;
        //     }
        // }
    }

    out_array
}

/// Returns the reader for a sample
///
/// @param sample_index - the index of the sample
/// @returns - a function to read each sample value
pub fn get_reader_for_sample(
    bits_per_sample: u16,
    format: u16,
) -> fn(buffer: &[u8], offset: usize, little_endian: bool) -> f64 {
    match format {
        1 => {
            // unsigned integer data
            if bits_per_sample <= 8 {
                return |buffer: &[u8], offset: usize, _little_endian: bool| -> f64 {
                    let value = buffer[offset];
                    value as f64
                };
            } else if bits_per_sample <= 16 {
                return |buffer: &[u8], offset: usize, little_endian: bool| -> f64 {
                    let u16_offset = offset / 2;
                    let bytes = [buffer[u16_offset], buffer[u16_offset + 1]];
                    let value = if little_endian {
                        u16::from_le_bytes(bytes)
                    } else {
                        u16::from_be_bytes(bytes)
                    };
                    value as f64
                };
            } else if bits_per_sample <= 32 {
                return |buffer: &[u8], offset: usize, little_endian: bool| -> f64 {
                    let start = offset / 4;
                    let bytes: [u8; 4] = buffer[start..start + 4].try_into().unwrap();
                    let value = if little_endian {
                        u32::from_le_bytes(bytes)
                    } else {
                        u32::from_be_bytes(bytes)
                    };
                    value as f64
                };
            }
        }
        2 => {
            // twos complement signed integer data
            if bits_per_sample <= 8 {
                return |buffer: &[u8], offset: usize, _little_endian: bool| -> f64 {
                    (buffer[offset] as i8) as f64
                };
            } else if bits_per_sample <= 16 {
                return |buffer: &[u8], offset: usize, little_endian: bool| -> f64 {
                    let i16_offset = offset / 2;
                    let bytes = [buffer[i16_offset], buffer[i16_offset + 1]];
                    (if little_endian {
                        i16::from_le_bytes(bytes)
                    } else {
                        i16::from_be_bytes(bytes)
                    }) as f64
                };
            } else if bits_per_sample <= 32 {
                return |buffer: &[u8], offset: usize, little_endian: bool| -> f64 {
                    let start = offset / 4;
                    let bytes: [u8; 4] = buffer[start..start + 4].try_into().unwrap();
                    (if little_endian {
                        i32::from_le_bytes(bytes)
                    } else {
                        i32::from_be_bytes(bytes)
                    }) as f64
                };
            }
        }
        3 => {
            if bits_per_sample <= 16 {
                return |buffer: &[u8], offset: usize, little_endian: bool| -> f64 {
                    let f16_offset = offset / 2;
                    let bytes = [buffer[f16_offset], buffer[f16_offset + 1]];
                    let value = if little_endian {
                        f16::from_le_bytes(bytes)
                    } else {
                        f16::from_be_bytes(bytes)
                    };
                    value.to_f64()
                };
            } else if bits_per_sample <= 32 {
                return |buffer: &[u8], offset: usize, little_endian: bool| -> f64 {
                    let start = offset / 4;
                    let bytes: [u8; 4] = buffer[start..start + 4].try_into().unwrap();
                    (if little_endian {
                        f32::from_le_bytes(bytes)
                    } else {
                        f32::from_be_bytes(bytes)
                    }) as f64
                };
            } else if bits_per_sample <= 64 {
                return |buffer: &[u8], offset: usize, little_endian: bool| -> f64 {
                    let start = offset / 8;
                    let bytes: [u8; 8] = buffer[start..start + 8].try_into().unwrap();
                    if little_endian {
                        f64::from_le_bytes(bytes)
                    } else {
                        f64::from_be_bytes(bytes)
                    }
                };
            }
        }
        _ => {}
    }
    panic!("Unsupported data format/bits_per_sample");
}
