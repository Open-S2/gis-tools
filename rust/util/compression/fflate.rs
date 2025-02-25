// Ported from https://github.com/101arrowz/fflate

// DEFLATE is a complex format; to read this code, you should probably check the RFC first:
// https://tools.ietf.org/html/rfc1951
// You may also wish to take a look at the guide I made about this program:
// https://gist.github.com/101arrowz/253f31eb5abc3d9275ab943003ffecad

// Some of the following code is similar to that of UZIP.js:
// https://github.com/photopea/UZIP.js
// However, the vast majority of the codebase has diverged from UZIP.js to increase performance
// and reduce bundle size.

// Sometimes 0 will appear where -1 would be more appropriate. This is because using a uint
// is better for memory in most engines (I *think*).

use alloc::vec;
use alloc::vec::Vec;

/// fixed length extra bits
const FLEB: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0,
    /* unused */ 0, 0, /* impossible */ 0,
];
/// fixed distance extra bits
const FDEB: [u8; 32] = [
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13,
    13, /* unused */ 0, 0,
];
/// code length index map
const CLIM: [u8; 19] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];

// fixed length map
const FLRM: [usize; 512] = [
    4103, 1288, 264, 4488, 4359, 1800, 776, 3081, 4231, 1544, 520, 2569, 8, 2056, 1032, 3593, 4167,
    1416, 392, 2313, 4423, 1928, 904, 3337, 4295, 1672, 648, 2825, 136, 2184, 1160, 3849, 4135,
    1352, 328, 4552, 4391, 1864, 840, 3209, 4263, 1608, 584, 2697, 72, 2120, 1096, 3721, 4199,
    1480, 456, 2441, 4455, 1992, 968, 3465, 4327, 1736, 712, 2953, 200, 2248, 1224, 3977, 4119,
    1320, 296, 4520, 4375, 1832, 808, 3145, 4247, 1576, 552, 2633, 40, 2088, 1064, 3657, 4183,
    1448, 424, 2377, 4439, 1960, 936, 3401, 4311, 1704, 680, 2889, 168, 2216, 1192, 3913, 4151,
    1384, 360, 4584, 4407, 1896, 872, 3273, 4279, 1640, 616, 2761, 104, 2152, 1128, 3785, 4215,
    1512, 488, 2505, 4471, 2024, 1000, 3529, 4343, 1768, 744, 3017, 232, 2280, 1256, 4041, 4103,
    1304, 280, 4504, 4359, 1816, 792, 3113, 4231, 1560, 536, 2601, 24, 2072, 1048, 3625, 4167,
    1432, 408, 2345, 4423, 1944, 920, 3369, 4295, 1688, 664, 2857, 152, 2200, 1176, 3881, 4135,
    1368, 344, 4568, 4391, 1880, 856, 3241, 4263, 1624, 600, 2729, 88, 2136, 1112, 3753, 4199,
    1496, 472, 2473, 4455, 2008, 984, 3497, 4327, 1752, 728, 2985, 216, 2264, 1240, 4009, 4119,
    1336, 312, 4536, 4375, 1848, 824, 3177, 4247, 1592, 568, 2665, 56, 2104, 1080, 3689, 4183,
    1464, 440, 2409, 4439, 1976, 952, 3433, 4311, 1720, 696, 2921, 184, 2232, 1208, 3945, 4151,
    1400, 376, 4600, 4407, 1912, 888, 3305, 4279, 1656, 632, 2793, 120, 2168, 1144, 3817, 4215,
    1528, 504, 2537, 4471, 2040, 1016, 3561, 4343, 1784, 760, 3049, 248, 2296, 1272, 4073, 4103,
    1288, 264, 4488, 4359, 1800, 776, 3097, 4231, 1544, 520, 2585, 8, 2056, 1032, 3609, 4167, 1416,
    392, 2329, 4423, 1928, 904, 3353, 4295, 1672, 648, 2841, 136, 2184, 1160, 3865, 4135, 1352,
    328, 4552, 4391, 1864, 840, 3225, 4263, 1608, 584, 2713, 72, 2120, 1096, 3737, 4199, 1480, 456,
    2457, 4455, 1992, 968, 3481, 4327, 1736, 712, 2969, 200, 2248, 1224, 3993, 4119, 1320, 296,
    4520, 4375, 1832, 808, 3161, 4247, 1576, 552, 2649, 40, 2088, 1064, 3673, 4183, 1448, 424,
    2393, 4439, 1960, 936, 3417, 4311, 1704, 680, 2905, 168, 2216, 1192, 3929, 4151, 1384, 360,
    4584, 4407, 1896, 872, 3289, 4279, 1640, 616, 2777, 104, 2152, 1128, 3801, 4215, 1512, 488,
    2521, 4471, 2024, 1000, 3545, 4343, 1768, 744, 3033, 232, 2280, 1256, 4057, 4103, 1304, 280,
    4504, 4359, 1816, 792, 3129, 4231, 1560, 536, 2617, 24, 2072, 1048, 3641, 4167, 1432, 408,
    2361, 4423, 1944, 920, 3385, 4295, 1688, 664, 2873, 152, 2200, 1176, 3897, 4135, 1368, 344,
    4568, 4391, 1880, 856, 3257, 4263, 1624, 600, 2745, 88, 2136, 1112, 3769, 4199, 1496, 472,
    2489, 4455, 2008, 984, 3513, 4327, 1752, 728, 3001, 216, 2264, 1240, 4025, 4119, 1336, 312,
    4536, 4375, 1848, 824, 3193, 4247, 1592, 568, 2681, 56, 2104, 1080, 3705, 4183, 1464, 440,
    2425, 4439, 1976, 952, 3449, 4311, 1720, 696, 2937, 184, 2232, 1208, 3961, 4151, 1400, 376,
    4600, 4407, 1912, 888, 3321, 4279, 1656, 632, 2809, 120, 2168, 1144, 3833, 4215, 1528, 504,
    2553, 4471, 2040, 1016, 3577, 4343, 1784, 760, 3065, 248, 2296, 1272, 4089,
];
// fixed distance map
const FDRM: [usize; 32] = [
    5, 261, 133, 389, 69, 325, 197, 453, 37, 293, 165, 421, 101, 357, 229, 485, 21, 277, 149, 405,
    85, 341, 213, 469, 53, 309, 181, 437, 117, 373, 245, 501,
];

