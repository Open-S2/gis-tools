use super::ImageData;
use crate::parsers::Buffer;
use alloc::{vec, vec::Vec};
use core::f64::consts::PI;
use libm::{floor, round, sin};

const FIXED_FRAC_BITS: f64 = 14.0;

/**
 * Filter input value given a filter window.
 * @param x - input
 * @param a - filter window
 * @returns - filtered value
 */
fn filter_value(x: f64, a: f64) -> f64 {
    if x <= -a || x >= a {
        return 0.;
    }
    if x == 0. {
        return 0.;
    }
    if x > -1.19209290e-07 && x < 1.19209290e-07 {
        return 1.;
    }
    let x_pi = x * PI;

    ((sin(x_pi) / x_pi) * sin(x_pi / a)) / (x_pi / a)
}

// /**
//  * Convert value to fixed point
//  * @param value - input
//  * @returns - fixed point
//  */
// fn to_fixed_point(value: f64) -> u64 {
//     round(value as f64 * ((1 << FIXED_FRAC_BITS) - 1)) as u64
// }

/**
 * Create a Lanczos filter
 * @param src_size - source image size
 * @param dest_size - destination image size
 * @param scale - scale factor
 * @param offset - offset to apply
 * @param use2 - use 2nd lanczos filter instead of 3rd
 * @returns - filter
 */
fn filters(src_size: usize, dest_size: usize, scale: usize, offset: usize, use2: bool) -> Vec<i16> {
    todo!()
    //   let a = if use2 { 2 } else { 3 };
    //   let scale_inverted = 1 / scale;
    //   let scale_clamped = min(1, scale) as f64; // For upscale

    //   // Filter window (averaging interval), scaled to src image
    //   let src_window = a / scale_clamped;

    //   let maxFilterElementSize = floor((src_window + 1) * 2);
    // let mut packed_filter = vec![0; (maxFilterElementSize + 2) * dest_size];

    //   let packed_filter_ptr = 0;

    //   // For each destination pixel calculate source range and built filter values
    //   for (let destPixel = 0; destPixel < dest_size; destPixel++) {
    //     // Scaling should be done relative to central pixel point
    //     let sourcePixel = (destPixel + 0.5) * scale_inverted + offset;
    //     let sourceFirst = max(0, floor(sourcePixel - src_window));
    //     let sourceLast = min(src_size - 1, ceil(sourcePixel + src_window));

    //     let filterElementSize = sourceLast - sourceFirst + 1;
    //     let floatFilter = new Float32Array(filterElementSize);
    //     let fxp_filter = new Int16Array(filterElementSize);

    //     let total = 0;

    //     // Fill filter values for calculated range
    //     let index = 0;
    //     for (let pixel = sourceFirst; pixel <= sourceLast; pixel++) {
    //       let floatValue = filter_value((pixel + 0.5 - sourcePixel) * scale_clamped, a);

    //       total += floatValue;
    //       floatFilter[index] = floatValue;

    //       index++;
    //     }

    //     // Normalize filter, convert to fixed point and accumulate conversion error
    //     let filterTotal = 0;

    //     for (let index = 0; index < floatFilter.length; index++) {
    //       let filter_value = floatFilter[index] / total;

    //       filterTotal += filter_value;
    //       fxp_filter[index] = to_fixed_point(filter_value);
    //     }

    //     // Compensate normalization error, to minimize brightness drift
    //     fxp_filter[dest_size >> 1] += to_fixed_point(1 - filterTotal);

    //     //
    //     // Now pack filter to useable form
    //     //
    //     // 1. Trim heading and tailing zero values, and compensate shitf/length
    //     // 2. Put all to single array in this format:
    //     //
    //     //    [ pos shift, data length, value1, value2, value3, ... ]
    //     //
    //     let left_not_empty = 0;
    //     while (left_not_empty < fxp_filter.len() && fxp_filter[left_not_empty] === 0) {
    //       left_not_empty++;
    //     }

    //     let right_not_empty = fxp_filter.len() - 1;
    //     while (right_not_empty > 0 && fxp_filter[right_not_empty] === 0) {
    //       right_not_empty--;
    //     }

    //     let filter_shift = sourceFirst + left_not_empty;
    //     let filter_size = right_not_empty - left_not_empty + 1;

    //     packed_filter[packed_filter_ptr++] = filter_shift; // shift
    //     packed_filter[packed_filter_ptr++] = filter_size; // size

    //     packed_filter.set(fxp_filter.subarray(left_not_empty, right_not_empty + 1), packed_filter_ptr);
    //     packed_filter_ptr += filter_size;
    //   }

    //   packed_filter
}

/**
 * Copy the contents of the source image to the destination image
 * @param source - the source image
 * @param dest - the destination image
 * @param sx - source starting x point [Default: 0]
 * @param sy - source starting y point [Default: 0]
 * @param sw - source width to use [Default: source width - sx]
 * @param sh - source height to use [Default: source height - sy]
 * @param dx - destination starting x point [Default: 0]
 * @param dy - destination starting y point [Default: 0]
 */
