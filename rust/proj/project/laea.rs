use crate::proj::{
    CoordinateStep, EPS10, LAMBERT_AZIMUTHAL_EQUAL_AREA, LAMBERT_AZIMUTHAL_EQUAL_AREA_SPHERICAL,
    Proj, ProjMethod, ProjMode, ProjectCoordinates, TransformCoordinates, authalic_lat,
    authalic_lat_compute_coeffs, authalic_lat_inverse, authalic_lat_q,
};
use alloc::vec::Vec;
use core::{
    cell::RefCell,
    f64::consts::{FRAC_PI_2, FRAC_PI_4},
};
use libm::{asin, atan2, cos, fabs, hypot, sin, sqrt};

/// Lambert Azimuthal Equal Area Variables
#[derive(Debug, Default, Clone, PartialEq)]
pub struct LaeaData {
    sinb1: f64,
    cosb1: f64,
    xmf: f64,
    ymf: f64,
    mmf: f64,
    qp: f64,
    dd: f64,
    rq: f64,
    apa: Vec<f64>,
    mode: ProjMode,
}

/// Lambert Azimuthal Equal Area Projection
pub type LambertAzimuthalEqualAreaProjection =
    LambertAzimuthalEqualAreaBase<LAMBERT_AZIMUTHAL_EQUAL_AREA>;
/// Lambert Azimuthal Equal Area (Spherical) Projection
pub type LambertAzimuthalEqualAreaSphericalProjection =
    LambertAzimuthalEqualAreaBase<LAMBERT_AZIMUTHAL_EQUAL_AREA_SPHERICAL>;

/// Lambert Azimuthal Equal Area Projection
#[derive(Debug, Clone, PartialEq)]
pub struct LambertAzimuthalEqualAreaBase<const C: i64> {
    proj: RefCell<Proj>,
    store: RefCell<LaeaData>,
    method: ProjMethod,
}
impl<const C: i64> ProjectCoordinates for LambertAzimuthalEqualAreaBase<C> {
    fn code(&self) -> i64 {
        C
    }
    fn name(&self) -> &'static str {
        "Lambert Azimuthal Equal Area"
    }
    fn names() -> &'static [&'static str] {
        &["Lambert Azimuthal Equal Area", "Lambert Azimuthal Equal Area (Spherical)", "laea"]
    }
}
impl<const C: i64> CoordinateStep for LambertAzimuthalEqualAreaBase<C> {
    fn new(proj: RefCell<Proj>) -> Self {
        let mut store = LaeaData::default();
        let method: ProjMethod;
        {
            let proj = &mut proj.borrow_mut();

            let t = fabs(proj.phi0);
            if t > FRAC_PI_2 + EPS10 {
                panic!("Invalid value for lat_0: |lat_0| should be <= 90°");
            }
            if fabs(t - FRAC_PI_2) < EPS10 {
                store.mode = if proj.phi0 < 0. { ProjMode::SPole } else { ProjMode::NPole };
            } else if fabs(t) < EPS10 {
                store.mode = ProjMode::Equit;
            } else {
                store.mode = ProjMode::Obliq;
            }
            method = if proj.es != 0.0 {
                proj.e = sqrt(proj.es);
                store.qp = authalic_lat_q(1.0, proj);
                store.mmf = 0.5 / (1. - proj.es);
                store.apa = authalic_lat_compute_coeffs(proj.n);
                match store.mode {
                    ProjMode::NPole | ProjMode::SPole => {
                        store.dd = 1.;
                    }
                    ProjMode::Equit => {
                        store.rq = sqrt(0.5 * store.qp);
                        store.dd = 1. / store.rq;
                        store.xmf = 1.;
                        store.ymf = 0.5 * store.qp;
                    }
                    ProjMode::Obliq => {
                        store.rq = sqrt(0.5 * store.qp);
                        let sinphi = sin(proj.phi0);
                        let cosphi = cos(proj.phi0);
                        let b1 =
                            authalic_lat(proj.phi0, sinphi, cosphi, &store.apa, proj, store.qp);
                        store.sinb1 = sin(b1);
                        store.cosb1 = cos(b1);
                        store.dd = cos(proj.phi0)
                            / (sqrt(1. - proj.es * sinphi * sinphi) * store.rq * store.cosb1);
                        store.xmf = store.rq;
                        store.ymf = store.xmf / store.dd;
                        store.xmf *= store.dd;
                    }
                }
                ProjMethod::Ellipsoidal
            } else {
                if store.mode == ProjMode::Obliq {
                    store.sinb1 = sin(proj.phi0);
                    store.cosb1 = cos(proj.phi0);
                }
                ProjMethod::Spheroidal
            }
        }

        LambertAzimuthalEqualAreaBase { proj, store: store.into(), method }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        if self.method == ProjMethod::Spheroidal {
            laea_s_forward(&self.store.borrow(), &self.proj.borrow(), p);
        } else {
            laea_e_forward(&self.store.borrow(), &self.proj.borrow(), p);
        }
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        if self.method == ProjMethod::Spheroidal {
            laea_s_inverse(&self.store.borrow(), &self.proj.borrow(), p);
        } else {
            laea_e_inverse(&self.store.borrow(), &self.proj.borrow(), p);
        }
    }
}

