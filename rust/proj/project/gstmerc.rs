use crate::proj::{CoordinateStep, Proj, ProjectCoordinates, TransformCoordinates, phi2, tsfn};
use alloc::rc::Rc;
use core::cell::RefCell;
use libm::{asin, atan, cos, cosh, exp, log, pow, sin, sinh, sqrt};

/// # Gauss-Schreiber Transverse Mercator (aka Gauss-Laborde Reunion)
///
/// **Classification**: Conformal
///
/// **Available forms**: Forward and inverse, spherical projection
///
/// **Defined area**: Global
///
/// **Alias**: gstmerc
///
/// **Domain**: 2D
///
/// **Input type**: Geodetic coordinates
///
/// **Output type**: Projected coordinates
///
/// ## Projection String
/// ```ini
/// +proj=gstmerc
/// ```
///
/// ## Optional Parameters
/// - `+k_0=<value>`: Scale factor at the central meridian.
/// - `+lon_0=<value>`: Longitude of the central meridian.
/// - `+lat_0=<value>`: Latitude of origin.
/// - `+ellps=<value>`: Ellipsoid name (e.g., GRS80, WGS84).
/// - `+R=<value>`: Radius of the sphere (used in spherical projections).
/// - `+x_0=<value>`: False easting.
/// - `+y_0=<value>`: False northing.
///
/// ## Usage Example
/// ```bash
/// echo 12 55 | proj +proj=gstmerc +ellps=WGS84
/// echo 12 55 | proj +proj=gstmerc +k_0=1 +lon_0=0 +x_0=500000 +y_0=0
/// ```
///
/// ![Gauss-Schreiber Transverse Mercator](https://github.com/Open-S2/gis-tools/blob/master/assets/proj4/projections/images/gstmerc.png?raw=true)
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GstmercData {
    lamc: f64,
    phic: f64,
    c: f64,
    n1: f64,
    n2: f64,
    xs: f64,
    ys: f64,
}

/// Gauss-Schreiber Transverse Mercator (aka Gauss-Laborde Reunion) Projection
#[derive(Debug, Clone, PartialEq)]
pub struct GaussSchreiberTransverseMercatorProjection {
    proj: Rc<RefCell<Proj>>,
    store: RefCell<GstmercData>,
}
impl ProjectCoordinates for GaussSchreiberTransverseMercatorProjection {
    fn code(&self) -> i64 {
        -1
    }
    fn name(&self) -> &'static str {
        "Gauss-Schreiber Transverse Mercator"
    }
    fn names() -> &'static [&'static str] {
        &[
            "Gauss-Schreiber Transverse Mercator (aka Gauss-Laborde Reunion)",
            "Gauss-Schreiber Transverse Mercator",
            "gstmerc",
        ]
    }
}
impl CoordinateStep for GaussSchreiberTransverseMercatorProjection {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        let mut store = GstmercData::default();
        {
            let proj = proj.borrow();
            store.lamc = proj.lam0;
            store.n1 = sqrt(1. + proj.es * pow(cos(proj.phi0), 4.0) / (1. - proj.es));
            store.phic = asin(sin(proj.phi0) / store.n1);
            store.c = log(tsfn(-store.phic, -sin(proj.phi0) / store.n1, 0.0))
                - store.n1 * log(tsfn(-proj.phi0, -sin(proj.phi0), proj.e));
            store.n2 = proj.k0 * proj.a * sqrt(1. - proj.es)
                / (1. - proj.es * sin(proj.phi0) * sin(proj.phi0));
            store.xs = 0.;
            store.ys = -store.n2 * store.phic;
        }
        GaussSchreiberTransverseMercatorProjection { proj, store: store.into() }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        gstmerc_s_forward(&self.store.borrow(), &self.proj.borrow(), p);
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        gstmerc_s_inverse(&self.store.borrow(), &self.proj.borrow(), p);
    }
}

/// Gauss-Schreiber Transverse Mercator Spheroidal forward project
pub fn gstmerc_s_forward<P: TransformCoordinates>(gstmerc: &GstmercData, proj: &Proj, p: &mut P) {
    let l = gstmerc.n1 * p.lam();
    let ls = gstmerc.c + gstmerc.n1 * log(tsfn(-p.phi(), -sin(p.phi()), proj.e));
    let sin_ls1 = sin(l) / cosh(ls);
    let ls1 = log(tsfn(-asin(sin_ls1), -sin_ls1, 0.0));
    p.set_x((gstmerc.xs + gstmerc.n2 * ls1) * proj.ra);
    p.set_y((gstmerc.ys + gstmerc.n2 * atan(sinh(ls) / cos(l))) * proj.ra);
}

/// Gauss-Schreiber Transverse Mercator Spheroidal inverse project
pub fn gstmerc_s_inverse<P: TransformCoordinates>(gstmerc: &GstmercData, proj: &Proj, p: &mut P) {
    let l = atan(
        sinh((p.x() * proj.a - gstmerc.xs) / gstmerc.n2)
            / cos((p.y() * proj.a - gstmerc.ys) / gstmerc.n2),
    );
    let sin_c = sin((p.y() * proj.a - gstmerc.ys) / gstmerc.n2)
        / cosh((p.x() * proj.a - gstmerc.xs) / gstmerc.n2);
    let lc = log(tsfn(-asin(sin_c), -sin_c, 0.0));
    p.set_lam(l / gstmerc.n1);
    p.set_phi(-phi2(exp((lc - gstmerc.c) / gstmerc.n1), proj.e));
}
