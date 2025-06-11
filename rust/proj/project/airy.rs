use crate::proj::{
    CoordinateStep, LAT_B, NO_CUT, Proj, ProjMode, ProjValue, ProjectCoordinates,
    TransformCoordinates,
};
use core::{cell::RefCell, f64::consts::FRAC_PI_2};
use libm::{cos, fabs, log, sin, tan};

/// Airy variables
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Airy {
    p_halfpi: f64,
    sinph0: f64,
    cosph0: f64,
    cb: f64,
    mode: ProjMode,
    // do not cut at hemisphere limit
    no_cut: bool,
}

const EPS: f64 = 1e-10;

/// Airy Projection
#[derive(Debug, Clone, PartialEq)]
pub struct AiryProjection {
    proj: RefCell<Proj>,
    store: RefCell<Airy>,
}
impl ProjectCoordinates for AiryProjection {
    fn code(&self) -> i64 {
        -1
    }
    fn name(&self) -> &'static str {
        "Airy"
    }
    fn names() -> &'static [&'static str] {
        &["Airy", "airy"]
    }
}
impl CoordinateStep for AiryProjection {
    fn new(proj: RefCell<Proj>) -> Self {
        let mut store = Airy {
            no_cut: proj.borrow().params.get(&NO_CUT).unwrap_or(&ProjValue::default()).bool(),
            ..Default::default()
        };
        {
            let proj = &mut proj.borrow_mut();
            let beta =
                0.5 * (FRAC_PI_2 - proj.params.get(&LAT_B).unwrap_or(&ProjValue::default()).f64());
            if fabs(beta) < EPS {
                store.cb = -0.5;
            } else {
                store.cb = 1. / tan(beta);
                store.cb = store.cb * store.cb * log(cos(beta));
            }

            if fabs(fabs(proj.phi0) - FRAC_PI_2) < EPS {
                if proj.phi0 < 0. {
                    store.p_halfpi = -FRAC_PI_2;
                    store.mode = ProjMode::SPole;
                } else {
                    store.p_halfpi = FRAC_PI_2;
                    store.mode = ProjMode::NPole;
                }
            } else if fabs(proj.phi0) < EPS {
                store.mode = ProjMode::Equit;
            } else {
                store.mode = ProjMode::Obliq;
                store.sinph0 = sin(proj.phi0);
                store.cosph0 = cos(proj.phi0);
            }
            proj.es = 0.;
        }
        AiryProjection { proj, store: store.into() }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        airy_s_forward(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
    }
    fn inverse<P: TransformCoordinates>(&self, _p: &mut P) {
        // There is no inverse
    }
}

/// Ellipsoid/spheroid, forward
pub fn airy_s_forward<P: TransformCoordinates>(airy: &mut Airy, _proj: &Proj, p: &mut P) {
    let sinlam = sin(p.lam());
    let coslam = cos(p.lam());
    match airy.mode {
        ProjMode::Equit | ProjMode::Obliq => {
            let sinphi = sin(p.phi());
            let cosphi = cos(p.phi());
            let mut cosz = cosphi * coslam;
            if airy.mode == ProjMode::Obliq {
                cosz = airy.sinph0 * sinphi + airy.cosph0 * cosz;
            }
            if !airy.no_cut && cosz < -EPS {
                panic!("Coordinates are outside the projection domain");
            }
            let s = 1. - cosz;
            let k_rho = if fabs(s) > EPS {
                let t = 0.5 * (1. + cosz);
                if t == 0. {
                    panic!("Coordinates are outside the projection domain");
                }
                -log(t) / s - airy.cb / t
            } else {
                0.5 - airy.cb
            };
            p.set_x(k_rho * cosphi * sinlam);
            if airy.mode == ProjMode::Obliq {
                p.set_y(k_rho * (airy.cosph0 * sinphi - airy.sinph0 * cosphi * coslam));
            } else {
                p.set_y(k_rho * sinphi);
            }
        }
        ProjMode::SPole | ProjMode::NPole => {
            p.set_phi(fabs(airy.p_halfpi - p.phi()));
            if !airy.no_cut && (p.phi() - EPS) > FRAC_PI_2 {
                panic!("Coordinates are outside the projection domain");
            }
            p.set_phi(p.phi() * 0.5);
            if p.phi() > EPS {
                let t = tan(p.phi());
                let k_rho = -2. * (log(cos(p.phi())) / t + t * airy.cb);
                p.set_x(k_rho * sinlam);
                p.set_y(k_rho * coslam);
                if airy.mode == ProjMode::NPole {
                    p.set_y(-p.y());
                }
            } else {
                p.set_x(0.);
                p.set_y(0.);
            }
        }
    }
}
