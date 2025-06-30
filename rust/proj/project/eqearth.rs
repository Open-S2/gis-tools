use crate::proj::{
    CoordinateStep, EQUAL_EARTH, Proj, ProjectCoordinates, TransformCoordinates,
    authalic_lat_compute_coeffs, authalic_lat_inverse, authalic_lat_q,
};
use alloc::{rc::Rc, vec::Vec};
use core::cell::RefCell;
use libm::{asin, cos, fabs, sin, sqrt};

// Equal Earth is a projection inspired by the Robinson projection, but unlike
// the Robinson projection retains the relative size of areas. The projection
// was designed in 2018 by Bojan Savric, Tom Patterson and Bernhard Jenny.
//
// Publication:
// Bojan Savric, Tom Patterson & Bernhard Jenny (2018). The Equal Earth map
// projection, International Journal of Geographical Information Science,
// DOI: 10.1080/13658816.2018.1504949
//
// Port to PROJ by Juernjakob Dugge, 16 August 2018
// Added ellipsoidal equations by Bojan Savric, 22 August 2018

// A1..A4, polynomial coefficients
const A1: f64 = 1.340264;
const A2: f64 = -0.081106;
const A3: f64 = 0.000893;
const A4: f64 = 0.003796;
const M: f64 = 0.8660254037844386; // sqrt(3.0) / 2.0;

// 90° latitude on a sphere with radius 1
const MAX_Y: f64 = 1.3173627591574;
const EPS: f64 = 1e-11;
const MAX_ITER: usize = 12;

/// Equal Earth variables
#[derive(Debug, Default, Clone, PartialEq)]
pub struct EqEarth {
    qp: f64,
    rqda: f64,
    apa: Vec<f64>,
}

/// Equal Earth Projection
#[derive(Debug, Clone, PartialEq)]
pub struct EqualEarthProjection {
    proj: Rc<RefCell<Proj>>,
    store: RefCell<EqEarth>,
}
impl ProjectCoordinates for EqualEarthProjection {
    fn code(&self) -> i64 {
        EQUAL_EARTH
    }
    fn name(&self) -> &'static str {
        "Equal Earth"
    }
    fn names() -> &'static [&'static str] {
        &["Equal Earth", "EqualEarth", "eqearth"]
    }
}
impl CoordinateStep for EqualEarthProjection {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        let mut store = EqEarth { rqda: 1.0, ..Default::default() };
        {
            let proj = &mut proj.borrow_mut();
            // Ellipsoidal case
            if proj.es != 0.0 {
                store.apa = authalic_lat_compute_coeffs(proj.n); // For auth_lat().
                store.qp = authalic_lat_q(1.0, proj); // For auth_lat().
                store.rqda = sqrt(0.5 * store.qp); // Authalic radius divided by major axis
            }
        }
        EqualEarthProjection { proj, store: store.into() }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        eqearth_e_forward(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        eqearth_e_inverse(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
    }
}

/// Equal Earth Ellipsoidal/spheroidal forward project
pub fn eqearth_e_forward<P: TransformCoordinates>(eq_earth: &mut EqEarth, proj: &Proj, p: &mut P) {
    // Spheroidal case, using sine latitude
    let mut sbeta = sin(p.phi());

    // In the ellipsoidal case, we convert sbeta to sine of authalic latitude
    if proj.es != 0.0 {
        sbeta = authalic_lat_q(sbeta, proj) / eq_earth.qp;

        // Rounding error.
        if fabs(sbeta) > 1. {
            sbeta = if sbeta > 0. { 1. } else { -1. };
        }
    }

    // Equal Earth projection
    let psi = asin(M * sbeta);
    let psi2 = psi * psi;
    let psi6 = psi2 * psi2 * psi2;

    let mut x =
        p.lam() * cos(psi) / (M * (A1 + 3. * A2 * psi2 + psi6 * (7. * A3 + 9. * A4 * psi2)));
    let mut y = psi * (A1 + A2 * psi2 + psi6 * (A3 + A4 * psi2));

    // Adjusting x and y for authalic radius
    x *= eq_earth.rqda;
    y *= eq_earth.rqda;

    p.set_x(x);
    p.set_y(y);
}

/// Equal Earth Ellipsoidal/spheroidal inverse project
pub fn eqearth_e_inverse<P: TransformCoordinates>(eq_earth: &mut EqEarth, proj: &Proj, p: &mut P) {
    // Adjusting x and y for authalic radius
    let x = p.x() / eq_earth.rqda;
    let mut y = p.y() / eq_earth.rqda;

    // Make sure y is inside valid range
    y = y.clamp(-MAX_Y, MAX_Y);

    let mut yc = y;

    // Newton-Raphson
    let mut i = MAX_ITER;
    while i > 0 {
        let y2 = yc * yc;
        let y6 = y2 * y2 * y2;

        let f = yc * (A1 + A2 * y2 + y6 * (A3 + A4 * y2)) - y;
        let fder = A1 + 3. * A2 * y2 + y6 * (7. * A3 + 9. * A4 * y2);

        let tol = f / fder;
        yc -= tol;

        if fabs(tol) < EPS {
            break;
        }
        i -= 1;
    }

    if i == 0 {
        panic!("Coordinate outside projection domain");
    }

    // Longitude
    let y2 = yc * yc;
    let y6 = y2 * y2 * y2;

    p.set_lam(M * x * (A1 + 3. * A2 * y2 + y6 * (7. * A3 + 9. * A4 * y2)) / cos(yc));

    // Latitude (for spheroidal case, this is latitude
    p.set_phi(asin(sin(yc) / M));

    // Ellipsoidal case, converting auth. latitude
    if proj.es != 0.0 {
        p.set_phi(authalic_lat_inverse(p.phi(), &eq_earth.apa, proj, eq_earth.qp));
    }
}
