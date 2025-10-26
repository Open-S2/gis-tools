use super::Proj;
use crate::proj::{Complex, CoordinateStep, Coords, TransformCoordinates};
use alloc::{vec, vec::Vec};
use core::f64::consts::{FRAC_PI_2, PI, TAU};
use libm::{acos, asin, atan, atan2, atanh, cos, exp, fabs, floor, fmax, sin, sinh, sqrt};

const ONE_TOL: f64 = 1.00000000000001;
const ATOL: f64 = 1e-50;

/// List of auxiliary latitudes
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum AuxLat {
    /// Geographic latitude, phi
    GEOGRAPHIC = 0,
    /// Parametric latitude, beta
    PARAMETRIC = 1,
    /// Geocentric latitude, theta
    GEOCENTRIC = 2,
    /// Rectifying latitude, mu
    RECTIFYING = 3,
    /// Conformal latitude, chi
    CONFORMAL = 4,
    /// Authlatic latitude, xi
    AUTHALIC = 5,
    /// Number of auxiliary latitudes (6)
    NUMBER = 6,
}
impl AuxLat {
    /// The order of the expansion in n (ACCIDENTALLY equal to AUXNUMBER)
    pub const ORDER: AuxLat = AuxLat::NUMBER;
}

/// Convert tau' = sinh(psi) = tan(chi) to tau = tan(phi).  The code is taken
/// from GeographicLib::Math::tauf(taup, e).
///
/// Here
///   phi = geographic latitude (radians)
/// psi is the isometric latitude
///   psi = asinh(tan(phi)) - e * atanh(e * sin(phi))
///       = asinh(tan(chi))
/// chi is the conformal latitude
///
/// The representation of latitudes via their tangents, tan(phi) and
/// tan(chi), maintains full *relative* accuracy close to latitude = 0 and
/// +/- pi/2. This is sometimes important, e.g., to compute the scale of the
/// transverse Mercator projection which involves cos(phi)/cos(chi) tan(phi)
///
/// From Karney (2011), Eq. 7,
///
///   tau' = sinh(psi) = sinh(asinh(tan(phi)) - e * atanh(e * sin(phi)))
///        = tan(phi) * cosh(e * atanh(e * sin(phi))) -
///          sec(phi) * sinh(e * atanh(e * sin(phi)))
///        = tau * sqrt(1 + sigma^2) - sqrt(1 + tau^2) * sigma
/// where
///   sigma = sinh(e * atanh( e * tau / sqrt(1 + tau^2) ))
///
/// For e small,
///
///    tau' = (1 - e^2) * tau
///
/// The relation tau'(tau) can therefore by reliably inverted by Newton's
/// method with
///
///    tau = tau' / (1 - e^2)
///
/// as an initial guess.  Newton's method requires dtau'/dtau.  Noting that
///
///   dsigma/dtau = e^2 * sqrt(1 + sigma^2) /
///                 (sqrt(1 + tau^2) * (1 + (1 - e^2) * tau^2))
///   d(sqrt(1 + tau^2))/dtau = tau / sqrt(1 + tau^2)
///
/// we have
///
///   dtau'/dtau = (1 - e^2) * sqrt(1 + tau'^2) * sqrt(1 + tau^2) /
///                (1 + (1 - e^2) * tau^2)
///
/// This works fine unless tau^2 and tau'^2 overflows.  This may be partially
/// cured by writing, e.g., sqrt(1 + tau^2) as hypot(1, tau).  However, nan
/// will still be generated with tau' = inf, since (inf - inf) will appear in
/// the Newton iteration.
///
/// If we note that for sufficiently large |tau|, i.e., |tau| >= 2/sqrt(eps),
/// sqrt(1 + tau^2) = |tau| and
///
///   tau' = exp(- e * atanh(e)) * tau
///
/// So
///
///   tau = exp(e * atanh(e)) * tau'
///
/// can be returned unless |tau| >= 2/sqrt(eps); this then avoids overflow
/// problems for large tau' and returns the correct result for tau' = +/-inf
/// and nan.
///
/// Newton's method usually take 2 iterations to converge to double precision
/// accuracy (for WGS84 flattening).  However only 1 iteration is needed for
/// |chi| < 3.35 deg.  In addition, only 1 iteration is needed for |chi| >
/// 89.18 deg (tau' > 70), if tau = exp(e * atanh(e)) * tau' is used as the
/// starting guess.
///
/// For small flattening, |f| <= 1/50, the series expansion in n can be
/// used:
///
/// Assuming n = e^2 / (1 + sqrt(1 - e^2))^2 is passed as an argument
///
///   double F[int(AuxLat::AUXORDER)];
///   auxlat_coeffs(n, AuxLat::CONFORMAL, AuxLat::GEOGRAPHIC, F);
///   double sphi, cphi;
///   //                schi                   cchi
///   auxlat_convert(taup/hypot(1.0, taup), 1/hypot(1.0, taup),
///                     sphi, cphi, F);
///   return sphi/cphi;
pub fn sinhpsi2tanphi(taup: f64, e: f64) -> f64 {
    // min iterations = 1, max iterations = 2; mean = 1.954
    let rooteps: f64 = sqrt(f64::EPSILON);
    let tol: f64 = rooteps / 10.; // the criterion for Newton's method
    let tmax: f64 = 2. / rooteps; // threshold for large arg limit exact
    let e2m: f64 = 1. - e * e;
    let stol: f64 = tol * fmax(1.0, fabs(taup));
    // The initial guess.  70 corresponds to chi = 89.18 deg (see above)
    let mut tau: f64 = if fabs(taup) > 70. { taup * exp(e * atanh(e)) } else { taup / e2m };
    // handles +/-inf and nan and e = 1
    if fabs(tau) >= tmax {
        return tau;
    }
    // If we need to deal with e > 1, then we could include:
    // if (e2m < 0) return std::numeric_limits<double>::quiet_NaN();
    let mut i: i32 = 5;
    loop {
        let tau1 = sqrt(1. + tau * tau);
        let sig = sinh(e * atanh(e * tau / tau1));
        let taupa = sqrt(1. + sig * sig) * tau - sig * tau1;
        let dtau =
            (taup - taupa) * (1. + e2m * (tau * tau)) / (e2m * tau1 * sqrt(1. + taupa * taupa));
        tau += dtau;
        // backwards test to allow nans to succeed.
        if fabs(dtau) < stol {
            break;
        }
        i -= 1;
        if i == 0 {
            panic!("Newton's method failed to converge");
        }
    }

    tau
}

