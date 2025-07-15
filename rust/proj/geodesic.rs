use core::f64::consts::{FRAC_1_SQRT_2, PI};
use libm::{
    atan, atan2, atanh, cbrt, copysign, cos, fabs, fmax, fmin, hypot, remainder, remquo, sin, sqrt,
};

const GEOGRAPHICLIB_GEODESIC_ORDER: usize = 6;
const N_A1: usize = GEOGRAPHICLIB_GEODESIC_ORDER;
const N_A2: usize = GEOGRAPHICLIB_GEODESIC_ORDER;
const N_A3: usize = GEOGRAPHICLIB_GEODESIC_ORDER;
const N_C: usize = GEOGRAPHICLIB_GEODESIC_ORDER + 1;
const N_C1: usize = GEOGRAPHICLIB_GEODESIC_ORDER;
const N_C1_P: usize = GEOGRAPHICLIB_GEODESIC_ORDER;
const N_C2: usize = GEOGRAPHICLIB_GEODESIC_ORDER;
const N_C3: usize = GEOGRAPHICLIB_GEODESIC_ORDER;
const N_C4: usize = GEOGRAPHICLIB_GEODESIC_ORDER;
const TOL0: f64 = f64::EPSILON;
const TOL1: f64 = 200. * TOL0;
const TOL2: f64 = 1.4901161193847656e-8; // sqrt(EPSILON);
const TOLB: f64 = TOL0;
const DEGREE: f64 = core::f64::consts::PI / 180.0;
const QD: f64 = 90.0;
const HD: f64 = 180.0;
const TD: f64 = 360.0;
const TINY: f64 = 1.4916681462400413e-154; // f64::MIN_POSITIVE.sqrt();
const DIGITS: u32 = f64::MANTISSA_DIGITS;
const MAXIT1: u32 = 20;
const MAXIT2: u32 = MAXIT1 + DIGITS + 10;
const XTHRESH: f64 = 1000. * TOL2;

/// mask values for the \e caps argument to geod_lineinit().
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum GeodMask {
    /// Calculate nothing
    #[default]
    GeodNone = 0, // < Calculate nothing */
    /// Calculate latitude
    GeodLatitude = 1 << 7, // < Calculate latitude */
    /// Calculate longitude
    GeodLongitude = 1 << 8 | 1 << 3, // < Calculate longitude */
    /// Calculate azimuth
    GeodAzimuth = 1 << 9, // < Calculate azimuth */
    /// Calculate distance
    GeodDistance = 1 << 10 | 1 << 0, // < Calculate distance */
    /// Allow distance as input
    GeodDistanceIn = 1 << 11 | 1 << 0 | 1 << 1, // < Allow distance as input  */
    /// Calculate reduced length
    GeodReducedlength = 1 << 12 | 1 << 0 | 1 << 2, // < Calculate reduced length */
    /// Calculate geodesic scale
    GeodGeodesicScale = 1 << 13 | 1 << 0 | 1 << 2, // < Calculate geodesic scale */
    /// Calculate area
    GeodArea = 1 << 14 | 1 << 4, // < Calculate reduced length */
    /// Calculate everything
    GeodAll = 0x7F80 | 0x1F, // < Calculate everything */
}

/// flag values for the \e flags argument to geod_gendirect() and geod_genposition()
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum GeodFlags {
    /// No flags
    #[default]
    GeodNoflags = 0, // < No flags */
    /// Position given in terms of arc distance
    GeodArcmode = 1 << 0, // < Position given in terms of arc distance */
    /// Unroll the longitude
    GeodLongUnroll = 1 << 15, // < Unroll the longitude */
}

/// Cap Types to build for the \e caps argument to geod_lineinit().
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum CapType {
    /// No caps
    #[default]
    CapNone = 0,
    /// Cap C1
    CapC1 = 1 << 0,
    /// Cap C1p
    CapC1p = 1 << 1,
    /// Cap C2
    CapC2 = 1 << 2,
    /// Cap C3
    CapC3 = 1 << 3,
    /// Cap C4
    CapC4 = 1 << 4,
    /// Cap C5
    CapAll = 0x1F,
    /// All caps
    OutAll = 0x7F80,
}

/// The struct containing information about the ellipsoid. This must be
/// initialized by geod_init() before use.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct GeodGeodesic {
    /// the equatorial radius
    pub a: f64,
    /// the flattening
    pub f: f64,
    /// the second flattening
    pub f1: f64,
    /// second eccentricity
    pub e2: f64,
    /// the second eccentricity squared
    pub ep2: f64,
    /// third  flattening
    pub n: f64,
    /// semiminor axis
    pub b: f64,
    /// TODO: I don't know what this represents
    pub c2: f64,
    /// the tolerance
    pub etol2: f64,
    /// TODO: I don't know what this represents
    pub a3x: [f64; 6],
    /// TODO: I don't know what this represents
    pub c3x: [f64; 15],
    /// TODO: I don't know what this represents
    pub c4x: [f64; 21],
}

/// The struct containing information about a single geodesic.  This must be
/// initialized by geod_lineinit(), geod_directline(), geod_gendirectline(),
/// or geod_inverseline() before use.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct GeodGeodesicline {
    /// < the starting latitude
    pub lat1: f64,
    /// < the starting longitude  
    pub lon1: f64,
    /// < the starting azimuth
    pub azi1: f64,
    /// < the equatorial radius       
    pub a: f64,
    /// < the flattening        
    pub f: f64,
    /// < sine of \e azi1     
    pub salp1: f64,
    /// < cosine of \e azi1      
    pub calp1: f64,
    /// < arc length to reference point            
    pub a13: f64,
    /// < distance to reference point          
    pub s13: f64,
    /// < @cond SKIP
    pub b: f64,
    /// UNKNOWN
    pub c2: f64,
    /// UNKNOWN
    pub f1: f64,
    /// UNKNOWN
    pub salp0: f64,
    /// UNKNOWN
    pub calp0: f64,
    /// UNKNOWN
    pub k2: f64,
    /// UNKNOWN
    pub ssig1: f64,
    /// UNKNOWN
    pub csig1: f64,
    /// UNKNOWN
    pub dn1: f64,
    /// UNKNOWN
    pub stau1: f64,
    /// UNKNOWN
    pub ctau1: f64,
    /// UNKNOWN
    pub somg1: f64,
    /// UNKNOWN
    pub comg1: f64,
    /// UNKNOWN
    pub a1m1: f64,
    /// UNKNOWN
    pub a2m1: f64,
    /// UNKNOWN
    pub a3c: f64,
    /// UNKNOWN
    pub b11: f64,
    /// UNKNOWN
    pub b21: f64,
    /// UNKNOWN
    pub b31: f64,
    /// UNKNOWN
    pub a4: f64,
    /// UNKNOWN
    pub b41: f64,
    /// UNKNOWN
    pub c1a: [f64; 7],
    /// UNKNOWN
    pub c1pa: [f64; 7],
    /// UNKNOWN
    pub c2a: [f64; 7],
    /// UNKNOWN
    pub c3a: [f64; 6],
    /// UNKNOWN
    pub c4a: [f64; 6],
    /// < @endcond
    /// < the capabilities
    caps: u32,
}

fn sq(x: f64) -> f64 {
    x * x
}

fn sumx(u: f64, v: f64, t: &mut f64) -> f64 {
    let s = u + v;
    let mut up = s - v;
    let mut vpp = s - up;
    up -= u;
    vpp -= v;
    if *t != 0. {
        *t = if s != 0. { 0. - (up + vpp) } else { s };
    }
    // error-free sum:
    // u + v =       s      + t
    //       = round(u + v) + t
    s
}

/// Initialize a GeodGeodesic object
pub fn geod_init(g: &mut GeodGeodesic, a: f64, f: f64) {
    g.a = a;
    g.f = f;
    g.f1 = 1. - g.f;
    g.e2 = g.f * (2. - g.f);
    g.ep2 = g.e2 / sq(g.f1); /* e2 / (1 - e2) */
    g.n = g.f / (2. - g.f);
    g.b = g.a * g.f1;
    g.c2 = (sq(g.a)
        + sq(g.b)
            * (if g.e2 == 0. {
                1.
            } else {
                (if g.e2 > 0. { atanh(sqrt(g.e2)) } else { atan(sqrt(-g.e2)) }) / sqrt(fabs(g.e2))
            }))
        / 2.; /* authalic radius squared */
    // The sig12 threshold for "really short".  Using the auxiliary sphere
    // solution with dnm computed at (bet1 + bet2) / 2, the relative error in the
    // azimuth consistency check is sig12^2 * abs(f) * min(1, 1-f/2) / 2.  (Error
    // measured for 1/100 < b/a < 100 and abs(f) >= 1/1000.  For a given f and
    // sig12, the max error occurs for lines near the pole.  If the old rule for
    // computing dnm = (dn1 + dn2)/2 is used, then the error increases by a
    // factor of 2.)  Setting this equal to epsilon gives sig12 = etol2.  Here
    // 0.1 is a safety factor (error decreased by 100) and max(0.001, abs(f))
    // stops etol2 getting too large in the nearly spherical case.
    g.etol2 = 0.1 * TOL2 / sqrt(fmax(0.001, fabs(g.f)) * fmin(1.0, 1. - g.f / 2.) / 2.);

    a3coeff(g);
    c3coeff(g);
    c4coeff(g);
}

/// Compute the geodesic inverse between two points
#[allow(clippy::too_many_arguments)]
pub fn geod_inverse(
    g: &mut GeodGeodesic,
    lat1: f64,
    lon1: f64,
    lat2: f64,
    lon2: f64,
    ps12: &mut f64,
    pazi1: &mut f64,
    pazi2: &mut f64,
) {
    geod_geninverse(
        g, lat1, lon1, lat2, lon2, ps12, pazi1, pazi2, &mut 0.0, &mut 0.0, &mut 0.0, &mut 0.0,
    );
}

/// Compute the geodesic directly between two points
#[allow(clippy::too_many_arguments)]
pub fn geod_gendirect(
    g: &mut GeodGeodesic,
    lat1: f64,
    lon1: f64,
    azi1: f64,
    flags: u32,
    s12_a12: f64,
    plat2: &mut f64,
    plon2: &mut f64,
    pazi2: &mut f64,
    ps12: &mut f64,
    pm12: &mut f64,
    p_m12: &mut f64,
    p_m21: &mut f64,
    p_s12: &mut f64,
) -> f64 {
    let mut l = GeodGeodesicline::default();
    let outmask: u32 =
        (if *plat2 != 0. { GeodMask::GeodLatitude as u32 } else { GeodMask::GeodNone as u32 })
            | (if *plon2 != 0. {
                GeodMask::GeodLongitude as u32
            } else {
                GeodMask::GeodNone as u32
            })
            | (if *pazi2 != 0. { GeodMask::GeodAzimuth as u32 } else { GeodMask::GeodNone as u32 })
            | (if *ps12 != 0. { GeodMask::GeodDistance as u32 } else { GeodMask::GeodNone as u32 })
            | (if *pm12 != 0. {
                GeodMask::GeodReducedlength as u32
            } else {
                GeodMask::GeodNone as u32
            })
            | (if *p_m12 != 0. || *p_m21 != 0. {
                GeodMask::GeodGeodesicScale as u32
            } else {
                GeodMask::GeodNone as u32
            })
            | (if *p_s12 != 0. { GeodMask::GeodArea as u32 } else { GeodMask::GeodNone as u32 });

    geod_lineinit(
        &mut l,
        g,
        lat1,
        lon1,
        azi1,
        // Automatically supply GeodMask::GeodDistance as u32_IN if necessary
        outmask
            | (if flags & (GeodFlags::GeodArcmode as u32) != 0 {
                GeodMask::GeodNone as u32
            } else {
                GeodMask::GeodDistanceIn as u32
            }),
    );
    geod_genposition(&l, flags, s12_a12, plat2, plon2, pazi2, ps12, pm12, p_m12, p_m21, p_s12)
}

