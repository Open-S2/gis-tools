use crate::proj::{
    _msfn, CoordinateStep, EPS10, LAMBERT_CONFORMAL_CONIC_1SP, LAMBERT_CONFORMAL_CONIC_2SP,
    LATITUDE_OF_FIRST_STANDARD_PARALLEL, LATITUDE_OF_PROJECTION_CENTRE,
    LATITUDE_OF_SECOND_STANDARD_PARALLEL, Proj, ProjValue, ProjectCoordinates,
    TransformCoordinates, phi2, tsfn,
};
use core::{
    cell::RefCell,
    f64::consts::{FRAC_PI_2, FRAC_PI_4},
};
use libm::{atan, atan2, cos, fabs, hypot, log, pow, sin, tan};

/// Lambert Conformal Conic variables
#[derive(Debug, Default, Clone, PartialEq)]
pub struct LccData {
    phi1: f64,
    phi2: f64,
    n: f64,
    rho0: f64,
    c: f64,
}

/// Lambert Conformal Conic projection
pub type LambertConformalConic1SPProjection =
    LambertConformalConicProjection<LAMBERT_CONFORMAL_CONIC_1SP>;
/// Lambert Conic Conformal (2SP)
pub type LambertConformalConic2SPProjection =
    LambertConformalConicProjection<LAMBERT_CONFORMAL_CONIC_2SP>;

