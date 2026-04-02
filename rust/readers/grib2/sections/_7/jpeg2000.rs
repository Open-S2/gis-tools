use crate::parsers::{BufferReader, Reader};
use crate::readers::{Grib2BitMapSection, Grib2DataRepresentationSection, Grib2Table6_0};
use openjpeg_sys as opj;
use std::ptr::NonNull;

/// # Data Template 7.40 - Grid point data - JPEG 2000 code stream format
///
/// [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp7-40.shtml)
///
/// ## Parameters
/// - `reader`: The raw data to convert
/// - `drs`: The data representation section
/// - `bms`: The bit map section
///
/// ## Returns
/// The converted data
pub fn jpeg2000_unpacking_fs(
    reader: &BufferReader,
    drs: &Grib2DataRepresentationSection,
    bms: &Grib2BitMapSection,
) -> Vec<f64> {
    let data = reader.slice(None, None);

    // 1. Initialize OpenJPEG Stream and Decoder
    let stream = Stream::from_bytes(&data).expect("Failed to create OpenJPEG stream");
    let codec = Codec::j2k().expect("Failed to create J2K codec");

    let mut params = unsafe { std::mem::zeroed::<opj::opj_dparameters>() };
    unsafe { opj::opj_set_default_decoder_parameters(&mut params) };

    unsafe {
        if opj::opj_setup_decoder(codec.0.as_ptr(), &mut params) != 1 {
            panic!("OpenJPEG decoder setup failed");
        }
    }

    // 2. Read Header and Decode Image
    let mut img_ptr: *mut opj::opj_image_t = std::ptr::null_mut();
    unsafe {
        if opj::opj_read_header(stream.0, codec.0.as_ptr(), &mut img_ptr) != 1 {
            panic!("Failed to read JPEG2000 header");
        }
    }
    let image = Image(NonNull::new(img_ptr).expect("Image initialization failed"));

    unsafe {
        if opj::opj_decode(codec.0.as_ptr(), stream.0, image.0.as_ptr()) != 1 {
            panic!("JPEG2000 decoding failed");
        }
    }

    // 3. Extract Grayscale Data (GRIB2 is single-component)
    let comp = unsafe { &*(*image.0.as_ptr()).comps };
    let len = (comp.w * comp.h) as usize;
    let pixels = unsafe { std::slice::from_raw_parts(comp.data, len) };

    let dr = &drs.data_representation;
    let ref_val = dr.reference_value();
    let ee = 2.0_f64.powf(dr.binary_scale_factor());
    let dd = 10.0_f64.powf(dr.decimal_scale_factor());

    let mut result = Vec::with_capacity(drs.number_of_data_points as usize);

    if dr.number_of_bits() == 0 {
        result.resize(drs.number_of_data_points as usize, ref_val);
        return result;
    }

    let mut data_index = 0;

    // 2. Handle the Bitmap
    if bms.bit_map_indicator == Grib2Table6_0::BitmapSpecifiedInThisSection {
        if let Some(bit_buffer) = &bms.bit_map {
            let bit_map_data = bit_buffer.slice(None, None);
            for i in 0..drs.number_of_data_points as usize {
                // GRIB2 Bitmaps are MSB first
                let byte = bit_map_data[i >> 3];
                let bit_is_set = (byte & (0x80 >> (i % 8))) != 0;

                if bit_is_set && data_index < pixels.len() {
                    let raw_value = pixels[data_index] as f64;
                    result.push((ref_val + raw_value * ee) / dd);
                    data_index += 1;
                } else {
                    result.push(f64::NAN);
                }
            }
        }
    } else {
        // 3. No bitmap: 1:1 mapping
        for raw_value in pixels {
            result.push((ref_val + (*raw_value as f64) * ee) / dd);
        }
    }

    // Ensure we meet the expected number of data points
    while result.len() < drs.number_of_data_points as usize {
        result.push(f64::NAN);
    }

    result
}

struct Stream(*mut opj::opj_stream_t);
impl Stream {
    fn from_bytes(buf: &[u8]) -> Option<Self> {
        unsafe {
            let stream = opj::opj_stream_default_create(1); // 1 = Read stream
            if stream.is_null() {
                return None;
            }

            let data = Box::into_raw(Box::new(Slice { offset: 0, buf }));

            opj::opj_stream_set_read_function(stream, Some(read_fn));
            opj::opj_stream_set_skip_function(stream, Some(skip_fn));
            opj::opj_stream_set_seek_function(stream, Some(seek_fn));
            opj::opj_stream_set_user_data_length(stream, buf.len() as u64);
            opj::opj_stream_set_user_data(stream, data as *mut _, Some(free_fn));

            Some(Stream(stream))
        }
    }
}
impl Drop for Stream {
    fn drop(&mut self) {
        unsafe {
            opj::opj_stream_destroy(self.0);
        }
    }
}

struct Codec(NonNull<opj::opj_codec_t>);
impl Codec {
    fn j2k() -> Option<Self> {
        let ptr = unsafe { opj::opj_create_decompress(opj::OPJ_CODEC_FORMAT::OPJ_CODEC_J2K) };
        NonNull::new(ptr).map(Codec)
    }
}
impl Drop for Codec {
    fn drop(&mut self) {
        unsafe {
            opj::opj_destroy_codec(self.0.as_ptr());
        }
    }
}

struct Image(NonNull<opj::opj_image_t>);
impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            opj::opj_image_destroy(self.0.as_ptr());
        }
    }
}

struct Slice<'a> {
    offset: usize,
    buf: &'a [u8],
}

extern "C" fn read_fn(
    p_buf: *mut std::ffi::c_void,
    nb: usize,
    p_data: *mut std::ffi::c_void,
) -> usize {
    let slice = unsafe { &mut *(p_data as *mut Slice) };
    let n = std::cmp::min(nb, slice.buf.len() - slice.offset);
    if n == 0 {
        return usize::MAX;
    }
    unsafe {
        std::ptr::copy_nonoverlapping(slice.buf.as_ptr().add(slice.offset), p_buf as *mut u8, n);
    }
    slice.offset += n;
    n
}

extern "C" fn skip_fn(nb: i64, p_data: *mut std::ffi::c_void) -> i64 {
    let slice = unsafe { &mut *(p_data as *mut Slice) };
    slice.offset = std::cmp::min(slice.buf.len(), slice.offset + nb as usize);
    slice.offset as i64
}

extern "C" fn seek_fn(nb: i64, p_data: *mut std::ffi::c_void) -> i32 {
    let slice = unsafe { &mut *(p_data as *mut Slice) };
    slice.offset = std::cmp::min(slice.buf.len(), nb as usize);
    1 // Success
}

extern "C" fn free_fn(p_data: *mut std::ffi::c_void) {
    unsafe {
        drop(Box::from_raw(p_data as *mut Slice));
    }
}
