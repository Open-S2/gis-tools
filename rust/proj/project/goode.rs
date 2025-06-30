use crate::proj::{
    CoordinateStep, MollweideProjection, Proj, ProjectCoordinates, SinusoidalProjection,
    TransformCoordinates,
};
use alloc::rc::Rc;
use core::cell::RefCell;
use libm::fabs;

const Y_COR: f64 = 0.05280;
const PHI_LIM: f64 = 0.710_930_781_979_023_6;

/// Goode Homolosine Variables
#[derive(Debug, Clone, PartialEq)]
pub struct GoodeData {
    sinu: SinusoidalProjection,
    moll: MollweideProjection,
}

/// Goode Homolosine Projection
#[derive(Debug, Clone, PartialEq)]
pub struct GoodeHomolosineProjection {
    proj: Rc<RefCell<Proj>>,
    store: RefCell<GoodeData>,
}
impl ProjectCoordinates for GoodeHomolosineProjection {
    fn code(&self) -> i64 {
        -1
    }
    fn name(&self) -> &'static str {
        "Goode Homolosine"
    }
    fn names() -> &'static [&'static str] {
        &["Goode Homolosine", "goode"]
    }
}
impl CoordinateStep for GoodeHomolosineProjection {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        proj.borrow_mut().es = 0.;
        let sinu = SinusoidalProjection::new(proj.clone());
        let moll = MollweideProjection::new(proj.clone());
        let store = GoodeData { sinu, moll };

        GoodeHomolosineProjection { proj, store: store.into() }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        goode_s_forward(&self.store.borrow(), p);
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        goode_s_inverse(&self.store.borrow(), p);
    }
}

/// Goode Homolosine Spheroidal forward project
pub fn goode_s_forward<P: TransformCoordinates>(goode: &GoodeData, p: &mut P) {
    if fabs(p.phi()) <= PHI_LIM {
        goode.sinu.forward(p);
    } else {
        goode.moll.forward(p);
        p.set_y(p.y() - if p.phi() >= 0. { Y_COR } else { -Y_COR });
    }
}

/// Goode Homolosine Spheroidal inverse project
pub fn goode_s_inverse<P: TransformCoordinates>(goode: &GoodeData, p: &mut P) {
    if fabs(p.y()) <= PHI_LIM {
        goode.sinu.inverse(p);
    } else {
        p.set_y(if p.y() >= 0. { Y_COR } else { -Y_COR });
        goode.moll.inverse(p);
    }
}
