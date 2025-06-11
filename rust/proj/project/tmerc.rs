use crate::proj::{
    ALGO, APPROX, AuxLat, CoordinateStep, EPS10, Proj, ProjValue, ProjectCoordinates, SOUTH,
    TRANSVERSE_MERCATOR, TRANSVERSE_MERCATOR_SOUTH_ORIENTATED, TransformCoordinates, ZONE, adjlon,
    auxlat_coeffs, auxlat_convert, auxlat_convert_mid, enfn, inv_mlfn, mlfn, rectifying_radius,
};
use alloc::vec::Vec;
use core::{
    cell::RefCell,
    f64::consts::{FRAC_PI_2, PI},
};
use libm::{
    acos, asin, asinh, atan2, copysign, cos, exp, fabs, floor, hypot, log, round, sin, sinh, sqrt,
};

/*
 *                   Transverse Mercator implementations
 *
 * In this file two transverse mercator implementations are found. One of Gerald
 * Evenden/John Snyder origin and one of Knud Poder/Karsten Engsager origin. The
 * former is regarded as "approximate" in the following and the latter is
 * "exact". This word choice has been made to distinguish between the two
 * algorithms, where the Evenden/Snyder implementation is the faster, less
 * accurate implementation and the Poder/Engsager algorithm is a slightly
 * slower, but more accurate implementation.
 */

/// Poder/Engsager if far from central meridian, otherwise Evenden/Snyder
#[derive(Debug, Default, Clone, PartialEq)]
pub enum TMercAlgo {
    /// Auto
    Auto,
    /// Evenden/Snyder
    EvendenSnyder,
    /// Poder/Engsager
    #[default]
    PoderEngsager,
}

/// Approximate: Evenden/Snyder
#[derive(Debug, Default, Clone, PartialEq)]
pub struct EvendenSnyder {
    esp: f64,
    ml0: f64,
    en: Vec<f64>,
}

/// More exact: Poder/Engsager
#[derive(Debug, Default, Clone, PartialEq)]
pub struct PoderEngsager {
    /// Merid. quad., scaled to the projection
    qn: f64,
    /// Radius vector in polar coord. systems
    zb: f64,
    /// Constants for Gauss -> Geo lat
    cgb: [f64; 6],
    /// Constants for Geo lat -> Gauss
    cbg: [f64; 6],
    /// Constants for transv. merc. -> geo
    utg: [f64; 6],
    /// Constants for geo -> transv. merc.
    gtu: [f64; 6],
}

/// Transverse Mercator Data
#[derive(Debug, Default, Clone, PartialEq)]
pub struct TmercData {
    approx: EvendenSnyder,
    exact: PoderEngsager,
}

/// Transverse Mercator Mode
#[derive(Debug, Default, Clone, PartialEq)]
pub enum TMercMode {
    /// Spherical
    #[default]
    Spherical,
    /// Approximate Ellipsoidal
    ApproxEllipsoidal,
    /// Exact Ellipsoidal
    ExactEllipsoidal,
    /// Auto Ellipsoidal
    AutoEllipsoidal,
}

// Constants for "approximate" transverse mercator
const FC1: f64 = 1.0;
const FC2: f64 = 0.5;
const FC3: f64 = 0.166_666_666_666_666_66;
const FC4: f64 = 0.083_333_333_333_333_33;
const FC5: f64 = 0.05;
const FC6: f64 = 0.033_333_333_333_333_33;
const FC7: f64 = 0.023_809_523_809_523_808;
const FC8: f64 = 0.017_857_142_857_142_856;

/// Constant for "exact" transverse mercator */
const PROJ_ETMERC_ORDER: i32 = 6;

