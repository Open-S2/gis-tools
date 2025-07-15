use crate::proj::{
    AZIMUTH_PROJECTION_CENTRE, CoordinateStep, LABORDE, Proj, ProjValue, ProjectCoordinates,
    TransformCoordinates,
};
use alloc::rc::Rc;
use core::cell::RefCell;
use libm::{atan, cos, exp, fabs, log, sin, sqrt, tan};

const M_FORTPI: f64 = 4. / core::f64::consts::PI;
const EPS: f64 = 1.0e-10;

/// Laborde variables
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct LabordeData {
    k_rg: f64,
    p0s: f64,
    a: f64,
    c: f64,
    ca: f64,
    cb: f64,
    cc: f64,
    cd: f64,
}

/// Laborde Projection
#[derive(Debug, Clone, PartialEq)]
pub struct LabordeProjection {
    proj: Rc<RefCell<Proj>>,
    store: RefCell<LabordeData>,
}
impl ProjectCoordinates for LabordeProjection {
    fn code(&self) -> i64 {
        LABORDE
    }
    fn name(&self) -> &'static str {
        "Laborde"
    }
    fn names() -> &'static [&'static str] {
        &["Laborde", "Laborde Oblique Mercator", "labrd"]
    }
}
impl CoordinateStep for LabordeProjection {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        let mut store = LabordeData::default();
        {
            let proj = &mut proj.borrow_mut();
            if proj.phi0 == 0. {
                panic!("Invalid value for lat_0: lat_0 should be different from 0");
            }

            let az = proj
                .params
                .get(&AZIMUTH_PROJECTION_CENTRE) // (lat_ts)
                .unwrap_or(&ProjValue::default())
                .f64();
            let sinp = sin(proj.phi0);
            let mut t = 1. - proj.es * sinp * sinp;
            let _n = 1. / sqrt(t);
            let _r = proj.one_es * _n / t;
            store.k_rg = proj.k0 * sqrt(_n * _r);
            store.p0s = atan(sqrt(_r / _n) * tan(proj.phi0));
            store.a = sinp / sin(store.p0s);
            t = proj.e * sinp;
            store.c = 0.5 * proj.e * store.a * log((1. + t) / (1. - t))
                + -store.a * log(tan(M_FORTPI + 0.5 * proj.phi0))
                + log(tan(M_FORTPI + 0.5 * store.p0s));
            t = az + az;
            store.cb = 1. / (12. * store.k_rg * store.k_rg);
            store.ca = (1. - cos(t)) * store.cb;
            store.cb *= sin(t);
            store.cc = 3. * (store.ca * store.ca - store.cb * store.cb);
            store.cd = 6. * store.ca * store.cb;
        }
        LabordeProjection { proj, store: store.into() }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        labrd_e_forward(&self.store.borrow(), &self.proj.borrow(), p);
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        labrd_e_inverse(&self.store.borrow(), &self.proj.borrow(), p);
    }
}

/// Laborde Ellipsoidal forward project
pub fn labrd_e_forward<P: TransformCoordinates>(laborde: &LabordeData, proj: &Proj, p: &mut P) {
    let phi = p.phi();
    let lam = p.lam();

    let mut v1 = laborde.a * log(tan(M_FORTPI + 0.5 * phi));
    let mut t = proj.e * sin(phi);
    let mut v2 = 0.5 * proj.e * laborde.a * log((1. + t) / (1. - t));
    let ps = 2. * (atan(exp(v1 - v2 + laborde.c)) - M_FORTPI);
    let i1 = ps - laborde.p0s;
    let cosps = cos(ps);
    let cosps2 = cosps * cosps;
    let sinps = sin(ps);
    let sinps2 = sinps * sinps;
    let i4 = laborde.a * cosps;
    let i2 = 0.5 * laborde.a * i4 * sinps;
    let i3 = i2 * laborde.a * laborde.a * (5. * cosps2 - sinps2) / 12.;
    let mut i6 = i4 * laborde.a * laborde.a;
    let i5 = i6 * (cosps2 - sinps2) / 6.;
    i6 *= laborde.a * laborde.a * (5. * cosps2 * cosps2 + sinps2 * (sinps2 - 18. * cosps2)) / 120.;
    t = lam * lam;
    let mut x = laborde.k_rg * lam * (i4 + t * (i5 + t * i6));
    let mut y = laborde.k_rg * (i1 + t * (i2 + t * i3));
    let x2 = x * x;
    let y2 = y * y;
    v1 = 3. * x * y2 - x * x2;
    v2 = y * y2 - 3. * x2 * y;
    x += laborde.ca * v1 + laborde.cb * v2;
    y += laborde.ca * v2 - laborde.cb * v1;

    p.set_x(x);
    p.set_y(y);
}

/// Laborde Ellipsoidal inverse project
pub fn labrd_e_inverse<P: TransformCoordinates>(laborde: &LabordeData, proj: &Proj, p: &mut P) {
    // t = 0.0 optimization is to avoid a false positive cppcheck warning
    let mut t;
    // (cppcheck git beaf29c15867984aa3c2a15cf15bd7576ccde2b3). Might no
    // longer be necessary with later versions.
    let mut x = p.x();
    let mut y = p.y();

    let mut x2 = x * x;
    let y2 = y * y;
    let mut v1 = 3. * x * y2 - x * x2;
    let mut v2 = y * y2 - 3. * x2 * y;
    let v3 = x * (5. * y2 * y2 + x2 * (-10. * y2 + x2));
    let v4 = y * (5. * x2 * x2 + y2 * (-10. * x2 + y2));
    x += -laborde.ca * v1 - laborde.cb * v2 + laborde.cc * v3 + laborde.cd * v4;
    y += laborde.cb * v1 - laborde.ca * v2 - laborde.cd * v3 + laborde.cc * v4;
    let ps = laborde.p0s + y / laborde.k_rg;
    let mut pe = ps + proj.phi0 - laborde.p0s;

    for _ in 0..20 {
        v1 = laborde.a * log(tan(M_FORTPI + 0.5 * pe));
        let tpe = proj.e * sin(pe);
        v2 = 0.5 * proj.e * laborde.a * log((1. + tpe) / (1. - tpe));
        t = ps - 2. * (atan(exp(v1 - v2 + laborde.c)) - M_FORTPI);
        pe += t;
        if fabs(t) < EPS {
            break;
        }
    }

    t = proj.e * sin(pe);
    t = 1. - t * t;
    let re = proj.one_es / (t * sqrt(t));
    t = tan(ps);
    let t2 = t * t;
    let s = laborde.k_rg * laborde.k_rg;
    let mut d = re * proj.k0 * laborde.k_rg;
    let i7 = t / (2. * d);
    let i8 = t * (5. + 3. * t2) / (24. * d * s);
    d = cos(ps) * laborde.k_rg * laborde.a;
    let i9 = 1. / d;
    d *= s;
    let i10 = (1. + 2. * t2) / (6. * d);
    let i11 = (5. + t2 * (28. + 24. * t2)) / (120. * d * s);
    x2 = x * x;

    p.set_phi(pe + x2 * (-i7 + i8 * x2));
    p.set_lam(x * (i9 + x2 * (-i10 + x2 * i11)));
}