/// Determine latitude angle phi-2.
/// Inputs:
///   ts = exp(-psi) where psi is the isometric latitude (dimensionless)
///        this variable is defined in Snyder (1987), Eq. (7-10)
///   e = eccentricity of the ellipsoid (dimensionless)
/// Output:
///   phi = geographic latitude (radians)
/// Here isometric latitude is defined by
///   psi = log( tan(pi/4 + phi/2) *
///              ( (1 - e*sin(phi)) / (1 + e*sin(phi)) )^(e/2) )
///       = asinh(tan(phi)) - e * atanh(e * sin(phi))
///       = asinh(tan(chi))
///   chi = conformal latitude
///
/// This routine converts t = exp(-psi) to
///
///   tau' = tan(chi) = sinh(psi) = (1/t - t)/2
///
/// returns atan(sinpsi2tanphi(tau'))
pub fn phi2(ts0: f64, e: f64) -> f64 {
    atan(sinhpsi2tanphi((1. / ts0 - ts0) / 2., e))
}

/// Compute the meridian scale factor
pub fn _msfn(sinphi: f64, cosphi: f64, es: f64) -> f64 {
    cosphi / sqrt(1. - es * sinphi * sinphi)
}

/// Compute the meridian scale factor
pub fn msfn(phi: f64, e2: f64) -> f64 {
    let sinphi = sin(phi);
    let cosphi = cos(phi);
    _msfn(sinphi, cosphi, e2)
}

/// Compute the geographic latitude from beta = authalic_latitude
/// where APA = pj_compute_coefficients_for_inverse_authalic_lat() and
/// qp = pj_authalic_lat_q(1, proj.e, proj.one_es)
pub fn authalic_lat_inverse(beta: f64, apa: &[f64], proj: &Proj, qp: f64) -> f64 {
    let mut phi = auxlat_convert(beta, apa, AuxLat::ORDER as i32);
    if authalic_series_valid(proj.n) {
        return phi;
    }
    // If the flattening is large, solve
    //   f(phi) = qp*sin(beta)/(1-e^2) - q(phi)/(1-e^2) = 0
    // for phi, using Newton's method, where
    //   q(phi)/(1-e^2) = sin(phi)/(1 - e^2*sin(phi)^2) + atanh(e*sin(phi))/e
    // and
    //   df(phi)/dphi = - dq(phi)/dphi / (1-e^2)
    //                = - 2 * (1-e^2) * cos(phi) / (1 - e^2*sinphi^2)^2
    // This is subject to large roundoff errors near the poles, so only use
    // this if the series isn't accurate.
    let q = sin(beta) * qp;
    let q_div_one_minus_es = q / proj.one_es;
    for _ in 0..10 {
        let sinphi = sin(phi);
        let cosphi = cos(phi);
        let one_minus_es_sin2phi = 1. - proj.es * (sinphi * sinphi);
        // Snyder uses 0.5 * ln((1-e*sinphi)/(1+e*sinphi) which is
        // -atanh(e*sinphi)
        let dphi = (one_minus_es_sin2phi * one_minus_es_sin2phi) / (2. * cosphi)
            * (q_div_one_minus_es
                - sinphi / one_minus_es_sin2phi
                - atanh(proj.e * sinphi) / proj.e);
        if fabs(dphi) < 1e-15 {
            break;
        }
        phi += dphi;
    }
    phi
}