/// Approximate Transverse Mercator functions
pub fn tmerc_approx_e_fwd<P: TransformCoordinates>(tmerc: &mut TmercData, proj: &Proj, p: &mut P) {
    let evenden_snyder = &tmerc.approx;
    /*
     * Fail if our longitude is more than 90 degrees from the
     * central meridian since the results are essentially garbage.
     * Is error -20 really an appropriate return value?
     *
     *  http://trac.osgeo.org/proj/ticket/5
     */
    if p.lam() < -FRAC_PI_2 || p.lam() > FRAC_PI_2 {
        panic!("Longitude out of range");
    }

    let sinphi = sin(p.phi());
    let cosphi = cos(p.phi());
    let mut t = if fabs(cosphi) > 1e-10 { sinphi / cosphi } else { 0. };
    t *= t;
    let mut al = cosphi * p.lam();
    let als = al * al;
    al /= sqrt(1. - proj.es * sinphi * sinphi);
    let n = evenden_snyder.esp * cosphi * cosphi;
    p.set_x(
        proj.k0
            * al
            * (FC1
                + FC3
                    * als
                    * (1. - t
                        + n
                        + FC5
                            * als
                            * (5.
                                + t * (t - 18.)
                                + n * (14. - 58. * t)
                                + FC7 * als * (61. + t * (t * (179. - t) - 479.))))),
    );
    p.set_y(
        proj.k0
            * (mlfn(p.phi(), sinphi, cosphi, &evenden_snyder.en) - evenden_snyder.ml0
                + sinphi
                    * al
                    * p.lam()
                    * FC2
                    * (1.
                        + FC4
                            * als
                            * (5. - t
                                + n * (9. + 4. * n)
                                + FC6
                                    * als
                                    * (61.
                                        + t * (t - 58.)
                                        + n * (270. - 330. * t)
                                        + FC8 * als * (1385. + t * (t * (543. - t) - 3111.)))))),
    );
}

/// Spherical Transverse Mercator forward project
pub fn tmerc_spherical_fwd<P: TransformCoordinates>(tmerc: &mut TmercData, proj: &Proj, p: &mut P) {
    let evenden_snyder = &tmerc.approx;

    let cosphi = cos(p.phi());
    let mut b = cosphi * sin(p.lam());
    if fabs(fabs(b) - 1.) <= EPS10 {
        panic!("Coordinate outside projection domain");
    }

    let x = evenden_snyder.ml0 * log((1. + b) / (1. - b));
    let mut y = cosphi * cos(p.lam()) / sqrt(1. - b * b);

    b = fabs(y);
    if cosphi == 1. && (p.lam() < -FRAC_PI_2 || p.lam() > FRAC_PI_2) {
        /* Helps to be able to roundtrip |longitudes| > 90 at lat=0 */
        /* We could also map to -M_PI ... */
        y = PI;
    } else if b >= 1. {
        if (b - 1.) > EPS10 {
            panic!("Coordinate outside projection domain");
        } else {
            y = 0.;
        }
    } else {
        y = acos(y);
    }

    if p.phi() < 0. {
        y = -y;
    }
    y = evenden_snyder.esp * (y - proj.phi0);

    p.set_x(x);
    p.set_y(y);
}

/// Approximate Transverse Mercator inverse project
pub fn tmerc_approx_e_inv<P: TransformCoordinates>(tmerc: &mut TmercData, proj: &Proj, p: &mut P) {
    let evenden_snyder = &tmerc.approx;

    let mut phi = inv_mlfn(evenden_snyder.ml0 + p.y() / proj.k0, &evenden_snyder.en);
    let lam: f64;
    if fabs(phi) >= FRAC_PI_2 {
        phi = if p.y() < 0. { -FRAC_PI_2 } else { FRAC_PI_2 };
        lam = 0.;
    } else {
        let sinphi = sin(phi);
        let cosphi = cos(phi);
        let mut t = if fabs(cosphi) > 1e-10 { sinphi / cosphi } else { 0. };
        let n = evenden_snyder.esp * cosphi * cosphi;
        let mut con = 1. - proj.es * sinphi * sinphi;
        let d = p.x() * sqrt(con) / proj.k0;
        con *= t;
        t *= t;
        let ds = d * d;
        phi -= (con * ds / (1. - proj.es))
            * FC2
            * (1.
                - ds * FC4
                    * (5. + t * (3. - 9. * n) + n * (1. - 4. * n)
                        - ds * FC6
                            * (61. + t * (90. - 252. * n + 45. * t) + 46. * n
                                - ds * FC8 * (1385. + t * (3633. + t * (4095. + 1575. * t))))));
        lam = d
            * (FC1
                - ds * FC3
                    * (1. + 2. * t + n
                        - ds * FC5
                            * (5. + t * (28. + 24. * t + 8. * n) + 6. * n
                                - ds * FC7 * (61. + t * (662. + t * (1320. + 720. * t))))))
            / cosphi;
    }
    p.set_phi(phi);
    p.set_lam(lam);
}

