use super::{ProjectCoordinates, TransformCoordinates};
use crate::proj::{
    _msfn, ALBERS_EQUAL_AREA, CoordinateStep, EPS10, LATITUDE_OF_FIRST_STANDARD_PARALLEL,
    LATITUDE_OF_SECOND_STANDARD_PARALLEL, Proj, ProjValue, SOUTH, authalic_lat_compute_coeffs,
    authalic_lat_inverse, authalic_lat_q,
};
use alloc::{rc::Rc, vec::Vec};
use core::{cell::RefCell, f64::consts::FRAC_PI_2};
use libm::{asin, atan2, cos, fabs, hypot, log, sin, sqrt};

const TOL7: f64 = 1.0e-7;

/// Albers Equal Area variables. Shared by aea and leac
#[derive(Debug, Default, Clone, PartialEq)]
pub struct AeaData {
    ec: f64,
    n: f64,
    c: f64,
    dd: f64,
    n2: f64,
    rho0: f64,
    rho: f64,
    phi1: f64,
    phi2: f64,
    ellips: bool,
    apa: Vec<f64>,
    qp: f64,
}
impl AeaData {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        let proj = &proj.borrow();
        let mut store = AeaData::default();

        if let Some(lat_1) = proj.params.get(&LATITUDE_OF_FIRST_STANDARD_PARALLEL) {
            store.phi1 = lat_1.f64();
        }
        if let Some(lat_2) = proj.params.get(&LATITUDE_OF_SECOND_STANDARD_PARALLEL) {
            store.phi2 = lat_2.f64();
        }

        if fabs(store.phi1) > FRAC_PI_2 {
            panic!("Invalid value for lat_1: |lat_1| should be <= 90°");
        }
        if fabs(store.phi2) > FRAC_PI_2 {
            panic!("Invalid value for lat_2: |lat_2| should be <= 90°");
        }
        if fabs(store.phi1 + store.phi2) < EPS10 {
            panic!("Invalid value for lat_1 and lat_2: |lat_1 + lat_2| should be > 0");
        }
        let mut sinphi = sin(store.phi1);
        store.n = sinphi;
        let mut cosphi = cos(store.phi1);
        let secant = fabs(store.phi1 - store.phi2) >= EPS10;
        store.ellips = proj.es > 0.;
        if store.ellips {
            store.apa = authalic_lat_compute_coeffs(proj.n);
            store.qp = authalic_lat_q(1.0, proj);
            let m1: f64 = _msfn(sinphi, cosphi, proj.es);
            let ml1: f64 = authalic_lat_q(sinphi, proj);
            if secant {
                // secant cone
                sinphi = sin(store.phi2);
                cosphi = cos(store.phi2);
                let m2: f64 = _msfn(sinphi, cosphi, proj.es);
                let ml2: f64 = authalic_lat_q(sinphi, proj);
                if ml2 == ml1 {
                    panic!("Invalid value for lat_1 and lat_2: latitudes are too close");
                }

                store.n = (m1 * m1 - m2 * m2) / (ml2 - ml1);
                if store.n == 0. {
                    // Not quite, but es is very close to 1...
                    panic!("Invalid value for eccentricity");
                }
            }
            store.ec = 1. - 0.5 * proj.one_es * log((1. - proj.e) / (1. + proj.e)) / proj.e;
            store.c = m1 * m1 + store.n * ml1;
            store.dd = 1. / store.n;
            store.rho0 = store.dd * sqrt(store.c - store.n * authalic_lat_q(sin(proj.phi0), proj));
        } else {
            if secant {
                store.n = 0.5 * (store.n + sin(store.phi2));
            }
            store.n2 = store.n + store.n;
            store.c = cosphi * cosphi + store.n2 * sinphi;
            store.dd = 1. / store.n;
            store.rho0 = store.dd * sqrt(store.c - store.n2 * sin(proj.phi0));
        }

        store
    }
}