/// Convert auxiliary latitude zeta to eta, given coefficients F produced by
/// pj_auxlats_coeffs(n, zeta, eta, F).  K is the size of F (defaults to
/// AuxLat::ORDER).
pub fn auxlat_convert(zeta: f64, f: &[f64], k: i32) -> f64 {
    auxlat_convert_mid(zeta, sin(zeta), cos(zeta), f, k)
}

/// Convert auxiliary latitude zeta to eta, given coefficients F produced by
/// pj_auxlats_coeffs(n, zeta, eta, F).  In this signature, szeta and czeta (the
/// sine and cosine of zeta) are given as inputs.  K is the size of F (defaults
/// to AuxLat::ORDER).
pub fn auxlat_convert_mid(zeta: f64, szeta: f64, czeta: f64, f: &[f64], k: i32) -> f64 {
    zeta + clenshaw(szeta, czeta, f, k)
}

/// Convert auxiliary latitude zeta to eta, given coefficients F produced by
/// pj_auxlats_coeffs(n, zeta, eta, F).  K is the size of F (defaults to
/// AuxLat::ORDER).  In this signature, the sine and cosine of eta are returned.
/// This provides high relative accuracy near the poles.
pub fn auxlat_convert_full(
    szeta: f64,
    czeta: f64,
    seta: &mut f64,
    ceta: &mut f64,
    f: &[f64],
    k: i32,
) {
    let delta: f64 = clenshaw(szeta, czeta, f, k);
    let sdelta = sin(delta);
    let cdelta = cos(delta);
    *seta = szeta * cdelta + czeta * sdelta;
    *ceta = czeta * cdelta - szeta * sdelta;
}

/// Compute the rectifying radius = quarter meridian / (pi/2 * a).  The accuracy
/// of this series is the same as those used for the computation of the
/// auxiliary latitudes.
pub fn rectifying_radius(n: f64) -> f64 {
    // Expansion of (quarter meridian) / ((a+b)/2 * pi/2) as series in n^2;
    // these coefficients are ( (2*k - 3)!! / (2*k)!! )^2 for k = 0..3
    // static const double coeff_rad[] = {1, 1.0 / 4, 1.0 / 64, 1.0 / 256};
    let coeff_rad = vec![1., 1. / 4., 1. / 64., 1. / 256.];
    // return pj_polyval(n * n, coeff_rad, 3) / (1 + n);
    polyval(n * n, &coeff_rad, 3) / (1. + n)
}

/// meridional distance for ellipsoid and inverse using 6th-order expansion in
/// the third flattening n.  This gives full double precision accuracy for |f|
/// <= 1/150.
pub fn enfn(n: f64) -> Vec<f64> {
    let l_max = AuxLat::ORDER as usize;
    // 2*l_max for the Fourier coeffs for each direction of conversion + 1 for
    // overall multiplier.
    // double *en;
    // en = (double *)malloc((2 * l_max + 1) * sizeof(double));
    let mut en = vec![0.; 2 * l_max + 1];
    en[0] = rectifying_radius(n);
    auxlat_coeffs(n, AuxLat::GEOGRAPHIC, AuxLat::RECTIFYING, en[1..].as_mut());
    auxlat_coeffs(n, AuxLat::RECTIFYING, AuxLat::GEOGRAPHIC, en[1 + l_max..].as_mut());
    en
}

/// meridional distance for ellipsoid and inverse using 6th-order expansion in
/// the third flattening n.
pub fn mlfn(phi: f64, sphi: f64, cphi: f64, en: &[f64]) -> f64 {
    en[0] * auxlat_convert_mid(phi, sphi, cphi, en[1..].as_ref(), 0)
}

/// inverse meridional distance
pub fn inv_mlfn(mu: f64, en: &[f64]) -> f64 {
    let l_max = AuxLat::ORDER as usize;
    auxlat_convert(mu / en[0], &en[(1 + l_max)..], AuxLat::ORDER as i32)
}