const FL: [usize; 31] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131,
    163, 195, 227, 258, 260, 261,
];
const FD: [usize; 31] = [
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537,
    2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577, 32769,
];
static REV: [usize; 32768] = compute_rev();
const fn compute_rev() -> [usize; 32768] {
    let mut rev = [0; 32768];
    let mut i = 0;
    while i < 32768 {
        let mut x = ((i & 0xaaaa) >> 1) | ((i & 0x5555) << 1);
        x = ((x & 0xcccc) >> 2) | ((x & 0x3333) << 2);
        x = ((x & 0xf0f0) >> 4) | ((x & 0x0f0f) << 4);
        x = (((x & 0xff00) >> 8) | ((x & 0x00ff) << 8)) >> 1;
        rev[x] = x;
        i += 1;
    }
    rev
}

/// Expands compressed GZIP, Zlib/DEFLATE, or DEFLATE_RAW data, automatically detecting the format
pub fn decompress_sync(data: &[u8], dict: Option<&[u8]>) -> Vec<u8> {
    let data_0 = data[0] as usize;
    let data_1 = data[1] as usize;
    let data_2 = data[2] as usize;
    if data_0 == 31 && data_1 == 139 && data_2 == 8 {
        gunzip_sync(data, dict)
    } else if (data_0 & 15) != 8 || data_0 >> 4 > 7 || ((data_0 << 8) | data_1) % 31 != 0 {
        inflate_sync(data, dict)
    } else {
        unzlib_sync(data, dict)
    }
}

/// Expands GZIP data
pub fn gunzip_sync(data: &[u8], dict: Option<&[u8]>) -> Vec<u8> {
    let st = gzs(data);
    if st + 8 > data.len() {
        err(6, None);
    }
    inflt(&data[st..data.len() - 8], Some(&vec![0; gzl(data)]), dict)
}

/// Expands DEFLATE data with no wrapper
pub fn inflate_sync(data: &[u8], dict: Option<&[u8]>) -> Vec<u8> {
    inflt(data, None, dict)
}

