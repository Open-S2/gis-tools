use crate::proj::{
    CoordinateStep, OBLIQUE_STEREOGRAPHIC, Proj, ProjectCoordinates, TransformCoordinates,
};
use alloc::rc::Rc;
use core::{
    cell::RefCell,
    f64::consts::{FRAC_PI_2, FRAC_PI_4},
};
use libm::{asin, atan, atan2, cos, fabs, hypot, pow, sin, sqrt, tan};

/// Gaussian Variables
#[derive(Debug, Default, Clone, PartialEq)]
struct Gauss {
    c: f64,
    k: f64,
    e: f64,
    ratexp: f64,
}

/// Oblique Stereographic Alternative Variables
#[derive(Debug, Default, Clone, PartialEq)]
pub struct StereaData {
    phic0: f64,
    cosc0: f64,
    sinc0: f64,
    r2: f64,
    en: Gauss,
}

const MAX_ITER: usize = 20;
const DEL_TOL: f64 = 1e-14;

/// Oblique Stereographic Alternative Projection
#[derive(Debug, Clone, PartialEq)]
pub struct ObliqueStereographicAlternativeProjection {
    proj: Rc<RefCell<Proj>>,
    store: RefCell<StereaData>,
}
impl ProjectCoordinates for ObliqueStereographicAlternativeProjection {
    fn code(&self) -> i64 {
        OBLIQUE_STEREOGRAPHIC
    }
    fn name(&self) -> &'static str {
        "Oblique Stereographic Alternative"
    }
    fn names() -> &'static [&'static str] {
        &["Oblique Stereographic Alternative", "sterea"]
    }
}
impl CoordinateStep for ObliqueStereographicAlternativeProjection {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        let mut store = StereaData::default();
        {
            let proj = &mut proj.borrow_mut();
            let mut r: f64 = 0.;

            store.en = gauss_ini(proj.e, proj.phi0, &mut store.phic0, &mut r);
            store.sinc0 = sin(store.phic0);
            store.cosc0 = cos(store.phic0);
            store.r2 = 2. * r;
        }
        ObliqueStereographicAlternativeProjection { proj, store: store.into() }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        sterea_e_forward(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        sterea_e_inverse(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
    }
}

/// Oblique Stereographic Alternative Ellipsoidal forward project
pub fn sterea_e_forward<P: TransformCoordinates>(sterea: &mut StereaData, proj: &Proj, p: &mut P) {
    gauss(p, &sterea.en);
    let sinc = sin(p.phi());
    let cosc = cos(p.phi());
    let cosl = cos(p.lam());
    let denom = 1. + sterea.sinc0 * sinc + sterea.cosc0 * cosc * cosl;
    if denom == 0.0 {
        panic!("Coordinate outside projection domain");
    }
    let k = proj.k0 * sterea.r2 / denom;
    p.set_x(k * cosc * sin(p.lam()));
    p.set_y(k * (sterea.cosc0 * sinc - sterea.sinc0 * cosc * cosl));
}

/// Oblique Stereographic Alternative Ellipsoidal inverse project
pub fn sterea_e_inverse<P: TransformCoordinates>(sterea: &mut StereaData, proj: &Proj, p: &mut P) {
    let x = p.x() / proj.k0;
    let y = p.y() / proj.k0;
    let rho = hypot(x, y);
    if rho != 0.0 {
        let c = 2. * atan2(rho, sterea.r2);
        let sinc = sin(c);
        let cosc = cos(c);
        p.set_phi(asin(cosc * sterea.sinc0 + y * sinc * sterea.cosc0 / rho));
        p.set_lam(atan2(x * sinc, rho * sterea.cosc0 * cosc - y * sterea.sinc0 * sinc));
    } else {
        p.set_phi(sterea.phic0);
        p.set_lam(0.);
    }
    inv_gauss(p, &sterea.en);
}

fn srat(esinp: f64, ratexp: f64) -> f64 {
    pow((1. - esinp) / (1. + esinp), ratexp)
}

fn gauss_ini(e: f64, phi0: f64, chi: &mut f64, rc: &mut f64) -> Gauss {
    let mut en = Gauss::default();

    let es = e * e;
    en.e = e;
    let sphi = sin(phi0);
    let mut cphi = cos(phi0);
    cphi *= cphi;
    *rc = sqrt(1. - es) / (1. - es * sphi * sphi);
    en.c = sqrt(1. + es * cphi * cphi / (1. - es));
    if en.c == 0.0 {
        panic!("Failed to initialize Gauss projection");
    }
    *chi = asin(sphi / en.c);
    en.ratexp = 0.5 * en.c * e;
    let srat_val = srat(en.e * sphi, en.ratexp);
    if srat_val == 0.0 {
        panic!("Failed to initialize Gauss projection");
    }
    if 0.5 * phi0 + FRAC_PI_4 < 1e-10 {
        en.k = 1.0 / srat_val;
    } else {
        en.k = tan(0.5 * *chi + FRAC_PI_4) / (pow(tan(0.5 * phi0 + FRAC_PI_4), en.c) * srat_val);
    }

    en
}

fn gauss<P: TransformCoordinates>(elp: &mut P, en: &Gauss) {
    elp.set_phi(
        2. * atan(
            en.k * pow(tan(0.5 * elp.phi() + FRAC_PI_4), en.c)
                * srat(en.e * sin(elp.phi()), en.ratexp),
        ) - FRAC_PI_2,
    );
    elp.set_lam(en.c * (elp.lam()));
}

fn inv_gauss<P: TransformCoordinates>(p: &mut P, en: &Gauss) {
    p.set_lam(p.lam() / en.c);
    let num = pow(tan(0.5 * p.phi() + FRAC_PI_4) / en.k, 1. / en.c);
    let mut i = MAX_ITER;
    while i > 0 {
        p.set_phi(2. * atan(num * srat(en.e * sin(p.phi()), -0.5 * en.e)) - FRAC_PI_2);
        if fabs(p.phi() - p.phi()) < DEL_TOL {
            break;
        }
        p.set_phi(p.phi());
        i -= 1;
    }
    // convergence failed
    if i != 0 {
        panic!("Coordinate outside projection domain");
    }
}
