use crate::proj::{
    CZECH, CoordinateStep, KROVAK, KROVAK_MODIFIED, KROVAK_MODIFIED_NORTH_ORIENTED,
    KROVAK_NORTH_ORIENTED, LATITUDE_OF_PROJECTION_CENTRE, LONGITUDE_OF_ORIGIN,
    LONGITUDE_OF_PROJECTION_CENTRE, Proj, ProjValue, ProjectCoordinates,
    SCALE_FACTOR_AT_NATURAL_ORIGIN, TransformCoordinates,
};
use alloc::rc::Rc;
use core::{
    cell::RefCell,
    f64::consts::{FRAC_PI_2, FRAC_PI_4},
};
use libm::{asin, atan, atan2, cos, fabs, pow, sin, sqrt, tan};
/*
******************************************************************************
 * A description of the (forward) projection is found in:
 *
 *      Bohuslav Veverka,
 *
 *      KROVAK’S PROJECTION AND ITS USE FOR THE
 *      CZECH REPUBLIC AND THE SLOVAK REPUBLIC,
 *
 *      50 years of the Research Institute of
 *      and the Slovak Republic Geodesy, Topography and Cartography
 *
 * which can be found via the Wayback Machine:
 *
 *      https://web.archive.org/web/20150216143806/https://www.vugtk.cz/odis/sborniky/sb2005/Sbornik_50_let_VUGTK/Part_1-Scientific_Contribution/16-Veverka.pdf
 *
 * Further info, including the inverse projection, is given by EPSG:
 *
 *      Guidance Note 7 part 2
 *      Coordinate Conversions and Transformations including Formulas
 *
 *      http://www.iogp.org/pubs/373-07-2.pdf
 *
 * Variable names in this file mostly follows what is used in the
 * paper by Veverka.
 *
 * According to EPSG the full Krovak projection method should have
 * the following parameters.  Within PROJ the azimuth, and pseudo
 * standard parallel are hardcoded in the algorithm and can't be
 * altered from outside. The others all have defaults to match the
 * common usage with Krovak projection.
 *
 *      lat_0 = latitude of centre of the projection
 *
 *      lon_0 = longitude of centre of the projection
 *
 *      ** = azimuth (true) of the centre line passing through the
 *           centre of the projection
 *
 *      ** = latitude of pseudo standard parallel
 *
 *      k  = scale factor on the pseudo standard parallel
 *
 *      x_0 = False Easting of the centre of the projection at the
 *            apex of the cone
 *
 *      y_0 = False Northing of the centre of the projection at
 *            the apex of the cone
 *
 *****************************************************************************/

const EPS: f64 = 1e-15;
const UQ: f64 = 1.04216856380474; // DU(2, 59, 42, 42.69689)
const S0: f64 = 1.37008346281555; // Latitude of pseudo standard parallel 78deg 30'00" N
// Not sure at all of the appropriate number for MAX_ITER...
const MAX_ITER: usize = 100;

/// Krovak variable data
#[derive(Debug, Default, Clone, PartialEq)]
pub struct KrovakData {
    alpha: f64,
    k: f64,
    n: f64,
    rho0: f64,
    ad: f64,
    // true, in default mode. false when using +czech
    easting_northing: bool,
    modified: bool,
}

const X0: f64 = 1089000.0;
const Y0: f64 = 654000.0;
const C1: f64 = 2.946529277E-02;
const C2: f64 = 2.515965696E-02;
const C3: f64 = 1.193845912E-07;
const C4: f64 = -4.668270147E-07;
const C5: f64 = 9.233980362E-12;
const C6: f64 = 1.523735715E-12;
const C7: f64 = 1.696780024E-18;
const C8: f64 = 4.408314235E-18;
const C9: f64 = -8.331083518E-24;
const C10: f64 = -3.689471323E-24;

/// Correction terms to be applied to regular Krovak to obtain Modified Krovak.
/// Note that x_r is a Southing in metres and y_r a Westing in metres,
/// and output (d_x, d_y) is a corrective term in (Southing, Westing) in metres
/// Reference:
/// https://www.cuzk.cz/Zememerictvi/Geodeticke-zaklady-na-uzemi-CR/GNSS/Nova-realizace-systemu-ETRS89-v-CR/Metodika-prevodu-ETRF2000-vs-S-JTSK-var2(101208).aspx
fn mod_krovak_compute_dx_dy(xr: f64, yr: f64) -> (f64, f64) {
    let x_r2 = xr * xr;
    let y_r2 = yr * yr;
    let x_r4 = x_r2 * x_r2;
    let y_r4 = y_r2 * y_r2;

    let d_x = C1 + C3 * xr - C4 * yr - 2. * C6 * xr * yr
        + C5 * (x_r2 - y_r2)
        + C7 * xr * (x_r2 - 3. * y_r2)
        - C8 * yr * (3. * x_r2 - y_r2)
        + 4. * C9 * xr * yr * (x_r2 - y_r2)
        + C10 * (x_r4 + y_r4 - 6. * x_r2 * y_r2);
    let d_y = C2
        + C3 * yr
        + C4 * xr
        + 2. * C5 * xr * yr
        + C6 * (x_r2 - y_r2)
        + C8 * xr * (x_r2 - 3. * y_r2)
        + C7 * yr * (3. * x_r2 - y_r2)
        - 4. * C10 * xr * yr * (x_r2 - y_r2)
        + C9 * (x_r4 + y_r4 - 6. * x_r2 * y_r2);

    (d_x, d_y)
}