/// Expands Zlib data
pub fn unzlib_sync(data: &[u8], dict: Option<&[u8]>) -> Vec<u8> {
    inflt(&data[zls(data, dict)..data.len() - 4], None, dict)
}

/// expands raw DEFLATE data
fn inflt(dat: &[u8], bf: Option<&[u8]>, dict: Option<&[u8]>) -> Vec<u8> {
    // source lengt - dict length
    let sl = dat.len();
    let mut dl: usize = 0;
    if let Some(d) = dict {
        dl = d.len();
    }
    if sl == 0 {
        return bf.unwrap_or(&[]).to_vec();
    }
    let no_buf = bf.is_none();
    // have to estimate size
    let resize = no_buf;
    // Assumes roughly 33% compression ratio average
    let mut buf: Vec<u8> = bf.unwrap_or(&vec![0; sl * 3]).to_vec();
    // ensure buffer can fit at least l elements
    let cbuf = |l: usize, buf: &mut Vec<u8>| {
        let bl = buf.len();
        if l > bl {
            let new_size = usize::max(bl * 2, l);
            buf.resize(new_size, 0); // Resize the buffer, filling new elements with `0`
        }
    };
    //  last chunk - bitpos - bytes
    let mut finl = 0;
    let mut pos = 0;
    let mut bt = 0;
    let mut _lm_vec: Vec<usize> = vec![];
    let mut lm: Option<&[usize]> = None;
    let mut _dm_vec: Vec<usize> = vec![];
    let mut dm: Option<&[usize]> = None;
    let mut lbt = 0;
    let mut dbt = 0;
    // total bits
    let tbts = sl * 8;
    loop {
        if lm.is_none() {
            // BFINAL - this is only 1 when last chunk is next
            finl = bits(dat, pos, 1);
            // type: 0 = no compression, 1 = fixed huffman, 2 = dynamic huffman
            let c_type = bits(dat, pos + 1, 3);
            pos += 3;
            if c_type == 0 {
                // go to end of byte boundary
                let s = shft(pos) + 4;
                let l: usize = (dat[s - 4] as usize) | ((dat[s - 3] as usize) << 8);
                let t = s + l;
                if t > sl {
                    err(0, None);
                }
                // ensure size
                if resize {
                    cbuf(bt + l, &mut buf);
                }
                // Copy over uncompressed data
                buf[(s + bt)..(t + bt)].copy_from_slice(&dat[s..t]);
                // Get new bitpos, update byte count
                bt += l;
                pos = t * 8;
                continue;
            } else if c_type == 1 {
                lm = Some(&FLRM);
                dm = Some(&FDRM);
                lbt = 9;
                dbt = 5;
            } else if c_type == 2 {
                //  literal lengths
                let h_lit = bits(dat, pos, 31) + 257;
                let hc_len = bits(dat, pos + 10, 15) + 4;
                let tl = h_lit + bits(dat, pos + 5, 31) + 1;
                pos += 14;
                // length + distance tree
                let mut ldt: Vec<usize> = vec![0; tl];
                // code length tree
                //   let clt = new Uint8Array(19);
                let mut clt: Vec<usize> = vec![0; 19];
                //   for (let i = 0; i < hcLen; ++i) {
                for i in 0..hc_len {
                    // use index map to get real code
                    clt[CLIM[i] as usize] = bits(dat, pos + i * 3, 7);
                }
                pos += hc_len * 3;
                // code lengths bits
                let clb = max(&clt);
                let clbmsk = (1 << clb) - 1;
                // code lengths map
                let clm = h_map(&clt, clb, true);
                let mut i = 0;
                loop {
                    let r = clm[bits(dat, pos, clbmsk)];
                    // bits read
                    pos += r & 15;
                    // symbol
                    let s = r >> 4;
                    // code length to copy
                    if s < 16 {
                        ldt[i] = s;
                        i += 1;
                    } else {
                        //  copy   count
                        let mut c = 0;
                        let mut n = 0;
                        if s == 16 {
                            n = 3 + bits(dat, pos, 3);
                            pos += 2;
                            c = ldt[i - 1];
                        } else if s == 17 {
                            n = 3 + bits(dat, pos, 7);
                            pos += 3;
                        } else if s == 18 {
                            n = 11 + bits(dat, pos, 127);
                            pos += 7;
                        }
                        while n != 0 {
                            n -= 1;
                            ldt[i] = c;
                            i += 1;
                        }
                    }
                    if i >= tl {
                        break;
                    }
                }
                //    length tree                 distance tree
                let lt = &ldt[0..h_lit];
                let dt = &ldt[h_lit..];
                // max length bits
                lbt = max(lt);
                // max dist bits
                dbt = max(dt);
                _lm_vec = h_map(lt, lbt, true);
                lm = Some(&_lm_vec);
                _dm_vec = h_map(dt, dbt, true);
                dm = Some(&_dm_vec);
            } else {
                err(1, None);
            }
            if pos > tbts {
                err(0, None);
            }
        }
        // Make sure the buffer can hold this + the largest possible addition
        // Maximum chunk size (practically, theoretically infinite) is 2^17
        if resize {
            cbuf(bt + 131072, &mut buf);
        }
        let lms = (1 << lbt) - 1;
        let dms = (1 << dbt) - 1;
        loop {
            // bits read, code
            let mut c = 0;
            if let Some(lm) = lm {
                c = lm[bits16(dat, pos) & lms]; // Safe access
            }
            let sym = c >> 4;
            pos += c & 15;
            if pos > tbts {
                err(0, None);
            }
            if c == 0 {
                err(2, None);
            }
            match sym {
                0..=255 => {
                    buf[bt] = sym as u8;
                    bt += 1;
                }
                256 => {
                    lm = None;
                    break;
                }
                _ => {
                    let mut add = sym - 254;
                    // no extra bits needed if less
                    if sym > 264 {
                        // index
                        let i = sym - 257;
                        let b = FLEB[i];
                        add = bits(dat, pos, (1 << b) - 1) + FL[i];
                        pos += b as usize;
                    }
                    // dist
                    let mut d = 0;
                    if let Some(dm) = dm {
                        d = dm[bits16(dat, pos) & dms];
                    }
                    let dsym = d >> 4;
                    if d == 0 {
                        err(3, None);
                    }
                    pos += d & 15;
                    let mut dt = FD[dsym];
                    if dsym > 3 {
                        let b = FDEB[dsym];
                        dt += bits16(dat, pos) & ((1 << b) - 1);
                        pos += b as usize;
                    }
                    if pos > tbts {
                        err(0, None);
                    }
                    if resize {
                        cbuf(bt + 131072, &mut buf);
                    }
                    let end = bt + add;
                    if bt < dt {
                        let shift = dl - dt;
                        let dend = usize::min(dt, end);
                        if let Some(dict) = dict {
                            loop {
                                buf[bt] = dict[shift + bt];
                                bt += 1;
                                if bt >= dend {
                                    break;
                                }
                            }
                        }
                    }
                    loop {
                        buf[bt] = buf[bt - dt];
                        bt += 1;
                        if bt >= end {
                            break;
                        }
                    }
                }
            }
        }
        if lm.is_some() {
            finl = 1;
        }
        if finl != 0 {
            break;
        }
    }
    // don't reallocate for streams or user buffers
    if bt != buf.len() && no_buf {
        slc(&buf, 0, bt).to_vec()
    } else {
        buf[0..bt].to_vec()
    }
}