/// Evaluate `y = sum(F[k] * sin((2*k+2) * zeta), k, 0, K-1)` by Clenshaw
/// summation. zeta is specify by its sine and cosine, szeta and czeta.
pub fn clenshaw(szeta: f64, czeta: f64, f: &[f64], mut k: i32) -> f64 {
    // Approx operation count = (K + 5) mult and (2 * K + 2) add
    let mut u0 = 0.; // accumulator for sum
    let mut u1 = 0.; // accumulator for sum
    let x = 2. * (czeta - szeta) * (czeta + szeta); // 2 * cos(2*zeta)
    while k > 0 {
        k -= 1;
        let t = x * u0 - u1 + f.get(k as usize).copied().unwrap_or(0.0);
        u1 = u0;
        u0 = t;
    }
    2. * szeta * czeta * u0 // sin(2*zeta) * u0
}

/// Computes coefficient q such that authalic_latitude = beta = asin(q / qp)
/// where qp is q at phi=90deg, i.e. qp = authalic_lat_q(1, e, one_es)
/// Cf  Snyder (3-11) and (3-12)
pub fn authalic_lat_q(sinphi: f64, proj: &Proj) -> f64 {
    if proj.e >= 1e-7 {
        let e_sinphi = proj.e * sinphi;
        let one_minus_e_sinphi_sq = 1.0 - e_sinphi * e_sinphi;

        // avoid zero division, fail gracefully
        if one_minus_e_sinphi_sq == 0.0 {
            return f64::INFINITY;
        }

        // Snyder uses 0.5 * ln((1-e*sinphi)/(1+e*sinphi) which is -atanh(e*sinphi)
        proj.one_es * (sinphi / one_minus_e_sinphi_sq + atanh(e_sinphi) / proj.e)
    } else {
        2. * sinphi
    }
}

/// Computes coefficients needed for conversions between geographic and authalic
/// latitude.  These are preferred over the analytical expressions for |n| <
/// 0.01.  However the inverse series is used to start the inverse method for
/// large |n|.
/// Ensure we use the cutoff in n consistently
pub fn authalic_lat_compute_coeffs(n: f64) -> Vec<f64> {
    let l_max = AuxLat::ORDER as usize;
    let apa_size = l_max * if authalic_series_valid(n) { 2 } else { 1 };
    let mut apa: Vec<f64> = vec![0.; apa_size];
    auxlat_coeffs(n, AuxLat::AUTHALIC, AuxLat::GEOGRAPHIC, &mut apa);
    if authalic_series_valid(n) {
        auxlat_coeffs(n, AuxLat::GEOGRAPHIC, AuxLat::AUTHALIC, &mut apa[l_max..]);
    }

    apa
}

/// Computes authalic latitude from the geographic latitude.
/// qp is q at phi=90deg, i.e. qp = pj_authalic_lat_q(1, e, one_es)
pub fn authalic_lat(phi: f64, sinphi: f64, cosphi: f64, apa: &[f64], proj: &Proj, qp: f64) -> f64 {
    if authalic_series_valid(proj.n) {
        let l_max = AuxLat::ORDER as usize;
        auxlat_convert_mid(phi, sinphi, cosphi, &apa[l_max..], 0)
    } else {
        // This result is ill-conditioned near the poles.  So don't use this if
        // the series are accurate.
        let q = authalic_lat_q(sinphi, proj);
        let mut ratio = q / qp;

        if fabs(ratio) > 1. {
            // Rounding error.
            ratio = if ratio > 0. { 1. } else { -1. };
        }
        asin(ratio)
    }
}

/// Ensure we use the cutoff in n consistently
pub fn authalic_series_valid(n: f64) -> bool {
    fabs(n) < 0.01
}