/// Initialize a geodesic line
pub fn geod_lineinit(
    l: &mut GeodGeodesicline,
    g: &GeodGeodesic,
    lat1: f64,
    lon1: f64,
    mut azi1: f64,
    caps: u32,
) {
    //   double salp1, calp1;
    let mut salp1 = 0.0;
    let mut calp1 = 0.0;
    azi1 = ang_normalize(azi1);
    // Guard against underflow in salp0
    sincosdx(ang_round(azi1), &mut salp1, &mut calp1);
    geod_lineinit_int(l, g, lat1, lon1, azi1, salp1, calp1, caps);
}

/// Compute the geodesic directly between two points
#[allow(clippy::too_many_arguments)]
pub fn geod_direct(
    g: &mut GeodGeodesic,
    lat1: f64,
    lon1: f64,
    azi1: f64,
    s12: f64,
    plat2: &mut f64,
    plon2: &mut f64,
    pazi2: &mut f64,
) {
    geod_gendirect(
        g,
        lat1,
        lon1,
        azi1,
        GeodFlags::GeodNoflags as u32,
        s12,
        plat2,
        plon2,
        pazi2,
        &mut 0.0,
        &mut 0.0,
        &mut 0.0,
        &mut 0.0,
        &mut 0.0,
    );
}

/// The scale factor A3 = mean value of (d/dsigma)I3
fn a3coeff(g: &mut GeodGeodesic) {
    let coeff: [f64; 18] = [
        // A3, coeff of eps^5, polynomial in n of order 0
        -3., 128., // A3, coeff of eps^4, polynomial in n of order 1
        -2., -3., 64., // A3, coeff of eps^3, polynomial in n of order 2
        -1., -3., -1., 16., // A3, coeff of eps^2, polynomial in n of order 2
        3., -1., -2., 8., // A3, coeff of eps^1, polynomial in n of order 1
        1., -1., 2., // A3, coeff of eps^0, polynomial in n of order 0
        1., 1.,
    ];
    let mut o = 0;
    // coeff of eps^j
    for (k, j) in (0..N_A3).rev().enumerate() {
        let m = if N_A3 - j - 1 < j { N_A3 - j - 1 } else { j }; /* order of polynomial in n */
        g.a3x[k] = polyvalx(m, &coeff[o..], g.n) / coeff[o + m + 1];
        o += m + 2;
    }
}

/// The coefficients C3[l] in the Fourier expansion of B3
fn c3coeff(g: &mut GeodGeodesic) {
    let coeff: [f64; 45] = [
        // C3[1], coeff of eps^5, polynomial in n of order 0
        3., 128., // C3[1], coeff of eps^4, polynomial in n of order 1
        2., 5., 128., // C3[1], coeff of eps^3, polynomial in n of order 2
        -1., 3., 3., 64., // C3[1], coeff of eps^2, polynomial in n of order 2
        -1., 0., 1., 8., // C3[1], coeff of eps^1, polynomial in n of order 1
        -1., 1., 4., // C3[2], coeff of eps^5, polynomial in n of order 0
        5., 256., // C3[2], coeff of eps^4, polynomial in n of order 1
        1., 3., 128., // C3[2], coeff of eps^3, polynomial in n of order 2
        -3., -2., 3., 64., // C3[2], coeff of eps^2, polynomial in n of order 2
        1., -3., 2., 32., // C3[3], coeff of eps^5, polynomial in n of order 0
        7., 512., // C3[3], coeff of eps^4, polynomial in n of order 1
        -10., 9., 384., // C3[3], coeff of eps^3, polynomial in n of order 2
        5., -9., 5., 192., // C3[4], coeff of eps^5, polynomial in n of order 0
        7., 512., // C3[4], coeff of eps^4, polynomial in n of order 1
        -14., 7., 512., // C3[5], coeff of eps^5, polynomial in n of order 0
        21., 2560.,
    ];
    let mut o = 0;
    let mut k = 0;
    // l is index of C3[l]
    for l in 1..N_C3 {
        // coeff of eps^j
        for j in (l..N_C3).rev() {
            let m = if N_C3 - j - 1 < j { N_C3 - j - 1 } else { j }; /* order of polynomial in n */
            g.c3x[k] = polyvalx(m, &coeff[o..], g.n) / coeff[o + m + 1];
            k += 1;
            o += m + 2;
        }
    }
}

/// The coefficients C4[l] in the Fourier expansion of I4
fn c4coeff(g: &mut GeodGeodesic) {
    let coeff: [f64; 77] = [
        // C4[0], coeff of eps^5, polynomial in n of order 0
        97., 15015., // C4[0], coeff of eps^4, polynomial in n of order 1
        1088., 156., 45045., // C4[0], coeff of eps^3, polynomial in n of order 2
        -224., -4784., 1573., 45045.,
        // C4[0], coeff of eps^2, polynomial in n of order 3
        -10656., 14144., -4576., -858., 45045.,
        // C4[0], coeff of eps^1, polynomial in n of order 4
        64., 624., -4576., 6864., -3003., 15015.,
        // C4[0], coeff of eps^0, polynomial in n of order 5
        100., 208., 572., 3432., -12012., 30030., 45045.,
        // C4[1], coeff of eps^5, polynomial in n of order 0
        1., 9009., // C4[1], coeff of eps^4, polynomial in n of order 1
        -2944., 468., 135135., // C4[1], coeff of eps^3, polynomial in n of order 2
        5792., 1040., -1287., 135135.,
        // C4[1], coeff of eps^2, polynomial in n of order 3
        5952., -11648., 9152., -2574., 135135.,
        // C4[1], coeff of eps^1, polynomial in n of order 4
        -64., -624., 4576., -6864., 3003., 135135.,
        // C4[2], coeff of eps^5, polynomial in n of order 0
        8., 10725., // C4[2], coeff of eps^4, polynomial in n of order 1
        1856., -936., 225225., // C4[2], coeff of eps^3, polynomial in n of order 2
        -8448., 4992., -1144., 225225.,
        // C4[2], coeff of eps^2, polynomial in n of order 3
        -1440., 4160., -4576., 1716., 225225.,
        // C4[3], coeff of eps^5, polynomial in n of order 0
        -136., 63063., // C4[3], coeff of eps^4, polynomial in n of order 1
        1024., -208., 105105., // C4[3], coeff of eps^3, polynomial in n of order 2
        3584., -3328., 1144., 315315.,
        // C4[4], coeff of eps^5, polynomial in n of order 0
        -128., 135135., // C4[4], coeff of eps^4, polynomial in n of order 1
        -2560., 832., 405405., // C4[5], coeff of eps^5, polynomial in n of order 0
        128., 99099.,
    ];
    let mut o = 0;
    let mut k = 0;
    // l is index of C4[l]
    for l in 0..N_C4 {
        // coeff of eps^j
        for j in (l..N_C4).rev() {
            let m = N_C4 - j - 1; // order of polynomial in n
            g.c4x[k] = polyvalx(m, &coeff[o..], g.n) / coeff[o + m + 1];
            k += 1;
            o += m + 2;
        }
    }
}

/// Evaluation sum(p[i] * x^i, i, 0, N) via Horner's method.
pub fn polyvalx(n: usize, p: &[f64], x: f64) -> f64 {
    if n == 0 {
        return 0.0;
    }
    let mut y = p[0];
    for val in p.iter().take(n).skip(1) {
        y = y * x + val;
    }

    y
}

/// Normalize an angle in degrees to the range [-180, 180]
pub fn ang_normalize(x: f64) -> f64 {
    let y = x % TD;
    if fabs(y) == HD {
        return copysign(HD, x);
    }
    y
}

/// Round an angle in degrees to the nearest integer
pub fn ang_round(x: f64) -> f64 {
    // False positive in cppcheck requires "1.0" instead of "1"
    let z = 1.0 / 16.0;
    let mut y = fabs(x);
    let w = z - y;
    // The compiler mustn't "simplify" z - (z - y) to y
    y = if w > 0. { z - w } else { y };
    copysign(y, x)
}

/// Compute the difference between two angles in degrees
pub fn ang_diff(x: f64, y: f64, e: &mut f64) -> f64 {
    // Use remainder instead of AngNormalize, since we treat boundary cases
    // later taking account of the error
    let mut t = 0.;
    let mut d = sumx(remainder(-x, TD), remainder(y, TD), &mut t);
    // This second sum can only change d if abs(d) < 128, so don't need to
    // apply remainder yet again.
    d = sumx(remainder(d, TD), t, &mut t);
    // Fix the sign if d = -180, 0, 180.
    if d == 0. || fabs(d) == HD {
        // If t == 0, take sign from y - x
        // else (t != 0, implies d = +/-180), d and t must have opposite signs
        d = copysign(d, if t == 0. { y - x } else { -t });
    }
    if *e != 0. {
        *e = t;
    }
    d
}

/// In order to minimize round-off errors, this function exactly reduces
/// the argument to the range [-45, 45] before converting it to radians.
pub fn sincosdx(x: f64, sinx: &mut f64, cosx: &mut f64) {
    let mut r = remquo(x, QD);
    r.0 *= DEGREE;
    // Possibly could call the gnu extension sincos
    let s = sin(r.0);
    let c = cos(r.0);
    match r.1 & 3 {
        0 => {
            *sinx = s;
            *cosx = c;
        }
        1 => {
            *sinx = c;
            *cosx = -s;
        }
        2 => {
            *sinx = -s;
            *cosx = -c;
        }
        _ => {
            *sinx = -c;
            *cosx = s;
        }
    }
    // http://www.open-std.org/jtc1/sc22/wg14/www/docs/n1950.pdf
    *cosx += 0.; // special values from F.10.1.12
    // special values from F.10.1.13
    if *sinx == 0. {
        *sinx = copysign(*sinx, x);
    }
}

/// Limit x to the range [-qd, qd]
pub fn lat_fix(x: f64) -> f64 {
    if fabs(x) > QD { f64::NAN } else { x }
}

/// Swap x and y
pub fn swapx(x: &mut f64, y: &mut f64) {
    core::mem::swap(&mut (*x), &mut (*y));
}

