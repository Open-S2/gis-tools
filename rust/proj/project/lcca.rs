use crate::proj::{
    CoordinateStep, Proj, ProjectCoordinates, TransformCoordinates, enfn, inv_mlfn, mlfn,
};
use alloc::{rc::Rc, vec::Vec};
use core::cell::RefCell;
use libm::{atan2, cos, fabs, sin, sqrt, tan};

// Lambert Conformal Conic Alternative
// -----------------------------------
//
// This is Gerald Evenden's 2003 implementation of an alternative
// "almost" LCC, which has been in use historically, but which
// should NOT be used for new projects - i.e: use this implementation
// if you need interoperability with old data represented in this
// projection, but not in any other case.
//
// The code was originally discussed on the PROJ.4 mailing list in
// a thread archived over at
//
// http://lists.maptools.org/pipermail/proj/2003-March/000644.html
//
// It was discussed again in the thread starting at
//
// http://lists.maptools.org/pipermail/proj/2017-October/007828.html
// and continuing at
// http://lists.maptools.org/pipermail/proj/2017-November/007831.html
//
// which prompted Clifford J. Mugnier to add these clarifying notes:
//
// The French Army Truncated Cubic Lambert (partially conformal) Conic
// projection is the Legal system for the projection in France between
// the late 1800s and 1948 when the French Legislature changed the law
// to recognize the fully conformal version.
//
// It was (might still be in one or two North African prior French
// Colonies) used in North Africa in Algeria, Tunisia, & Morocco, as
// well as in Syria during the Levant.
//
// Last time I have seen it used was about 30+ years ago in
// Algeria when it was used to define Lease Block boundaries for
// Petroleum Exploration & Production.
//
// (signed)
//
// Clifford J. Mugnier, c.p., c.m.s.
// Chief of Geodesy
// LSU Center for GeoInformatics
// Dept. of Civil Engineering
// LOUISIANA STATE UNIVERSITY

const MAX_ITER: usize = 10;
const DEL_TOL: f64 = 1e-12;

/// Lambert Conformal Conic Alternative variables
#[derive(Debug, Default, Clone, PartialEq)]
pub struct LccaData {
    en: Vec<f64>,
    r0: f64,
    l: f64,
    m0: f64,
    c: f64,
}

/// Lambert Conformal Conic Alternative Projection
#[derive(Debug, Clone, PartialEq)]
pub struct LambertConformalConicAlternativeProjection {
    proj: Rc<RefCell<Proj>>,
    store: RefCell<LccaData>,
}
impl ProjectCoordinates for LambertConformalConicAlternativeProjection {
    fn code(&self) -> i64 {
        -1
    }
    fn name(&self) -> &'static str {
        "Lambert Conformal Conic Alternative"
    }
    fn names() -> &'static [&'static str] {
        &["Lambert Conformal Conic Alternative", "lcca"]
    }
}
impl CoordinateStep for LambertConformalConicAlternativeProjection {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        let mut store = LccaData::default();
        {
            let proj = &mut proj.borrow_mut();
            store.en = enfn(proj.n);
            if proj.phi0 == 0. {
                panic!("Invalid value for lat_0: it should be different from 0.");
            }
            store.l = sin(proj.phi0);
            store.m0 = mlfn(proj.phi0, store.l, cos(proj.phi0), &store.en);
            let s2p0 = store.l * store.l;
            let mut r0 = 1. / (1. - proj.es * s2p0);
            let n0 = sqrt(r0);
            r0 *= proj.one_es * n0;
            let tan0 = tan(proj.phi0);
            store.r0 = n0 / tan0;
            store.c = 1. / (6. * r0 * n0);
        }
        LambertConformalConicAlternativeProjection { proj, store: store.into() }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        lcca_e_forward(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        lcca_e_inverse(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
    }
}

fn f_s(s: f64, c: f64) -> f64 {
    // func to compute dr
    s * (1. + s * s * c)
}

fn f_sp(s: f64, c: f64) -> f64 {
    // deriv of fs
    1. + 3. * s * s * c
}

/// Lambert Conformal Conic Alternative Ellipsoidal forward project
pub fn lcca_e_forward<P: TransformCoordinates>(lcca: &mut LccaData, proj: &Proj, p: &mut P) {
    let s = mlfn(p.phi(), sin(p.phi()), cos(p.phi()), &lcca.en) - lcca.m0;
    let dr = f_s(s, lcca.c);
    let r = lcca.r0 - dr;
    let lam_mul_l = p.lam() * lcca.l;
    p.set_x(proj.k0 * (r * sin(lam_mul_l)));
    p.set_y(proj.k0 * (lcca.r0 - r * cos(lam_mul_l)));
}

/// Lambert Conformal Conic Alternative Ellipsoidal inverse project
pub fn lcca_e_inverse<P: TransformCoordinates>(lcca: &mut LccaData, proj: &Proj, p: &mut P) {
    let x = p.x() / proj.k0;
    let y = p.y() / proj.k0;
    let theta = atan2(x, lcca.r0 - y);
    let dr = y - x * tan(0.5 * theta);
    p.set_lam(theta / lcca.l);
    let mut s = dr;
    let mut i = MAX_ITER;
    while i > 0 {
        let dif = f_s(s, lcca.c) - dr;
        s -= dif / f_sp(s, lcca.c);
        if fabs(dif) < DEL_TOL {
            break;
        }
        i -= 1;
    }
    if i != 0 {
        panic!("Coordinate outside projection domain");
    }
    p.set_phi(inv_mlfn(s + lcca.m0, &lcca.en));
}