/// The following routines auxlat_coeffs, polyvol, clenshaw,
/// auxlat_convert (3 signatures) provide a uniform interface for converting
/// between any pair of auxiliary latitudes using series expansions in the third
/// flattening, n.  There are 6 (= AuxLat::NUMBER) auxiliary latitudes
/// supported labeled by
///
///   AuxLat::GEOGRAPHIC for geographic latitude, phi
///   AuxLat::PARAMETRIC for parametric latitude, beta
///   AuxLat::GEOCENTRIC for geocentric latitude, theta
///   AuxLat::RECTIFYING for rectifying latitude, mu
///   AuxLat::CONFORMAL for conformal latitude, chi
///   AuxLat::AUTHALIC for authlatic latitude, xi
///
/// This is adapted from
///
///   C. F. F. Karney, On auxiliary latitudes,
///   Survey Review 56, 165-180 (2024)
///   <https://doi.org/10.1080/00396265.2023.2217604>
///   Preprint: <https://arxiv.org/abs/2212.05818>
///
/// The typical calling sequence is
///
///   `constexpr int L = int(AuxLat::ORDER);`
///   // Managing the memory for the coefficent array is the
///   // responibility of the calling routine.
///   `double F[L];`
///   // Fill F[] with coefficients to convert conformal to geographic
///   `auxlat_coeffs(proj.n, AuxLat::CONFORMAL, AuxLat::GEOGRAPHIC, F);`
///   ...
///   `double chi = 1;`                 // known conformal latitude
///   // compute corresponding geographic latitude
///   `double phi = auxlat_convert(chi, F);`
///
/// The conversions are Fourier series in the auxiliary latitude where each
/// coefficient is given as a Taylor series in n truncated at order 6 (=
/// AuxLat::ORDER).  This suffices to give full double precision accuracy for
/// |f| <= 1/150 and probably provide satisfactory results for |f| <= 1/50.  The
/// coefficients for these Taylor series are given by matrics listed in
/// Eqs. (A1-A28) of this paper.
///
/// These coefficients are bundled up into a single array coeffs in
/// auxlat_coeffs.  Only the upper triangular portion of the matrices are
/// included.  Furthermore, half the coefficients for the conversions between
/// any of phi, bete, theta, and mu are zero (the Taylor series are expansions
/// in n^2), these zero elements are excluded.
///
/// The coefficent array, coeffs, is machine-generated by the Maxima code
/// auxlat.mac bundled with GeographicLib.  To use
///
/// * Ensure that l_max (set near the top of the file) is set to 6 (=
///   AuxLat::ORDER).
/// * run
///   $ maxima
///   Maxima 5.47.0 <https://maxima.sourceforge.io>
///   (%i1) load("auxlat.mac")$
///   (%i2) writecppproj()$
///   ....
///   "CLOSED OUTPUT BUFFERED FILE-STREAM CHARACTER auxvalsproj6.cpp"
/// * The results are in the file auxvalsproj6.cpp
///
/// Only a subset of the conversion matrices are written out.  To add others,
/// include them in the list "required" in writecppproj().  The conversions
/// currently supported are
///
///   phi <-> mu for meridian distance
///   phi <-> chi for tmerc
///   phi <-> xi for authalic latitude conversions
///   chi <-> mu for tmerc
///
/// Because all the matrices are concatenated together into a single array,
/// coeff, an auxiliary array, ptrs, or length 37 = AUXNUMBER^2 + 1, is written
/// out to give the starting point of any particular matrix.
///
/// Input:
///   n -- the third flattening (a-b)/(a+b)
///   auxin, auxout -- compute the coefficients for converting auxin (zeta) to
///     auxout (eta).
/// Output:
///   F -- `F[eta,zeta] = C[eta,zeta]`. P(n), where C is a matrix of constants
///     and `P(n) = [n, n^2, n^3, ...]^T`; the first AuxLat::ORDER elements of F
///     are filled.
pub fn auxlat_coeffs(n: f64, auxin: AuxLat, auxout: AuxLat, out: &mut [f64]) {
    // Generated by Maxima on 2025-03-23 19:13:00-04:00 (rewritten for rust)
    let l_max = 6;
    let coeffs: [f64; 150] = [
        // C[phi,phi] skipped
        // C[phi,beta] skipped
        // C[phi,theta] skipped
        // C[phi,mu]; even coeffs only
        3.0 / 2.0,
        -27.0 / 32.0,
        269.0 / 512.0,
        21.0 / 16.0,
        -55.0 / 32.0,
        6759.0 / 4096.0,
        151.0 / 96.0,
        -417.0 / 128.0,
        1097.0 / 512.0,
        -15543.0 / 2560.0,
        8011.0 / 2560.0,
        293393.0 / 61440.0,
        // C[phi,chi]
        2.0,
        -2.0 / 3.0,
        -2.0,
        116.0 / 45.0,
        26.0 / 45.0,
        -2854.0 / 675.0,
        7.0 / 3.0,
        -8.0 / 5.0,
        -227.0 / 45.0,
        2704.0 / 315.0,
        2323.0 / 945.0,
        56.0 / 15.0,
        -136.0 / 35.0,
        -1262.0 / 105.0,
        73814.0 / 2835.0,
        4279.0 / 630.0,
        -332.0 / 35.0,
        -399572.0 / 14175.0,
        4174.0 / 315.0,
        -144838.0 / 6237.0,
        601676.0 / 22275.0,
        // C[phi,xi]
        4.0 / 3.0,
        4.0 / 45.0,
        -16.0 / 35.0,
        -2582.0 / 14175.0,
        60136.0 / 467775.0,
        28112932.0 / 212837625.0,
        46.0 / 45.0,
        152.0 / 945.0,
        -11966.0 / 14175.0,
        -21016.0 / 51975.0,
        251310128.0 / 638512875.0,
        3044.0 / 2835.0,
        3802.0 / 14175.0,
        -94388.0 / 66825.0,
        -8797648.0 / 10945935.0,
        6059.0 / 4725.0,
        41072.0 / 93555.0,
        -1472637812.0 / 638512875.0,
        768272.0 / 467775.0,
        455935736.0 / 638512875.0,
        4210684958.0 / 1915538625.0,
        // C[beta,phi] skipped
        // C[beta,beta] skipped
        // C[beta,theta] skipped
        // C[beta,mu] skipped
        // C[beta,chi] skipped
        // C[beta,xi] skipped
        // C[theta,phi] skipped
        // C[theta,beta] skipped
        // C[theta,theta] skipped
        // C[theta,mu] skipped
        // C[theta,chi] skipped
        // C[theta,xi] skipped
        // C[mu,phi]; even coeffs only
        -3.0 / 2.0,
        9.0 / 16.0,
        -3.0 / 32.0,
        15.0 / 16.0,
        -15.0 / 32.0,
        135.0 / 2048.0,
        -35.0 / 48.0,
        105.0 / 256.0,
        315.0 / 512.0,
        -189.0 / 512.0,
        -693.0 / 1280.0,
        1001.0 / 2048.0,
        // C[mu,beta] skipped
        // C[mu,theta] skipped
        // C[mu,mu] skipped
        // C[mu,chi]
        1.0 / 2.0,
        -2.0 / 3.0,
        5.0 / 16.0,
        41.0 / 180.0,
        -127.0 / 288.0,
        7891.0 / 37800.0,
        13.0 / 48.0,
        -3.0 / 5.0,
        557.0 / 1440.0,
        281.0 / 630.0,
        -1983433.0 / 1935360.0,
        61.0 / 240.0,
        -103.0 / 140.0,
        15061.0 / 26880.0,
        167603.0 / 181440.0,
        49561.0 / 161280.0,
        -179.0 / 168.0,
        6601661.0 / 7257600.0,
        34729.0 / 80640.0,
        -3418889.0 / 1995840.0,
        212378941.0 / 319334400.0,
        // C[mu,xi] skipped
        // C[chi,phi]
        -2.0,
        2.0 / 3.0,
        4.0 / 3.0,
        -82.0 / 45.0,
        32.0 / 45.0,
        4642.0 / 4725.0,
        5.0 / 3.0,
        -16.0 / 15.0,
        -13.0 / 9.0,
        904.0 / 315.0,
        -1522.0 / 945.0,
        -26.0 / 15.0,
        34.0 / 21.0,
        8.0 / 5.0,
        -12686.0 / 2835.0,
        1237.0 / 630.0,
        -12.0 / 5.0,
        -24832.0 / 14175.0,
        -734.0 / 315.0,
        109598.0 / 31185.0,
        444337.0 / 155925.0,
        // C[chi,beta] skipped
        // C[chi,theta] skipped
        // C[chi,mu]
        -1.0 / 2.0,
        2.0 / 3.0,
        -37.0 / 96.0,
        1.0 / 360.0,
        81.0 / 512.0,
        -96199.0 / 604800.0,
        -1.0 / 48.0,
        -1.0 / 15.0,
        437.0 / 1440.0,
        -46.0 / 105.0,
        1118711.0 / 3870720.0,
        -17.0 / 480.0,
        37.0 / 840.0,
        209.0 / 4480.0,
        -5569.0 / 90720.0,
        -4397.0 / 161280.0,
        11.0 / 504.0,
        830251.0 / 7257600.0,
        -4583.0 / 161280.0,
        108847.0 / 3991680.0,
        -20648693.0 / 638668800.0,
        // C[chi,chi] skipped
        // C[chi,xi] skipped
        // C[xi,phi]
        -4.0 / 3.0,
        -4.0 / 45.0,
        88.0 / 315.0,
        538.0 / 4725.0,
        20824.0 / 467775.0,
        -44732.0 / 2837835.0,
        34.0 / 45.0,
        8.0 / 105.0,
        -2482.0 / 14175.0,
        -37192.0 / 467775.0,
        -12467764.0 / 212837625.0,
        -1532.0 / 2835.0,
        -898.0 / 14175.0,
        54968.0 / 467775.0,
        100320856.0 / 1915538625.0,
        6007.0 / 14175.0,
        24496.0 / 467775.0,
        -5884124.0 / 70945875.0,
        -23356.0 / 66825.0,
        -839792.0 / 19348875.0,
        570284222.0 / 1915538625.0,
        // C[xi,beta] skipped
        // C[xi,theta] skipped
        // C[xi,mu] skipped
        // C[xi,chi] skipped
        // C[xi,xi] skipped
    ];
    let ptrs: [usize; 37] = [
        0, 0, 0, 0, 12, 33, 54, 54, 54, 54, 54, 54, 54, 54, 54, 54, 54, 54, 54, 66, 66, 66, 66, 87,
        87, 108, 108, 108, 129, 129, 129, 150, 150, 150, 150, 150, 150,
    ];
    assert!(
        auxin >= AuxLat::GEOGRAPHIC
            && auxin < AuxLat::NUMBER
            && auxout >= AuxLat::GEOGRAPHIC
            && auxout < AuxLat::NUMBER,
        "Bad specification for auxiliary latitude"
    );
    let k = AuxLat::NUMBER as usize * auxout as usize + auxin as usize;
    let mut o = ptrs[k];
    // if (o == ptrs[k+1])
    //     throw std::out_of_range
    //         ("Unsupported conversion between auxiliary latitudes");
    assert!(o != ptrs[k + 1], "Unsupported conversion between auxiliary latitudes");
    let mut d = n;
    let n2 = n * n;
    if auxin <= AuxLat::RECTIFYING && auxout <= AuxLat::RECTIFYING {
        for l in 0..l_max {
            let m = (l_max - l - 1) / 2; // order of polynomial in n^2
            out[l as usize] = d * polyval(n2, &coeffs[o..], m);
            o += m as usize + 1;
            d *= n;
        }
    } else {
        for l in 0..l_max {
            let m = l_max - l - 1; // order of polynomial in n
            out[l as usize] = d * polyval(n, &coeffs[o..], m);
            o += m as usize + 1;
            d *= n;
        }
    }
    assert!(o == ptrs[k + 1]);
}