/// Normalize sin(x) and cos(x)
pub fn norm2(sinx: &mut f64, cosx: &mut f64) {
    let r = hypot(*sinx, *cosx);
    *sinx /= r;
    *cosx /= r;
}

/// Evaluate
/// y = sinp ? sum(c[i] * sin( 2*i    * x), i, 1, n) :
///            sum(c[i] * cos((2*i+1) * x), i, 0, n-1)
/// using Clenshaw summation.  N.B. c[0] is unused for sin series
/// Approx operation count = (n + 5) mult and (2 * n + 2) add */
pub fn sin_cos_series(sinp: bool, sinx: f64, cosx: f64, c: &[f64], mut n: usize) -> f64 {
    // Point to one beyond last element
    let mut c_index = if sinp { 1 } else { 0 } + n - 1;
    let ar = 2. * (cosx - sinx) * (cosx + sinx); /* 2 * cos(2 * x) */
    c_index -= 1;
    let mut y0 = if n & 1 != 0 { c[c_index] } else { 0. };
    let mut y1 = 0.;
    // Now n is even
    n /= 2;
    for _ in 0..n {
        // Unroll loop x 2, so accumulators return to their original role */
        c_index -= 1;
        y1 = ar * y0 - y1 + c[c_index];
        c_index -= 1;
        y0 = ar * y1 - y0 + c[c_index];
    }

    if sinp
    { 2. * sinx * cosx * y0 }      /* sin(2 * x) * y0 */
    else { cosx * (y0 - y1) } /* cos(x) * (y0 - y1) */
}

/// The scale factor A1-1 = mean value of (d/dsigma)I1 - 1
pub fn a1m1f(eps: f64) -> f64 {
    // (1-eps)*A1-1, polynomial in eps2 of order 3
    let coeff: [f64; 5] = [1., 4., 64., 0., 256.];
    let m = N_A1 / 2;
    let t = polyvalx(m, &coeff, sq(eps)) / coeff[m + 1];
    (t + eps) / (1. - eps)
}

/// In order to minimize round-off errors, this function rearranges the
/// arguments so that result of atan2 is in the range [-pi/4, pi/4] before
/// converting it to degrees and mapping the result to the correct
/// quadrant.
pub fn atan2dx(mut y: f64, mut x: f64) -> f64 {
    let mut q = 0;
    if fabs(y) > fabs(x) {
        swapx(&mut x, &mut y);
        q = 2;
    }
    if x.is_sign_negative() {
        x = -x;
        q += 1;
    }
    // here x >= 0 and x >= abs(y), so angle is in [-pi/4, pi/4]
    let mut ang = atan2(y, x) / DEGREE;
    match q {
        1 => {
            ang = copysign(HD, y) - ang;
        }
        2 => {
            ang = QD - ang;
        }
        3 => {
            ang += -QD;
        }
        _ => {}
    }
    ang
}

/// Compute the geodesic inverse between two points
#[allow(clippy::too_many_arguments)]
pub fn geod_geninverse(
    geod_geodesic: &mut GeodGeodesic,
    lat1: f64,
    lon1: f64,
    lat2: f64,
    lon2: f64,
    ps12: &mut f64,
    pazi1: &mut f64,
    pazi2: &mut f64,
    pm12: &mut f64,
    p_m12: &mut f64,
    p_m21: &mut f64,
    p_s12: &mut f64,
) -> f64 {
    let mut salp1 = 0.0;
    let mut calp1 = 0.0;
    let mut salp2 = 0.0;
    let mut calp2 = 0.0;
    let a12 = geod_geninverse_int(
        geod_geodesic,
        lat1,
        lon1,
        lat2,
        lon2,
        ps12,
        &mut salp1,
        &mut calp1,
        &mut salp2,
        &mut calp2,
        pm12,
        p_m12,
        p_m21,
        p_s12,
    );
    if *pazi1 != 0. {
        *pazi1 = atan2dx(salp1, calp1);
    }
    if *pazi2 != 0. {
        *pazi2 = atan2dx(salp2, calp2);
    }
    a12
}