/// Spherical Transverse Mercator inverse project
pub fn tmerc_spherical_inv<P: TransformCoordinates>(tmerc: &mut TmercData, proj: &Proj, p: &mut P) {
    let evenden_snyder = &tmerc.approx;
    let mut h = exp(p.x() / evenden_snyder.esp);
    if h == 0. {
        panic!("Coordinate outside projection domain");
    }
    let g = 0.5 * (h - 1. / h);
    /* D, as in equation 8-8 of USGS "Map Projections - A Working Manual" */
    let d = proj.phi0 + p.y() / evenden_snyder.esp;
    h = cos(d);
    p.set_phi(asin(sqrt((1. - h * h) / (1. + g * g))));
    // Make sure that phi is on the correct hemisphere when false northing is used
    p.set_phi(copysign(p.phi(), d));
    p.set_lam(if g != 0.0 || h != 0.0 { atan2(g, h) } else { 0. });
}

fn setup_approx(tmerc: &mut TmercData, proj: &Proj) {
    let evenden_snyder = &mut tmerc.approx;

    if proj.es != 0.0 {
        evenden_snyder.en = enfn(proj.n);
        if evenden_snyder.en.is_empty() {
            panic!("Projection setup failed");
        }

        evenden_snyder.ml0 = mlfn(proj.phi0, sin(proj.phi0), cos(proj.phi0), &evenden_snyder.en);
        evenden_snyder.esp = proj.es / (1. - proj.es);
    } else {
        evenden_snyder.esp = proj.k0;
        evenden_snyder.ml0 = 0.5 * evenden_snyder.esp;
    }
}

//
//                  Exact Transverse Mercator functions
//
//
// The code in this file is largly based upon procedures:
//
// Written by: Knud Poder and Karsten Engsager
//
// Based on math from: R.Koenig and K.H. Weise, "Mathematische
// Grundlagen der hoeheren Geodaesie und Kartographie,
// Springer-Verlag, Berlin/Goettingen" Heidelberg, 1951.
//
// Modified and used here by permission of Reference Networks
// Division, Kort og Matrikelstyrelsen (KMS), Copenhagen, Denmark

/// Complex Clenshaw summation
#[inline]
fn clen_s(
    a: &[f64],
    sin_arg_r: f64,
    cos_arg_r: f64,
    sinh_arg_i: f64,
    cosh_arg_i: f64,
) -> (f64, f64) {
    let mut r: f64;
    let mut i: f64;
    let mut hr: f64;
    let mut hr1: f64 = 0.0;
    let mut hr2: f64;
    let mut hi: f64 = 0.0;
    let mut hi1: f64 = 0.0;
    let mut hi2: f64;

    // arguments
    let mut p = a.len();
    r = 2.0 * cos_arg_r * cosh_arg_i;
    i = -2.0 * sin_arg_r * sinh_arg_i;

    // summation loop
    p -= 1;
    hr = a[p];
    while p > 0 {
        p -= 1;
        hr2 = hr1;
        hi2 = hi1;
        hr1 = hr;
        hi1 = hi;
        hr = -hr2 + r * hr1 - i * hi1 + a[p];
        hi = -hi2 + i * hr1 + r * hi1;
    }

    r = sin_arg_r * cosh_arg_i;
    i = cos_arg_r * sinh_arg_i;
    let real = r * hr - i * hi;
    let imag = r * hi + i * hr;
    (real, imag)
}