/// create huffman tree from u8 "map": index -> code length for code index
/// mb (max bits) must be at most 15
/// if r is true, its a decoder, otherwise its an encoder
fn h_map(cd: &[usize], mb: usize, r: bool) -> Vec<usize> {
    let s = cd.len();
    // index
    let i = 0;
    // u16 "map": index -> # of codes with bit length = index
    let mut l: Vec<usize> = vec![0; mb];
    // length of cd must be 288 (total # of codes)
    for i in i..s {
        if cd[i] != 0 {
            l[cd[i] - 1] += 1;
        }
    }
    // u16 "map": index -> minimum code for bit length = index
    let mut le = vec![0; mb];
    for i in 1..mb {
        le[i] = (le[i - 1] + l[i - 1]) << 1;
    }
    let mut co: Vec<usize>;
    if r {
        // u16 "map": index -> number of actual bits, symbol for code
        co = vec![0; 1 << mb];
        // bits to remove for reverser
        let rvb = 15 - mb;
        //   for (i = 0; i < s; ++i) {
        for i in 0..s {
            // ignore 0 lengths
            if cd[i] != 0 {
                // num encoding both symbol and bits read
                let sv = (i << 4) | (cd[i]);
                // free bits
                let r = mb - cd[i];
                // start value
                let v = (le[cd[i] - 1] + 1) << r;
                // m is end value
                //   for (let m = v | ((1 << r) - 1); v <= m; ++v) {
                let m = v | ((1 << r) - 1);
                for v in v..=m {
                    // every 16 bit value starting with the code yields the same result
                    co[REV[v] >> rvb] = sv;
                }
            }
        }
    } else {
        co = vec![0; s];
        // we don't use any of this encoder stuff
        // for i in 0..s {
        //     if cd[i] != 0 {
        //         //   co[i] = rev[le[cd[i] - 1]++] >> (15 - cd[i]);
        //         let index = cd[i] - 1;
        //         let value = le[index];
        //         co[i] = REV[value] >> (15 - cd[i]);
        //         le[index] += 1;
        //     }
        // }
    }

    co
}