/// Compute the geodesic inverse between two points
#[allow(clippy::too_many_arguments)]
pub fn geod_geninverse_int(
    g: &mut GeodGeodesic,
    mut lat1: f64,
    lon1: f64,
    mut lat2: f64,
    lon2: f64,
    ps12: &mut f64,
    psalp1: &mut f64,
    pcalp1: &mut f64,
    psalp2: &mut f64,
    pcalp2: &mut f64,
    pm12: &mut f64,
    p_m12: &mut f64,
    p_m21: &mut f64,
    p_s12: &mut f64,
) -> f64 {
    //   double s12 = 0, m12 = 0, M12 = 0, M21 = 0, S12 = 0;
    let mut s12 = 0.0;
    let mut m12 = 0.0;
    let mut _m12: f64 = 0.0;
    let mut _m21 = 0.0;
    let mut _s12 = 0.0;
    //   double lon12, lon12s;
    let mut lon12s = 0.0;
    //   int latsign, lonsign, swapp;
    //   double sbet1, cbet1, sbet2, cbet2, s12x = 0, m12x = 0;
    let mut sbet1 = 0.0;
    let mut cbet1 = 0.0;
    let mut sbet2 = 0.0;
    let mut cbet2 = 0.0;
    let mut s12x = 0.0;
    let mut m12x = 0.0;
    //   double dn1, dn2, lam12, slam12, clam12;
    let mut slam12 = 0.0;
    let mut clam12 = 0.0;
    //   double a12 = 0, sig12, calp1 = 0, salp1 = 0, calp2 = 0, salp2 = 0;
    let mut a12 = 0.0;
    let mut sig12;
    let mut calp1 = 0.0;
    let mut salp1 = 0.0;
    let mut calp2 = 0.0;
    let mut salp2 = 0.0;
    //   double Ca[nC];
    let mut ca = [0.0; N_C];
    //   boolx meridian;
    // somg12 == 2 marks that it needs to be calculated
    let mut omg12 = 0.;
    let mut somg12 = 2.;
    let mut comg12 = 0.;

    let mut outmask: u32 =
        (if *ps12 != 0. { GeodMask::GeodDistance as u32 } else { GeodMask::GeodNone as u32 })
            | (if *pm12 != 0. {
                GeodMask::GeodReducedlength as u32
            } else {
                GeodMask::GeodNone as u32
            })
            | (if *p_m12 != 0. || *p_m21 != 0. {
                GeodMask::GeodGeodesicScale as u32
            } else {
                GeodMask::GeodNone as u32
            })
            | (if *p_s12 != 0. {
                GeodFlags::GeodLongUnroll as u32
            } else {
                GeodMask::GeodNone as u32
            });

    outmask &= CapType::OutAll as u32;
    // Compute longitude difference (ang_diff does this carefully).  Result is
    // in [-180, 180] but -180 is only for west-going geodesics.  180 is for
    // east-going and meridional geodesics.
    let mut lon12 = ang_diff(lon1, lon2, &mut lon12s);
    // Make longitude difference positive.
    let mut lonsign = if lon12.is_sign_positive() { -1. } else { 1. };
    lon12 *= lonsign;
    lon12s *= lonsign;
    let lam12 = lon12 * DEGREE;
    // Calculate sincos of lon12 + error (this applies ang_round internally).
    sincosde(lon12, lon12s, &mut slam12, &mut clam12);
    lon12s = (HD - lon12) - lon12s; /* the supplementary longitude difference */

    // If really close to the equator, treat as on equator.
    lat1 = ang_round(lat_fix(lat1));
    lat2 = ang_round(lat_fix(lat2));
    // Swap points so that point with higher (abs) latitude is point 1
    // If one latitude is a nan, then it becomes lat1.
    let swapp = if fabs(lat1) < fabs(lat2) { -1. } else { 1. };
    if swapp < 0. {
        lonsign *= -1.;
        swapx(&mut lat1, &mut lat2);
    }
    // Make lat1 <= -0
    let latsign = if lat1.is_sign_positive() { 1. } else { -1. };
    lat1 *= latsign;
    lat2 *= latsign;
    // Now we have
    //
    //     0 <= lon12 <= 180
    //     -90 <= lat1 <= -0
    //     lat1 <= lat2 <= -lat1
    //
    // longsign, swapp, latsign register the transformation to bring the
    // coordinates to this canonical form.  In all cases, 1 means no change was
    // made.  We make these transformations so that there are few cases to
    // check, e.g., on verifying quadrants in atan2.  In addition, this
    // enforces some symmetries in the results returned.

    sincosdx(lat1, &mut sbet1, &mut cbet1);
    sbet1 *= g.f1;
    // Ensure cbet1 = +epsilon at poles
    norm2(&mut sbet1, &mut cbet1);
    cbet1 = fmax(TINY, cbet1);

    sincosdx(lat2, &mut sbet2, &mut cbet2);
    sbet2 *= g.f1;
    // Ensure cbet2 = +epsilon at poles
    norm2(&mut sbet2, &mut cbet2);
    cbet2 = fmax(TINY, cbet2);

    // If cbet1 < -sbet1, then cbet2 - cbet1 is a sensitive measure of the
    // |bet1| - |bet2|.  Alternatively (cbet1 >= -sbet1), abs(sbet2) + sbet1 is
    // a better measure.  This logic is used in assigning calp2 in Lambda12.
    // Sometimes these quantities vanish and in that case we force bet2 = +/-
    // bet1 exactly.  An example where is is necessary is the inverse problem
    // 48.522876735459 0 -48.52287673545898293 179.599720456223079643
    // which failed with Visual Studio 10 (Release and Debug)

    if cbet1 < -sbet1 {
        if cbet2 == cbet1 {
            sbet2 = copysign(sbet1, sbet2);
        }
    } else if fabs(sbet2) == -sbet1 {
        cbet2 = cbet1;
    }

    let dn1 = sqrt(1. + g.ep2 * sq(sbet1));
    let dn2 = sqrt(1. + g.ep2 * sq(sbet2));

    let mut meridian = lat1 == -QD || slam12 == 0.;

    if meridian {
        // Endpoints are on a single full meridian, so the geodesic might lie on
        // a meridian.

        calp1 = clam12;
        salp1 = slam12; /* Head to the target longitude */
        calp2 = 1.;
        salp2 = 0.; /* At the target we're heading north */
        // tan(bet) = tan(sig) * cos(alp)
        let ssig1 = sbet1;
        let csig1 = calp1 * cbet1;
        let ssig2 = sbet2;
        let csig2 = calp2 * cbet2;

        // sig12 = sig2 - sig1
        sig12 = atan2(fmax(0.0, csig1 * ssig2 - ssig1 * csig2) + 0., csig1 * csig2 + ssig1 * ssig2);
        let mut _tmp_m12 = 0.;
        let mut _tmp_m21 = 0.;
        lengths(
            g,
            g.n,
            sig12,
            ssig1,
            csig1,
            dn1,
            ssig2,
            csig2,
            dn2,
            cbet1,
            cbet2,
            &mut s12x,
            &mut m12x,
            &mut 0.,
            if (outmask & GeodMask::GeodGeodesicScale as u32) != 0 {
                &mut _m12
            } else {
                &mut _tmp_m12
            },
            if (outmask & GeodMask::GeodGeodesicScale as u32) != 0 {
                &mut _m21
            } else {
                &mut _tmp_m21
            },
            &mut ca,
        );
        // Add the check for sig12 since zero length geodesics might yield m12 <
        // 0.  Test case was
        //
        //    echo 20.001 0 20.001 0 | GeodSolve -i
        //
        // In fact, we will have sig12 > pi/2 for meridional geodesic which is
        // not a shortest path.
        if sig12 < 1. || m12x >= 0. {
            // Need at least 2, to handle 90 0 90 180
            if sig12 < 3. * TINY ||
              /* Prevent negative s12 or m12 for short lines */
              (sig12 < TOL0 && (s12x < 0. || m12x < 0.))
            {
                s12x = 0.;
                m12x = s12x;
                sig12 = m12x;
            }
            m12x *= g.b;
            s12x *= g.b;
            a12 = sig12 / DEGREE;
        } else {
            // m12 < 0, i.e., prolate and too close to anti-podal
            meridian = false;
        }
    }

    if !meridian &&
          sbet1 == 0. &&           /* and sbet2 == 0 */
          /* Mimic the way Lambda12 works with calp1 = 0 */
          (g.f <= 0. || lon12s >= g.f * HD)
    {
        // Geodesic runs along equator
        calp2 = 0.;
        calp1 = calp2;
        salp2 = 1.;
        salp1 = salp2;
        s12x = g.a * lam12;
        omg12 = lam12 / g.f1;
        sig12 = omg12;
        m12x = g.b * sin(sig12);
        if (outmask & GeodMask::GeodGeodesicScale as u32) != 0 {
            _m21 = cos(sig12);
            _m12 = _m21;
        }
        a12 = lon12 / g.f1;
    } else if !meridian {
        // Now point1 and point2 belong within a hemisphere bounded by a
        // meridian and geodesic is neither meridional or equatorial.

        // Figure a starting point for Newton's method
        let mut dnm = 0.;
        sig12 = inverse_start(
            g, sbet1, cbet1, dn1, sbet2, cbet2, dn2, lam12, slam12, clam12, &mut salp1, &mut calp1,
            &mut salp2, &mut calp2, &mut dnm, &mut ca,
        );

        if sig12 >= 0. {
            // Short lines (inverse_start sets salp2, calp2, dnm)
            s12x = sig12 * g.b * dnm;
            m12x = sq(dnm) * g.b * sin(sig12 / dnm);
            if (outmask & GeodMask::GeodGeodesicScale as u32) != 0 {
                _m21 = cos(sig12 / dnm);
                _m12 = _m21;
            }
            a12 = sig12 / DEGREE;
            omg12 = lam12 / (g.f1 * dnm);
        } else {
            // Newton's method.  This is a straightforward solution of f(alp1) =
            // lambda12(alp1) - lam12 = 0 with one wrinkle.  f(alp) has exactly one
            // root in the interval (0, pi) and its derivative is positive at the
            // root.  Thus f(alp) is positive for alp > alp1 and negative for alp <
            // alp1.  During the course of the iteration, a range (alp1a, alp1b) is
            // maintained which brackets the root and with each evaluation of
            // f(alp) the range is shrunk, if possible.  Newton's method is
            // restarted whenever the derivative of f is negative (because the new
            // value of alp1 is then further from the solution) or if the new
            // estimate of alp1 lies outside (0,pi); in this case, the new starting
            // guess is taken to be (alp1a + alp1b) / 2.
            let mut ssig1 = 0.;
            let mut csig1 = 0.;
            let mut ssig2 = 0.;
            let mut csig2 = 0.;
            let mut eps = 0.;
            let mut domg12 = 0.;
            let mut numit: u32 = 0;
            // Bracketing range
            let mut salp1a = TINY;
            let mut calp1a = 1.;
            let mut salp1b = TINY;
            let mut calp1b = -1.;
            let mut tripn = false;
            let mut tripb = false;
            //   for (;; ++numit) {
            loop {
                numit += 1;
                // the WGS84 test set: mean = 1.47, sd = 1.25, max = 16
                // WGS84 and random input: mean = 2.85, sd = 0.60
                let mut dv = 0.;
                let v = lambda12(
                    g,
                    sbet1,
                    cbet1,
                    dn1,
                    sbet2,
                    cbet2,
                    dn2,
                    salp1,
                    calp1,
                    slam12,
                    clam12,
                    &mut salp2,
                    &mut calp2,
                    &mut sig12,
                    &mut ssig1,
                    &mut csig1,
                    &mut ssig2,
                    &mut csig2,
                    &mut eps,
                    &mut domg12,
                    numit < MAXIT1,
                    &mut dv,
                    &mut ca,
                );
                if tripb ||
                /* Reversed test to allow escape with NaNs */
                (fabs(v) < (if tripn { 8. } else { 1. }) * TOL0) ||
                /* Enough bisections to get accurate result */
                numit == MAXIT2
                {
                    break;
                }
                // Update bracketing values
                if v > 0. && (numit > MAXIT1 || calp1 / salp1 > calp1b / salp1b) {
                    salp1b = salp1;
                    calp1b = calp1;
                } else if v < 0. && (numit > MAXIT1 || calp1 / salp1 < calp1a / salp1a) {
                    salp1a = salp1;
                    calp1a = calp1;
                }
                if numit < MAXIT1 && dv > 0. {
                    let dalp1 = -v / dv;
                    if fabs(dalp1) < PI {
                        let sdalp1 = sin(dalp1);
                        let cdalp1 = cos(dalp1);
                        let nsalp1 = salp1 * cdalp1 + calp1 * sdalp1;
                        if nsalp1 > 0. {
                            calp1 = calp1 * cdalp1 - salp1 * sdalp1;
                            salp1 = nsalp1;
                            norm2(&mut salp1, &mut calp1);
                            // In some regimes we don't get quadratic convergence because
                            // slope -> 0.  So use convergence conditions based on epsilon
                            // instead of sqrt(epsilon).
                            tripn = fabs(v) <= 16. * TOL0;
                            continue;
                        }
                    }
                }
                // Either dv was not positive or updated value was outside legal
                // range.  Use the midpoint of the bracket as the next estimate.
                // This mechanism is not needed for the WGS84 ellipsoid, but it does
                // catch problems with more eccentric ellipsoids.  Its efficacy is
                // such for the WGS84 test set with the starting guess set to alp1 =
                // 90deg:
                // the WGS84 test set: mean = 5.21, sd = 3.93, max = 24
                // WGS84 and random input: mean = 4.74, sd = 0.99
                salp1 = (salp1a + salp1b) / 2.;
                calp1 = (calp1a + calp1b) / 2.;
                norm2(&mut salp1, &mut calp1);
                tripn = false;
                tripb = fabs(salp1a - salp1) + (calp1a - calp1) < TOLB
                    || fabs(salp1 - salp1b) + (calp1 - calp1b) < TOLB;
            }
            let mut _tmp_m12 = 0.;
            let mut _tmp_m21 = 0.;
            lengths(
                g,
                eps,
                sig12,
                ssig1,
                csig1,
                dn1,
                ssig2,
                csig2,
                dn2,
                cbet1,
                cbet2,
                &mut s12x,
                &mut m12x,
                &mut 0.,
                if (outmask & GeodMask::GeodGeodesicScale as u32) != 0 {
                    &mut _m12
                } else {
                    &mut _tmp_m12
                },
                if (outmask & GeodMask::GeodGeodesicScale as u32) != 0 {
                    &mut _m21
                } else {
                    &mut _tmp_m21
                },
                &mut ca,
            );
            m12x *= g.b;
            s12x *= g.b;
            a12 = sig12 / DEGREE;
            if (outmask & GeodFlags::GeodLongUnroll as u32) != 0 {
                // omg12 = lam12 - domg12
                let sdomg12 = sin(domg12);
                let cdomg12 = cos(domg12);
                somg12 = slam12 * cdomg12 - clam12 * sdomg12;
                comg12 = clam12 * cdomg12 + slam12 * sdomg12;
            }
        }
    }

    if (outmask & GeodMask::GeodDistance as u32) != 0 {
        s12 = 0. + s12x; /* Convert -0 to 0 */
    }

    if (outmask & GeodMask::GeodReducedlength as u32) != 0 {
        m12 = 0. + m12x; /* Convert -0 to 0 */
    }

    if (outmask & GeodFlags::GeodLongUnroll as u32) != 0 {
        let
          /* From Lambda12: sin(alp1) * cos(bet1) = sin(alp0) */
          salp0 = salp1 * cbet1;
        let calp0 = hypot(calp1, salp1 * sbet1); /* calp0 > 0 */
        let alp12;
        if calp0 != 0. && salp0 != 0. {
            // From Lambda12: tan(bet) = tan(sig) * cos(alp)
            let mut ssig1 = sbet1;
            let mut csig1 = calp1 * cbet1;
            let mut ssig2 = sbet2;
            let mut csig2 = calp2 * cbet2;
            let k2 = sq(calp0) * g.ep2;
            let eps = k2 / (2. * (1. + sqrt(1. + k2)) + k2);
            // Multiplier = a^2 * e^2 * cos(alpha0) * sin(alpha0).
            let a4 = sq(g.a) * calp0 * salp0 * g.e2;

            norm2(&mut ssig1, &mut csig1);
            norm2(&mut ssig2, &mut csig2);
            c4f(g, eps, &mut ca);
            let b41 = sin_cos_series(false, ssig1, csig1, &ca, N_C4);
            let b42 = sin_cos_series(false, ssig2, csig2, &ca, N_C4);
            _s12 = a4 * (b42 - b41);
        } else {
            // Avoid problems with indeterminate sig1, sig2 on equator
            _s12 = 0.;
        }

        if !meridian && somg12 == 2. {
            somg12 = sin(omg12);
            comg12 = cos(omg12);
        }

        if !meridian &&
            /* omg12 < 3/4 * pi */
            comg12 > FRAC_1_SQRT_2 &&     /* Long difference not too big */
            sbet2 - sbet1 < 1.75
        {
            /* Lat difference not too big */
            /* Use tan(Gamma/2) = tan(omg12/2)
             * * (tan(bet1/2)+tan(bet2/2))/(1+tan(bet1/2)*tan(bet2/2))
             * with tan(x/2) = sin(x)/(1+cos(x)) */
            let domg12 = 1. + comg12;
            let dbet1 = 1. + cbet1;
            let dbet2 = 1. + cbet2;
            alp12 = 2.
                * atan2(
                    somg12 * (sbet1 * dbet2 + sbet2 * dbet1),
                    domg12 * (sbet1 * sbet2 + dbet1 * dbet2),
                );
        } else {
            /* alp12 = alp2 - alp1, used in atan2 so no need to normalize */
            let mut salp12 = salp2 * calp1 - calp2 * salp1;
            let mut calp12 = calp2 * calp1 + salp2 * salp1;
            /* The right thing appears to happen if alp1 = +/-180 and alp2 = 0, viz
             * salp12 = -0 and alp12 = -180.  However this depends on the sign
             * being attached to 0 correctly.  The following ensures the correct
             * behavior. */
            if salp12 == 0. && calp12 < 0. {
                salp12 = TINY * calp1;
                calp12 = -1.;
            }
            alp12 = atan2(salp12, calp12);
        }
        _s12 += g.c2 * alp12;
        _s12 *= swapp * lonsign * latsign;
        // Convert -0 to 0
        _s12 += 0.;
    }

    // Convert calp, salp to azimuth accounting for lonsign, swapp, latsign.
    if swapp < 0. {
        swapx(&mut salp1, &mut salp2);
        swapx(&mut calp1, &mut calp2);
        if (outmask & GeodMask::GeodGeodesicScale as u32) != 0 {
            swapx(&mut _m12, &mut _m21);
        }
    }

    salp1 *= swapp * lonsign;
    calp1 *= swapp * latsign;
    salp2 *= swapp * lonsign;
    calp2 *= swapp * latsign;

    if *psalp1 != 0. {
        *psalp1 = salp1;
    }
    if *pcalp1 != 0. {
        *pcalp1 = calp1;
    }
    if *psalp2 != 0. {
        *psalp2 = salp2;
    }
    if *pcalp2 != 0. {
        *pcalp2 = calp2;
    }

    if (outmask & GeodMask::GeodDistance as u32) != 0 {
        *ps12 = s12;
    }
    if (outmask & GeodMask::GeodReducedlength as u32) != 0 {
        *pm12 = m12;
    }
    if (outmask & GeodMask::GeodGeodesicScale as u32) != 0 {
        if *p_m12 != 0. {
            *p_m12 = _m12;
        }
        if *p_m21 != 0. {
            *p_m21 = _m21;
        }
    }
    if (outmask & GeodFlags::GeodLongUnroll as u32) != 0 {
        *p_s12 = _s12;
    }

    // Returned value in [0, 180]
    a12
}