/// Transverse Mercator Ellipsoidal forward project
pub fn tmerc_exact_e_fwd<P: TransformCoordinates>(tmerc: &mut TmercData, p: &mut P) {
    let poder_engsager = &tmerc.exact;

    /* ell. LAT, LNG -> Gaussian LAT, LNG */
    let mut cn = auxlat_convert(p.phi(), &poder_engsager.cbg, PROJ_ETMERC_ORDER);
    /* Gaussian LAT, LNG -> compl. sph. LAT */
    let sin_cn = sin(cn);
    let cos_cn = cos(cn);
    let sin_ce = sin(p.lam());
    let cos_ce = cos(p.lam());

    let cos_cn_cos_ce = cos_cn * cos_ce;
    cn = atan2(sin_cn, cos_cn_cos_ce);

    let inv_denom_tan_ce = 1. / hypot(sin_cn, cos_cn_cos_ce);
    let tan_ce = sin_ce * cos_cn * inv_denom_tan_ce;
    // Variant of the above: found not to be measurably faster
    // let sin_ce_cos_cn = sin_ce * cos_cn;
    // let denom = sqrt(1. - sin_ce_cos_cn * sin_ce_cos_cn);
    // let tan_ce = sin_ce_cos_cn / denom;

    /* compl. sph. N, E -> ell. norm. N, E */
    let mut ce = asinh(tan_ce); /* Replaces: Ce  = log(tan(FORTPI + Ce*0.5)); */
    /*
     *  Non-optimized version:
     *  let sin_arg_r  = sin(2*Cn);
     *  let cos_arg_r  = cos(2*Cn);
     *
     *  Given:
     *      sin(2 * Cn) = 2 sin(Cn) cos(Cn)
     *          sin(atan(y)) = y / sqrt(1 + y^2)
     *          cos(atan(y)) = 1 / sqrt(1 + y^2)
     *      ==> sin(2 * Cn) = 2 tan_Cn / (1 + tan_Cn^2)
     *
     *      cos(2 * Cn) = 2cos^2(Cn) - 1
     *                  = 2 / (1 + tan_Cn^2) - 1
     */
    let two_inv_denom_tan_ce = 2. * inv_denom_tan_ce;
    let two_inv_denom_tan_ce_square = two_inv_denom_tan_ce * inv_denom_tan_ce;
    let tmp_r = cos_cn_cos_ce * two_inv_denom_tan_ce_square;
    let sin_arg_r = sin_cn * tmp_r;
    let cos_arg_r = cos_cn_cos_ce * tmp_r - 1.;

    /*
     *  Non-optimized version:
     *  let sinh_arg_i = sinh(2*Ce);
     *  let cosh_arg_i = cosh(2*Ce);
     *
     *  Given
     *      sinh(2 * Ce) = 2 sinh(Ce) cosh(Ce)
     *          sinh(asinh(y)) = y
     *          cosh(asinh(y)) = sqrt(1 + y^2)
     *      ==> sinh(2 * Ce) = 2 tan_ce sqrt(1 + tan_ce^2)
     *
     *      cosh(2 * Ce) = 2cosh^2(Ce) - 1
     *                   = 2 * (1 + tan_ce^2) - 1
     *
     * and 1+tan_ce^2 = 1 + sin_ce^2 * cos_cn^2 / (sin_cn^2 + cos_cn^2 *
     * cos_ce^2) = (sin_cn^2 + cos_cn^2 * cos_ce^2 + sin_ce^2 * cos_cn^2) /
     * (sin_cn^2 + cos_cn^2 * cos_ce^2) = 1. / (sin_cn^2 + cos_cn^2 * cos_ce^2)
     * = inv_denom_tan_ce^2
     *
     */
    let sinh_arg_i = tan_ce * two_inv_denom_tan_ce;
    let cosh_arg_i = two_inv_denom_tan_ce_square - 1.;
    let (d_cn, d_ce) = clen_s(&poder_engsager.gtu, sin_arg_r, cos_arg_r, sinh_arg_i, cosh_arg_i);
    cn += d_cn;
    ce += d_ce;
    if fabs(ce) <= 2.623395162778 {
        // Northing
        p.set_y(poder_engsager.qn * cn + poder_engsager.zb);
        // Easting
        p.set_x(poder_engsager.qn * ce);
    } else {
        panic!("Coordinate outside projection domain");
    }
}