/// Evaluation `sum(p[i] * x^i, i, 0, N)` via Horner's method.  N.B. p is of length N+1.
pub fn polyval(x: f64, p: &[f64], mut m: i32) -> f64 {
    let mut y = if m < 0 { 0. } else { p[m as usize] };
    while m > 0 {
        m -= 1;
        y = y * x + p[m as usize];
    }
    y
}

/// Determine function ts(phi) defined in Snyder (1987), Eq. (7-10)
/// Inputs:
///   phi = geographic latitude (radians)
///   e = eccentricity of the ellipsoid (dimensionless)
/// Output:
///   ts = exp(-psi) where psi is the isometric latitude (dimensionless)
///      = 1 / (tan(chi) + sec(chi))
/// Here isometric latitude is defined by
///   psi = log( tan(pi/4 + phi/2) *
///              ( (1 - e*sin(phi)) / (1 + e*sin(phi)) )^(e/2) )
///       = asinh(tan(phi)) - e * atanh(e * sin(phi))
///       = asinh(tan(chi))
///   chi = conformal latitude
pub fn tsfn(phi: f64, sinphi: f64, e: f64) -> f64 {
    let cosphi = cos(phi);
    // exp(-asinh(tan(phi))) = 1 / (tan(phi) + sec(phi))
    //                       = cos(phi) / (1 + sin(phi)) good for phi > 0
    //                       = (1 - sin(phi)) / cos(phi) good for phi < 0
    exp(e * atanh(e * sinphi))
        * (if sinphi > 0. { cosphi / (1. + sinphi) } else { (1. - sinphi) / cosphi })
}