/// Compute the geodesic position between two points
#[allow(clippy::too_many_arguments)]
pub fn geod_genposition(
    l: &GeodGeodesicline,
    flags: u32,
    s12_a12: f64,
    plat2: &mut f64,
    plon2: &mut f64,
    pazi2: &mut f64,
    ps12: &mut f64,
    pm12: &mut f64,
    p_m12: &mut f64,
    p_m21: &mut f64,
    p_s12: &mut f64,
) -> f64 {
    let mut lat2 = 0.0;
    let mut lon2 = 0.0;
    let mut azi2 = 0.0;
    let mut s12 = 0.0;
    let mut m12 = 0.0;
    let mut _m12 = 0.0;
    let mut _m21 = 0.0;
    let mut _s12 = 0.0;
    // Avoid warning about uninitialized B12.
    let mut sig12;
    let mut ssig12 = 0.;
    let mut csig12 = 0.;
    let mut b12 = 0.;
    let mut ab1 = 0.;
    let omg12;
    let lam12;
    let lon12;
    let mut ssig2;
    let mut csig2;
    let mut cbet2;
    let somg2;
    let comg2;

    let mut outmask: u32 =
        (if *plat2 != 0. { GeodMask::GeodLatitude as u32 } else { GeodMask::GeodNone as u32 })
            | (if *plon2 != 0. {
                GeodMask::GeodLongitude as u32
            } else {
                GeodMask::GeodNone as u32
            })
            | (if *pazi2 != 0. { GeodMask::GeodAzimuth as u32 } else { GeodMask::GeodNone as u32 })
            | (if *ps12 != 0. { GeodMask::GeodDistance as u32 } else { GeodMask::GeodNone as u32 })
            | (if *pm12 != 0. {
                GeodMask::GeodReducedlength as u32
            } else {
                GeodMask::GeodNone as u32
            })
            | (if *p_m12 != 0. || *p_m21 != 0. {
                GeodMask::GeodGeodesicScale as u32
            } else {
                GeodMask::GeodNone as u32
            })
            | (if *p_s12 != 0. { GeodMask::GeodArea as u32 } else { GeodMask::GeodNone as u32 });

    outmask &= l.caps & CapType::OutAll as u32;
    if !(((flags & GeodFlags::GeodArcmode as u32) != 0)
        || (l.caps & (GeodMask::GeodDistanceIn as u32 & CapType::OutAll as u32)) != 0)
    {
        // Impossible distance calculation requested
        return f64::NAN;
    }

    if (flags & GeodFlags::GeodArcmode as u32) != 0 {
        // Interpret s12_a12 as spherical arc length
        sig12 = s12_a12 * DEGREE;
        sincosdx(s12_a12, &mut ssig12, &mut csig12);
    } else {
        // Interpret s12_a12 as distance
        let tau12 = s12_a12 / (l.b * (1. + l.a1m1));
        let s = sin(tau12);
        let c = cos(tau12);
        // tau2 = tau1 + tau12
        b12 = -sin_cos_series(
            true,
            l.stau1 * c + l.ctau1 * s,
            l.ctau1 * c - l.stau1 * s,
            &l.c1pa,
            N_C1_P,
        );
        sig12 = tau12 - (b12 - l.b11);
        ssig12 = sin(sig12);
        csig12 = cos(sig12);
        if fabs(l.f) > 0.01 {
            // Reverted distance series is inaccurate for |f| > 1/100, so correct
            // sig12 with 1 Newton iteration.  The following table shows the
            // approximate maximum error for a = WGS_a() and various f relative to
            // GeodesicExact.
            //     erri = the error in the inverse solution (nm)
            //     errd = the error in the direct solution (series only) (nm)
            //     errda = the error in the direct solution (series + 1 Newton) (nm)
            //
            //       f     erri  errd errda
            //     -1/5    12e6 1.2e9  69e6
            //     -1/10  123e3  12e6 765e3
            //     -1/20   1110 108e3  7155
            //     -1/50  18.63 200.9 27.12
            //     -1/100 18.63 23.78 23.37
            //     -1/150 18.63 21.05 20.26
            //      1/150 22.35 24.73 25.83
            //      1/100 22.35 25.03 25.31
            //      1/50  29.80 231.9 30.44
            //      1/20   5376 146e3  10e3
            //      1/10  829e3  22e6 1.5e6
            //      1/5   157e6 3.8e9 280e6

            ssig2 = l.ssig1 * csig12 + l.csig1 * ssig12;
            csig2 = l.csig1 * csig12 - l.ssig1 * ssig12;
            b12 = sin_cos_series(true, ssig2, csig2, &l.c1a, N_C1);
            let serr = (1. + l.a1m1) * (sig12 + (b12 - l.b11)) - s12_a12 / l.b;
            sig12 -= serr / sqrt(1. + l.k2 * sq(ssig2));
            ssig12 = sin(sig12);
            csig12 = cos(sig12);
            // Update B12 below
        }
    }

    // sig2 = sig1 + sig12
    ssig2 = l.ssig1 * csig12 + l.csig1 * ssig12;
    csig2 = l.csig1 * csig12 - l.ssig1 * ssig12;
    let dn2 = sqrt(1. + l.k2 * sq(ssig2));
    if (outmask
        & (GeodMask::GeodDistance as u32
            | GeodMask::GeodReducedlength as u32
            | GeodMask::GeodGeodesicScale as u32))
        != 0
    {
        if (flags & GeodFlags::GeodArcmode as u32 != 0) || fabs(l.f) > 0.01 {
            b12 = sin_cos_series(true, ssig2, csig2, &l.c1a, N_C1);
        }
        ab1 = (1. + l.a1m1) * (b12 - l.b11);
    }
    // sin(bet2) = cos(alp0) * sin(sig2)
    let sbet2 = l.calp0 * ssig2;
    // Alt: cbet2 = hypot(csig2, salp0 * ssig2);
    cbet2 = hypot(l.salp0, l.calp0 * csig2);
    if cbet2 == 0. {
        // I.e., salp0 = 0, csig2 = 0.  Break the degeneracy in this case
        csig2 = TINY;
        cbet2 = csig2;
    }
    // tan(alp0) = cos(sig2)*tan(alp2)
    let salp2 = l.salp0;
    let calp2 = l.calp0 * csig2; /* No need to normalize */

    if (outmask & GeodMask::GeodDistance as u32) != 0 {
        s12 = if (flags & GeodFlags::GeodArcmode as u32) != 0 {
            l.b * ((1. + l.a1m1) * sig12 + ab1)
        } else {
            s12_a12
        };
    }

    if (outmask & GeodMask::GeodLongitude as u32) != 0 {
        let e = copysign(1., l.salp0); /* east or west going? */
        // tan(omg2) = sin(alp0) * tan(sig2)
        somg2 = l.salp0 * ssig2;
        comg2 = csig2; /* No need to normalize */
        // omg12 = omg2 - omg1
        omg12 = if (flags & GeodFlags::GeodLongUnroll as u32) != 0 {
            e * (sig12 - (atan2(ssig2, csig2) - atan2(l.ssig1, l.csig1))
                + (atan2(e * somg2, comg2) - atan2(e * l.somg1, l.comg1)))
        } else {
            atan2(somg2 * l.comg1 - comg2 * l.somg1, comg2 * l.comg1 + somg2 * l.somg1)
        };
        lam12 = omg12
            + l.a3c * (sig12 + (sin_cos_series(true, ssig2, csig2, &l.c3a, N_C3 - 1) - l.b31));
        lon12 = lam12 / DEGREE;
        lon2 = if (flags & GeodFlags::GeodLongUnroll as u32) != 0 {
            l.lon1 + lon12
        } else {
            ang_normalize(ang_normalize(l.lon1) + ang_normalize(lon12))
        };
    }

    if (outmask & GeodMask::GeodLatitude as u32) != 0 {
        lat2 = atan2dx(sbet2, l.f1 * cbet2);
    }

    if (outmask & GeodMask::GeodAzimuth as u32) != 0 {
        azi2 = atan2dx(salp2, calp2);
    }

    if (outmask & (GeodMask::GeodReducedlength as u32 | GeodMask::GeodGeodesicScale as u32)) != 0 {
        let b22 = sin_cos_series(true, ssig2, csig2, &l.c2a, N_C2);
        let ab2 = (1. + l.a2m1) * (b22 - l.b21);
        let j12 = (l.a1m1 - l.a2m1) * sig12 + (ab1 - ab2);
        if (outmask & GeodMask::GeodReducedlength as u32) != 0 {
            // Add parens around (csig1 * ssig2) and (ssig1 * csig2) to ensure
            // accurate cancellation in the case of coincident points.
            m12 = l.b
                * ((dn2 * (l.csig1 * ssig2) - l.dn1 * (l.ssig1 * csig2)) - l.csig1 * csig2 * j12);
        }
        if (outmask & GeodMask::GeodGeodesicScale as u32) != 0 {
            let t = l.k2 * (ssig2 - l.ssig1) * (ssig2 + l.ssig1) / (l.dn1 + dn2);
            _m12 = csig12 + (t * ssig2 - csig2 * j12) * l.ssig1 / l.dn1;
            _m21 = csig12 - (t * l.ssig1 - l.csig1 * j12) * ssig2 / dn2;
        }
    }

    if (outmask & GeodFlags::GeodLongUnroll as u32) != 0 {
        let b42 = sin_cos_series(false, ssig2, csig2, &l.c4a, N_C4);
        let salp12;
        let calp12;
        if l.calp0 == 0. || l.salp0 == 0. {
            // alp12 = alp2 - alp1, used in atan2 so no need to normalize
            salp12 = salp2 * l.calp1 - calp2 * l.salp1;
            calp12 = calp2 * l.calp1 + salp2 * l.salp1;
        } else {
            /* tan(alp) = tan(alp0) * sec(sig)
             * tan(alp2-alp1) = (tan(alp2) -tan(alp1)) / (tan(alp2)*tan(alp1)+1)
             * = calp0 * salp0 * (csig1-csig2) / (salp0^2 + calp0^2 * csig1*csig2)
             * If csig12 > 0, write
             *   csig1 - csig2 = ssig12 * (csig1 * ssig12 / (1 + csig12) + ssig1)
             * else
             *   csig1 - csig2 = csig1 * (1 - csig12) + ssig12 * ssig1
             * No need to normalize */
            salp12 = l.calp0
                * l.salp0
                * (if csig12 <= 0. {
                    l.csig1 * (1. - csig12) + ssig12 * l.ssig1
                } else {
                    ssig12 * (l.csig1 * ssig12 / (1. + csig12) + l.ssig1)
                });
            calp12 = sq(l.salp0) + sq(l.calp0) * l.csig1 * csig2;
        }
        _s12 = l.c2 * atan2(salp12, calp12) + l.a4 * (b42 - l.b41);
    }

    // In the pattern
    //
    //   if ((outmask & GEOD_XX) && pYY)
    //     *pYY = YY;
    //
    // the second check "&& pYY" is redundant.  It's there to make the CLang
    // static analyzer happy.
    if ((outmask & GeodMask::GeodLatitude as u32) != 0) && *plat2 != 0. {
        *plat2 = lat2;
    }
    if ((outmask & GeodMask::GeodLongitude as u32) != 0) && *plon2 != 0. {
        *plon2 = lon2;
    }
    if ((outmask & GeodMask::GeodAzimuth as u32) != 0) && *pazi2 != 0. {
        *pazi2 = azi2;
    }
    if (outmask & GeodMask::GeodDistance as u32 != 0) && *ps12 != 0. {
        *ps12 = s12;
    }
    if ((outmask & GeodMask::GeodReducedlength as u32) != 0) && *pm12 != 0. {
        *pm12 = m12;
    }
    if (outmask & GeodMask::GeodGeodesicScale as u32) != 0 {
        if *p_m12 != 0. {
            *p_m12 = _m12;
        }
        if *p_m21 != 0. {
            *p_m21 = _m21;
        }
    }
    if ((outmask & GeodFlags::GeodLongUnroll as u32) != 0) && *p_s12 != 0. {
        *p_s12 = _s12;
    }

    if (flags & GeodFlags::GeodArcmode as u32) != 0 { s12_a12 } else { sig12 / DEGREE }
}