/// Equidistant Conic Ellipsoidal forward project
pub fn laea_e_forward<P: TransformCoordinates>(laea: &LaeaData, proj: &Proj, p: &mut P) {
    let coslam = cos(p.lam());
    let sinlam = sin(p.lam());
    let sinphi = sin(p.phi());
    let cosphi = cos(p.phi());
    let mut sinb = 0.0;
    let mut cosb = 0.0;
    let xi = authalic_lat(p.phi(), sinphi, cosphi, &laea.apa, proj, laea.qp);
    let mut q = sin(xi) * laea.qp;
    let mut b;

    if laea.mode == ProjMode::Obliq || laea.mode == ProjMode::Equit {
        sinb = sin(xi);
        cosb = cos(xi);
    }

    match laea.mode {
        ProjMode::Obliq => {
            b = 1. + laea.sinb1 * sinb + laea.cosb1 * cosb * coslam;
        }
        ProjMode::Equit => {
            b = 1. + cosb * coslam;
        }
        ProjMode::NPole => {
            b = FRAC_PI_2 + p.phi();
            q = laea.qp - q;
        }
        ProjMode::SPole => {
            b = p.phi() - FRAC_PI_2;
            q += laea.qp;
        }
    }
    if fabs(b) < EPS10 {
        panic!("Coordinate outside projection domain");
    }

    match laea.mode {
        ProjMode::Obliq => {
            b = sqrt(2. / b);
            p.set_y(laea.ymf * b * (laea.cosb1 * sinb - laea.sinb1 * cosb * coslam));
            p.set_x(laea.xmf * b * cosb * sinlam);
        }
        ProjMode::Equit => {
            b = sqrt(2. / (1. + cosb * coslam));
            p.set_y(b * sinb * laea.ymf);
            p.set_x(laea.xmf * b * cosb * sinlam);
        }
        ProjMode::NPole | ProjMode::SPole => {
            if q >= 1e-15 {
                b = sqrt(q);
                p.set_x(b * sinlam);
                p.set_y(coslam * (if laea.mode == ProjMode::SPole { b } else { -b }));
            } else {
                p.set_x(0.);
                p.set_y(0.);
            }
        }
    }
}

/// Equidistant Conic Spheroidal forward project
pub fn laea_s_forward<P: TransformCoordinates>(laea: &LaeaData, proj: &Proj, p: &mut P) {
    let sinphi = sin(p.phi());
    let cosphi = cos(p.phi());
    let mut coslam = cos(p.lam());
    let x;
    let mut y;
    match laea.mode {
        ProjMode::Equit => {
            y = 1. + cosphi * coslam;
            if y <= EPS10 {
                panic!("Coordinate outside projection domain");
            }
            y = sqrt(2. / y);
            x = y * cosphi * sin(p.lam());
            y *= if laea.mode == ProjMode::Equit {
                sinphi
            } else {
                laea.cosb1 * sinphi - laea.sinb1 * cosphi * coslam
            };
        }
        ProjMode::Obliq => {
            y = 1. + laea.sinb1 * sinphi + laea.cosb1 * cosphi * coslam;
            if y <= EPS10 {
                panic!("Coordinate outside projection domain");
            }
            y = sqrt(2. / y);
            x = y * cosphi * sin(p.lam());
            y *= if laea.mode == ProjMode::Equit {
                sinphi
            } else {
                laea.cosb1 * sinphi - laea.sinb1 * cosphi * coslam
            };
        }
        ProjMode::NPole => {
            coslam = -coslam;
            if fabs(p.phi() + proj.phi0) < EPS10 {
                panic!("Coordinate outside projection domain");
            }
            y = FRAC_PI_4 - p.phi() * 0.5;
            y = 2. * (if laea.mode == ProjMode::SPole { cos(y) } else { sin(y) });
            x = y * sin(p.lam());
            y *= coslam;
        }
        ProjMode::SPole => {
            if fabs(p.phi() + proj.phi0) < EPS10 {
                panic!("Coordinate outside projection domain");
            }
            y = FRAC_PI_4 - p.phi() * 0.5;
            y = 2. * (if laea.mode == ProjMode::SPole { cos(y) } else { sin(y) });
            x = y * sin(p.lam());
            y *= coslam;
        }
    }
    p.set_x(x);
    p.set_y(y);
}