/// find max of array
fn max(a: &[usize]) -> usize {
    let mut m = a[0];
    for val in a.iter().skip(1) {
        let a_i = *val;
        if a_i > m {
            m = a_i;
        }
    }
    m
}

/// read d, starting at bit p and mask with m
fn bits(d: &[u8], p: usize, m: usize) -> usize {
    let o = p / 8;
    (((d[o] as usize) | ((d[o + 1] as usize) << 8)) >> (p & 7)) & m
}

/// read d, starting at bit p continuing for at least 16 bits
fn bits16(d: &[u8], p: usize) -> usize {
    let o = p / 8;
    let d_0 = d.get(o).copied().unwrap_or(0) as usize;
    let d_1 = d.get(o + 1).copied().unwrap_or(0) as usize;
    let d_2 = d.get(o + 2).copied().unwrap_or(0) as usize;
    (d_0 | (d_1 << 8) | (d_2 << 16)) >> (p & 7)
}

/// get end of byte
fn shft(p: usize) -> usize {
    (p + 7) / 8
}

/// typed array slice - allows garbage collector to free original reference,
/// while being more compatible than .slice
fn slc(v: &[u8], s: usize, mut e: usize) -> &[u8] {
    e = e.min(v.len());

    &v[s..e]
}

// error codes
const EC: &[&str; 7] = &[
    "unexpected EOF",
    "invalid block type",
    "invalid length/literal",
    "invalid distance",
    "stream finished",
    "no stream handler", // determined by compression function
    "no callback",
    // OTHER ERRORS GO PAST WHAT WE ACTUALLY USE FOR THIS LIBRARY
    // "invalid UTF-8 data",
    // "extra field too long",
    // "date not in range 1980-2099",
    // "filename too long",
    // "stream finishing",
    // "invalid zip data",
    // determined by unknown compression method
];

fn err(code: usize, msg: Option<&str>) -> ! {
    let msg = msg.unwrap_or("Invalid gzip data");
    panic!("flate error: {} ({})", msg, EC[code]);
}

/// Determines the start position of gzip-compressed data.
fn gzs(d: &[u8]) -> usize {
    if d.len() < 10 || d[0] != 31 || d[1] != 139 || d[2] != 8 {
        err(6, None);
    }

    let flg = d[3];
    let mut st = 10;

    if (flg & 4) != 0 {
        st += (d[10] as usize | ((d[11] as usize) << 8)) + 2;
    }

    let mut zs = ((flg >> 3) & 1) + ((flg >> 4) & 1);
    while zs > 0 {
        if d[st] == 0 {
            zs -= 1;
        }
        st += 1;
    }

    st + ((flg & 2) as usize)
}

/// gzip length
fn gzl(d: &[u8]) -> usize {
    let l = d.len();
    (d[l - 4] as usize)
        | ((d[l - 3] as usize) << 8)
        | ((d[l - 2] as usize) << 16)
        | ((d[l - 1] as usize) << 24)
}

