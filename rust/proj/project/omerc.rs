use crate::proj::{
    aasin, adjlon, phi2, tsfn, CoordinateStep, Proj, ProjValue, ProjectCoordinates,
    TransformCoordinates, ANGLE_RECTIFIED_TO_SKEW_GRID, AZIMUTH_PROJECTION_CENTRE,
    HOTINE_OBLIQUE_MERCATOR_VARIANT_A, HOTINE_OBLIQUE_MERCATOR_VARIANT_B, LATITUDE_OF_FIRST_POINT,
    LATITUDE_OF_SECOND_POINT, LONGITUDE_OF_FIRST_POINT, LONGITUDE_OF_PROJECTION_CENTRE,
    LONGITUDE_OF_SECOND_POINT, NO_OFF, NO_ROTATION, NO_UOFF
};
use alloc::rc::Rc;
use core::{
    cell::RefCell,
    f64::consts::{FRAC_PI_2, FRAC_PI_4, PI, TAU},
};
use libm::{atan, atan2, cos, exp, fabs, log, pow, sin, sqrt, tan};

/// Oblique Mercator Variables
/// INTERNA
#[derive(Debug, Default, Clone, PartialEq)]
pub struct OmercData {
    a: f64,
    b: f64,
    e: f64,
    ab: f64,
    ar_b: f64,
    br_a: f64,
    r_b: f64,
    singam: f64,
    cosgam: f64,
    sinrot: f64,
    cosrot: f64,
    v_pole_n: f64,
    v_pole_s: f64,
    u_0: f64,
    no_rot: bool,
}

const TOL: f64 = 1e-7;
const EPS: f64 = 1e-10;

/// Hotine Oblique Mercator (variant A) Projection
/// EPSG Codes Used by Hotine Oblique Mercator (variant A): 8811, 8812, 8813, 8814, 8815, 8806, 8807
pub type HotineObliqueMercatorVariantAProjection = ObliqueMercatorProjection<HOTINE_OBLIQUE_MERCATOR_VARIANT_A>;
/// Hotine Oblique Mercator (variant B) Projection
/// EPSG Codes Used by Hotine Oblique Mercator (variant B): 8811, 8812, 8813, 8814, 8815, 8816, 8817
pub type HotineObliqueMercatorVariantBProjection = ObliqueMercatorProjection<HOTINE_OBLIQUE_MERCATOR_VARIANT_B>;

/// Oblique Mercator Projection
#[derive(Debug, Clone, PartialEq)]
pub struct ObliqueMercatorProjection<const C: i64> {
    proj: Rc<RefCell<Proj>>,
    store: RefCell<OmercData>,
}
impl<const C: i64> ProjectCoordinates for ObliqueMercatorProjection<C> {
    fn code(&self) -> i64 {
        C
    }
    fn name(&self) -> &'static str {
        "Oblique Mercator"
    }
    fn names() -> &'static [&'static str] {
        &[
            "Hotine_Oblique_Mercator",
            "Hotine Oblique Mercator",
            "Hotine_Oblique_Mercator_Azimuth_Natural_Origin",
            "Hotine Oblique Mercator Azimuth Natural Origin",
            "Hotine_Oblique_Mercator_Two_Point_Natural_Origin",
            "Hotine Oblique Mercator Two Point Natural Origin",
            "Hotine_Oblique_Mercator_Azimuth_Center",
            "Hotine Oblique Mercator Azimuth Center",
            "Hotine Oblique Mercator (variant A)",
            "Hotine Oblique Mercator (variant B)",
            "Oblique_Mercator",
            "Oblique Mercator",
            "omerc",
        ]
    }
}
impl<const C: i64> CoordinateStep for ObliqueMercatorProjection<C> {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        let mut store = OmercData::default();
        {
            let proj = &mut proj.borrow_mut();

            let mut no_off = false;
            let mut lam1 = 0.;
            let mut lam2 = 0.;
            let mut phi1 = 0.;
            let mut phi2 = 0.;
            let gamma0;
            let mut con;
            let mut _f;
            let _d;
            let mut lamc = 0.;

