use crate::proj::{
    CoordinateStep, EQUIDISTANT_CYLINDRICAL, LATITUDE_STD_PARALLEL, Proj, ProjValue,
    ProjectCoordinates, TransformCoordinates,
};
use alloc::rc::Rc;
use core::cell::RefCell;
use libm::cos;

/// Equidistant Cylindrical variables
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Eqc {
    rc: f64,
}

/// Equidistant Cylindrical Projection
#[derive(Debug, Clone, PartialEq)]
pub struct EquidistantCylindricalProjection {
    proj: Rc<RefCell<Proj>>,
    store: RefCell<Eqc>,
}
impl ProjectCoordinates for EquidistantCylindricalProjection {
    fn code(&self) -> i64 {
        EQUIDISTANT_CYLINDRICAL
    }
    fn name(&self) -> &'static str {
        "Equidistant Cylindrical"
    }
    fn names() -> &'static [&'static str] {
        &[
            "Equidistant Cylindrical",
            "EquidistantCylindrical",
            "Equidistant Cylindrical (Plate Carree)",
            "eqc",
        ]
    }
}
impl CoordinateStep for EquidistantCylindricalProjection {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        let mut store = Eqc::default();
        {
            let proj = &mut proj.borrow_mut();
            let lat_ts = proj
                .params
                .get(&LATITUDE_STD_PARALLEL) // (lat_ts)
                .unwrap_or(&ProjValue::default())
                .f64()
                .to_radians();
            if cos(lat_ts) <= 0. {
                panic!("Invalid value for lat_ts: |lat_ts| should be <= 90°");
            }
            store.rc = lat_ts;
            proj.es = 0.;
        }
        EquidistantCylindricalProjection { proj, store: store.into() }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        eqc_s_forward(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        eqc_s_inverse(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
    }
}

/// Equidistant Cylindrical Spheroidal forward project
pub fn eqc_s_forward<P: TransformCoordinates>(eqc: &mut Eqc, proj: &Proj, p: &mut P) {
    p.set_x(eqc.rc * p.lam());
    p.set_y(p.phi() - proj.phi0);
}

/// Equidistant Cylindrical Spheroidal inverse project
pub fn eqc_s_inverse<P: TransformCoordinates>(eqc: &mut Eqc, proj: &Proj, p: &mut P) {
    p.set_lam(p.x() / eqc.rc);
    p.set_phi(p.y() + proj.phi0);
}