/// Compute (lam, phi) corresponding to input (xy.x, xy.y) for projection P.
///
/// Uses Newton-Raphson method, extended to 2D variables, that is using
/// inversion of the Jacobian 2D matrix of partial derivatives. The derivatives
/// are estimated numerically from the proj.fwd method evaluated at close points.
///
/// Note: thresholds used have been verified to work with adams_ws2 and wink2
///
/// Starts with initial guess provided by user in lp_initial
pub fn generic_inverse_2d<C: CoordinateStep, P: TransformCoordinates>(
    xy: &P,
    step: &C,
    lp: &mut P,
    delta_xy_tolerance: f64,
) {
    let mut deriv_lam_x = 0.;
    let mut deriv_lam_y = 0.;
    let mut deriv_phi_x = 0.;
    let mut deriv_phi_y = 0.;
    for i in 0..15 {
        let mut xy_approx = lp.clone();
        step.forward(&mut xy_approx);
        let delta_x = xy_approx.x() - xy.x();
        let delta_y = xy_approx.y() - xy.y();
        if fabs(delta_x) < delta_xy_tolerance && fabs(delta_y) < delta_xy_tolerance {
            return;
        }

        if i == 0 || fabs(delta_x) > 1e-6 || fabs(delta_y) > 1e-6 {
            // Compute Jacobian matrix (only if we aren't close to the final
            // result to speed things a bit)
            let mut lp2 = Coords::default();
            let d_lam = if lp.lam() > 0. { -1e-6 } else { 1e-6 };
            lp2.set_lam(lp.lam() + d_lam);
            lp2.set_phi(lp.phi());
            step.forward(&mut lp2);
            let mut xy2 = lp2;
            let deriv_x_lam = (xy2.x() - xy_approx.x()) / d_lam;
            let deriv_y_lam = (xy2.y() - xy_approx.y()) / d_lam;

            let d_phi = if lp.phi() > 0. { -1e-6 } else { 1e-6 };
            lp2.set_lam(lp.lam());
            lp2.set_phi(lp.phi() + d_phi);
            step.forward(&mut lp2);
            xy2 = lp2;
            let deriv_x_phi = (xy2.x() - xy_approx.x()) / d_phi;
            let deriv_y_phi = (xy2.y() - xy_approx.y()) / d_phi;

            // Inverse of Jacobian matrix
            let det = deriv_x_lam * deriv_y_phi - deriv_x_phi * deriv_y_lam;
            if det != 0. {
                deriv_lam_x = deriv_y_phi / det;
                deriv_lam_y = -deriv_x_phi / det;
                deriv_phi_x = -deriv_y_lam / det;
                deriv_phi_y = deriv_x_lam / det;
            }
        }

        // Limit the amplitude of correction to avoid overshoots due to
        // bad initial guess
        let delta_lam = (delta_x * deriv_lam_x + delta_y * deriv_lam_y).clamp(-0.3, 0.3);
        lp.set_lam(lp.lam() - delta_lam);
        if lp.lam() < -PI {
            lp.set_lam(-PI);
        } else if lp.lam() > PI {
            lp.set_lam(PI);
        }

        let delta_phi = (delta_x * deriv_phi_x + delta_y * deriv_phi_y).clamp(-0.3, 0.3);
        lp.set_phi(lp.phi() - delta_phi);
        if lp.phi() < -FRAC_PI_2 {
            lp.set_phi(-FRAC_PI_2);
        } else if lp.phi() > FRAC_PI_2 {
            lp.set_phi(FRAC_PI_2);
        }
    }
}