// static PJ *krovak_setup(PJ *P, bool modified) {
fn krovak_setup(proj: &mut Proj, modified: bool) -> KrovakData {
    let mut data = KrovakData::default();

    // we want Bessel as fixed ellipsoid
    proj.a = 6377397.155;
    proj.es = 0.006674372230614;
    proj.e = sqrt(proj.es);

    let lat_0 =
        proj.params.get(&LATITUDE_OF_PROJECTION_CENTRE).unwrap_or(&ProjValue::default()).f64();
    proj.phi0 = if lat_0 == 0. { 0.863937979737193 } else { lat_0 };

    // if center long is not set use 42d30'E of Ferro - 17d40' for Ferro
    // that will correspond to using longitudes relative to greenwich
    // as input and output, instead of lat/long relative to Ferro
    let lon_0 = proj
        .params
        .get(&LONGITUDE_OF_ORIGIN)
        .or_else(|| proj.params.get(&LONGITUDE_OF_PROJECTION_CENTRE))
        .unwrap_or(&ProjValue::default())
        .f64();
    proj.lam0 = if lon_0 == 0. { 0.7417649320975901 - 0.308341501185665 } else { lon_0 };

    // if scale not set default to 0.9999
    let k = proj.params.get(&SCALE_FACTOR_AT_NATURAL_ORIGIN).unwrap_or(&ProjValue::default()).f64();
    proj.k0 = if k == 0. { 0.9999 } else { k };

    data.modified = modified;

    data.easting_northing = true;
    if proj.params.contains_key(&CZECH) {
        data.easting_northing = false;
    }

    // Set up shared parameters between forward and inverse
    data.alpha = sqrt(1. + (proj.es * pow(cos(proj.phi0), 4.)) / (1. - proj.es));
    let u0 = asin(sin(proj.phi0) / data.alpha);
    let g = pow(
        (1. + proj.e * sin(proj.phi0)) / (1. - proj.e * sin(proj.phi0)),
        data.alpha * proj.e / 2.,
    );
    let tan_half_phi0_plus_pi_4 = tan(proj.phi0 / 2. + FRAC_PI_4);
    if tan_half_phi0_plus_pi_4 == 0.0 {
        panic!("Invalid value for lat_0: lat_0 + PI/4 should be different from 0");
    }
    data.k = tan(u0 / 2. + FRAC_PI_4) / pow(tan_half_phi0_plus_pi_4, data.alpha) * g;
    let n0 = sqrt(1. - proj.es) / (1. - proj.es * pow(sin(proj.phi0), 2.));
    data.n = sin(S0);
    data.rho0 = proj.k0 * n0 / tan(S0);
    data.ad = FRAC_PI_2 - UQ;

    data
}

/// Krovak Projection
pub type KrovakProjection = KrovakBaseProjection<KROVAK, false>;
/// Krovak North Oriented Projection
pub type KrovakNorthOrientedProjection = KrovakBaseProjection<KROVAK_NORTH_ORIENTED, false>;
/// Krovak Modified Projection
pub type KrovakModifiedProjection = KrovakBaseProjection<KROVAK_MODIFIED, true>;
/// Krovak Modified North Oriented Projection
pub type KrovakModifiedNorthOrientedProjection =
    KrovakBaseProjection<KROVAK_MODIFIED_NORTH_ORIENTED, true>;

/// Krovak Projection
#[derive(Debug, Clone, PartialEq)]
pub struct KrovakBaseProjection<const C: i64, const E: bool> {
    proj: Rc<RefCell<Proj>>,
    store: RefCell<KrovakData>,
}
impl<const C: i64, const E: bool> ProjectCoordinates for KrovakBaseProjection<C, E> {
    fn code(&self) -> i64 {
        C
    }
    fn name(&self) -> &'static str {
        "Krovak"
    }
    fn names() -> &'static [&'static str] {
        &["Krovak", "Modified Krovak"]
    }
}
impl<const C: i64, const E: bool> CoordinateStep for KrovakBaseProjection<C, E> {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        let store = krovak_setup(&mut proj.borrow_mut(), E);
        KrovakBaseProjection { proj, store: store.into() }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        krovak_e_forward(&self.store.borrow(), &self.proj.borrow(), p);
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        krovak_e_inverse(&self.store.borrow(), &self.proj.borrow(), p);
    }
}

