use super::{
    Raster,
    constants::{ExtraSamplesValues, PhotometricInterpretations},
};
use alloc::{vec, vec::Vec};
use libm::{fmax, fmin, pow};

/// Converts photometric interpretation to samples
///
/// @param pi - photometric interpretation
/// @param bitsPerSample - bits per sample
/// @param extraSamples - extra samples
/// @returns - sample output
pub fn build_samples(
    pi: PhotometricInterpretations,
    bits_per_sample: Option<Vec<u16>>,
    extra_samples: Option<ExtraSamplesValues>,
) -> Vec<u16> {
    let extra_samples = extra_samples.unwrap_or(ExtraSamplesValues::Unspecified);
    let bits_per_sample = bits_per_sample.unwrap_or(vec![0]);
    let mut samples;
    if pi == PhotometricInterpretations::RGB {
        samples = vec![0, 1, 2, 3];
        // support alpha if it exists
        if extra_samples != ExtraSamplesValues::Unspecified {
            samples = vec![];
            for i in 0..bits_per_sample.len() {
                samples.push(i as u16);
            }
        }
    } else {
        match pi {
            PhotometricInterpretations::WhiteIsZero
            | PhotometricInterpretations::BlackIsZero
            | PhotometricInterpretations::Palette => {
                samples = vec![0];
            }
            PhotometricInterpretations::CMYK => {
                samples = vec![0, 1, 2, 3];
            }
            PhotometricInterpretations::YCbCr
            | PhotometricInterpretations::CIELab
            | PhotometricInterpretations::ICCLab => {
                samples = vec![0, 1, 2];
            }
            _ => panic!("Invalid or unsupported photometric interpretation."),
        }
    }

    samples
}

/// Convert color space raster to RGB
/// TODO: ICCLAB, ITULAB
///
/// @param pi - photometric interpretation
/// @param raster_data - raster data
/// @param max - maximum value if needed
/// @param color_map - color map if needed
pub fn convert_color_space(
    pi: PhotometricInterpretations,
    raster: &mut Raster,
    max: f64,
    color_map: Option<Vec<u16>>,
) {
    if pi == PhotometricInterpretations::RGB {
    } else if pi == PhotometricInterpretations::WhiteIsZero {
        from_white_is_zero(raster, max);
    } else if pi == PhotometricInterpretations::BlackIsZero {
        from_black_is_zero(raster, max);
    } else if pi == PhotometricInterpretations::Palette {
        from_palette(raster, color_map);
    } else if pi == PhotometricInterpretations::CMYK {
        from_cmyk(raster);
    } else if pi == PhotometricInterpretations::YCbCr {
        from_ycb_cr(raster);
    } else if pi == PhotometricInterpretations::CIELab {
        from_cei_lab(raster);
    } else {
        panic!("Unsupported photometric interpretation {:?}.", pi);
    }
}

/// Converts raster with white is zero and max is one to RGB
/// @param raster - raster
/// @param max - maximum value
pub fn from_white_is_zero(raster: &mut Raster, max: f64) {
    let mut rbgdata = vec![0_f64; raster.width * raster.height * 3];
    let data = &raster.data;
    let mut i = 0;
    let mut j = 0;
    while i < data.len() {
        let value = 256. - (data[i] / max) * 256.;
        rbgdata[j] = value;
        rbgdata[j + 1] = value;
        rbgdata[j + 2] = value;

        i += 1;
        j += 3;
    }
    raster.data = rbgdata;
}

/// Converts raster with black is zero and max is one to RGB
///
/// @param raster - raster
/// @param max - maximum value
pub fn from_black_is_zero(raster: &mut Raster, max: f64) {
    let mut rbgdata = vec![0_f64; raster.width * raster.height * 3];
    let data = &raster.data;
    let mut i = 0;
    let mut j = 0;
    while i < data.len() {
        let value = (data[i] / max) * 256.;
        rbgdata[j] = value;
        rbgdata[j + 1] = value;
        rbgdata[j + 2] = value;

        i += 1;
        j += 3;
    }
    raster.data = rbgdata;
}

