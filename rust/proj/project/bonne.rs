use crate::proj::{
    BONNE, CoordinateStep, EPS10, LATITUDE_OF_FIRST_STANDARD_PARALLEL, Proj, ProjMethod,
    ProjectCoordinates, TransformCoordinates, enfn, inv_mlfn, mlfn,
};
use alloc::vec::Vec;
use core::{cell::RefCell, f64::consts::FRAC_PI_2};
use libm::{atan2, copysign, cos, fabs, hypot, sin, sqrt, tan};

/// Bonne Variables
#[derive(Debug, Default, Clone, PartialEq)]
pub struct BonneData {
    phi1: f64,
    cphi1: f64,
    am1: f64,
    m1: f64,
    en: Vec<f64>,
}

/// # Bonne (Werner lat_1=90) Projection
///
/// **Classification**: Miscellaneous
///
/// **Available forms**: Forward and inverse, spherical and ellipsoidal
///
/// **Defined area**: Global
///
/// **Alias**: `bonne`
///
/// **Domain**: 2D
///
/// **Input type**: Geodetic coordinates
///
/// **Output type**: Projected coordinates
///
/// ## Projection String
/// ```ini
/// +proj=bonne +lat_1=10
/// ```
///
/// ## Required Parameters
/// - `lat1`: Latitude of first standard parallel
///
/// ## Optional Parameters
/// - `lon0`: Longitude of origin
/// - `ellps`: Ellipsoid name
/// - `R`: Radius of sphere
/// - `x0`: False easting
/// - `y0`: False northing
///
/// ![Bonne (Werner lat_1=90) Projection](https://github.com/Open-S2/gis-tools/blob/master/assets/proj4/projections/images/bonne.png?raw=true)
#[derive(Debug, Clone, PartialEq)]
pub struct BonneProjection {
    proj: RefCell<Proj>,
    store: RefCell<BonneData>,
    method: ProjMethod,
}
impl ProjectCoordinates for BonneProjection {
    fn code(&self) -> i64 {
        BONNE
    }
    fn name(&self) -> &'static str {
        "Bonne"
    }
    fn names() -> &'static [&'static str] {
        &["Bonne (Werner lat_1=90)", "bonne_werner", "Bonne", "bonne"]
    }
}
impl CoordinateStep for BonneProjection {
    fn new(proj: RefCell<Proj>) -> Self {
        let mut store = BonneData::default();
        let method: ProjMethod;
        {
            let proj = &mut proj.borrow_mut();
            store.phi1 = proj.params.get(&LATITUDE_OF_FIRST_STANDARD_PARALLEL).unwrap().f64();
            if fabs(store.phi1) < EPS10 {
                panic!("Invalid value for lat_1: |lat_1| should be > 0");
            }

            method = if proj.es != 0.0 {
                store.en = enfn(proj.n);
                store.am1 = sin(store.phi1);
                let c = cos(store.phi1);
                store.m1 = mlfn(store.phi1, store.am1, c, &store.en);
                store.am1 = c / (sqrt(1. - proj.es * store.am1 * store.am1) * store.am1);
                ProjMethod::Ellipsoidal
            } else {
                if fabs(store.phi1) + EPS10 >= FRAC_PI_2 {
                    store.cphi1 = 0.;
                } else {
                    store.cphi1 = 1. / tan(store.phi1);
                }
                ProjMethod::Spheroidal
            };
        }
        BonneProjection { proj, store: store.into(), method }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        if self.method == ProjMethod::Ellipsoidal {
            bonne_e_forward(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
        } else {
            bonne_s_forward(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
        }
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        if self.method == ProjMethod::Ellipsoidal {
            bonne_e_inverse(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
        } else {
            bonne_s_inverse(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
        }
    }
}

/// Bonne Ellipsoidal Forward
pub fn bonne_e_forward<P: TransformCoordinates>(bonne: &mut BonneData, proj: &Proj, p: &mut P) {
    let mut e = sin(p.phi());
    let c = cos(p.phi());
    let rh = bonne.am1 + bonne.m1 - mlfn(p.phi(), e, c, &bonne.en);
    if fabs(rh) > EPS10 {
        e = c * p.lam() / (rh * sqrt(1. - proj.es * e * e));
        p.set_x(rh * sin(e));
        p.set_y(bonne.am1 - rh * cos(e));
    } else {
        p.set_x(0.);
        p.set_y(0.);
    }
}

/// Bonne Spheroidal Forward
pub fn bonne_s_forward<P: TransformCoordinates>(bonne: &mut BonneData, _proj: &Proj, p: &mut P) {
    let rh = bonne.cphi1 + bonne.phi1 - p.phi();
    if fabs(rh) > EPS10 {
        let e = p.lam() * cos(p.phi()) / rh;
        p.set_x(rh * sin(e));
        p.set_y(bonne.cphi1 - rh * cos(e));
    } else {
        p.set_x(0.);
        p.set_y(0.);
    }
}

/// Bonne Spheroidal Inverse
pub fn bonne_s_inverse<P: TransformCoordinates>(bonne: &mut BonneData, _proj: &Proj, p: &mut P) {
    p.set_y(bonne.cphi1 - p.y());
    let rh = copysign(hypot(p.x(), p.y()), bonne.phi1);
    let phi = bonne.cphi1 + bonne.phi1 - rh;
    let lam: f64;
    let abs_phi = fabs(phi);
    if abs_phi > FRAC_PI_2 {
        panic!("Coordinate outside projection domain");
    }
    if FRAC_PI_2 - abs_phi <= EPS10 {
        lam = 0.;
    } else {
        let lm = rh / cos(phi);
        if bonne.phi1 > 0. {
            lam = lm * atan2(p.x(), p.y());
        } else {
            lam = lm * atan2(-p.x(), -p.y());
        }
    }

    p.set_phi(phi);
    p.set_lam(lam);
}

/// Bonne Ellipsoidal Inverse
pub fn bonne_e_inverse<P: TransformCoordinates>(bonne: &mut BonneData, proj: &Proj, p: &mut P) {
    p.set_y(bonne.am1 - p.y());
    let rh = copysign(hypot(p.x(), p.y()), bonne.phi1);
    let phi = inv_mlfn(bonne.am1 + bonne.m1 - rh, &bonne.en);
    let lam: f64;
    let abs_phi = fabs(phi);
    if abs_phi < FRAC_PI_2 {
        let sinphi = sin(phi);
        let lm = rh * sqrt(1. - proj.es * sinphi * sinphi) / cos(phi);
        if bonne.phi1 > 0. {
            lam = lm * atan2(p.x(), p.y());
        } else {
            lam = lm * atan2(-p.x(), -p.y());
        }
    } else if abs_phi - FRAC_PI_2 <= EPS10 {
        lam = 0.;
    } else {
        panic!("Coordinates are outside the projection domain");
    }
    p.set_phi(phi);
    p.set_lam(lam);
}