/// Transverse Mercator Ellipsoidal inverse project
pub fn tmerc_exact_e_inv<P: TransformCoordinates>(tmerc: &mut TmercData, p: &mut P) {
    let poder_engsager = &tmerc.exact;

    /* normalize N, E */
    let mut cn = (p.y() - poder_engsager.zb) / poder_engsager.qn;
    let mut ce = p.x() / poder_engsager.qn;

    if fabs(ce) <= 2.623395162778 {
        /* 150 degrees */
        /* norm. N, E -> compl. sph. LAT, LNG */
        let sin_arg_r = sin(2. * cn);
        let cos_arg_r = cos(2. * cn);

        // let sinh_arg_i = sinh(2*Ce);
        // let cosh_arg_i = cosh(2*Ce);
        let exp_2_ce = exp(2. * ce);
        let half_inv_exp_2_ce = 0.5 / exp_2_ce;
        let sinh_arg_i = 0.5 * exp_2_ce - half_inv_exp_2_ce;
        let cosh_arg_i = 0.5 * exp_2_ce + half_inv_exp_2_ce;
        let (d_cn, d_ce) =
            clen_s(&poder_engsager.utg, sin_arg_r, cos_arg_r, sinh_arg_i, cosh_arg_i);
        cn += d_cn;
        ce += d_ce;

        /* compl. sph. LAT -> Gaussian LAT, LNG */
        let sin_cn = sin(cn);
        let cos_cn = cos(cn);

        // #if 0
        //         // Non-optimized version:
        //         double sin_ce, cos_ce;
        //         Ce = atan (sinh (Ce));  // Replaces: Ce = 2*(atan(exp(Ce)) - FORTPI);
        //         sin_ce = sin (Ce);
        //         cos_ce = cos (Ce);
        //         Ce     = atan2 (sin_ce, cos_ce*cos_cn);
        //         Cn     = atan2 (sin_cn*cos_ce,  hypot (sin_ce, cos_ce*cos_cn));
        // #else
        /*
         *      One can divide both member of Ce = atan2(...) by cos_ce, which
         * gives: Ce     = atan2 (tan_ce, cos_cn) = atan2(sinh(Ce), cos_cn)
         *
         *      and the same for Cn = atan2(...)
         *      Cn     = atan2 (sin_cn, hypot (sin_ce, cos_ce*cos_cn)/cos_ce)
         *             = atan2 (sin_cn, hypot (sin_ce/cos_ce, cos_cn))
         *             = atan2 (sin_cn, hypot (tan_ce, cos_cn))
         *             = atan2 (sin_cn, hypot (sinh_ce, cos_cn))
         */
        let sinh_ce = sinh(ce);
        ce = atan2(sinh_ce, cos_cn);
        let modulus_ce = hypot(sinh_ce, cos_cn);
        let rr = hypot(sin_cn, modulus_ce);
        cn = atan2(sin_cn, modulus_ce);
        // #endif

        // Gaussian LAT, LNG -> ell. LAT, LNG
        p.set_phi(auxlat_convert_mid(
            cn,
            sin_cn / rr,
            modulus_ce / rr,
            &poder_engsager.cgb,
            PROJ_ETMERC_ORDER,
        ));
        p.set_lam(ce);
    } else {
        panic!("Coordinate outside projection domain");
    }
}

fn setup_exact(tmerc: &mut TmercData, proj: &Proj) {
    let poder_engsager = &mut tmerc.exact;
    assert!(proj.es == 0., "Eccentricity must be zero");
    assert!(PROJ_ETMERC_ORDER == AuxLat::ORDER as i32, "Inconsistent orders etmerc vs auxorder");
    // third flattening
    let n = proj.n;

    // N.B., Engsager and Poder terminology (simplifying a little here...)
    //   geodetic coordinates = geographic latitude
    //   Soldner sphere + complex gaussian coordinates = conformal latitude
    //   transverse Mercator coordinates = rectifying latitude

    // COEF. OF TRIG SERIES GEO <-> GAUSS
    // cgb := Gaussian -> Geodetic, KW p190 - 191 (61) - (62)
    // cbg := Geodetic -> Gaussian, KW p186 - 187 (51) - (52)
    // PROJ_ETMERC_ORDER = 6th degree : Engsager and Poder: ICC2007
    auxlat_coeffs(n, AuxLat::CONFORMAL, AuxLat::GEOGRAPHIC, &mut poder_engsager.cgb);
    auxlat_coeffs(n, AuxLat::GEOGRAPHIC, AuxLat::CONFORMAL, &mut poder_engsager.cbg);
    // Constants of the projections
    // Transverse Mercator (UTM, ITM, etc)
    // Norm. mer. quad, K&W p.50 (96), p.19 (38b), p.5 (2)
    poder_engsager.qn = proj.k0 * rectifying_radius(n);
    // coef of trig series
    // utg := ell. N, E -> sph. N, E,  KW p194 (65)
    // gtu := sph. N, E -> ell. N, E,  KW p196 (69)
    auxlat_coeffs(n, AuxLat::RECTIFYING, AuxLat::CONFORMAL, &mut poder_engsager.utg);
    auxlat_coeffs(n, AuxLat::CONFORMAL, AuxLat::RECTIFYING, &mut poder_engsager.gtu);
    // Gaussian latitude value of the origin latitude
    let z = auxlat_convert(proj.phi0, &poder_engsager.cbg, PROJ_ETMERC_ORDER);

    // Origin northing minus true northing at the origin latitude
    // i.e. true northing = N - proj.zb
    poder_engsager.zb =
        -poder_engsager.qn * auxlat_convert(z, &poder_engsager.gtu, PROJ_ETMERC_ORDER);
}

