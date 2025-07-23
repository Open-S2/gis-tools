use crate::proj::{
    CoordinateStep, LATITUDE_STD_PARALLEL, Proj, ProjMethod, ProjectCoordinates,
    TransformCoordinates, authalic_lat_compute_coeffs, authalic_lat_inverse, authalic_lat_q,
};
use alloc::{rc::Rc, vec::Vec};
use core::{cell::RefCell, f64::consts::FRAC_PI_2};
use libm::{asin, cos, fabs, sin, sqrt};

/// Equal Area Cylindrical Variables
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CeaData {
    qp: f64,
    apa: Vec<f64>,
}

const EPS: f64 = 1e-10;

/// # Equal Area Cylindrical Projection
///
/// **Classification**: Cylindrical
///
/// **Available forms**: Forward and Inverse, spherical and ellipsoidal
///
/// **Defined area**: Global
///
/// **Alias**: `cea`
///
/// **Domain**: 2D
///
/// **Input type**: Geodetic coordinates
///
/// **Output type**: Projected coordinates
///
/// ## Projection String
/// ```ini
/// +proj=cea
/// ```
///
/// ## Named Specializations
///
/// The Equal Area Cylindrical projection is sometimes known under other names when instantiated with particular values of the `lat_ts` parameter:
///
/// - **Lambert cylindrical equal-area**: `lat_ts = 0`
/// - **Behrmann**: `lat_ts = 30`
/// - **Gall-Peters**: `lat_ts = 45`
///
/// ## Optional Parameters
/// - `lat_ts`: Latitude of true scale
/// - `lon_0`: Longitude of origin
/// - `ellps`: Ellipsoid
/// - `R`: Radius of the sphere
/// - `k_0`: Scaling factor
/// - `x_0`: False easting
/// - `y_0`: False northing
///
/// **Note**: `lat_ts` and `k_0` are mutually exclusive. If `lat_ts` is specified, it is equivalent to setting `k_0` to:
///
/// $$k_0 = \cos(lat_{ts}) / \sqrt{1 - e^2 \sin^2(lat_{ts})}$$
///
/// reference:
/// "Cartographic Projection Procedures for the UNIX Environment-
/// A User's Manual" by Gerald I. Evenden,
/// USGS Open File Report 90-284and Release 4 Interim Reports (2003)
///
/// ![Equal Area Cylindrical Projection](https://github.com/Open-S2/gis-tools/blob/master/assets/proj4/projections/images/cae.png?raw=true)
#[derive(Debug, Clone, PartialEq)]
pub struct EqualAreaCylindricalProjection {
    proj: Rc<RefCell<Proj>>,
    store: RefCell<CeaData>,
    method: ProjMethod,
}
impl ProjectCoordinates for EqualAreaCylindricalProjection {
    fn code(&self) -> i64 {
        -1
    }
    fn name(&self) -> &'static str {
        "Equal Area Cylindrical"
    }
    fn names() -> &'static [&'static str] {
        &["Equal Area Cylindrical", "cea"]
    }
}
impl CoordinateStep for EqualAreaCylindricalProjection {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        let mut store = CeaData::default();
        let method: ProjMethod;
        {
            let proj = &mut proj.borrow_mut();
            let mut t = 0.0;

            if let Some(lat_ts) = proj.params.get(&LATITUDE_STD_PARALLEL) {
                t = lat_ts.f64();
                proj.k0 = cos(t);
                if proj.k0 < 0.0 {
                    panic!("Invalid value for lat_ts: |lat_ts| should be <= 90°");
                }
            }
            method = if proj.es != 0.0 {
                t = sin(t);
                proj.k0 /= sqrt(1. - proj.es * t * t);
                proj.e = sqrt(proj.es);
                store.apa = authalic_lat_compute_coeffs(proj.n);

                store.qp = authalic_lat_q(1.0, proj);
                ProjMethod::Ellipsoidal
            } else {
                ProjMethod::Spheroidal
            };
        }
        EqualAreaCylindricalProjection { proj, store: store.into(), method }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        if self.method == ProjMethod::Spheroidal {
            cea_s_forward(&self.proj.borrow(), p);
        } else {
            cea_e_forward(&self.proj.borrow(), p);
        }
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        if self.method == ProjMethod::Spheroidal {
            cea_s_inverse(&self.proj.borrow(), p);
        } else {
            cea_e_inverse(&self.store.borrow(), &self.proj.borrow(), p);
        }
    }
}

/// Equal Area Cylindrical Ellipsoidal forward project
pub fn cea_e_forward<P: TransformCoordinates>(proj: &Proj, p: &mut P) {
    p.set_x(proj.k0 * p.lam());
    p.set_y(0.5 * authalic_lat_q(sin(p.phi()), proj) / proj.k0);
}

/// Equal Area Spheroidal forward project
pub fn cea_s_forward<P: TransformCoordinates>(proj: &Proj, p: &mut P) {
    p.set_x(proj.k0 * p.lam());
    p.set_y(sin(p.phi()) / proj.k0);
}

/// Equal Area Ellipsoidal inverse project
pub fn cea_e_inverse<P: TransformCoordinates>(cea: &CeaData, proj: &Proj, p: &mut P) {
    p.set_phi(authalic_lat_inverse(asin(2. * p.y() * proj.k0 / cea.qp), &cea.apa, proj, cea.qp));
    p.set_lam(p.x() / proj.k0);
}

/// Equal Area Spheroidal inverse project
pub fn cea_s_inverse<P: TransformCoordinates>(proj: &Proj, p: &mut P) {
    let y = p.y() * proj.k0;
    let x = p.x();
    let t = fabs(y);
    if t - EPS <= 1. {
        if t >= 1. {
            p.set_phi(if y < 0. { -FRAC_PI_2 } else { FRAC_PI_2 });
        } else {
            p.set_phi(asin(y));
        }
        p.set_lam(x / proj.k0);
    } else {
        panic!("Coordinate outside projection domain");
    }
}
