use crate::proj::{
    CASSINI, CoordinateStep, HYPERBOLIC, Proj, ProjMethod, ProjectCoordinates,
    TransformCoordinates, enfn, generic_inverse_2d, inv_mlfn, mlfn,
};
use alloc::{rc::Rc, vec::Vec};
use core::cell::RefCell;
use libm::{asin, atan2, cos, sin, sqrt, tan};

const C1: f64 = 0.166_666_666_666_666_66;
const C2: f64 = 0.008_333_333_333_333_333;
const C3: f64 = 0.041_666_666_666_666_664;
const C4: f64 = 0.333_333_333_333_333_3;
const C5: f64 = 0.066_666_666_666_666_67;

/// Cassini variables
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CassData {
    en: Vec<f64>,
    m0: f64,
    hyperbolic: bool,
}

/// Cassini Projection
#[derive(Debug, Clone, PartialEq)]
pub struct CassiniProjection {
    proj: Rc<RefCell<Proj>>,
    store: RefCell<CassData>,
    method: ProjMethod,
}
impl ProjectCoordinates for CassiniProjection {
    fn code(&self) -> i64 {
        CASSINI
    }
    fn name(&self) -> &'static str {
        "Cassini"
    }
    fn names() -> &'static [&'static str] {
        &["Cassini", "Cassini-Soldner", "cass"]
    }
}
impl CoordinateStep for CassiniProjection {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        let mut store = CassData::default();
        let method: ProjMethod;
        {
            let proj = &mut proj.borrow_mut();
            // Spheroidal?
            method = if 0. == proj.es {
                ProjMethod::Spheroidal
            } else {
                // otherwise it's ellipsoidal
                store.en = enfn(proj.n);
                store.m0 = mlfn(proj.phi0, sin(proj.phi0), cos(proj.phi0), &store.en);
                if proj.params.contains_key(&HYPERBOLIC) {
                    store.hyperbolic = true;
                }
                ProjMethod::Ellipsoidal
            };
        }
        CassiniProjection { proj, store: store.into(), method }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        if self.method == ProjMethod::Ellipsoidal {
            cass_e_forward(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
        } else {
            cass_s_forward(&self.proj.borrow(), p);
        }
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        if self.method == ProjMethod::Ellipsoidal {
            let es = self.proj.borrow().es;
            cass_e_inverse(self, &mut self.store.borrow_mut(), es, p);
        } else {
            cass_s_inverse(&self.proj.borrow(), p);
        }
    }
}

/// Cassini Ellipsoidal forward project
pub fn cass_e_forward<P: TransformCoordinates>(cass: &mut CassData, proj: &Proj, p: &mut P) {
    let sinphi = sin(p.phi());
    let cosphi = cos(p.phi());
    let m = mlfn(p.phi(), sinphi, cosphi, &cass.en);

    let nu_square = 1. / (1. - proj.es * sinphi * sinphi);
    let nu = sqrt(nu_square);
    let tanphi = tan(p.phi());
    let t = tanphi * tanphi;
    let a = p.lam() * cosphi;
    let c = proj.es * (cosphi * cosphi) / (1. - proj.es);
    let a2 = a * a;

    let x = nu * a * (1. - a2 * t * (C1 + (8. - t + 8. * c) * a2 * C2));
    let mut y = m - cass.m0 + nu * tanphi * a2 * (0.5 + (5. - t + 6. * c) * a2 * C3);
    if cass.hyperbolic {
        let rho = nu_square * (1. - proj.es) * nu;
        y -= y * y * y / (6. * rho * nu);
    }

    p.set_x(x);
    p.set_y(y);
}

/// Cassini Spheroidal forward project
pub fn cass_s_forward<P: TransformCoordinates>(proj: &Proj, p: &mut P) {
    p.set_x(asin(cos(p.phi()) * sin(p.lam())));
    p.set_y(atan2(tan(p.phi()), cos(p.lam())) - proj.phi0);
}

/// Cassini Ellipsoidal inverse project
pub fn cass_e_inverse<P: TransformCoordinates>(
    cass: &CassiniProjection,
    cass_data: &mut CassData,
    es: f64,
    p: &mut P,
) {
    let phi1 = inv_mlfn(cass_data.m0 + p.y(), &cass_data.en);
    let tanphi1 = tan(phi1);
    let t1 = tanphi1 * tanphi1;
    let sinphi1 = sin(phi1);
    let nu1_square = 1. / (1. - es * sinphi1 * sinphi1);
    let nu1 = sqrt(nu1_square);
    let rho1 = nu1_square * (1. - es) * nu1;
    let d = p.x() / nu1;
    let d2 = d * d;
    let mut lp = P::default();
    lp.set_phi(phi1 - (nu1 * tanphi1 / rho1) * d2 * (0.5 - (1. + 3. * t1) * d2 * C3));
    lp.set_lam(d * (1. + t1 * d2 * (-C4 + (1. + 3. * t1) * d2 * C5)) / cos(phi1));

    // EPSG guidance note 7-2 suggests a custom approximation for the
    // 'Vanua Levu 1915 / Vanua Levu Grid' case, but better use the
    // generic inversion method
    // Actually use it in the non-hyperbolic case. It enables to make the
    // 5108.gie roundtripping tests to success, with at most 2 iterations.
    let delta_xy_tolerance = 1e-12;
    generic_inverse_2d(p, cass, &mut lp, delta_xy_tolerance);

    p.set_phi(lp.phi());
    p.set_lam(lp.lam());
}

/// Cassini Spheroidal inverse project
pub fn cass_s_inverse<P: TransformCoordinates>(proj: &Proj, p: &mut P) {
    let dd = p.y() + proj.phi0;
    p.set_phi(asin(sin(dd) * cos(p.x())));
    p.set_lam(atan2(tan(p.x()), cos(dd)));
}