#[allow(clippy::too_many_arguments)]
pub fn copy_image(
    source: &mut ImageData,
    dest: &mut ImageData,
    sx: Option<isize>,
    sy: Option<isize>,
    sw: Option<isize>,
    sh: Option<isize>,
    dx: Option<isize>,
    dy: Option<isize>,
) {
    let sx = sx.unwrap_or(0);
    let sy = sy.unwrap_or(0);
    let sw = sw.unwrap_or(source.width as isize - sx);
    let sh = sh.unwrap_or(source.height as isize - sy);
    let dx = dx.unwrap_or(0);
    let dy = dy.unwrap_or(0);

    if sw <= 0 || sh <= 0 {
        return;
    }

    let source_data = &mut source.data;
    let dest_data = &mut dest.data;

    for y in 0..sh {
        let source_y = sy + y;
        if source_y < 0 || source_y >= source.height.try_into().unwrap() {
            continue;
        }

        let dest_y = dy + y;
        if dest_y < 0 || dest_y >= dest.height.try_into().unwrap() {
            continue;
        }

        for x in 0..sw {
            let source_x = sx + x;
            if source_x < 0 || source_x >= source.width.try_into().unwrap() {
                continue;
            }

            let dest_x = dx + x;
            if dest_x < 0 || dest_x >= dest.width.try_into().unwrap() {
                continue;
            }

            let source_index = (source_y * source.width as isize + source_x) as usize;
            let dest_index = (dest_y * dest.width as isize + dest_x) as usize;

            dest_data.set_u16_at(dest_index * 2, source_data.get_u16_at(source_index));
        }
    }
}

/**
 * Create an image given the size, fill color and number of channels
 * @param width - the image width
 * @param height - the image height
 * @param data - the image data [Default: creates a new array]
 * @param fill - the fill color [Default: [0, 0, 0, 0]]
 * @param channels - the number of channels [Default: 4]
 * @returns - the created image
 */
pub fn create_image(
    width: usize,
    height: usize,
    data: Option<Buffer>,
    fill: Option<[u8; 4]>,
    channels: Option<usize>,
) -> ImageData {
    // SETUP
    let channels = channels.unwrap_or(4);
    if width < 1 || height < 1 {
        panic!("Index or size is negative or greater than the allowed amount");
    }
    let length = width * height * channels;
    let mut data = data.unwrap_or(Buffer::new(vec![0; length]));
    if data.len() != length {
        panic!("Index or size is negative or greater than the allowed amount");
    }
    let fill = fill.unwrap_or([0, 0, 0, 0]);

    for y in 0..height {
        for x in 0..width {
            let index = (y * width + x) * channels;
            for (c, val) in fill.iter().enumerate().take(channels) {
                data.set_u8_at(index + c, *val)
            }
        }
    }

    ImageData { data, width, height }
}

/**
 * Lanczos resize fn
 * @param source - the source image
 * @param dest - the destination image
 * @param use2 - use 2nd lanczos filter instead of 3rd
 */
pub fn resize_image(source: &mut ImageData, dest: &mut ImageData, use2: Option<bool>) {
    let use2 = use2.unwrap_or(false);
    let x_ratio = dest.width / source.width;
    let y_ratio = dest.height / source.height;

    let filters_x = filters(source.width, dest.width, x_ratio, 0, use2);
    let filters_y = filters(source.height, dest.height, y_ratio, 0, use2);

    //   let tmp = new Uint8ClampedArray(dest.width * source.height * 4);
    let mut tmp = Buffer::new(vec![0; dest.width * source.height * 4]);

    convolve_image(&source.data, &mut tmp, source.width, source.height, dest.width, &filters_x);
    convolve_image(&tmp, &mut dest.data, source.height, dest.width, dest.height, &filters_y);
}

/**
 * Convolve an image with a filter
 * @param source - the source image
 * @param dest - the destination image
 * @param sw - source width
 * @param sh - source height
 * @param dw - destination width
 * @param filters - image filter
 */
pub fn convolve_image(
    source: &Buffer,
    dest: &mut Buffer,
    sw: usize,
    sh: usize,
    dw: usize,
    filters: &[i16], // Int16Array,
) {
    todo!()
    //   let src_offset = 0;
    //   let dest_offset = 0;

    //   // For each row
    //   for (let source_y = 0; source_y < sh; source_y++) {
    //     let filterPtr = 0;
    //     // Apply precomputed filters to each destination row point
    //     for (let dest_x = 0; dest_x < dw; dest_x++) {
    //       // Get the filter that determines the current output pixel.
    //       let filter_shift = filters[filterPtr++];

    //       let srcPtr = (src_offset + filter_shift * 4) | 0;

    //       let r = 0;
    //       let g = 0;
    //       let b = 0;
    //       let a = 0;

    //       // Apply the filter to the row to get the destination pixel r, g, b, a
    //       for (let filter_size = filters[filterPtr++]; filter_size > 0; filter_size--) {
    //         let filter_value = filters[filterPtr++];

    //         r = (r + filter_value * source[srcPtr]) | 0;
    //         g = (g + filter_value * source[srcPtr + 1]) | 0;
    //         b = (b + filter_value * source[srcPtr + 2]) | 0;
    //         a = (a + filter_value * source[srcPtr + 3]) | 0;

    //         srcPtr = (srcPtr + 4) | 0;
    //       }

    //       // Bring this value back in range. All of the filter scaling factors
    //       // are in fixed point with FIXED_FRAC_BITS bits of fractional part.
    //       //
    //       // (!) Add 1/2 of value before clamping to get proper rounding. In other
    //       // case brightness loss will be noticeable if you resize image with white
    //       // border and place it on white background.
    //       //
    //       dest[dest_offset] = (r + (1 << 13)) >> FIXED_FRAC_BITS;
    //       dest[dest_offset + 1] = (g + (1 << 13)) >> FIXED_FRAC_BITS;
    //       dest[dest_offset + 2] = (b + (1 << 13)) >> FIXED_FRAC_BITS;
    //       dest[dest_offset + 3] = (a + (1 << 13)) >> FIXED_FRAC_BITS;

    //       dest_offset = (dest_offset + sh * 4) | 0;
    //     }

    //     dest_offset = ((source_y + 1) * 4) | 0;
    //     src_offset = ((source_y + 1) * sw * 4) | 0;
    //   }
}
