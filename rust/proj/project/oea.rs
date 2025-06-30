use crate::proj::{
    CoordinateStep, M_VAL, N_VAL, Proj, ProjValue, ProjectCoordinates, THETA, TransformCoordinates,
    aacos, aasin, aatan2,
};
use alloc::rc::Rc;
use core::cell::RefCell;
use libm::{cos, hypot, sin};

/// Oblated Equal Area Variables
#[derive(Debug, Default, Clone, PartialEq)]
pub struct OeaData {
    theta: f64,
    m: f64,
    n: f64,
    two_r_m: f64,
    two_r_n: f64,
    rm: f64,
    rn: f64,
    hm: f64,
    hn: f64,
    cp0: f64,
    sp0: f64,
}

/// Oblated Equal Area Projection
#[derive(Debug, Clone, PartialEq)]
pub struct OblatedEqualAreaProjection {
    proj: Rc<RefCell<Proj>>,
    store: RefCell<OeaData>,
}
impl ProjectCoordinates for OblatedEqualAreaProjection {
    fn code(&self) -> i64 {
        -1
    }
    fn name(&self) -> &'static str {
        "Oblated Equal Area"
    }
    fn names() -> &'static [&'static str] {
        &["Oblated Equal Area", "oea"]
    }
}
impl CoordinateStep for OblatedEqualAreaProjection {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        let mut store = OeaData::default();
        {
            let proj = &mut proj.borrow_mut();

            store.n = proj.params.get(&N_VAL).unwrap_or(&ProjValue::default()).f64();
            if store.n <= 0. {
                panic!("Invalid value for n: it should be > 0");
            }
            store.m = proj.params.get(&M_VAL).unwrap_or(&ProjValue::default()).f64();
            if store.m <= 0. {
                panic!("Invalid value for m: it should be > 0");
            }
            store.theta =
                proj.params.get(&THETA).unwrap_or(&ProjValue::default()).f64().to_radians();

            store.sp0 = sin(proj.phi0);
            store.cp0 = cos(proj.phi0);
            store.rn = 1. / store.n;
            store.rm = 1. / store.m;
            store.two_r_n = 2. * store.rn;
            store.two_r_m = 2. * store.rm;
            store.hm = 0.5 * store.m;
            store.hn = 0.5 * store.n;
            proj.es = 0.;
        }
        OblatedEqualAreaProjection { proj, store: store.into() }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        oea_s_forward(&self.store.borrow(), p);
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        oea_s_inverse(&self.store.borrow(), p);
    }
}

/// Oblated Equal Area Spheroidal forward project
pub fn oea_s_forward<P: TransformCoordinates>(oae: &OeaData, p: &mut P) {
    let cp = cos(p.phi());
    let sp = sin(p.phi());
    let cl = cos(p.lam());
    let az = aatan2(cp * sin(p.lam()), oae.cp0 * sp - oae.sp0 * cp * cl) + oae.theta;
    let shz = sin(0.5 * aacos(oae.sp0 * sp + oae.cp0 * cp * cl));
    let m = aasin(shz * sin(az));
    let n = aasin(shz * cos(az) * cos(m) / cos(m * oae.two_r_m));
    p.set_y(oae.n * sin(n * oae.two_r_n));
    p.set_x(oae.m * sin(m * oae.two_r_m) * cos(n) / cos(n * oae.two_r_n));
}

/// Oblated Equal Area Spheroidal inverse project
pub fn oea_s_inverse<P: TransformCoordinates>(oae: &OeaData, p: &mut P) {
    let n = oae.hn * aasin(p.y() * oae.rn);
    let m = oae.hm * aasin(p.x() * oae.rm * cos(n * oae.two_r_n) / cos(n));
    let xp = 2. * sin(m);
    let yp = 2. * sin(n) * cos(m * oae.two_r_m) / cos(m);
    let az = aatan2(xp, yp) - oae.theta;
    let c_az = cos(az);
    let z = 2. * aasin(0.5 * hypot(xp, yp));
    let sz = sin(z);
    let cz = cos(z);
    p.set_phi(aasin(oae.sp0 * cz + oae.cp0 * sz * c_az));
    p.set_lam(aatan2(sz * sin(az), oae.cp0 * cz - oae.sp0 * sz * c_az));
}