/// Transverse Mercator Auto forward project
pub fn tmerc_auto_e_fwd<P: TransformCoordinates>(tmerc: &mut TmercData, proj: &Proj, p: &mut P) {
    if fabs(p.lam()) > 3.0_f64.to_radians() {
        tmerc_exact_e_fwd(tmerc, p);
    } else {
        tmerc_approx_e_fwd(tmerc, proj, p);
    }
}

/// Transverse Mercator Auto inverse project
pub fn tmerc_auto_e_inv<P: TransformCoordinates>(tmerc: &mut TmercData, proj: &Proj, p: &mut P) {
    // static PJ_LP tmerc_auto_e_inv(PJ_XY xy, PJ *P) {
    // For k = 1 and long = 3 (from central meridian),
    // At lat = 0, we get x ~= 0.052, y = 0
    // At lat = 90, we get x = 0, y ~= 1.57 }
    // And the shape of this x=f(y) frontier curve is very very roughly a
    // parabola. Hence:
    if fabs(p.x()) > 0.053 - 0.022 * p.y() * p.y() {
        tmerc_exact_e_inv(tmerc, p);
    } else {
        tmerc_approx_e_inv(tmerc, proj, p);
    }
}

fn get_algo_from_params(proj: &Proj, algo: &mut TMercAlgo) -> bool {
    let approx = proj.params.get(&APPROX).unwrap_or(&ProjValue::default()).bool();
    if approx {
        *algo = TMercAlgo::EvendenSnyder;
        return true;
    }

    let algo_str = proj.params.get(&ALGO).unwrap_or(&ProjValue::default()).string();
    if !algo_str.is_empty() {
        if algo_str == "evenden_snyder" {
            *algo = TMercAlgo::EvendenSnyder;
            return true;
        }
        if algo_str == "poder_engsager" {
            *algo = TMercAlgo::PoderEngsager;
            return true;
        }
        if algo_str == "auto" {
            *algo = TMercAlgo::Auto;
            // Don't return so that we can run a later validity check
        } else {
            panic!("unknown value for +algo");
        }
    }

    // We haven't worked on the criterion on inverse transformation
    // when phi0 != 0 or if k0 is not close to 1 or for very oblate
    // ellipsoid (es > 0.1 is ~ rf < 200)
    if *algo == TMercAlgo::Auto && (proj.es > 0.1 || proj.phi0 != 0. || fabs(proj.k0 - 1.) > 0.01) {
        *algo = TMercAlgo::PoderEngsager;
    }

    true
}

fn setup(proj: &mut Proj, e_alg: &mut TMercAlgo) -> (TmercData, TMercMode) {
    let mut tmerc = TmercData::default();

    if proj.es == 0. {
        *e_alg = TMercAlgo::EvendenSnyder;
    }
    let mode = match *e_alg {
        TMercAlgo::EvendenSnyder => {
            setup_approx(&mut tmerc, proj);
            if proj.es == 0. { TMercMode::Spherical } else { TMercMode::ApproxEllipsoidal }
        }
        TMercAlgo::PoderEngsager => {
            setup_exact(&mut tmerc, proj);
            TMercMode::ExactEllipsoidal
        }
        TMercAlgo::Auto => {
            setup_approx(&mut tmerc, proj);
            setup_exact(&mut tmerc, proj);
            TMercMode::AutoEllipsoidal
        }
    };

    (tmerc, mode)
}

/// Transverse Mercator
pub type TransverseMercatorProjection = TransverseMercatorBaseProjection<TRANSVERSE_MERCATOR>;
/// Transverse Mercator (South Oriented)
pub type TransverseMercatorSouthOrientedProjection =
    TransverseMercatorBaseProjection<TRANSVERSE_MERCATOR_SOUTH_ORIENTATED>;