/// Initialize a geodesic line
#[allow(clippy::too_many_arguments)]
pub fn geod_lineinit_int(
    l: &mut GeodGeodesicline,
    g: &GeodGeodesic,
    lat1: f64,
    lon1: f64,
    azi1: f64,
    salp1: f64,
    calp1: f64,
    caps: u32,
) {
    let mut cbet1 = 0.0;
    let mut sbet1 = 0.0;
    l.a = g.a;
    l.f = g.f;
    l.b = g.b;
    l.c2 = g.c2;
    l.f1 = g.f1;
    // If caps is 0 assume the standard direct calculation
    l.caps = (if caps != 0 { caps } else { GeodMask::GeodDistanceIn as u32 | GeodMask::GeodLongitude as u32 }) |
        // always allow latitude and azimuth and unrolling of longitude
        (GeodMask::GeodLatitude as u32 | GeodMask::GeodAzimuth as u32 | GeodFlags::GeodLongUnroll as u32);

    l.lat1 = lat_fix(lat1);
    l.lon1 = lon1;
    l.azi1 = azi1;
    l.salp1 = salp1;
    l.calp1 = calp1;

    sincosdx(ang_round(l.lat1), &mut sbet1, &mut cbet1);
    sbet1 *= l.f1;
    // Ensure cbet1 = +epsilon at poles
    norm2(&mut sbet1, &mut cbet1);
    cbet1 = fmax(TINY, cbet1);
    l.dn1 = sqrt(1. + g.ep2 * sq(sbet1));

    // Evaluate alp0 from sin(alp1) * cos(bet1) = sin(alp0),
    l.salp0 = l.salp1 * cbet1; /* alp0 in [0, pi/2 - |bet1|] */
    // Alt: calp0 = hypot(sbet1, calp1 * cbet1).  The following
    // is slightly better (consider the case salp1 = 0).
    l.calp0 = hypot(l.calp1, l.salp1 * sbet1);
    // Evaluate sig with tan(bet1) = tan(sig1) * cos(alp1).
    // sig = 0 is nearest northward crossing of equator.
    // With bet1 = 0, alp1 = pi/2, we have sig1 = 0 (equatorial line).
    // With bet1 =  pi/2, alp1 = -pi, sig1 =  pi/2
    // With bet1 = -pi/2, alp1 =  0 , sig1 = -pi/2
    // Evaluate omg1 with tan(omg1) = sin(alp0) * tan(sig1).
    // With alp0 in (0, pi/2], quadrants for sig and omg coincide.
    // No atan2(0,0) ambiguity at poles since cbet1 = +epsilon.
    // With alp0 = 0, omg1 = 0 for alp1 = 0, omg1 = pi for alp1 = pi.
    l.ssig1 = sbet1;
    l.somg1 = l.salp0 * sbet1;
    l.comg1 = sbet1;
    l.csig1 = if l.comg1 != 0. || l.calp1 != 0. { cbet1 * l.calp1 } else { 1. };
    norm2(&mut l.ssig1, &mut l.csig1); /* sig1 in (-pi, pi] */
    // norm2(somg1, comg1); -- don't need to normalize!

    l.k2 = sq(l.calp0) * g.ep2;
    let eps = l.k2 / (2. * (1. + sqrt(1. + l.k2)) + l.k2);

    if (l.caps & CapType::CapC1 as u32) != 0 {
        l.a1m1 = a1m1f(eps);
        c1f(eps, &mut l.c1a);
        l.b11 = sin_cos_series(true, l.ssig1, l.csig1, &l.c1a, N_C1);
        let s = sin(l.b11);
        let c = cos(l.b11);
        // tau1 = sig1 + b11
        l.stau1 = l.ssig1 * c + l.csig1 * s;
        l.ctau1 = l.csig1 * c - l.ssig1 * s;
        // Not necessary because c1pa reverts c1a
        //    b11 = -sin_cos_series(true, stau1, ctau1, c1pa, N_C1_P);
    }

    if (l.caps & CapType::CapC1p as u32) != 0 {
        c1pf(eps, &mut l.c1pa);
    }

    if (l.caps & CapType::CapC2 as u32) != 0 {
        l.a2m1 = a2m1f(eps);
        c2f(eps, &mut l.c2a);
        l.b21 = sin_cos_series(true, l.ssig1, l.csig1, &l.c2a, N_C2);
    }

    if (l.caps & CapType::CapC3 as u32) != 0 {
        c3f(g, eps, &mut l.c3a);
        l.a3c = -l.f * l.salp0 * a3f(g, eps);
        l.b31 = sin_cos_series(true, l.ssig1, l.csig1, &l.c3a, N_C3 - 1);
    }

    if (l.caps & CapType::CapC4 as u32) != 0 {
        c4f(g, eps, &mut l.c4a);
        // Multiplier = a^2 * e^2 * cos(alpha0) * sin(alpha0)
        l.a4 = sq(l.a) * l.calp0 * l.salp0 * g.e2;
        l.b41 = sin_cos_series(false, l.ssig1, l.csig1, &l.c4a, N_C4);
    }

    l.s13 = f64::NAN;
    l.a13 = f64::NAN;
}

/// Evaluate A3
fn a3f(g: &GeodGeodesic, eps: f64) -> f64 {
    // Evaluate A3
    polyvalx(N_A3 - 1, &g.a3x, eps)
}

/// Evaluate C3
fn c3f(g: &GeodGeodesic, eps: f64, c: &mut [f64]) {
    // Evaluate C3 coeffs Elements c[1] through c[N_C3 - 1] are set */
    let mut mult = 1.;
    let mut o = 0;
    #[allow(clippy::needless_range_loop)]
    for l in 1..N_C3 {
        // l is index of C3[l]
        let m = N_C3 - l - 1; // order of polynomial in eps
        mult *= eps;
        c[l] = mult * polyvalx(m, &g.c3x[o..], eps);
        o += m + 1;
    }
}

/// Evaluate C4
fn c4f(g: &GeodGeodesic, eps: f64, c: &mut [f64]) {
    // Evaluate C4 coeffs Elements c[0] through c[N_C4 - 1] are set
    let mut mult = 1.;
    let mut o = 0;
    #[allow(clippy::needless_range_loop)]
    for l in 0..N_C4 {
        // l is index of C4[l]
        let m = N_C4 - l - 1; // order of polynomial in eps
        c[l] = mult * polyvalx(m, &g.c4x[o..], eps);
        o += m + 1;
        mult *= eps;
    }
}

/// The coefficients C1[l] in the Fourier expansion of B1 */
fn c1f(eps: f64, c: &mut [f64]) {
    let coeff: [f64; 18] = [
        // C1[1]/eps^1, polynomial in eps2 of order 2
        -1., 6., -16., 32., // C1[2]/eps^2, polynomial in eps2 of order 2
        -9., 64., -128., 2048., // C1[3]/eps^3, polynomial in eps2 of order 1
        9., -16., 768., // C1[4]/eps^4, polynomial in eps2 of order 1
        3., -5., 512., // C1[5]/eps^5, polynomial in eps2 of order 0
        -7., 1280., // C1[6]/eps^6, polynomial in eps2 of order 0
        -7., 2048.,
    ];
    let eps2 = sq(eps);
    let mut d = eps;
    let mut o = 0;
    #[allow(clippy::needless_range_loop)]
    for l in 1..=N_C1 {
        // l is index of C1p[l]
        let m = (N_C1 - l) / 2; // order of polynomial in eps^2
        c[l] = d * polyvalx(m, &coeff[o..], eps2) / coeff[o + m + 1];
        o += m + 2;
        d *= eps;
    }
}