/// Adjust longitude to be in -180..+180 range (but in radians)
pub fn adjlon(mut longitude: f64) -> f64 {
    // Let longitude slightly overshoot, to avoid spurious sign switching at the date line
    if fabs(longitude) < PI + 1e-12 {
        return longitude;
    }
    // adjust to 0..2pi range
    longitude += PI;
    // remove integral # of 'revolutions'*/
    longitude -= TAU * floor(longitude / TAU);
    // adjust back to -pi..pi range
    longitude -= PI;

    longitude
}

/// Adjusted ArcSine
pub fn aasin(v: f64) -> f64 {
    let av = fabs(v);
    if av >= 1. {
        if av > ONE_TOL {
            panic!("Coordinate outside projection domain");
        }
        return if v < 0. { -FRAC_PI_2 } else { FRAC_PI_2 };
    }
    asin(v)
}

/// Adjusted ArcCosine
pub fn aacos(v: f64) -> f64 {
    let av = fabs(v);
    if av >= 1. {
        if av > ONE_TOL {
            panic!("Coordinate outside projection domain");
        }
        return if v < 0. { PI } else { 0. };
    }
    acos(v)
}

/// Adjusted square root
pub fn asqrt(v: f64) -> f64 {
    if v <= 0. { 0. } else { sqrt(v) }
}

/// Adjusted atan2
pub fn aatan2(n: f64, d: f64) -> f64 {
    if fabs(n) < ATOL && fabs(d) < ATOL { 0. } else { atan2(n, d) }
}

/// note: coefficients are always from C_1 to C_n
/// i.e. C_0 == (0., 0)
/// n should always be >= 1 though no checks are made
pub fn zpoly1(z: Complex, c: &[Complex], mut n: usize) -> Complex {
    let mut t;
    let mut a = c[n - 1];

    while n > 0 {
        n -= 1;
        t = a.r;
        a = c[n];
        a.r = a.r + z.r * t - z.i * a.i;
        a.i = a.i + z.r * a.i + z.i * t;
    }

    t = a.r;
    a.r = z.r * t - z.i * a.i;
    a.i = z.r * a.i + z.i * t;

    a
}

/// evaluate complex polynomial and derivative
pub fn zpolyd1(z: Complex, c: &[Complex], mut n: usize, der: &mut Complex) -> Complex {
    let mut t;
    let mut first = true;

    let mut a = c[n - 1];
    let mut b = a;
    while n > 0 {
        n -= 1;
        if first {
            first = false;
        } else {
            t = b.r;
            b.r = a.r + z.r * t - z.i * b.i;
            b.i = a.i + z.r * b.i + z.i * t;
        }
        t = a.r;
        a = c[n];
        a.r = a.r + z.r * t - z.i * a.i;
        a.i = a.i + z.r * a.i + z.i * t;
    }
    t = b.r;
    b.r = a.r + z.r * (t) - z.i * b.i;
    b.i = a.i + z.r * b.i + z.i * t;
    t = a.r;
    a.r = z.r * (t) - z.i * a.i;
    a.i = z.r * a.i + z.i * t;
    *der = b;

    a
}
