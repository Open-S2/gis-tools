use crate::proj::{CoordinateStep, EPS10, Proj, ProjectCoordinates, TransformCoordinates};
use alloc::rc::Rc;
use core::cell::RefCell;
use libm::{atan2, cos, sin, sqrt, tan};

/// Transverse Central Cylindrical Projection
#[derive(Debug, Clone, PartialEq)]
pub struct TransverseCentralCylindricalProjection {
    proj: Rc<RefCell<Proj>>,
}
impl ProjectCoordinates for TransverseCentralCylindricalProjection {
    fn code(&self) -> i64 {
        -1
    }
    fn name(&self) -> &'static str {
        "Transverse Central Cylindrical"
    }
    fn names() -> &'static [&'static str] {
        &["Transverse Central Cylindrical", "tcc"]
    }
}
impl CoordinateStep for TransverseCentralCylindricalProjection {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        proj.borrow_mut().es = 0.;
        TransverseCentralCylindricalProjection { proj }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        tcc_s_forward(p);
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        tcc_s_inverse(p);
    }
}

/// Transverse Central Cylindrical forward project
/// let b = cos(φ) * sin(λ);
/// x = b / sqrt(1 - b²);
/// y = atan2(tan(φ), cos(λ));
pub fn tcc_s_forward<P: TransformCoordinates>(p: &mut P) {
    let b = cos(p.phi()) * sin(p.lam());
    let bt = 1. - b * b;
    if bt < EPS10 {
        panic!("Coordinate outside projection domain");
    }
    p.set_x(b / sqrt(bt));
    p.set_y(atan2(tan(p.phi()), cos(p.lam())));
}

/// Transverse Central Cylindrical inverse project
pub fn tcc_s_inverse<P: TransformCoordinates>(p: &mut P) {
    let x = p.x();
    let y = p.y();

    let denom = sqrt(1. + x * x);
    let phi = (y.sin() / denom).atan();
    let lam = (x / y.cos()).atan();

    p.set_phi(phi);
    p.set_lam(lam);
}
