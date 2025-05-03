/// Splitter value
pub const SPLITTER: f64 = 134217729.0;
/// Result error bound
pub const RESULTERRBOUND: f64 = (3.0 + 8.0 * f64::EPSILON) * f64::EPSILON;
/// CCW error bound A
pub const CCWERRBOUND_A: f64 = (3.0 + 16.0 * f64::EPSILON) * f64::EPSILON;
/// CCW error bound B
pub const CCWERRBOUND_B: f64 = (2.0 + 12.0 * f64::EPSILON) * f64::EPSILON;
/// CCW error bound C
pub const CCWERRBOUND_C: f64 = (9.0 + 64.0 * f64::EPSILON) * f64::EPSILON * f64::EPSILON;

/// ast_expansion_sum_zeroelim routine from oritinal code
pub fn pred_sum(elen: usize, e: &[f64], flen: usize, f: &[f64], h: &mut [f64]) -> usize {
    let mut q: f64;
    let mut qnew: f64;
    let mut hh: f64;
    let mut bvirt: f64;
    let mut enow = e[0];
    let mut fnow = f[0];
    let mut eindex = 0;
    let mut findex = 0;
    if (fnow > enow) == (fnow > -enow) {
        q = enow;
        eindex += 1;
        enow = e.get(eindex).copied().unwrap_or(0.);
    } else {
        q = fnow;
        findex += 1;
        fnow = f.get(findex).copied().unwrap_or(0.);
    }
    let mut hindex = 0;
    if eindex < elen && findex < flen {
        if (fnow > enow) == (fnow > -enow) {
            qnew = enow + q;
            hh = q - (qnew - enow);
            eindex += 1;
            // enow = e[eindex];
            enow = e.get(eindex).copied().unwrap_or(0.);
        } else {
            qnew = fnow + q;
            hh = q - (qnew - fnow);
            findex += 1;
            // fnow = f[findex];
            fnow = f.get(findex).copied().unwrap_or(0.);
        }
        q = qnew;
        if hh != 0. {
            h[hindex] = hh;
            hindex += 1;
        }
        while eindex < elen && findex < flen {
            if (fnow > enow) == (fnow > -enow) {
                qnew = q + enow;
                bvirt = qnew - q;
                hh = q - (qnew - bvirt) + (enow - bvirt);
                eindex += 1;
                enow = e.get(eindex).copied().unwrap_or(0.);
            } else {
                qnew = q + fnow;
                bvirt = qnew - q;
                hh = q - (qnew - bvirt) + (fnow - bvirt);
                findex += 1;
                // fnow = f[findex];
                fnow = f.get(findex).copied().unwrap_or(0.);
            }
            q = qnew;
            if hh != 0. {
                h[hindex] = hh;
                hindex += 1;
            }
        }
    }
    while eindex < elen {
        qnew = q + enow;
        bvirt = qnew - q;
        hh = q - (qnew - bvirt) + (enow - bvirt);
        eindex += 1;
        // enow = e[eindex];
        enow = e.get(eindex).copied().unwrap_or(0.);
        q = qnew;
        if hh != 0. {
            h[hindex] = hh;
            hindex += 1;
        }
    }
    while findex < flen {
        qnew = q + fnow;
        bvirt = qnew - q;
        hh = q - (qnew - bvirt) + (fnow - bvirt);
        findex += 1;
        // fnow = f[findex];
        fnow = f.get(findex).copied().unwrap_or(0.);
        q = qnew;
        if hh != 0. {
            h[hindex] = hh;
            hindex += 1;
        }
    }
    if q != 0. || hindex == 0 {
        h[hindex] = q;
        hindex += 1;
    }
    hindex
}

// /// Predicate sum of three points
// #[allow(clippy::too_many_arguments)]
// pub fn pred_sum_three(
//     alen: usize,
//     a: &[f64],
//     blen: usize,
//     b: &[f64],
//     clen: usize,
//     c: &[f64],
//     tmp: &mut [f64],
//     out: &mut [f64],
// ) -> usize {
//     pred_sum(pred_sum(alen, a, blen, b, tmp), tmp, clen, c, out)
// }

// /// scale_expansion_zeroelim routine from oritinal code
// pub fn scale(elen: usize, e: &[f64], b: f64, h: &mut [f64]) -> usize {
//     let mut q: f64;
//     let mut sum: f64;
//     let mut hh: f64;
//     let mut product1: f64;
//     let mut product0: f64;
//     let mut bvirt: f64;
//     let mut c: f64;
//     let mut ahi: f64;
//     let mut alo: f64;

//     c = SPLITTER * b;
//     let bhi = c - (c - b);
//     let blo = b - bhi;
//     let mut enow = e[0];
//     q = enow * b;
//     c = SPLITTER * enow;
//     ahi = c - (c - enow);
//     alo = enow - ahi;
//     hh = alo * blo - (q - ahi * bhi - alo * bhi - ahi * blo);
//     let mut hindex = 0;
//     if hh != 0. {
//         h[hindex] = hh;
//         hindex += 1;
//     }

//     for item in e.iter().take(elen).skip(1) {
//         enow = *item;
//         product1 = enow * b;
//         c = SPLITTER * enow;
//         ahi = c - (c - enow);
//         alo = enow - ahi;
//         product0 = alo * blo - (product1 - ahi * bhi - alo * bhi - ahi * blo);
//         sum = q + product0;
//         bvirt = sum - q;
//         hh = q - (sum - bvirt) + (product0 - bvirt);
//         if hh != 0. {
//             h[hindex] = hh;
//             hindex += 1;
//         }
//         q = product1 + sum;
//         hh = sum - (q - product1);
//         if hh != 0. {
//             h[hindex] = hh;
//             hindex += 1;
//         }
//     }
//     if q != 0. || hindex == 0 {
//         h[hindex] = q;
//         hindex += 1;
//     }

//     hindex
// }

/// estimation of expansion sum
pub fn estimate(elen: usize, e: &[f64]) -> f64 {
    e.iter().take(elen).skip(1).sum()
}