/// # Albers Conic Equal Area Projection
///
/// **Classification**: Conic
///
/// **Available forms**: Forward and inverse, spherical and ellipsoidal
///
/// **Defined area**: Global
///
/// **Alias**: `aea`
///
/// **Domain**: 2D
///
/// **Input type**: Geodetic coordinates
///
/// **Output type**: Projected coordinates
///
/// ## Projection String
/// ```sh
/// +proj=aea +lat_1=29.5 +lat_2=42.5
/// ```
///
/// ## Required Parameters
/// - `lat1`
/// - `lat2`
///
/// ## Optional Parameters
/// - `lon0`: Longitude of the central meridian.
/// - `ellps`: Name of the reference ellipsoid.
/// - `R`: Radius of the sphere if `ellps` is not specified.
/// - `x0`: False easting (coordinate offset in the x-direction).
/// - `y0`: False northing (coordinate offset in the y-direction).
///
/// ## References
/// - https://en.wikipedia.org/wiki/Albers_projection
///
/// ![Albers Conic Equal Area Projection](https://github.com/Open-S2/gis-tools/blob/master/assets/proj4/projections/images/aea.png?raw=true)
#[derive(Debug, Clone, PartialEq)]
pub struct AlbersConicEqualAreaProjection {
    proj: Rc<RefCell<Proj>>,
    store: RefCell<AeaData>,
}
impl ProjectCoordinates for AlbersConicEqualAreaProjection {
    fn code(&self) -> i64 {
        ALBERS_EQUAL_AREA
    }
    fn name(&self) -> &'static str {
        "Albers Conic Equal Area"
    }
    fn names() -> &'static [&'static str] {
        &["Albers Conic Equal Area", "Albers_Conic_Equal_Area", "Albers", "aea", "9822"]
    }
}
impl CoordinateStep for AlbersConicEqualAreaProjection {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        let store = AeaData::new(proj.clone());
        AlbersConicEqualAreaProjection { proj, store: store.into() }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        aea_e_forward(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        aea_e_inverse(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
    }
}

/// # Lambert Equal Area Conic Projection
///
/// **Classification**: Conical
///
/// **Available forms**: Forward and inverse, spherical and ellipsoidal
///
/// **Defined area**: Global
///
/// **Alias**: `leac`
///
/// **Domain**: 2D
///
/// **Input type**: Geodetic coordinates
///
/// **Output type**: Projected coordinates
///
/// ## Projection String
/// ```ini
/// +proj=leac
/// ```
///
/// ## Parameters
///
/// **Note**: All parameters are optional for the Lambert Equal Area Conic projection.
///
/// ## Required Parameters
/// - `lat1`: Latitude of the first standard parallel.
/// - `+south`: Sets the second standard parallel to 90°S. When the flag is off, the second standard parallel is set to 90°N.
///
/// ## Optional Parameters
/// - `lon0`: Longitude of the central meridian.
/// - `ellps`: Name of the reference ellipsoid.
/// - `R`: Radius of the sphere if `ellps` is not specified.
/// - `x0`: False easting (coordinate offset in the x-direction).
/// - `y0`: False northing (coordinate offset in the y-direction).
///
/// ## References
/// - https://en.wikipedia.org/wiki/Lambert_conformal_conic_projection (Note: While the name is similar, this link describes the conformal variant. A specific link for the equal-area conic might be needed)
///
/// ![Lambert Equal Area Conic Projection](https://github.com/Open-S2/gis-tools/blob/master/assets/proj4/projections/images/leac.png?raw=true)
#[derive(Debug, Clone, PartialEq)]
pub struct LambertEqualAreaConicProjection {
    proj: Rc<RefCell<Proj>>,
    store: RefCell<AeaData>,
}
impl ProjectCoordinates for LambertEqualAreaConicProjection {
    fn code(&self) -> i64 {
        -1
    }
    fn name(&self) -> &'static str {
        "Lambert Equal Area Conic"
    }
    fn names() -> &'static [&'static str] {
        &["Lambert Equal Area Conic", "leac"]
    }
}
impl CoordinateStep for LambertEqualAreaConicProjection {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        let mut store = AeaData::new(proj.clone());
        store.phi2 = proj.borrow().params.get(&LATITUDE_OF_FIRST_STANDARD_PARALLEL).unwrap().f64();
        store.phi1 = if proj.borrow().params.get(&SOUTH).unwrap_or(&ProjValue::default()).bool() {
            -FRAC_PI_2
        } else {
            FRAC_PI_2
        };
        LambertEqualAreaConicProjection { proj, store: store.into() }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        aea_e_forward(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        aea_e_inverse(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
    }
}

/// Ellipsoid/spheroid, forward
pub fn aea_e_forward<P: TransformCoordinates>(aea: &mut AeaData, proj: &Proj, p: &mut P) {
    let phi = p.phi();
    let mut lam = p.lam();
    aea.rho =
        aea.c - if aea.ellips { aea.n * authalic_lat_q(sin(phi), proj) } else { aea.n2 * sin(phi) };

    if aea.rho < 0. {
        // ERROR: PROJ_ERR_COORD_TRANSFM_OUTSIDE_PROJECTION_DOMAIN
        return;
    }
    aea.rho = aea.dd * sqrt(aea.rho);
    lam *= aea.n;
    p.set_x(aea.rho * sin(lam));
    p.set_y(aea.rho0 - aea.rho * cos(lam));
}

/// Ellipsoid/spheroid, inverse
pub fn aea_e_inverse<P: TransformCoordinates>(aea: &mut AeaData, proj: &Proj, p: &mut P) {
    let mut x = p.x();
    let mut y = p.y();
    y = aea.rho0 - y;
    aea.rho = hypot(x, y);
    if aea.rho != 0.0 {
        if aea.n < 0. {
            aea.rho = -aea.rho;
            x = -x;
            y = -y;
        }
        p.set_phi(aea.rho / aea.dd);
        if aea.ellips {
            let qs = (aea.c - p.phi() * p.phi()) / aea.n;
            if fabs(aea.ec - fabs(qs)) > TOL7 {
                if fabs(qs) > 2. {
                    // ERROR: PROJ_ERR_COORD_TRANSFM_OUTSIDE_PROJECTION_DOMAIN
                    return;
                }
                p.set_phi(authalic_lat_inverse(asin(qs / aea.qp), &aea.apa, proj, aea.qp));
                if p.phi() == f64::INFINITY {
                    // ERROR: PROJ_ERR_COORD_TRANSFM_OUTSIDE_PROJECTION_DOMAIN
                    return;
                }
            } else {
                p.set_phi(if qs < 0. { -FRAC_PI_2 } else { FRAC_PI_2 });
            }
        } else {
            let qs_div_2 = (aea.c - p.phi() * p.phi()) / aea.n2;
            if fabs(qs_div_2) <= 1. {
                p.set_phi(asin(qs_div_2));
            } else {
                p.set_phi(if qs_div_2 < 0. { -FRAC_PI_2 } else { FRAC_PI_2 });
            }
        }
        p.set_lam(atan2(x, y) / aea.n);
    } else {
        p.set_lam(0.);
        p.set_phi(if aea.n > 0. { FRAC_PI_2 } else { -FRAC_PI_2 });
    }
}