/// Krovak Ellipsoidal forward project
pub fn krovak_e_forward<P: TransformCoordinates>(krovak: &KrovakData, proj: &Proj, p: &mut P) {
    let mut x;
    let mut y;

    let gfi = pow(
        (1. + proj.e * sin(p.phi())) / (1. - proj.e * sin(p.phi())),
        krovak.alpha * proj.e / 2.,
    );

    let u =
        2. * (atan(krovak.k * pow(tan(p.phi() / 2. + FRAC_PI_4), krovak.alpha) / gfi) - FRAC_PI_4);
    let deltav = -p.lam() * krovak.alpha;

    let s = asin(cos(krovak.ad) * sin(u) + sin(krovak.ad) * cos(u) * cos(deltav));
    let cos_s = cos(s);
    if cos_s < 1e-12 {
        x = 0.;
        y = 0.;
        p.set_x(x);
        p.set_y(y);
        return;
    }
    let d = asin(cos(u) * sin(deltav) / cos_s);

    let eps = krovak.n * d;
    let rho = krovak.rho0 * pow(tan(S0 / 2. + FRAC_PI_4), krovak.n)
        / pow(tan(s / 2. + FRAC_PI_4), krovak.n);

    x = rho * cos(eps);
    y = rho * sin(eps);

    // At this point, x is a southing and y is a westing

    if krovak.modified {
        let xp = x;
        let yp = y;

        // Reduced X and Y
        let xr = xp * proj.a - X0;
        let yr = yp * proj.a - Y0;

        let (dx, dy) = mod_krovak_compute_dx_dy(xr, yr);

        x = xp - dx / proj.a;
        y = yp - dy / proj.a;
    }

    // PROJ always return values in (easting, northing) (default mode)
    // or (westing, southing) (+czech mode), so swap X/Y
    core::mem::swap(&mut x, &mut y);

    if krovak.easting_northing {
        // The default non-Czech convention uses easting, northing, so we have
        // to reverse the sign of the coordinates. But to do so, we have to take
        // into account the false easting/northing.
        x = -x - 2. * proj.x0 / proj.a;
        y = -y - 2. * proj.y0 / proj.a;
    }

    p.set_x(x);
    p.set_y(y);
}

/// Krovak Ellipsoidal inverse project
pub fn krovak_e_inverse<P: TransformCoordinates>(krovak: &KrovakData, proj: &Proj, p: &mut P) {
    let mut x = p.x();
    let mut y = p.y();

    if krovak.easting_northing {
        // The default non-Czech convention uses easting, northing, so we have
        // to reverse the sign of the coordinates. But to do so, we have to take
        // into account the false easting/northing.
        y = -y - 2. * proj.x0 / proj.a;
        x = -x - 2. * proj.y0 / proj.a;
    }

    core::mem::swap(&mut x, &mut y);

    if krovak.modified {
        // Note: in EPSG guidance node 7-2, below x_r/y_r/d_x/d_y are actually
        // x_r'/y_r'/d_x'/d_y'
        let x_r = x * proj.a - X0;
        let y_r = y * proj.a - Y0;

        let (d_x, d_y) = mod_krovak_compute_dx_dy(x_r, y_r);

        x += d_x / proj.a;
        y += d_y / proj.a;
    }

    let rho = sqrt(x * x + y * y);
    let eps = atan2(y, x);

    let d = eps / sin(S0);
    let s = if rho == 0.0 {
        FRAC_PI_2
    } else {
        2. * (atan(pow(krovak.rho0 / rho, 1. / krovak.n) * tan(S0 / 2. + FRAC_PI_4)) - FRAC_PI_4)
    };

    let u = asin(cos(krovak.ad) * sin(s) - sin(krovak.ad) * cos(s) * cos(d));
    let deltav = asin(cos(s) * sin(d) / cos(u));

    p.set_lam(proj.lam0 - deltav / krovak.alpha);

    // ITERATION FOR p.phi
    let mut fi1 = u;

    let mut i = MAX_ITER;
    while i > 0 {
        p.set_phi(
            2. * (atan(
                pow(krovak.k, -1. / krovak.alpha)
                    * pow(tan(u / 2. + FRAC_PI_4), 1. / krovak.alpha)
                    * pow((1. + proj.e * sin(fi1)) / (1. - proj.e * sin(fi1)), proj.e / 2.),
            ) - FRAC_PI_4),
        );

        if fabs(fi1 - p.phi()) < EPS {
            break;
        }
        fi1 = p.phi();
        i -= 1;
    }
    if i == 0 {
        panic!("Coordinate outside projection domain")
    }

    p.set_lam(p.lam() - proj.lam0);
}