/// The coefficients C1p[l] in the Fourier expansion of B1p
fn c1pf(eps: f64, c: &mut [f64]) {
    let coeff: [f64; 18] = [
        // C1p[1]/eps^1, polynomial in eps2 of order 2
        205., -432., 768., 1536., // C1p[2]/eps^2, polynomial in eps2 of order 2
        4005., -4736., 3840., 12288., // C1p[3]/eps^3, polynomial in eps2 of order 1
        -225., 116., 384., // C1p[4]/eps^4, polynomial in eps2 of order 1
        -7173., 2695., 7680., // C1p[5]/eps^5, polynomial in eps2 of order 0
        3467., 7680., // C1p[6]/eps^6, polynomial in eps2 of order 0
        38081., 61440.,
    ];
    let eps2 = sq(eps);
    let mut d = eps;
    let mut o = 0;
    #[allow(clippy::needless_range_loop)]
    for l in 1..=N_C1_P {
        // l is index of C1p[l]
        let m = (N_C1_P - l) / 2; // order of polynomial in eps^2
        c[l] = d * polyvalx(m, &coeff[o..], eps2) / coeff[o + m + 1];
        o += m + 2;
        d *= eps;
    }
}

/// The scale factor A2-1 = mean value of (d/dsigma)I2 - 1
fn a2m1f(eps: f64) -> f64 {
    // (eps+1)*A2-1, polynomial in eps2 of order 3
    let coeff: [f64; 5] = [-11., -28., -192., 0., 256.];
    let m = N_A2 / 2;
    let t = polyvalx(m, &coeff, sq(eps)) / coeff[m + 1];
    (t - eps) / (1. + eps)
}

/// The coefficients C2[l] in the Fourier expansion of B2
fn c2f(eps: f64, c: &mut [f64]) {
    let coeff: [f64; 18] = [
        // C2[1]/eps^1, polynomial in eps2 of order 2
        1., 2., 16., 32., // C2[2]/eps^2, polynomial in eps2 of order 2
        35., 64., 384., 2048., // C2[3]/eps^3, polynomial in eps2 of order 1
        15., 80., 768., // C2[4]/eps^4, polynomial in eps2 of order 1
        7., 35., 512., // C2[5]/eps^5, polynomial in eps2 of order 0
        63., 1280., // C2[6]/eps^6, polynomial in eps2 of order 0
        77., 2048.,
    ];
    let eps2 = sq(eps);
    let mut d = eps;
    let mut o = 0;
    #[allow(clippy::needless_range_loop)]
    for l in 1..=N_C2 {
        // l is index of C2[l]
        let m = (N_C2 - l) / 2; /* order of polynomial in eps^2 */
        c[l] = d * polyvalx(m, &coeff[o..], eps2) / coeff[o + m + 1];
        o += m + 2;
        d *= eps;
    }
}

/// Inverse geodesic
#[allow(clippy::too_many_arguments)]
pub fn inverse_start(
    g: &GeodGeodesic,
    sbet1: f64,
    cbet1: f64,
    dn1: f64,
    sbet2: f64,
    cbet2: f64,
    dn2: f64,
    lam12: f64,
    slam12: f64,
    clam12: f64,
    psalp1: &mut f64,
    pcalp1: &mut f64,
    // Only updated if return val >= 0
    psalp2: &mut f64,
    pcalp2: &mut f64,
    // Only updated for short lines
    pdnm: &mut f64,
    // Scratch area of the right size
    ca: &mut [f64],
) -> f64 {
    let mut salp1;
    let mut calp1;
    let mut salp2 = 0.;
    let mut calp2 = 0.;
    let mut dnm = 0.;

    // Return a starting point for Newton's method in salp1 and calp1 (function
    // value is -1).  If Newton's method doesn't need to be used, return also
    // salp2 and calp2 and function value is sig12.
    let mut sig12 = -1.; /* Return value */
    // bet12 = bet2 - bet1 in [0, pi); bet12a = bet2 + bet1 in (-pi, 0]
    let sbet12 = sbet2 * cbet1 - cbet2 * sbet1;
    let cbet12 = cbet2 * cbet1 + sbet2 * sbet1;

    let shortline = cbet12 >= 0. && sbet12 < 0.5 && cbet2 * lam12 < 0.5;
    let mut somg12;
    let mut comg12;

    let sbet12a = sbet2 * cbet1 + cbet2 * sbet1;
    if shortline {
        let mut sbetm2 = sq(sbet1 + sbet2);

        // sin((bet1+bet2)/2)^2
        // =  (sbet1 + sbet2)^2 / ((sbet1 + sbet2)^2 + (cbet1 + cbet2)^2)
        sbetm2 /= sbetm2 + sq(cbet1 + cbet2);
        dnm = sqrt(1. + g.ep2 * sbetm2);
        let omg12 = lam12 / (g.f1 * dnm);
        somg12 = sin(omg12);
        comg12 = cos(omg12);
    } else {
        somg12 = slam12;
        comg12 = clam12;
    }

    salp1 = cbet2 * somg12;
    calp1 = if comg12 >= 0. {
        sbet12 + cbet2 * sbet1 * sq(somg12) / (1. + comg12)
    } else {
        sbet12a - cbet2 * sbet1 * sq(somg12) / (1. - comg12)
    };

    let ssig12 = hypot(salp1, calp1);
    let csig12 = sbet1 * sbet2 + cbet1 * cbet2 * comg12;

    if shortline && ssig12 < g.etol2 {
        // really short lines
        salp2 = cbet1 * somg12;
        calp2 = sbet12
            - cbet1 * sbet2 * (if comg12 >= 0. { sq(somg12) / (1. + comg12) } else { 1. - comg12 });
        norm2(&mut salp2, &mut calp2);
        // Set return value
        sig12 = atan2(ssig12, csig12);
    } else if fabs(g.n) > 0.1 || /* No a calc if too eccentric */
                 csig12 >= 0. ||
                 ssig12 >= 6. * fabs(g.n) * PI * sq(cbet1)
    {
        // Nothing to do, zeroth order spherical approximation is OK
    } else {
        // Scale lam12 and bet2 to x, y coordinate system where antipodal point
        // is at origin and singular point is at y = 0, x = -1.
        let x;
        let y;
        let lamscale;
        let betscale;
        let lam12x = atan2(-slam12, -clam12); /* lam12 - pi */
        if g.f >= 0. {
            // In fact f == 0 does not get here
            // x = dlong, y = dlat
            {
                let k2 = sq(sbet1) * g.ep2;
                let eps = k2 / (2. * (1. + sqrt(1. + k2)) + k2);
                lamscale = g.f * cbet1 * a3f(g, eps) * PI;
            }
            betscale = lamscale * cbet1;

            x = lam12x / lamscale;
            y = sbet12a / betscale;
        } else {
            // f < 0
            // x = dlat, y = dlong
            let cbet12a = cbet2 * cbet1 - sbet2 * sbet1;
            let bet12a = atan2(sbet12a, cbet12a);
            let mut m12b = 0.;
            let mut m0 = 0.;
            // In the case of lon12 = 180, this repeats a calculation made in
            // Inverse.
            lengths(
                g,
                g.n,
                PI + bet12a,
                sbet1,
                -cbet1,
                dn1,
                sbet2,
                cbet2,
                dn2,
                cbet1,
                cbet2,
                &mut 0.,
                &mut m12b,
                &mut m0,
                &mut 0.,
                &mut 0.,
                ca,
            );
            x = -1. + m12b / (cbet1 * cbet2 * m0 * PI);
            betscale = if x < -0.01 { sbet12a / x } else { -g.f * sq(cbet1) * PI };
            lamscale = betscale / cbet1;
            y = lam12x / lamscale;
        }

        if y > -TOL1 && x > -1. - XTHRESH {
            // strip near cut
            if g.f >= 0. {
                salp1 = fmin(1.0, -x);
                calp1 = -sqrt(1. - sq(salp1));
            } else {
                calp1 = fmax(if x > -TOL1 { 0.0 } else { -1.0 }, x);
                salp1 = sqrt(1. - sq(calp1));
            }
        } else {
            // Estimate alp1, by solving the a problem.
            //
            // Could estimate alpha1 = theta + pi/2, directly, i.e.,
            //   calp1 = y/k; salp1 = -x/(1+k);  for f >= 0
            //   calp1 = x/(1+k); salp1 = -y/k;  for f < 0 (need to check)
            //
            // However, it's better to estimate omg12 from a and use
            // spherical formula to compute alp1.  This reduces the mean number of
            // Newton iterations for a cases from 2.24 (min 0, max 6) to 2.12
            // (min 0 max 5).  The changes in the number of iterations are as
            // follows:
            //
            // change percent
            //    1       5
            //    0      78
            //   -1      16
            //   -2       0.6
            //   -3       0.04
            //   -4       0.002
            //
            // The histogram of iterations is (m = number of iterations estimating
            // alp1 directly, n = number of iterations estimating via omg12, total
            // number of trials = 148605):
            //
            //  iter    m      n
            //    0   148    186
            //    1 13046  13845
            //    2 93315 102225
            //    3 36189  32341
            //    4  5396      7
            //    5   455      1
            //    6    56      0
            //
            // Because omg12 is near pi, estimate work with omg12a = pi - omg12
            let k = astroid(x, y);
            let omg12a = lamscale * (if g.f >= 0. { -x * k / (1. + k) } else { -y * (1. + k) / k });
            somg12 = sin(omg12a);
            comg12 = -cos(omg12a);
            // Update spherical estimate of alp1 using omg12 instead of lam12
            salp1 = cbet2 * somg12;
            calp1 = sbet12a - cbet2 * sbet1 * sq(somg12) / (1. - comg12);
        }
    }
    // Sanity check on starting guess.  Backwards check allows NaN through.
    if salp1 > 0. {
        norm2(&mut salp1, &mut calp1);
    } else {
        salp1 = 1.;
        calp1 = 0.;
    }

    *psalp1 = salp1;
    *pcalp1 = calp1;
    if shortline {
        *pdnm = dnm;
    }
    if sig12 >= 0. {
        *psalp2 = salp2;
        *pcalp2 = calp2;
    }

    sig12
}

