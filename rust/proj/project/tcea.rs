use crate::proj::{CoordinateStep, Proj, ProjectCoordinates, TransformCoordinates};
use core::cell::RefCell;
use libm::{asin, atan2, cos, sin, sqrt, tan};

/// Transverse Cylindrical Equal Area Projection
#[derive(Debug, Clone, PartialEq)]
pub struct TransverseCylindricalEqualArealProjection {
    proj: RefCell<Proj>,
}
impl ProjectCoordinates for TransverseCylindricalEqualArealProjection {
    fn code(&self) -> i64 {
        -1
    }
    fn name(&self) -> &'static str {
        "Transverse Cylindrical Equal Area"
    }
    fn names() -> &'static [&'static str] {
        &["Transverse Cylindrical Equal Area", "tcea"]
    }
}
impl CoordinateStep for TransverseCylindricalEqualArealProjection {
    fn new(proj: RefCell<Proj>) -> Self {
        proj.borrow_mut().es = 0.;
        TransverseCylindricalEqualArealProjection { proj }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        tcea_s_forward(&self.proj.borrow(), p);
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        tcea_s_inverse(&self.proj.borrow(), p);
    }
}

/// Transverse Cylindrical Equal Area forward project
pub fn tcea_s_forward<P: TransformCoordinates>(proj: &Proj, p: &mut P) {
    p.set_x(cos(p.phi()) * sin(p.lam()) / proj.k0);
    p.set_y(proj.k0 * (atan2(tan(p.phi()), cos(p.lam())) - proj.phi0));
}

/// Transverse Cylindrical Equal Area inverse project
pub fn tcea_s_inverse<P: TransformCoordinates>(proj: &Proj, p: &mut P) {
    let y = p.y() / proj.k0 + proj.phi0;
    let x = p.x() * proj.k0;
    let t = sqrt(1. - x * x);
    p.set_phi(asin(t * sin(y)));
    p.set_lam(atan2(x, t * cos(y)));
}