/// zlib start
fn zls(d: &[u8], dict: Option<&[u8]>) -> usize {
    if (d[0] & 15) != 8 || d[0] >> 4 > 7 || (((d[0] as u32) << 8_u32) | d[1] as u32) % 31 != 0 {
        err(6, None);
    }
    if ((d[1] >> 5) & 1) == 0 && dict.is_some() {
        err(6, Some("Invalid gzip data; need/expcepted dictionary"));
    }

    (((d[1] as usize) >> 3) & 4) + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    // use std::fs;
    // use std::path::PathBuf;
    // use std::println;

    // #[test]
    // fn deflate_sync_dictionary() {
    //     // get dictionary
    //     let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //     path.push("tests/util/fixtures/spdyDict.txt");
    //     let dictionary: Vec<u8> = fs::read(&path).expect("Failed to read file dict");
    //     // get expected
    //     let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //     path.push("tests/util/fixtures/lorem_en_100k.txt");
    //     let expected: Vec<u8> = fs::read(&path).expect("Failed to read file expected");
    //     // get compressed
    //     let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //     path.push("tests/util/fixtures/deflateSync_dictionary_compressed.bin");
    //     let compressed: Vec<u8> = fs::read(&path).expect("Failed to read file compressed");

    //     let decompressed = decompress_sync(&compressed, Some(&dictionary));

    //     assert_eq!(decompressed, expected);
    // }

    // #[test]
    // fn deflate_sync_level_9() {
    //     // get expected
    //     let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //     path.push("tests/util/fixtures/lorem_en_100k.txt");
    //     let expected: Vec<u8> = fs::read(&path).expect("Failed to read file expected");
    //     println!("expected: {:?}", &expected[0..20]);
    //     // get compressed
    //     let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //     path.push("tests/util/fixtures/deflateSync_level_9_compressed.bin");
    //     let compressed: Vec<u8> = fs::read(&path).expect("Failed to read file compressed");
    //     println!("compressed: {:?}", &compressed[0..20]);

    //     let decompressed = decompress_sync(&compressed, None);

    //     assert_eq!(decompressed, expected);
    // }

    // #[test]
    // fn deflate_sync_mem_level_9() {
    //     // get expected
    //     let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //     path.push("tests/util/fixtures/lorem_en_100k.txt");
    //     let expected: Vec<u8> = fs::read(&path).expect("Failed to read file expected");
    //     // get compressed
    //     let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //     path.push("tests/util/fixtures/deflateSync_memLevel_9_compressed.bin");
    //     let compressed: Vec<u8> = fs::read(&path).expect("Failed to read file compressed");

    //     let decompressed = decompress_sync(&compressed, None);

    //     assert_eq!(decompressed, expected);
    // }

    // #[test]
    // fn deflate_sync_strategy_0() {
    //     // get expected
    //     let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //     path.push("tests/util/fixtures/lorem_en_100k.txt");
    //     let expected: Vec<u8> = fs::read(&path).expect("Failed to read file expected");
    //     // get compressed
    //     let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //     path.push("tests/util/fixtures/deflateSync_strategy_0_compressed.bin");
    //     let compressed: Vec<u8> = fs::read(&path).expect("Failed to read file compressed");

    //     let decompressed = decompress_sync(&compressed, None);

    //     assert_eq!(decompressed, expected);
    // }

    // #[test]
    // fn deflate_raw_sync_window_bits_15() {
    //     // get expected
    //     let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //     path.push("tests/util/fixtures/lorem_en_100k.txt");
    //     let expected: Vec<u8> = fs::read(&path).expect("Failed to read file expected");
    //     // get compressed
    //     let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //     path.push("tests/util/fixtures/deflateRawSync_windowBits_15_compressed.bin");
    //     let compressed: Vec<u8> = fs::read(&path).expect("Failed to read file compressed");

    //     let decompressed = decompress_sync(&compressed, None);

    //     assert_eq!(decompressed, expected);
    // }

    // #[test]
    // fn deflate_raw_sync_level_0() {
    //     // get expected
    //     let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //     path.push("tests/util/fixtures/lorem_en_100k.txt");
    //     let expected: Vec<u8> = fs::read(&path).expect("Failed to read file expected");
    //     // get compressed
    //     let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //     path.push("tests/util/fixtures/deflateRawSync_level_0_compressed.bin");
    //     let compressed: Vec<u8> = fs::read(&path).expect("Failed to read file compressed");

    //     let decompressed = decompress_sync(&compressed, None);

    //     assert_eq!(decompressed, expected);
    // }

    #[test]
    fn dictionary() {
        let dict: Vec<u8> = [97, 98, 99, 100].to_vec();
        let compressed: Vec<u8> = [
            120, 187, 3, 216, 1, 139, 203, 72, 205, 201, 201, 7, 19, 10, 229, 249, 69, 57, 41, 0,
            55, 19, 6, 113,
        ]
        .to_vec();
        let expected: Vec<u8> =
            [104, 101, 108, 108, 111, 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]
                .to_vec();

        let decompressed = decompress_sync(&compressed, Some(&dict));

        assert_eq!(decompressed, expected);
    }

    #[test]
    fn simple_gzip() {
        let compressed_gzip: Vec<u8> = [
            31, 139, 8, 8, 230, 176, 184, 103, 0, 3, 101, 120, 112, 101, 99, 116, 101, 100, 46,
            116, 120, 116, 0, 11, 201, 200, 44, 86, 0, 162, 68, 133, 146, 212, 226, 18, 133, 252,
            52, 133, 204, 188, 130, 210, 18, 133, 148, 196, 146, 68, 46, 174, 16, 168, 108, 110,
            126, 81, 42, 88, 72, 143, 139, 203, 208, 200, 216, 196, 216, 204, 212, 156, 11, 0, 162,
            255, 102, 10, 59, 0, 0, 0,
        ]
        .to_vec();
        let expected = [
            84, 104, 105, 115, 32, 105, 115, 32, 97, 32, 116, 101, 115, 116, 32, 111, 102, 32, 105,
            110, 112, 117, 116, 32, 100, 97, 116, 97, 10, 10, 84, 104, 105, 115, 32, 105, 115, 32,
            109, 111, 114, 101, 32, 100, 97, 116, 97, 46, 10, 10, 49, 50, 51, 52, 51, 54, 53, 55,
            10,
        ]
        .to_vec();

        let decompressed = decompress_sync(&compressed_gzip, None);

        assert_eq!(decompressed, expected);
    }

    #[test]
    fn simple_deflate() {
        let compressed_gzip: Vec<u8> = [
            120, 156, 11, 201, 200, 44, 86, 0, 162, 68, 133, 146, 212, 226, 18, 133, 252, 52, 133,
            204, 188, 130, 210, 18, 133, 148, 196, 146, 68, 46, 174, 16, 168, 108, 110, 126, 81,
            42, 88, 72, 143, 139, 203, 208, 200, 216, 196, 216, 204, 212, 156, 11, 0, 80, 157, 18,
            21,
        ]
        .to_vec();
        let expected = [
            84, 104, 105, 115, 32, 105, 115, 32, 97, 32, 116, 101, 115, 116, 32, 111, 102, 32, 105,
            110, 112, 117, 116, 32, 100, 97, 116, 97, 10, 10, 84, 104, 105, 115, 32, 105, 115, 32,
            109, 111, 114, 101, 32, 100, 97, 116, 97, 46, 10, 10, 49, 50, 51, 52, 51, 54, 53, 55,
            10,
        ]
        .to_vec();

        let decompressed = decompress_sync(&compressed_gzip, None);

        assert_eq!(decompressed, expected);
    }

    #[test]
    fn simple_deflate_raw() {
        let compressed_gzip: Vec<u8> = [
            11, 201, 200, 44, 86, 0, 162, 68, 133, 146, 212, 226, 18, 133, 252, 52, 133, 204, 188,
            130, 210, 18, 133, 148, 196, 146, 68, 46, 174, 16, 168, 108, 110, 126, 81, 42, 88, 72,
            143, 139, 203, 208, 200, 216, 196, 216, 204, 212, 156, 11, 0, 80, 157, 18,
        ]
        .to_vec();
        let expected = [
            84, 104, 105, 115, 32, 105, 115, 32, 97, 32, 116, 101, 115, 116, 32, 111, 102, 32, 105,
            110, 112, 117, 116, 32, 100, 97, 116, 97, 10, 10, 84, 104, 105, 115, 32, 105, 115, 32,
            109, 111, 114, 101, 32, 100, 97, 116, 97, 46, 10, 10, 49, 50, 51, 52, 51, 54, 53, 55,
            10,
        ]
        .to_vec();

        let decompressed = decompress_sync(&compressed_gzip, None);

        assert_eq!(decompressed, expected);
    }
}