/// Transverse Mercator
///
/// Note: exact transverse mercator only exists in ellipsoidal form,
/// use approximate version if +a sphere is requested
#[derive(Debug, Clone, PartialEq)]
pub struct TransverseMercatorBaseProjection<const C: i64> {
    proj: RefCell<Proj>,
    mode: TMercMode,
    store: RefCell<TmercData>,
    algo: RefCell<TMercAlgo>,
}
impl<const C: i64> ProjectCoordinates for TransverseMercatorBaseProjection<C> {
    fn code(&self) -> i64 {
        C
    }
    fn name(&self) -> &'static str {
        "Transverse Mercator"
    }
    fn names() -> &'static [&'static str] {
        &["Transverse Mercator", "Transverse Mercator (South Oriented)", "tmerc"]
    }
}
impl<const C: i64> CoordinateStep for TransverseMercatorBaseProjection<C> {
    fn new(proj: RefCell<Proj>) -> Self {
        let mut algo = TMercAlgo::default();
        get_algo_from_params(&proj.borrow(), &mut algo);
        let (store, mode) = setup(&mut proj.borrow_mut(), &mut algo);
        TransverseMercatorBaseProjection { proj, mode, store: store.into(), algo: algo.into() }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        match self.mode {
            TMercMode::Spherical => {
                tmerc_spherical_fwd(&mut self.store.borrow_mut(), &self.proj.borrow(), p)
            }
            TMercMode::ApproxEllipsoidal => {
                tmerc_approx_e_fwd(&mut self.store.borrow_mut(), &self.proj.borrow(), p)
            }
            TMercMode::ExactEllipsoidal => tmerc_exact_e_fwd(&mut self.store.borrow_mut(), p),
            TMercMode::AutoEllipsoidal => {
                tmerc_auto_e_fwd(&mut self.store.borrow_mut(), &self.proj.borrow(), p)
            }
        }
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        match self.mode {
            TMercMode::Spherical => {
                tmerc_spherical_inv(&mut self.store.borrow_mut(), &self.proj.borrow(), p)
            }
            TMercMode::ApproxEllipsoidal => {
                tmerc_approx_e_inv(&mut self.store.borrow_mut(), &self.proj.borrow(), p)
            }
            TMercMode::ExactEllipsoidal => tmerc_exact_e_inv(&mut self.store.borrow_mut(), p),
            TMercMode::AutoEllipsoidal => {
                tmerc_auto_e_inv(&mut self.store.borrow_mut(), &self.proj.borrow(), p)
            }
        }
    }
}

/// Extended Transverse Mercator
#[derive(Debug, Clone, PartialEq)]
pub struct ExtendedTransverseMercatorProjection {
    proj: RefCell<Proj>,
    mode: TMercMode,
    store: RefCell<TmercData>,
    algo: RefCell<TMercAlgo>,
}
impl ProjectCoordinates for ExtendedTransverseMercatorProjection {
    fn code(&self) -> i64 {
        -1
    }
    fn name(&self) -> &'static str {
        "Extended Transverse Mercator"
    }
    fn names() -> &'static [&'static str] {
        &["Extended Transverse Mercator", "etmerc"]
    }
}
impl CoordinateStep for ExtendedTransverseMercatorProjection {
    fn new(proj: RefCell<Proj>) -> Self {
        if proj.borrow().es == 0.0 {
            panic!("Invalid value for eccentricity: it should not be zero");
        }
        let mut algo = TMercAlgo::PoderEngsager;
        get_algo_from_params(&proj.borrow(), &mut algo);
        let (store, mode) = setup(&mut proj.borrow_mut(), &mut algo);
        ExtendedTransverseMercatorProjection { proj, mode, store: store.into(), algo: algo.into() }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        match self.mode {
            TMercMode::Spherical => {
                tmerc_spherical_fwd(&mut self.store.borrow_mut(), &self.proj.borrow(), p)
            }
            TMercMode::ApproxEllipsoidal => {
                tmerc_approx_e_fwd(&mut self.store.borrow_mut(), &self.proj.borrow(), p)
            }
            TMercMode::ExactEllipsoidal => tmerc_exact_e_fwd(&mut self.store.borrow_mut(), p),
            TMercMode::AutoEllipsoidal => {
                tmerc_auto_e_fwd(&mut self.store.borrow_mut(), &self.proj.borrow(), p)
            }
        }
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        match self.mode {
            TMercMode::Spherical => {
                tmerc_spherical_inv(&mut self.store.borrow_mut(), &self.proj.borrow(), p)
            }
            TMercMode::ApproxEllipsoidal => {
                tmerc_approx_e_inv(&mut self.store.borrow_mut(), &self.proj.borrow(), p)
            }
            TMercMode::ExactEllipsoidal => tmerc_exact_e_inv(&mut self.store.borrow_mut(), p),
            TMercMode::AutoEllipsoidal => {
                tmerc_auto_e_inv(&mut self.store.borrow_mut(), &self.proj.borrow(), p)
            }
        }
    }
}