/// Converts raster with a color map to RGB
///
/// @param raster - raster
/// @param color_map - color map
pub fn from_palette(raster: &mut Raster, color_map: Option<Vec<u16>>) {
    let color_map = color_map.unwrap_or_default();
    let mut rbgdata = vec![0_f64; raster.width * raster.height * 3];
    let data = &raster.data;
    let green_offset = color_map.len() / 3;
    let blue_offset = (color_map.len() / 3) * 2;
    let mut i = 0;
    let mut j = 0;
    while i < data.len() {
        let map_index = data[i] as usize;
        rbgdata[j] = (color_map[map_index] as f64 / 65_536.) * 256.;
        rbgdata[j + 1] = (color_map[map_index + green_offset] as f64 / 65_536.) * 256.;
        rbgdata[j + 2] = (color_map[map_index + blue_offset] as f64 / 65_536.) * 256.;

        i += 1;
        j += 3;
    }
    raster.data = rbgdata;
}

/// Converts CMYK to RGB
///
/// @param raster - CMYK raster
pub fn from_cmyk(raster: &mut Raster) {
    let mut rbgdata = vec![0_f64; raster.width * raster.height * 3];
    let data = &raster.data;
    let mut i = 0;
    let mut j = 0;
    while i < data.len() {
        let c = data[i] / 255.0;
        let m = data[i + 1] / 255.0;
        let y = data[i + 2] / 255.0;
        let k = data[i + 3] / 255.0;

        rbgdata[j] = 255.0 * (1.0 - c) * (1.0 - k);
        rbgdata[j + 1] = 255.0 * (1.0 - m) * (1.0 - k);
        rbgdata[j + 2] = 255.0 * (1.0 - y) * (1.0 - k);

        i += 4;
        j += 3;
    }
    raster.data = rbgdata;
}

/// Converts YCbCr to RGB
///
/// @param raster - YCbCr raster
pub fn from_ycb_cr(raster: &mut Raster) {
    let mut rbgdata = vec![0_f64; raster.width * raster.height * 3];
    let data = &raster.data;
    let mut i = 0;
    let mut j = 0;
    while i < data.len() {
        let y = data[i];
        let cb = data[i + 1] - 0x80 as f64;
        let cr = data[i + 2] - 0x80 as f64;

        rbgdata[j] = y + 1.402 * cr;
        rbgdata[j + 1] = y - 0.34414 * cb - 0.71414 * cr;
        rbgdata[j + 2] = y + 1.772 * cb;
        i += 3;
        j += 3;
    }
    raster.data = rbgdata;
}

const XN: f64 = 0.95047;
const YN: f64 = 1.0;
const ZN: f64 = 1.08883;

/// Converts CIELab to RGB
/// https://github.com/antimatter15/rgb-lab/blob/master/color.js
///
/// @param raster - CIELab raster
pub fn from_cei_lab(raster: &mut Raster) {
    let mut rbgdata = vec![0_f64; raster.width * raster.height * 3];
    let data = &raster.data;

    let mut l: f64;
    let mut a_: f64;
    let mut b_: f64;
    let mut x: f64;
    let mut y: f64;
    let mut z: f64;
    let mut r: f64;
    let mut g: f64;
    let mut b: f64;
    let mut i = 0;
    let mut j = 0;
    while i < data.len() {
        l = data[i];
        a_ = (data[i + 1] as i8) as f64; // conversion from uint8 to int8
        b_ = (data[i + 2] as i8) as f64; // same
        y = (l + 16.) / 116.;
        x = a_ / 500. + y;
        z = y - b_ / 200.;

        x = XN * if x * x * x > 0.008856 { x * x * x } else { (x - 16. / 116.) / 7.787 };
        y = YN * if y * y * y > 0.008856 { y * y * y } else { (y - 16. / 116.) / 7.787 };
        z = ZN * if z * z * z > 0.008856 { z * z * z } else { (z - 16. / 116.) / 7.787 };

        r = x * 3.2406 + y * -1.5372 + z * -0.4986;
        g = x * -0.9689 + y * 1.8758 + z * 0.0415;
        b = x * 0.0557 + y * -0.204 + z * 1.057;

        r = if r > 0.0031308 { 1.055 * pow(r, (1. / 2.4) - 0.055) } else { 12.92 * r };
        g = if g > 0.0031308 { 1.055 * pow(g, (1. / 2.4) - 0.055) } else { 12.92 * g };
        b = if b > 0.0031308 { 1.055 * pow(b, (1. / 2.4) - 0.055) } else { 12.92 * b };

        rbgdata[j] = fmax(0., fmin(1., r)) * 255.;
        rbgdata[j + 1] = fmax(0., fmin(1., g)) * 255.;
        rbgdata[j + 2] = fmax(0., fmin(1., b)) * 255.;
        i += 3;
        j += 3;
    }
    raster.data = rbgdata;
}
