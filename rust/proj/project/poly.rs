use crate::proj::{
    _msfn, CoordinateStep, POLYCONIC, Proj, ProjMethod, ProjectCoordinates, TransformCoordinates,
    enfn, mlfn,
};
use alloc::{rc::Rc, vec::Vec};
use core::cell::RefCell;
use libm::{asin, cos, fabs, sin, sqrt, tan};

/// Polyconic Variables
#[derive(Debug, Default, Clone, PartialEq)]
pub struct PolyData {
    ml0: f64,
    en: Vec<f64>,
}

const TOL: f64 = 1e-10;
const CONV: f64 = 1e-10;
const N_ITER: usize = 10;
const I_ITER: usize = 20;
const ITOL: f64 = 1e-12;

/// Polyconic (American) Projection
#[derive(Debug, Clone, PartialEq)]
pub struct PolyconicProjection {
    proj: Rc<RefCell<Proj>>,
    store: RefCell<PolyData>,
    method: ProjMethod,
}
impl ProjectCoordinates for PolyconicProjection {
    fn code(&self) -> i64 {
        POLYCONIC
    }
    fn name(&self) -> &'static str {
        "Polyconic"
    }
    fn names() -> &'static [&'static str] {
        &["Polyconic", "Polyconic (American)", "poly"]
    }
}
impl CoordinateStep for PolyconicProjection {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        let mut store = PolyData::default();
        let method: ProjMethod;
        {
            let proj = &mut proj.borrow_mut();

            method = if proj.es != 0.0 {
                store.en = enfn(proj.n);
                store.ml0 = mlfn(proj.phi0, sin(proj.phi0), cos(proj.phi0), &store.en);
                ProjMethod::Ellipsoidal
            } else {
                store.ml0 = -proj.phi0;
                ProjMethod::Spheroidal
            }
        }

        PolyconicProjection { proj, store: store.into(), method }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        if self.method == ProjMethod::Spheroidal {
            poly_s_forward(&self.store.borrow(), &self.proj.borrow(), p);
        } else {
            poly_e_forward(&self.store.borrow(), &self.proj.borrow(), p);
        }
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        if self.method == ProjMethod::Spheroidal {
            poly_s_inverse(&self.proj.borrow(), p);
        } else {
            poly_e_inverse(&self.store.borrow(), &self.proj.borrow(), p);
        }
    }
}

/// Polyconic Ellipsoidal forward project
pub fn poly_e_forward<P: TransformCoordinates>(poly: &PolyData, proj: &Proj, p: &mut P) {
    if fabs(p.phi()) <= TOL {
        p.set_x(p.lam());
        p.set_y(-poly.ml0);
    } else {
        let sp = sin(p.phi());
        let cp = cos(p.phi());
        let ms = if fabs(cp) > TOL { _msfn(sp, cp, proj.es) / sp } else { 0. };
        p.set_lam(p.lam() * sp);
        p.set_x(ms * sin(p.lam()));
        p.set_y((mlfn(p.phi(), sp, cp, &poly.en) - poly.ml0) + ms * (1. - cos(p.lam())));
    }
}

/// Polyconic Spheroidal forward project
pub fn poly_s_forward<P: TransformCoordinates>(poly: &PolyData, proj: &Proj, p: &mut P) {
    if fabs(p.phi()) <= TOL {
        p.set_x(p.lam());
        p.set_y(poly.ml0);
    } else {
        let cot = 1. / tan(p.phi());
        let e = p.lam() * sin(p.phi());
        p.set_x(sin(e) * cot);
        p.set_y(p.phi() - proj.phi0 + cot * (1. - cos(e)));
    }
}

/// Polyconic Ellipsoidal inverse project
pub fn poly_e_inverse<P: TransformCoordinates>(poly: &PolyData, proj: &Proj, p: &mut P) {
    let x = p.x();
    let mut y = p.y();
    let mut phi;
    let lam;
    y += poly.ml0;
    if fabs(y) <= TOL {
        lam = x;
        phi = 0.;
    } else {
        let r = y * y + x * x;
        phi = y;
        let mut i = I_ITER;
        while i > 0 {
            let sp = sin(phi);
            let cp = cos(phi);
            let s2ph = sp * cp;
            if fabs(cp) < ITOL {
                panic!("Coordinate outside projection domain");
            }
            let mut mlp = sqrt(1. - proj.es * sp * sp);
            let c = sp * mlp / cp;
            let ml = mlfn(phi, sp, cp, &poly.en);
            let mlb = ml * ml + r;
            mlp = proj.one_es / (mlp * mlp * mlp);
            let d_phi = (ml + ml + c * mlb - 2. * y * (c * ml + 1.))
                / (proj.es * s2ph * (mlb - 2. * y * ml) / c
                    + 2. * (y - ml) * (c * mlp - 1. / s2ph)
                    - mlp
                    - mlp);
            phi += d_phi;
            if fabs(d_phi) <= ITOL {
                break;
            }
            i -= 1;
        }
        if i != 0 {
            panic!("Coordinate outside projection domain");
        }
        let c = sin(phi);
        lam = asin(x * tan(phi) * sqrt(1. - proj.es * c * c)) / sin(phi);
    }

    p.set_lam(lam);
    p.set_phi(phi);
}

/// Polyconic Spheroidal inverse project
pub fn poly_s_inverse<P: TransformCoordinates>(proj: &Proj, p: &mut P) {
    let x = p.x();
    let y = proj.phi0 + p.y();
    if fabs(y) <= TOL {
        p.set_lam(x);
        p.set_phi(0.);
    } else {
        p.set_phi(y);
        let b = x * x + y * y;
        let mut i = N_ITER;
        loop {
            let tp = tan(p.phi());
            let dphi = (y * (p.phi() * tp + 1.) - p.phi() - 0.5 * (p.phi() * p.phi() + b) * tp)
                / ((p.phi() - y) / tp - 1.);
            p.set_phi(p.phi() - dphi);
            if fabs(dphi) <= CONV {
                break;
            }
            i -= 1;
            if i == 0 {
                panic!("Coordinate outside projection domain");
            }
        }
        p.set_lam(asin(x * tan(p.phi())) / sin(p.phi()));
    }
}