/// Lambert Conformal Conic projection
#[derive(Debug, Clone, PartialEq)]
pub struct LambertConformalConicProjection<const C: i64> {
    proj: RefCell<Proj>,
    store: RefCell<LccData>,
}
impl<const C: i64> ProjectCoordinates for LambertConformalConicProjection<C> {
    fn code(&self) -> i64 {
        C
    }
    fn name(&self) -> &'static str {
        "Lambert Conformal Conic"
    }
    fn names() -> &'static [&'static str] {
        &[
            "Lambert Conic Conformal (1SP)",
            "Lambert Conic Conformal (2SP)",
            "Lambert Conic Conformal (LCC)",
            "Lambert Conformal Conic",
            "LambertConformalConic",
            "lcc",
        ]
    }
}
impl<const C: i64> CoordinateStep for LambertConformalConicProjection<C> {
    fn new(proj: RefCell<Proj>) -> Self {
        let mut store = LccData::default();
        {
            let proj = &mut proj.borrow_mut();
            let lat_1 = proj
                .params
                .get(&LATITUDE_OF_FIRST_STANDARD_PARALLEL)
                .unwrap_or(&ProjValue::default())
                .f64();
            let lat_2 = proj
                .params
                .get(&LATITUDE_OF_SECOND_STANDARD_PARALLEL)
                .unwrap_or(&ProjValue::default())
                .f64();
            store.phi1 = lat_1;
            if lat_2 != 0. {
                store.phi2 = lat_2;
            } else {
                store.phi2 = store.phi1;
                if proj.params.contains_key(&LATITUDE_OF_PROJECTION_CENTRE) {
                    proj.phi0 = store.phi1;
                }
            }

            if fabs(store.phi1 + store.phi2) < EPS10 {
                panic!("Invalid value for lat_1 and lat_2: |lat_1 + lat_2| should be > 0");
            }

            let mut sinphi = sin(store.phi1);
            store.n = sinphi;
            let cosphi = cos(store.phi1);

            if fabs(cosphi) < EPS10 || fabs(store.phi1) >= FRAC_PI_2 {
                panic!("Invalid value for lat_1: |lat_1| should be < 90°");
            }
            if fabs(cos(store.phi2)) < EPS10 || fabs(store.phi2) >= FRAC_PI_2 {
                panic!("Invalid value for lat_2: |lat_2| should be < 90°");
            }

            let secant = fabs(store.phi1 - store.phi2) >= EPS10;
            if proj.es != 0. {
                let m1 = _msfn(sinphi, cosphi, proj.es);
                let ml1 = tsfn(store.phi1, sinphi, proj.e);
                if secant {
                    /* secant cone */
                    sinphi = sin(store.phi2);
                    store.n = log(m1 / _msfn(sinphi, cos(store.phi2), proj.es));
                    if store.n == 0. {
                        panic!("Invalid value for eccentricity");
                    }
                    let ml2 = tsfn(store.phi2, sinphi, proj.e);
                    let denom = log(ml1 / ml2);
                    if denom == 0. {
                        panic!("Invalid value for eccentricity");
                    }
                    store.n /= denom;
                }
                store.rho0 = m1 * pow(ml1, -store.n) / store.n;
                store.c = store.rho0;
                store.rho0 *= if fabs(fabs(proj.phi0) - FRAC_PI_2) < EPS10 {
                    0.
                } else {
                    pow(tsfn(proj.phi0, sin(proj.phi0), proj.e), store.n)
                };
            } else {
                if secant {
                    store.n = log(cosphi / cos(store.phi2))
                        / log(tan(FRAC_PI_4 + 0.5 * store.phi2) / tan(FRAC_PI_4 + 0.5 * store.phi1));
                }
                if store.n == 0. {
                    // Likely reason is that phi1 / phi2 are too close to zero.
                    // Can be reproduced with +proj=lcc +a=1 +lat_2=.0000001
                    panic!("Invalid value for lat_1 and lat_2: |lat_1 + lat_2| should be > 0");
                }
                store.c = cosphi * pow(tan(FRAC_PI_4 + 0.5 * store.phi1), store.n) / store.n;
                store.rho0 = if fabs(fabs(proj.phi0) - FRAC_PI_2) < EPS10 {
                    0.
                } else {
                    store.c * pow(tan(FRAC_PI_4 + 0.5 * proj.phi0), -store.n)
                };
            }
        }
        LambertConformalConicProjection { proj, store: store.into() }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        lcc_e_forward(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        lcc_e_inverse(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
    }
}

/// Lambert Conformal Conic Ellipsoidal forward project
pub fn lcc_e_forward<P: TransformCoordinates>(lcc: &mut LccData, proj: &Proj, p: &mut P) {
    let rho = if fabs(fabs(p.phi()) - FRAC_PI_2) < EPS10 {
        if (p.phi() * lcc.n) <= 0. {
            panic!("Coordinate outside projection domain");
        }
        0.
    } else {
        lcc.c
            * (if proj.es != 0. {
                pow(tsfn(p.phi(), sin(p.phi()), proj.e), lcc.n)
            } else {
                pow(tan(FRAC_PI_4 + 0.5 * p.phi()), -lcc.n)
            })
    };
    p.set_lam(p.lam() * lcc.n);
    p.set_x(proj.k0 * (rho * sin(p.lam())));
    p.set_y(proj.k0 * (lcc.rho0 - rho * cos(p.lam())));
}

/// Lambert Conformal Conic Ellipsoidal inverse project
pub fn lcc_e_inverse<P: TransformCoordinates>(lcc: &mut LccData, proj: &Proj, p: &mut P) {
    let mut x = p.x() / proj.k0;
    let mut y = p.y() / proj.k0;
    let phi;
    let lam;

    y = lcc.rho0 - y;
    let mut rho = hypot(x, y);
    if rho != 0. {
        if lcc.n < 0. {
            rho = -rho;
            x = -x;
            y = -y;
        }
        if proj.es != 0. {
            phi = phi2(pow(rho / lcc.c, 1. / lcc.n), proj.e);
            if phi == f64::MAX {
                panic!("Coordinate outside projection domain");
            }
        } else {
            phi = 2. * atan(pow(lcc.c / rho, 1. / lcc.n)) - FRAC_PI_2;
        }
        lam = atan2(x, y) / lcc.n;
    } else {
        lam = 0.;
        phi = if lcc.n > 0. { FRAC_PI_2 } else { -FRAC_PI_2 };
    }

    p.set_phi(phi);
    p.set_lam(lam);
}