/// Equidistant Conic Ellipsoidal inverse project
pub fn laea_e_inverse<P: TransformCoordinates>(laea: &LaeaData, proj: &Proj, p: &mut P) {
    // static PJ_LP laea_e_inverse(PJ_XY xy, PJ *P) { /* Ellipsoidal, inverse */
    //     PJ_LP lp = {0.0, 0.0};
    //     struct pj_laea_data *Q = static_cast<struct pj_laea_data *>(proj.opaque);
    //     double c_ce, s_ce, q, rho, ab = 0.0;
    let mut x = p.x();
    let mut y = p.y();
    let mut ab;

    match laea.mode {
        ProjMode::Equit | ProjMode::Obliq => {
            x /= laea.dd;
            y *= laea.dd;
            let rho = hypot(x, y);
            if rho < EPS10 {
                p.set_lam(0.);
                p.set_phi(proj.phi0);
                return;
            }
            let asin_argument = 0.5 * rho / laea.rq;
            if asin_argument > 1. {
                // proj_errno_set(P, PROJ_ERR_COORD_TRANSFM_OUTSIDE_PROJECTION_DOMAIN);
                // return lp;
                panic!("Coordinate outside projection domain");
            }
            let s_ce = 2. * asin(asin_argument);
            let c_ce = cos(s_ce);
            let s_ce = sin(s_ce);
            x *= s_ce;
            if laea.mode == ProjMode::Obliq {
                ab = c_ce * laea.sinb1 + y * s_ce * laea.cosb1 / rho;
                y = rho * laea.cosb1 * c_ce - y * laea.sinb1 * s_ce;
            } else {
                ab = y * s_ce / rho;
                y = rho * c_ce;
            }
        }
        ProjMode::NPole => {
            y = -y;
            let q = x * x + y * y;
            if q == 0.0 {
                p.set_lam(0.);
                p.set_phi(proj.phi0);
                return;
            }
            ab = 1. - q / laea.qp;
            if laea.mode == ProjMode::SPole {
                ab = -ab;
            }
        }
        ProjMode::SPole => {
            let q = x * x + y * y;
            if q == 0.0 {
                p.set_lam(0.);
                p.set_phi(proj.phi0);
                return;
            }
            ab = 1. - q / laea.qp;
            if laea.mode == ProjMode::SPole {
                ab = -ab;
            }
        }
    }
    p.set_lam(atan2(x, y));
    p.set_phi(authalic_lat_inverse(asin(ab), &laea.apa, proj, laea.qp));
}

/// Equidistant Conic Spheroidal inverse project
pub fn laea_s_inverse<P: TransformCoordinates>(laea: &LaeaData, proj: &Proj, p: &mut P) {
    // static PJ_LP laea_s_inverse(PJ_XY xy, PJ *P) { /* Spheroidal, inverse */
    //     PJ_LP lp = {0.0, 0.0};
    //     struct pj_laea_data *Q = static_cast<struct pj_laea_data *>(proj.opaque);
    //     double cosz = 0.0, rh, sinz = 0.0;

    let mut cosz = 0.0;
    let mut sinz = 0.0;
    let mut x = p.x();
    let mut y = p.y();
    let rh = hypot(x, y);
    let mut phi = rh * 0.5;
    if phi > 1. {
        // proj_errno_set(P, PROJ_ERR_COORD_TRANSFM_OUTSIDE_PROJECTION_DOMAIN);
        // return lp;
        panic!("Coordinate outside projection domain");
    }
    phi = 2. * asin(phi);
    if laea.mode == ProjMode::Obliq || laea.mode == ProjMode::Equit {
        sinz = sin(phi);
        cosz = cos(phi);
    }
    match laea.mode {
        ProjMode::Equit => {
            phi = if fabs(rh) <= EPS10 { 0. } else { asin(y * sinz / rh) };
            x *= sinz;
            y = cosz * rh;
        }
        ProjMode::Obliq => {
            phi = if fabs(rh) <= EPS10 {
                proj.phi0
            } else {
                asin(cosz * laea.sinb1 + y * sinz * laea.cosb1 / rh)
            };
            x *= sinz * laea.cosb1;
            y = (cosz - sin(phi) * laea.sinb1) * rh;
        }
        ProjMode::NPole => {
            y = -y;
            phi = FRAC_PI_2 - phi;
        }
        ProjMode::SPole => {
            phi -= FRAC_PI_2;
        }
    }
    let lam = if y == 0. && (laea.mode == ProjMode::Equit || laea.mode == ProjMode::Obliq) {
        0.
    } else {
        atan2(x, y)
    };

    p.set_lam(lam);
    p.set_phi(phi);
}