/// Universal Transverse Mercator
/// UTM uses the Poder/Engsager implementation for the underlying projection
/// UNLESS +approx is set in which case the Evenden/Snyder implementation is used.
#[derive(Debug, Clone, PartialEq)]
pub struct UniversalTransverseMercatorProjection {
    proj: RefCell<Proj>,
    mode: TMercMode,
    store: RefCell<TmercData>,
    algo: RefCell<TMercAlgo>,
}
impl ProjectCoordinates for UniversalTransverseMercatorProjection {
    fn code(&self) -> i64 {
        -1
    }
    fn name(&self) -> &'static str {
        "Universal Transverse Mercator"
    }
    fn names() -> &'static [&'static str] {
        &["Universal Transverse Mercator", "Universal Transverse Mercator (UTM)", "utm"]
    }
}
impl CoordinateStep for UniversalTransverseMercatorProjection {
    fn new(proj: RefCell<Proj>) -> Self {
        {
            let proj = &mut proj.borrow_mut();
            if proj.es == 0.0 {
                panic!("Invalid value for eccentricity: it should not be zero");
            }

            if proj.lam0 < -1000.0 || proj.lam0 > 1000.0 {
                panic!("Invalid value for lon_0");
            }

            if proj.params.contains_key(&SOUTH) {
                proj.y0 = 10000000.
            } else {
                proj.y0 = 0.
            }
            proj.x0 = 500000.;
            // zone input ?
            let zone = if let Some(zone) = proj.params.get(&ZONE) {
                let mut zone = zone.i64();
                if zone > 0 && zone <= 60 {
                    zone -= 1;
                } else {
                    panic!("Invalid value for zone");
                }
                zone
            } else {
                // nearest central meridian input
                let mut zone = round(floor((adjlon(proj.lam0) + PI) * 30. / PI)) as i64;
                if zone < 0 {
                    zone = 0;
                } else if zone >= 60 {
                    zone = 59;
                }
                zone
            };
            proj.lam0 = ((zone as f64) + 0.5) * PI / 30. - PI;
            proj.k0 = 0.9996;
            proj.phi0 = 0.;
        }
        let mut algo = TMercAlgo::PoderEngsager;
        get_algo_from_params(&proj.borrow(), &mut algo);
        let (store, mode) = setup(&mut proj.borrow_mut(), &mut algo);
        UniversalTransverseMercatorProjection { proj, mode, store: store.into(), algo: algo.into() }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        match self.mode {
            TMercMode::Spherical => {
                tmerc_spherical_fwd(&mut self.store.borrow_mut(), &self.proj.borrow(), p)
            }
            TMercMode::ApproxEllipsoidal => {
                tmerc_approx_e_fwd(&mut self.store.borrow_mut(), &self.proj.borrow(), p)
            }
            TMercMode::ExactEllipsoidal => tmerc_exact_e_fwd(&mut self.store.borrow_mut(), p),
            TMercMode::AutoEllipsoidal => {
                tmerc_auto_e_fwd(&mut self.store.borrow_mut(), &self.proj.borrow(), p)
            }
        }
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        match self.mode {
            TMercMode::Spherical => {
                tmerc_spherical_inv(&mut self.store.borrow_mut(), &self.proj.borrow(), p)
            }
            TMercMode::ApproxEllipsoidal => {
                tmerc_approx_e_inv(&mut self.store.borrow_mut(), &self.proj.borrow(), p)
            }
            TMercMode::ExactEllipsoidal => tmerc_exact_e_inv(&mut self.store.borrow_mut(), p),
            TMercMode::AutoEllipsoidal => {
                tmerc_auto_e_inv(&mut self.store.borrow_mut(), &self.proj.borrow(), p)
            }
        }
    }
}
