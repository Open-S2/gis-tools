use libm::fabs;

use super::util::{estimate, pred_sum, RESULTERRBOUND, SPLITTER};

const CCWERRBOUND_A: f64 = 3.3306690738754716e-16; // (3 + 16 * epsilon) * epsilon;
const CCWERRBOUND_B: f64 = 2.2204460492503146e-16; // (2 + 12 * epsilon) * epsilon;
const CCWERRBOUND_C: f64 = 1.1093356479670487e-31; // (9 + 64 * epsilon) * epsilon * epsilon;

/// a negative value if the points a, b, and c occur in counterclockwise order
/// (c lies to the left of the directed line defined by points a and b).
/// - Returns a positive value if they occur in clockwise order (c lies to the right of the directed line ab).
/// - Returns zero if they are collinear.
pub fn orient2dadapt(ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64, detsum: f64) -> f64 {
    let mut b: [f64; 4] = [0.0; 4];
    let mut c1: [f64; 8] = [0.0; 8];
    let mut c2: [f64; 12] = [0.0; 12];
    let mut d: [f64; 16] = [0.0; 16];
    let mut u: [f64; 4] = [0.0; 4];
    let mut bvirt: f64;
    let mut c: f64;
    let mut ahi: f64;
    let mut alo: f64;
    let mut bhi: f64;
    let mut blo: f64;
    let mut _i: f64;
    let mut _j: f64;
    let mut _z: f64;
    let mut s1: f64;
    let mut s0: f64;
    let mut t1: f64;
    let mut t0: f64;
    let mut u3: f64;

    let acx = ax - cx;
    let bcx = bx - cx;
    let acy = ay - cy;
    let bcy = by - cy;

    s1 = acx * bcy;
    c = SPLITTER * acx;
    ahi = c - (c - acx);
    alo = acx - ahi;
    c = SPLITTER * bcy;
    bhi = c - (c - bcy);
    blo = bcy - bhi;
    s0 = alo * blo - (s1 - ahi * bhi - alo * bhi - ahi * blo);
    t1 = acy * bcx;
    c = SPLITTER * acy;
    ahi = c - (c - acy);
    alo = acy - ahi;
    c = SPLITTER * bcx;
    bhi = c - (c - bcx);
    blo = bcx - bhi;
    t0 = alo * blo - (t1 - ahi * bhi - alo * bhi - ahi * blo);
    _i = s0 - t0;
    bvirt = s0 - _i;
    b[0] = s0 - (_i + bvirt) + (bvirt - t0);
    _j = s1 + _i;
    bvirt = _j - s1;
    _z = s1 - (_j - bvirt) + (_i - bvirt);
    _i = _z - t1;
    bvirt = _z - _i;
    b[1] = _z - (_i + bvirt) + (bvirt - t1);
    u3 = _j + _i;
    bvirt = u3 - _j;
    b[2] = _j - (u3 - bvirt) + (_i - bvirt);
    b[3] = u3;

    let mut det = estimate(4, &b);
    let mut errbound = CCWERRBOUND_B * detsum;
    if det >= errbound || -det >= errbound {
        return det;
    }

    bvirt = ax - acx;
    let acxtail = ax - (acx + bvirt) + (bvirt - cx);
    bvirt = bx - bcx;
    let bcxtail = bx - (bcx + bvirt) + (bvirt - cx);
    bvirt = ay - acy;
    let acytail = ay - (acy + bvirt) + (bvirt - cy);
    bvirt = by - bcy;
    let bcytail = by - (bcy + bvirt) + (bvirt - cy);

    if acxtail == 0. && acytail == 0. && bcxtail == 0. && bcytail == 0. {
        return det;
    }

    errbound = CCWERRBOUND_C * detsum + RESULTERRBOUND * fabs(det);
    det += acx * bcytail + bcy * acxtail - (acy * bcxtail + bcx * acytail);
    if det >= errbound || -det >= errbound {
        return det;
    }

    s1 = acxtail * bcy;
    c = SPLITTER * acxtail;
    ahi = c - (c - acxtail);
    alo = acxtail - ahi;
    c = SPLITTER * bcy;
    bhi = c - (c - bcy);
    blo = bcy - bhi;
    s0 = alo * blo - (s1 - ahi * bhi - alo * bhi - ahi * blo);
    t1 = acytail * bcx;
    c = SPLITTER * acytail;
    ahi = c - (c - acytail);
    alo = acytail - ahi;
    c = SPLITTER * bcx;
    bhi = c - (c - bcx);
    blo = bcx - bhi;
    t0 = alo * blo - (t1 - ahi * bhi - alo * bhi - ahi * blo);
    _i = s0 - t0;
    bvirt = s0 - _i;
    u[0] = s0 - (_i + bvirt) + (bvirt - t0);
    _j = s1 + _i;
    bvirt = _j - s1;
    _z = s1 - (_j - bvirt) + (_i - bvirt);
    _i = _z - t1;
    bvirt = _z - _i;
    u[1] = _z - (_i + bvirt) + (bvirt - t1);
    u3 = _j + _i;
    bvirt = u3 - _j;
    u[2] = _j - (u3 - bvirt) + (_i - bvirt);
    u[3] = u3;
    let c1len = pred_sum(4, &b, 4, &u, &mut c1);

    s1 = acx * bcytail;
    c = SPLITTER * acx;
    ahi = c - (c - acx);
    alo = acx - ahi;
    c = SPLITTER * bcytail;
    bhi = c - (c - bcytail);
    blo = bcytail - bhi;
    s0 = alo * blo - (s1 - ahi * bhi - alo * bhi - ahi * blo);
    t1 = acy * bcxtail;
    c = SPLITTER * acy;
    ahi = c - (c - acy);
    alo = acy - ahi;
    c = SPLITTER * bcxtail;
    bhi = c - (c - bcxtail);
    blo = bcxtail - bhi;
    t0 = alo * blo - (t1 - ahi * bhi - alo * bhi - ahi * blo);
    _i = s0 - t0;
    bvirt = s0 - _i;
    u[0] = s0 - (_i + bvirt) + (bvirt - t0);
    _j = s1 + _i;
    bvirt = _j - s1;
    _z = s1 - (_j - bvirt) + (_i - bvirt);
    _i = _z - t1;
    bvirt = _z - _i;
    u[1] = _z - (_i + bvirt) + (bvirt - t1);
    u3 = _j + _i;
    bvirt = u3 - _j;
    u[2] = _j - (u3 - bvirt) + (_i - bvirt);
    u[3] = u3;
    let c2len = pred_sum(c1len, &c1, 4, &u, &mut c2);

    s1 = acxtail * bcytail;
    c = SPLITTER * acxtail;
    ahi = c - (c - acxtail);
    alo = acxtail - ahi;
    c = SPLITTER * bcytail;
    bhi = c - (c - bcytail);
    blo = bcytail - bhi;
    s0 = alo * blo - (s1 - ahi * bhi - alo * bhi - ahi * blo);
    t1 = acytail * bcxtail;
    c = SPLITTER * acytail;
    ahi = c - (c - acytail);
    alo = acytail - ahi;
    c = SPLITTER * bcxtail;
    bhi = c - (c - bcxtail);
    blo = bcxtail - bhi;
    t0 = alo * blo - (t1 - ahi * bhi - alo * bhi - ahi * blo);
    _i = s0 - t0;
    bvirt = s0 - _i;
    u[0] = s0 - (_i + bvirt) + (bvirt - t0);
    _j = s1 + _i;
    bvirt = _j - s1;
    _z = s1 - (_j - bvirt) + (_i - bvirt);
    _i = _z - t1;
    bvirt = _z - _i;
    u[1] = _z - (_i + bvirt) + (bvirt - t1);
    u3 = _j + _i;
    bvirt = u3 - _j;
    u[2] = _j - (u3 - bvirt) + (_i - bvirt);
    u[3] = u3;
    let dlen = pred_sum(c2len, &c2, 4, &u, &mut d);

    d[dlen - 1]
}

/// returns a positive value if the points a, b, and c occur in counterclockwise order
/// (c lies to the left of the directed line defined by points a and b).
/// - Returns a negative value if they occur in clockwise order (c lies to the right of the directed line ab).
/// - Returns zero if they are collinear.
pub fn orient2d(ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64) -> f64 {
    let detleft = (ay - cy) * (bx - cx);
    let detright = (ax - cx) * (by - cy);
    let det = detleft - detright;

    let detsum = fabs(detleft + detright);
    if fabs(det) >= CCWERRBOUND_A * detsum {
        return det;
    }

    -orient2dadapt(ax, ay, bx, by, cx, cy, detsum)
}

/// returns a positive value if the points a, b, and c occur in counterclockwise order
/// (c lies to the left of the directed line defined by points a and b).
/// - Returns a negative value if they occur in clockwise order (c lies to the right of the directed line ab).
/// - Returns zero if they are collinear.
pub fn orient2dfast(ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64) -> f64 {
    (ay - cy) * (bx - cx) - (ax - cx) * (by - cy)
}
