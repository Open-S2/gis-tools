use crate::proj::{
    _msfn, CoordinateStep, EPS10, EQUIDISTANT_CONIC, LATITUDE_OF_FIRST_STANDARD_PARALLEL,
    LATITUDE_OF_SECOND_STANDARD_PARALLEL, Proj, ProjValue, ProjectCoordinates,
    TransformCoordinates, enfn, inv_mlfn, mlfn,
};
use alloc::{rc::Rc, vec::Vec};
use core::{cell::RefCell, f64::consts::FRAC_PI_2};
use libm::{atan2, cos, fabs, hypot, sin};

/// Equidistant Conic Variables
#[derive(Debug, Default, Clone, PartialEq)]
pub struct EqdcData {
    phi1: f64,
    phi2: f64,
    n: f64,
    rho: f64,
    rho0: f64,
    c: f64,
    en: Vec<f64>,
    ellips: bool,
}

/// Equidistant Conic Projection
#[derive(Debug, Clone, PartialEq)]
pub struct EquidistantConicProjection {
    proj: Rc<RefCell<Proj>>,
    store: RefCell<EqdcData>,
}
impl ProjectCoordinates for EquidistantConicProjection {
    fn code(&self) -> i64 {
        EQUIDISTANT_CONIC
    }
    fn name(&self) -> &'static str {
        "Equidistant Conic"
    }
    fn names() -> &'static [&'static str] {
        &["Equidistant Conic", "Equidistant_Conic", "eqdc"]
    }
}
impl CoordinateStep for EquidistantConicProjection {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        let mut store = EqdcData::default();

        {
            let proj = &mut proj.borrow_mut();

            store.phi1 = proj
                .params
                .get(&LATITUDE_OF_FIRST_STANDARD_PARALLEL)
                .unwrap_or(&ProjValue::default())
                .f64();
            store.phi2 = proj
                .params
                .get(&LATITUDE_OF_SECOND_STANDARD_PARALLEL)
                .unwrap_or(&ProjValue::default())
                .f64();

            if fabs(store.phi1) > FRAC_PI_2 {
                panic!("Invalid value for lat_1: |lat_1| should be <= 90°");
            }

            if fabs(store.phi2) > FRAC_PI_2 {
                panic!("Invalid value for lat_2: |lat_2| should be <= 90°");
            }
            if fabs(store.phi1 + store.phi2) < EPS10 {
                panic!("Invalid value for lat_1 and lat_2: |lat_1 + lat_2| should be > 0");
            }

            store.en = enfn(proj.n);

            let mut sinphi = sin(store.phi1);
            store.n = sinphi;
            let mut cosphi = cos(store.phi1);
            let secant = fabs(store.phi1 - store.phi2) >= EPS10;
            store.ellips = proj.es > 0.;
            if store.ellips {
                let m1 = _msfn(sinphi, cosphi, proj.es);
                let ml1 = mlfn(store.phi1, sinphi, cosphi, &store.en);
                if secant {
                    // secant cone
                    sinphi = sin(store.phi2);
                    cosphi = cos(store.phi2);
                    let ml2 = mlfn(store.phi2, sinphi, cosphi, &store.en);
                    if ml1 == ml2 {
                        panic!("Eccentricity too close to 1");
                    }
                    store.n = (m1 - _msfn(sinphi, cosphi, proj.es)) / (ml2 - ml1);
                    if store.n == 0. {
                        // Not quite, but es is very close to 1...
                        panic!("Invalid value for eccentricity");
                    }
                }
                store.c = ml1 + m1 / store.n;
                store.rho0 = store.c - mlfn(proj.phi0, sin(proj.phi0), cos(proj.phi0), &store.en);
            } else {
                if secant {
                    store.n = (cosphi - cos(store.phi2)) / (store.phi2 - store.phi1);
                }
                if store.n == 0. {
                    panic!("Invalid value for lat_1 and lat_2: lat_1 + lat_2 should be > 0");
                }
                store.c = store.phi1 + cos(store.phi1) / store.n;
                store.rho0 = store.c - proj.phi0;
            }
        }

        EquidistantConicProjection { proj, store: store.into() }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        eqdc_e_forward(&mut self.store.borrow_mut(), p);
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        eqdc_e_inverse(&mut self.store.borrow_mut(), p);
    }
}

/// Equidistant Conic Ellipsoidal forward project
pub fn eqdc_e_forward<P: TransformCoordinates>(eqdc: &mut EqdcData, p: &mut P) {
    eqdc.rho = eqdc.c
        - (if eqdc.ellips { mlfn(p.phi(), sin(p.phi()), cos(p.phi()), &eqdc.en) } else { p.phi() });
    let lam_mul_n = p.lam() * eqdc.n;
    p.set_x(eqdc.rho * sin(lam_mul_n));
    p.set_y(eqdc.rho0 - eqdc.rho * cos(lam_mul_n));
}

/// Equidistant Conic Ellipsoidal inverse project
pub fn eqdc_e_inverse<P: TransformCoordinates>(eqdc: &mut EqdcData, p: &mut P) {
    let mut x = p.x();
    let mut y = p.y();
    y = eqdc.rho0 - y;
    eqdc.rho = hypot(x, y);

    if eqdc.rho != 0.0 {
        if eqdc.n < 0. {
            eqdc.rho = -eqdc.rho;
            x = -x;
            y = -y;
        }
        p.set_phi(eqdc.c - eqdc.rho);
        if eqdc.ellips {
            p.set_phi(inv_mlfn(p.phi(), &eqdc.en));
        }
        p.set_lam(atan2(x, y) / eqdc.n);
    } else {
        p.set_lam(0.);
        p.set_phi(if eqdc.n > 0. { FRAC_PI_2 } else { -FRAC_PI_2 });
    }
}