/// Compute lambda12
#[allow(clippy::too_many_arguments)]
pub fn lambda12(
    g: &GeodGeodesic,
    sbet1: f64,
    cbet1: f64,
    dn1: f64,
    sbet2: f64,
    cbet2: f64,
    dn2: f64,
    salp1: f64,
    mut calp1: f64,
    slam120: f64,
    clam120: f64,
    psalp2: &mut f64,
    pcalp2: &mut f64,
    psig12: &mut f64,
    pssig1: &mut f64,
    pcsig1: &mut f64,
    pssig2: &mut f64,
    pcsig2: &mut f64,
    peps: &mut f64,
    pdomg12: &mut f64,
    diffp: bool,
    pdlam12: &mut f64,
    // Scratch area of the right size
    ca: &mut [f64],
) -> f64 {
    let mut ssig1;
    let mut csig1;
    let mut ssig2;
    let mut csig2;
    let mut dlam12 = 0.0;

    if sbet1 == 0. && calp1 == 0. {
        // Break degeneracy of equatorial line.  This case has already been handled.
        calp1 = -TINY;
    }

    // sin(alp1) * cos(bet1) = sin(alp0)
    let salp0 = salp1 * cbet1;
    let calp0 = hypot(calp1, salp1 * sbet1); /* calp0 > 0 */

    // tan(bet1) = tan(sig1) * cos(alp1)
    // tan(omg1) = sin(alp0) * tan(sig1) = tan(omg1)=tan(alp1)*sin(bet1)
    ssig1 = sbet1;
    let somg1 = salp0 * sbet1;
    let comg1 = calp1 * cbet1;
    csig1 = comg1;
    norm2(&mut ssig1, &mut csig1);
    // norm2(&somg1, &comg1); -- don't need to normalize!

    // Enforce symmetries in the case abs(bet2) = -bet1.  Need to be careful
    // about this case, since this can yield singularities in the Newton
    // iteration.
    // sin(alp2) * cos(bet2) = sin(alp0)
    let salp2 = if cbet2 != cbet1 { salp0 / cbet2 } else { salp1 };
    // calp2 = sqrt(1 - sq(salp2))
    //       = sqrt(sq(calp0) - sq(sbet2)) / cbet2
    // and subst for calp0 and rearrange to give (choose positive sqrt
    // to give alp2 in [0, pi/2]).

    let calp2 = if cbet2 != cbet1 || fabs(sbet2) != -sbet1 {
        sqrt(
            sq(calp1 * cbet1)
                + (if cbet1 < -sbet1 {
                    (cbet2 - cbet1) * (cbet1 + cbet2)
                } else {
                    (sbet1 - sbet2) * (sbet1 + sbet2)
                }),
        ) / cbet2
    } else {
        fabs(calp1)
    };

    // tan(bet2) = tan(sig2) * cos(alp2)
    // tan(omg2) = sin(alp0) * tan(sig2).
    ssig2 = sbet2;
    let somg2 = salp0 * sbet2;
    let comg2 = calp2 * cbet2;
    csig2 = comg2;
    norm2(&mut ssig2, &mut csig2);
    // norm2(&somg2, &comg2); -- don't need to normalize!

    // sig12 = sig2 - sig1, limit to [0, pi]
    let sig12 = atan2(fmax(0.0, csig1 * ssig2 - ssig1 * csig2) + 0., csig1 * csig2 + ssig1 * ssig2);

    // omg12 = omg2 - omg1, limit to [0, pi]
    let somg12 = fmax(0.0, comg1 * somg2 - somg1 * comg2) + 0.;
    let comg12 = comg1 * comg2 + somg1 * somg2;
    // eta = omg12 - lam120
    let eta = atan2(somg12 * clam120 - comg12 * slam120, comg12 * clam120 + somg12 * slam120);
    let k2 = sq(calp0) * g.ep2;
    let eps = k2 / (2. * (1. + sqrt(1. + k2)) + k2);
    c3f(g, eps, ca);
    let b312 = sin_cos_series(true, ssig2, csig2, ca, N_C3 - 1)
        - sin_cos_series(true, ssig1, csig1, ca, N_C3 - 1);
    let domg12 = -g.f * a3f(g, eps) * salp0 * (sig12 + b312);
    let lam12 = eta + domg12;

    if diffp {
        if calp2 == 0. {
            dlam12 = -2. * g.f1 * dn1 / sbet1;
        } else {
            lengths(
                g,
                eps,
                sig12,
                ssig1,
                csig1,
                dn1,
                ssig2,
                csig2,
                dn2,
                cbet1,
                cbet2,
                &mut 0.,
                &mut dlam12,
                &mut 0.,
                &mut 0.,
                &mut 0.,
                ca,
            );
            dlam12 *= g.f1 / (calp2 * cbet2);
        }
    }

    *psalp2 = salp2;
    *pcalp2 = calp2;
    *psig12 = sig12;
    *pssig1 = ssig1;
    *pcsig1 = csig1;
    *pssig2 = ssig2;
    *pcsig2 = csig2;
    *peps = eps;
    *pdomg12 = domg12;
    if diffp {
        *pdlam12 = dlam12;
    }

    lam12
}

/// Compute length of geodesic
#[allow(clippy::too_many_arguments)]
fn lengths(
    g: &GeodGeodesic,
    eps: f64,
    sig12: f64,
    ssig1: f64,
    csig1: f64,
    dn1: f64,
    ssig2: f64,
    csig2: f64,
    dn2: f64,
    cbet1: f64,
    cbet2: f64,
    ps12b: &mut f64,
    pm12b: &mut f64,
    pm0: &mut f64,
    p_m12: &mut f64,
    p_m21: &mut f64,
    // Scratch area of the right size
    ca: &mut [f64],
) {
    //   double m0 = 0, J12 = 0, A1 = 0, A2 = 0;
    let mut m0 = 0.0;
    let mut j12 = 0.0;
    let mut a1 = 0.0;
    let mut a2 = 0.0;
    let mut cb = [0.0; N_C];

    // Return m12b = (reduced length)/b; also calculate s12b = distance/b,
    // and m0 = coefficient of secular term in expression for reduced length.
    let redlp = *pm12b != 0. || *pm0 != 0. || *p_m12 != 0. || *p_m21 != 0.;
    if *ps12b != 0. || redlp {
        a1 = a1m1f(eps);
        c1f(eps, ca);
        if redlp {
            a2 = a2m1f(eps);
            c2f(eps, &mut cb);
            m0 = a1 - a2;
            a2 += 1.;
        }
        a1 += 1.;
    }
    if *ps12b != 0. {
        let b1 = sin_cos_series(true, ssig2, csig2, ca, N_C1)
            - sin_cos_series(true, ssig1, csig1, ca, N_C1);
        // Missing a factor of b
        *ps12b = a1 * (sig12 + b1);
        // if redlp {
        //     let b2 = sin_cos_series(true, ssig2, csig2, &cb, N_C2)
        //         - sin_cos_series(true, ssig1, csig1, &cb, N_C2);
        //     j12 = m0 * sig12 + (A1 * b1 - A2 * b2);
        // }
    } else if redlp {
        // Assume here that nC1 >= nC2
        // int l;
        // for (l = 1; l <= nC2; ++l)
        for l in 1..N_C2 {
            cb[l] = a1 * ca[l] - a2 * cb[l];
            j12 = m0 * sig12
                + (sin_cos_series(true, ssig2, csig2, &cb, N_C2)
                    - sin_cos_series(true, ssig1, csig1, &cb, N_C2));
        }
        if *pm0 != 0. {
            *pm0 = m0;
        }
        if *pm12b != 0. {
            /* Missing a factor of b.
             * Add parens around (csig1 * ssig2) and (ssig1 * csig2) to ensure
             * accurate cancellation in the case of coincident points. */
            *pm12b = dn2 * (csig1 * ssig2) - dn1 * (ssig1 * csig2) - csig1 * csig2 * j12;
        }
        if *p_m12 != 0. || *p_m21 != 0. {
            let csig12 = csig1 * csig2 + ssig1 * ssig2;
            let t = g.ep2 * (cbet1 - cbet2) * (cbet1 + cbet2) / (dn1 + dn2);
            if *p_m12 != 0. {
                *p_m12 = csig12 + (t * ssig2 - csig2 * j12) * ssig1 / dn1;
            }
            if *p_m21 != 0. {
                *p_m21 = csig12 - (t * ssig1 - csig1 * j12) * ssig2 / dn2;
            }
        }
    }
}

fn sincosde(x: f64, t: f64, sinx: &mut f64, cosx: &mut f64) {
    // In order to minimize round-off errors, this function exactly reduces
    // the argument to the range [-45, 45] before converting it to radians.
    //   double r, s, c; int q = 0;
    let q = remquo(x, QD);
    let mut r = ang_round(q.0 + t);
    // now abs(r) <= 45
    r *= DEGREE;
    // Possibly could call the gnu extension sincos
    let s = sin(r);
    let c = cos(r);
    match q.1 & 3 {
        0 => {
            *sinx = s;
            *cosx = c;
        }
        1 => {
            *sinx = c;
            *cosx = -s;
        }
        2 => {
            *sinx = -s;
            *cosx = -c;
        }
        _ => {
            *sinx = -c;
            *cosx = s;
        } // case 3U
    }
    // http://www.open-std.org/jtc1/sc22/wg14/www/docs/n1950.pdf
    *cosx += 0.; /* special values from F.10.1.12 */
    // special values from F.10.1.13
    if *sinx == 0. {
        *sinx = copysign(*sinx, x);
    }
}

/// Solve k^4+2*k^3-(x^2+y^2-1)*k^2-2*y^2*k-y^2 = 0 for positive root k
/// This solution is adapted from Geocentric::Reverse
pub fn astroid(x: f64, y: f64) -> f64 {
    let p = sq(x);
    let q = sq(y);
    let r = (p + q - 1.) / 6.;
    if !(q == 0. && r <= 0.) {
        // Avoid possible division by zero when r = 0 by multiplying equations
        // for s and t by r^3 and r, resp.
        let _s = p * q / 4.; /* S = r^3 * s */
        let r2 = sq(r);
        let r3 = r * r2;
        // The discriminant of the quadratic equation for _t3.  This is zero on
        // the evolute curve p^(1/3)+q^(1/3) = 1
        let disc = _s * (_s + 2. * r3);
        let mut u = r;

        if disc >= 0. {
            let mut _t3 = _s + r3;
            // Pick the sign on the sqrt to maximize abs(_t3).  This minimizes loss
            // of precision due to cancellation.  The result is unchanged because
            // of the way the T is used in definition of u.
            _t3 += if _t3 < 0. { -sqrt(disc) } else { sqrt(disc) }; /* _t3 = (r * t)^3 */
            // N.B. cbrt always returns the double root.  cbrt(-8) = -2.
            let t = cbrt(_t3); /* T = r * t */
            // T can be zero; but then r2 / T -> 0.
            u += t + (if t != 0. { r2 / t } else { 0. });
        } else {
            // T is complex, but the way u is defined the result is double.
            let ang = atan2(sqrt(-disc), -(_s + r3));
            // There are three possible cube roots.  We choose the root which
            // avoids cancellation.  Note that disc < 0 implies that r < 0.
            u += 2. * r * cos(ang / 3.);
        }
        let v = sqrt(sq(u) + q); /* guaranteed positive */
        // Avoid loss of accuracy when u < 0.
        let uv = if u < 0. { q / (v - u) } else { u + v }; /* u+v, guaranteed positive */
        let w = (uv - q) / (2. * v); /* positive? */
        // Rearrange expression for k to avoid loss of accuracy due to
        // subtraction.  Division by 0 not possible because uv > 0, w >= 0.
        uv / (sqrt(uv + sq(w)) + w) /* guaranteed positive */
    } else {
        // q == 0 && r <= 0
        // y = 0 with |x| <= 1.  Handle this case directly.
        // for y small, positive root is k = abs(y)/sqrt(1-x^2)
        0.
    }
}