            store.no_rot = proj.params.get(&NO_ROTATION).unwrap_or(&ProjValue::default()).bool();
            let mut alpha_c =
                proj.params.get(&AZIMUTH_PROJECTION_CENTRE).unwrap_or(&ProjValue::default()).f64();
            let alp = alpha_c != 0.;
            let mut gamma = proj
                .params
                .get(&ANGLE_RECTIFIED_TO_SKEW_GRID)
                .unwrap_or(&ProjValue::default())
                .f64();
            let gam = gamma != 0.;
            if alp || gam {
                lamc = proj
                    .params
                    .get(&LONGITUDE_OF_PROJECTION_CENTRE)
                    .unwrap_or(&ProjValue::default())
                    .f64();
                no_off = 
                    // For libproj4 compatibility
                    proj.params.get(&NO_OFF).unwrap_or(&ProjValue::default()).bool()
                    // for backward compatibility
                    || proj.params.get(&NO_UOFF).unwrap_or(&ProjValue::default()).bool();
            } else {
                lam1 = proj
                    .params
                    .get(&LONGITUDE_OF_FIRST_POINT)
                    .unwrap_or(&ProjValue::default())
                    .f64();
                phi1 = proj
                    .params
                    .get(&LATITUDE_OF_FIRST_POINT)
                    .unwrap_or(&ProjValue::default())
                    .f64();
                lam2 = proj
                    .params
                    .get(&LONGITUDE_OF_SECOND_POINT)
                    .unwrap_or(&ProjValue::default())
                    .f64();
                phi2 = proj
                    .params
                    .get(&LATITUDE_OF_SECOND_POINT)
                    .unwrap_or(&ProjValue::default())
                    .f64();
                con = fabs(phi1);

                if fabs(phi1) > FRAC_PI_2 - TOL {
                    panic!("Invalid value for lat_1: |lat_1| should be < 90°");
                }
                if fabs(phi2) > FRAC_PI_2 - TOL {
                    panic!("Invalid value for lat_2: |lat_2| should be < 90°");
                }
                if fabs(phi1 - phi2) <= TOL {
                    panic!("Invalid value for lat_1/lat_2: lat_1 should be different from lat_2");
                }
                if con <= TOL {
                    panic!("Invalid value for lat_1: lat_1 should be different from 0");
                }
                if fabs(fabs(proj.phi0) - FRAC_PI_2) <= TOL {
                    panic!("Invalid value for lat_0: |lat_0| should be < 90°");
                }
            }

            let com = sqrt(proj.one_es);
            if fabs(proj.phi0) > EPS {
                let sinph0 = sin(proj.phi0);
                let cosph0 = cos(proj.phi0);
                con = 1. - proj.es * sinph0 * sinph0;
                store.b = cosph0 * cosph0;
                store.b = sqrt(1. + proj.es * store.b * store.b / proj.one_es);
                store.a = store.b * proj.k0 * com / con;
                _d = store.b * com / (cosph0 * sqrt(con));
                _f = _d * _d - 1.;
                if _f <= 0. {
                    _f = 0.;
                } else {
                    _f = sqrt(_f);
                    if proj.phi0 < 0. {
                        _f = -_f;
                    }
                }
                _f += _d;
                store.e = _f;
                store.e *= pow(tsfn(proj.phi0, sinph0, proj.e), store.b);
            } else {
                store.b = 1. / com;
                store.a = proj.k0;
                _f = 1.;
                _d = _f;
                store.e = _d;
            }
            if alp || gam {
                if alp {
                    gamma0 = aasin(sin(alpha_c) / _d);
                    if !gam {
                        gamma = alpha_c;
                    }
                } else {
                    gamma0 = gamma;
                    alpha_c = aasin(_d * sin(gamma0));
                    if gamma <= 90. - proj.phi0 {
                        // For a sphere, |gamma| must be <= 90 - |lat_0|
                        // On an ellipsoid, this is very slightly above
                        panic!("Invalid value for gamma: given lat_0 value, |gamma| should be <= ");
                    }
                }

                if fabs(fabs(proj.phi0) - FRAC_PI_2) <= TOL {
                    panic!("Invalid value for lat_0: |lat_0| should be < 90°");
                }

                proj.lam0 = lamc - aasin(0.5 * (_f - 1. / _f) * tan(gamma0)) / store.b;
            } else {
                let _h = pow(tsfn(phi1, sin(phi1), proj.e), store.b);
                let l = pow(tsfn(phi2, sin(phi2), proj.e), store.b);
                _f = store.e / _h;
                let p = (l - _h) / (l + _h);
                if p == 0. {
                    // Not quite, but es is very close to 1...
                    panic!("Invalid value for eccentricity");
                }
                let mut j = store.e * store.e;
                j = (j - l * _h) / (j + l * _h);
                con = lam1 - lam2;
                if con < (-PI) {
                    lam2 -= TAU;
                } else if con > PI {
                    lam2 += TAU;
                }
                proj.lam0 = adjlon(
                    0.5 * (lam1 + lam2)
                        - atan(j * tan(0.5 * store.b * (lam1 - lam2)) / p) / store.b,
                );
                let denom = _f - 1. / _f;
                if denom == 0. {
                    panic!("Invalid value for eccentricity");
                }
                gamma0 = atan(2. * sin(store.b * adjlon(lam1 - proj.lam0)) / denom);
                alpha_c = aasin(_d * sin(gamma0));
                gamma = alpha_c;
            }
            store.singam = sin(gamma0);
            store.cosgam = cos(gamma0);
            store.sinrot = sin(gamma);
            store.cosrot = cos(gamma);
            store.r_b = 1. / store.b;
            store.ar_b = store.a * store.r_b;
            store.br_a = 1. / (store.ar_b);
            store.ab = store.a * store.b;
            if no_off {
                store.u_0 = 0.;
            } else {
                store.u_0 = fabs(store.ar_b * atan(sqrt(_d * _d - 1.) / cos(alpha_c)));
                if proj.phi0 < 0. {
                    store.u_0 = -store.u_0;
                }
            }
            _f = 0.5 * gamma0;
            store.v_pole_n = store.ar_b * log(tan(FRAC_PI_4 - _f));
            store.v_pole_s = store.ar_b * log(tan(FRAC_PI_4 + _f));
        }

