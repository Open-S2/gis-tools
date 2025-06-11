use crate::proj::{CoordinateStep, Proj, ProjectCoordinates, TransformCoordinates, aasin};
use core::{
    cell::RefCell,
    f64::consts::{FRAC_PI_2, PI, TAU},
};
use libm::{cos, fabs, sin, sqrt};

const MAX_ITER: usize = 30;
const LOOP_TOL: f64 = 1e-7;

/// Mollweide Variables
#[derive(Debug, Default, Clone, PartialEq)]
pub struct MollData {
    c_x: f64,
    c_y: f64,
    c_p: f64,
}

fn setup(proj: &mut Proj, p: f64) -> MollData {
    let mut store = MollData::default();
    let p2 = p + p;

    proj.es = 0.;
    let sp = sin(p);
    let r = sqrt(TAU * sp / (p2 + sin(p2)));

    store.c_x = 2. * r / PI;
    store.c_y = r / sp;
    store.c_p = p2 + sin(p2);

    store
}

/// Mollweide Projection
#[derive(Debug, Clone, PartialEq)]
pub struct MollweideProjection {
    proj: RefCell<Proj>,
    store: RefCell<MollData>,
}
impl ProjectCoordinates for MollweideProjection {
    fn code(&self) -> i64 {
        -1
    }
    fn name(&self) -> &'static str {
        "Mollweide"
    }
    fn names() -> &'static [&'static str] {
        &["Mollweide", "moll"]
    }
}
impl CoordinateStep for MollweideProjection {
    fn new(proj: RefCell<Proj>) -> Self {
        let store = setup(&mut proj.borrow_mut(), FRAC_PI_2);
        MollweideProjection { proj, store: store.into() }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        moll_s_forward(&self.store.borrow(), p);
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        moll_s_inverse(&self.store.borrow(), p);
    }
}

/// Wagner IV Projection
#[derive(Debug, Clone, PartialEq)]
pub struct WagnerIVProjection {
    proj: RefCell<Proj>,
    store: RefCell<MollData>,
}
impl ProjectCoordinates for WagnerIVProjection {
    fn code(&self) -> i64 {
        -1
    }
    fn name(&self) -> &'static str {
        "Wagner IV"
    }
    fn names() -> &'static [&'static str] {
        &["Wagner IV", "wag4"]
    }
}
impl CoordinateStep for WagnerIVProjection {
    fn new(proj: RefCell<Proj>) -> Self {
        let store = setup(&mut proj.borrow_mut(), PI / 3.);
        WagnerIVProjection { proj, store: store.into() }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        moll_s_forward(&self.store.borrow(), p);
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        moll_s_inverse(&self.store.borrow(), p);
    }
}

/// Wagner V Projection
#[derive(Debug, Clone, PartialEq)]
pub struct WagnerVProjection {
    proj: RefCell<Proj>,
    store: RefCell<MollData>,
}
impl ProjectCoordinates for WagnerVProjection {
    fn code(&self) -> i64 {
        -1
    }
    fn name(&self) -> &'static str {
        "Wagner V"
    }
    fn names() -> &'static [&'static str] {
        &["Wagner V", "wag5"]
    }
}
impl CoordinateStep for WagnerVProjection {
    fn new(proj: RefCell<Proj>) -> Self {
        proj.borrow_mut().es = 0.0;
        let store = MollData { c_x: 0.90977, c_y: 1.65014, c_p: 3.00896 };
        WagnerVProjection { proj, store: store.into() }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        moll_s_forward(&self.store.borrow(), p);
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        moll_s_inverse(&self.store.borrow(), p);
    }
}

/// Mollweide Spheroidal inverse project
pub fn moll_s_forward<P: TransformCoordinates>(moll: &MollData, p: &mut P) {
    let k = moll.c_p * sin(p.phi());
    let mut i = MAX_ITER;
    while i > 0 {
        let v = (p.phi() + sin(p.phi()) - k) / (1. + cos(p.phi()));
        p.set_phi(p.phi() - v);
        if fabs(v) < LOOP_TOL {
            break;
        }
        i -= 1;
    }
    if i != 0 {
        p.set_phi(if p.phi() < 0. { -FRAC_PI_2 } else { FRAC_PI_2 });
    } else {
        p.set_phi(p.phi() * 0.5);
    }
    p.set_x(moll.c_x * p.lam() * cos(p.phi()));
    p.set_y(moll.c_y * sin(p.phi()));
}

/// Mollweide Spheroidal inverse project
pub fn moll_s_inverse<P: TransformCoordinates>(moll: &MollData, p: &mut P) {
    let mut phi = aasin(p.y() / moll.c_y);
    let mut lam = p.x() / (moll.c_x * cos(phi));
    if fabs(lam) < PI {
        phi += phi;
        phi = aasin((phi + sin(phi)) / moll.c_p);
    } else {
        phi = f64::MAX;
        lam = f64::MAX;
    }
    p.set_phi(phi);
    p.set_lam(lam);
}