        ObliqueMercatorProjection { proj, store: store.into() }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        omerc_e_forward(&self.store.borrow(), &self.proj.borrow(), p);
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        omerc_e_inverse(&self.store.borrow(), &self.proj.borrow(), p);
    }
}

/// Oblique Mercator Ellipsoidal forward project
pub fn omerc_e_forward<P: TransformCoordinates>(omerc: &OmercData, proj: &Proj, p: &mut P) {
    let mut u;
    let v;

    if fabs(fabs(p.phi()) - FRAC_PI_2) > EPS {
        let w = omerc.e / pow(tsfn(p.phi(), sin(p.phi()), proj.e), omerc.b);
        let one_div_w = 1. / w;
        let s = 0.5 * (w - one_div_w);
        let t = 0.5 * (w + one_div_w);
        let _v = sin(omerc.b * p.lam());
        let _u = (s * omerc.singam - _v * omerc.cosgam) / t;
        if fabs(fabs(_u) - 1.0) < EPS {
            panic!("Coordinate outside projection domain");
        }
        v = 0.5 * omerc.ar_b * log((1. - _u) / (1. + _u));
        let temp = cos(omerc.b * p.lam());
        if fabs(temp) < TOL {
            u = omerc.a * p.lam();
        } else {
            u = omerc.ar_b * atan2(s * omerc.cosgam + _v * omerc.singam, temp);
        }
    } else {
        v = if p.phi() > 0. { omerc.v_pole_n } else { omerc.v_pole_s };
        u = omerc.ar_b * p.phi();
    }
    if omerc.no_rot {
        p.set_x(u);
        p.set_y(v);
    } else {
        u -= omerc.u_0;
        p.set_x(v * omerc.cosrot + u * omerc.sinrot);
        p.set_y(u * omerc.cosrot - v * omerc.sinrot);
    }
}

/// Oblique Mercator Ellipsoidal inverse project
pub fn omerc_e_inverse<P: TransformCoordinates>(omerc: &OmercData, proj: &Proj, p: &mut P) {
    let u;
    let v;
    if omerc.no_rot {
        v = p.y();
        u = p.x();
    } else {
        v = p.x() * omerc.cosrot - p.y() * omerc.sinrot;
        u = p.y() * omerc.cosrot + p.x() * omerc.sinrot + omerc.u_0;
    }
    let q_p = exp(-omerc.br_a * v);
    if q_p == 0. {
        panic!("Coordinate outside projection domain");
    }
    let s_p = 0.5 * (q_p - 1. / q_p);
    let t_p = 0.5 * (q_p + 1. / q_p);
    let v_p = sin(omerc.br_a * u);
    let u_p = (v_p * omerc.cosgam + s_p * omerc.singam) / t_p;
    if fabs(fabs(u_p) - 1.) < EPS {
        p.set_lam(0.);
        p.set_phi(if u_p < 0. { -FRAC_PI_2 } else { FRAC_PI_2 });
    } else {
        p.set_phi(omerc.e / sqrt((1. + u_p) / (1. - u_p)));
        p.set_phi(phi2(pow(p.phi(), 1. / omerc.b), proj.e));
        if p.phi() == f64::MAX {
            panic!("Coordinate outside projection domain");
        }
        p.set_lam(-omerc.r_b * atan2(s_p * omerc.cosgam - v_p * omerc.singam, cos(omerc.br_a * u)));
    }
}
