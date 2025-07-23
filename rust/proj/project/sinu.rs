use crate::proj::{
    CoordinateStep, EPS10, M_VAL, N_VAL, Proj, ProjMethod, ProjectCoordinates,
    TransformCoordinates, aasin, enfn, inv_mlfn, mlfn,
};
use alloc::{rc::Rc, vec::Vec};
use core::{cell::RefCell, f64::consts::FRAC_PI_2};
use libm::{cos, fabs, sin, sqrt};

const MAX_ITER: usize = 8;
const LOOP_TOL: f64 = 1e-7;

/// Sinusoidal Variables
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SinuData {
    m: f64,
    n: f64,
    c_x: f64,
    c_y: f64,
    en: Vec<f64>,
}

/// for spheres, only
fn sinu_setup(proj: &mut Proj, sinu: &mut SinuData) {
    proj.es = 0.;
    sinu.c_y = sqrt((sinu.m + 1.) / sinu.n);
    sinu.c_x = sinu.c_y / (sinu.m + 1.);
}

/// # Sinusoidal (Sanson-Flamsteed)
///
/// **Classification**: Pseudocylindrical
///
/// **Available forms**: Forward and inverse, spherical and ellipsoidal
///
/// **Defined area**: Global
///
/// **Alias**: sinu
///
/// **Domain**: 2D
///
/// **Input type**: Geodetic coordinates
///
/// **Output type**: Projected coordinates
///
/// ## Projection String
/// ```ini
/// +proj=sinu
/// ```
///
/// ## Parameters
///
/// All parameters are optional.
///
/// - `+lon_0=<value>`: Central meridian.
/// - `+R=<value>`: Radius of the sphere or semi-major axis of the ellipsoid.
/// - `+x_0=<value>`: False easting.
/// - `+y_0=<value>`: False northing.
///
/// ## Mathematical Definition
///
/// MacBryde and Thomas developed generalized formulas for several of the
/// pseudocylindricals with sinusoidal meridians. The formulas describing the Sinusoidal
/// projection are:
///
/// Forward projection:
/// $$x = C\lambda(m+cos\theta) / ( m + 1)$$
/// $$y = C\theta$$
///
/// Inverse projection:
/// $$\lambda = x \cdot \frac{m + 1}{C \cdot (m + \cos(y / C))}$$
/// $$\theta = y / C$$
///
/// Where:
/// $$C = \sqrt { (m + 1 ) / n }$$
///
/// ## Further Reading
/// - [Wikipedia](https://en.wikipedia.org/wiki/Sinusoidal_projection)
///
/// ![Sinusoidal (Sanson-Flamsteed)](https://github.com/Open-S2/gis-tools/blob/master/assets/proj4/projections/images/sinu.png?raw=true)
#[derive(Debug, Clone, PartialEq)]
pub struct SinusoidalProjection {
    proj: Rc<RefCell<Proj>>,
    store: RefCell<SinuData>,
    method: ProjMethod,
}
impl ProjectCoordinates for SinusoidalProjection {
    fn code(&self) -> i64 {
        -1
    }
    fn name(&self) -> &'static str {
        "Sinusoidal (Sanson-Flamsteed)"
    }
    fn names() -> &'static [&'static str] {
        &["Sinusoidal", "Sinusoidal (Sanson-Flamsteed)", "sinu"]
    }
}
impl CoordinateStep for SinusoidalProjection {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        let mut store = SinuData::default();
        let method: ProjMethod;
        {
            let proj = &mut proj.borrow_mut();

            store.en = enfn(proj.n);

            method = if proj.es != 0.0 {
                ProjMethod::Ellipsoidal
            } else {
                store.n = 1.;
                store.m = 0.;
                sinu_setup(proj, &mut store);
                ProjMethod::Spheroidal
            };
        }
        SinusoidalProjection { proj, store: store.into(), method }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        if self.method == ProjMethod::Ellipsoidal {
            sinu_e_forward(&self.store.borrow(), &self.proj.borrow(), p);
        } else {
            sinu_s_forward(&self.store.borrow(), p);
        }
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        if self.method == ProjMethod::Ellipsoidal {
            sinu_e_inverse(&self.store.borrow(), &self.proj.borrow(), p);
        } else {
            sinu_s_inverse(&self.store.borrow(), p);
        }
    }
}

/// Eckert VI Projection
#[derive(Debug, Clone, PartialEq)]
pub struct EckertVIProjection {
    proj: Rc<RefCell<Proj>>,
    store: RefCell<SinuData>,
}
impl ProjectCoordinates for EckertVIProjection {
    fn code(&self) -> i64 {
        -1
    }
    fn name(&self) -> &'static str {
        "Eckert VI"
    }
    fn names() -> &'static [&'static str] {
        &["Eckert VI", "eck6"]
    }
}
impl CoordinateStep for EckertVIProjection {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        let mut store = SinuData::default();
        {
            let proj = &mut proj.borrow_mut();

            store.m = 1.;
            store.n = 2.570_796_326_794_896_6;
            sinu_setup(proj, &mut store);
        }
        EckertVIProjection { proj, store: store.into() }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        sinu_s_forward(&self.store.borrow(), p);
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        sinu_s_inverse(&self.store.borrow(), p);
    }
}

/// McBryde-Thomas Flat-Polar Sinusoidal Projection
#[derive(Debug, Clone, PartialEq)]
pub struct McBrydeThomasFlatPolarSinusoidalProjection {
    proj: Rc<RefCell<Proj>>,
    store: RefCell<SinuData>,
}
impl ProjectCoordinates for McBrydeThomasFlatPolarSinusoidalProjection {
    fn code(&self) -> i64 {
        -1
    }
    fn name(&self) -> &'static str {
        "McBryde-Thomas Flat-Polar Sinusoidal"
    }
    fn names() -> &'static [&'static str] {
        &["McBryde-Thomas Flat-Polar Sinusoidal", "mbtfps"]
    }
}
impl CoordinateStep for McBrydeThomasFlatPolarSinusoidalProjection {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        let mut store = SinuData::default();
        {
            let proj = &mut proj.borrow_mut();

            store.m = 0.5;
            store.n = 1.785_398_163_397_448_3;
            sinu_setup(proj, &mut store);
        }
        McBrydeThomasFlatPolarSinusoidalProjection { proj, store: store.into() }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        sinu_s_forward(&self.store.borrow(), p);
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        sinu_s_inverse(&self.store.borrow(), p);
    }
}

/// General Sinusoidal Series Projection
#[derive(Debug, Clone, PartialEq)]
pub struct GeneralSinusoidalSeriesProjection {
    proj: Rc<RefCell<Proj>>,
    store: RefCell<SinuData>,
}
impl ProjectCoordinates for GeneralSinusoidalSeriesProjection {
    fn code(&self) -> i64 {
        -1
    }
    fn name(&self) -> &'static str {
        "General Sinusoidal Series"
    }
    fn names() -> &'static [&'static str] {
        &["General Sinusoidal Series", "General Sinusoidal", "gn_sinu"]
    }
}
impl CoordinateStep for GeneralSinusoidalSeriesProjection {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        let mut store = SinuData::default();
        {
            let proj = &mut proj.borrow_mut();

            if let Some(n) = proj.params.get(&N_VAL) {
                store.n = n.f64();
            } else {
                panic!("Missing parameter n.");
            }
            if let Some(m) = proj.params.get(&M_VAL) {
                store.m = m.f64();
            } else {
                panic!("Missing parameter m.");
            }
            if store.n <= 0. {
                panic!("Invalid value for n: it should be > 0.");
            }
            if store.m < 0. {
                panic!("Invalid value for m: it should be >= 0.");
            }

            sinu_setup(proj, &mut store);
        }
        GeneralSinusoidalSeriesProjection { proj, store: store.into() }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        sinu_s_forward(&self.store.borrow(), p);
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        sinu_s_inverse(&self.store.borrow(), p);
    }
}

/// Sinusoidal Ellipsoidal forward project
pub fn sinu_e_forward<P: TransformCoordinates>(sinu: &SinuData, proj: &Proj, p: &mut P) {
    let s = sin(p.phi());
    let c = cos(p.phi());
    p.set_y(mlfn(p.phi(), s, c, &sinu.en));
    p.set_x(p.lam() * c / sqrt(1. - proj.es * s * s));
}

/// Sinusoidal Ellipsoidal inverse project
pub fn sinu_e_inverse<P: TransformCoordinates>(sinu: &SinuData, proj: &Proj, p: &mut P) {
    let x = p.x();
    let y = p.y();

    p.set_phi(inv_mlfn(y, &sinu.en));
    let mut s = fabs(p.phi());
    if s < FRAC_PI_2 {
        s = sin(p.phi());
        p.set_lam(x * sqrt(1. - proj.es * s * s) / cos(p.phi()));
    } else if (s - EPS10) < FRAC_PI_2 {
        p.set_lam(0.);
    } else {
        panic!("Coordinate outside projection domain");
    }
}

/// Sinusoidal Spheroidal forward project
pub fn sinu_s_forward<P: TransformCoordinates>(sinu: &SinuData, p: &mut P) {
    if sinu.m == 0.0 {
        p.set_phi(if sinu.n != 1. { aasin(sinu.n * sin(p.phi())) } else { p.phi() });
    } else {
        let k = sinu.n * sin(p.phi());
        let mut i = MAX_ITER;
        while i > 0 {
            i -= 1;
            let v = (sinu.m * p.phi() + sin(p.phi()) - k) / (sinu.m + cos(p.phi()));
            p.set_phi(p.phi() - v);
            if fabs(v) < LOOP_TOL {
                break;
            }
        }
        if i != 0 {
            panic!("Coordinate outside projection domain");
        }
    }
    p.set_x(sinu.c_x * p.lam() * (sinu.m + cos(p.phi())));
    p.set_y(sinu.c_y * p.phi());
}

/// Sinusoidal Spheroidal inverse project
pub fn sinu_s_inverse<P: TransformCoordinates>(sinu: &SinuData, p: &mut P) {
    let mut y = p.y();

    y /= sinu.c_y;
    p.set_phi(if sinu.m != 0.0 {
        aasin((sinu.m * y + sin(y)) / sinu.n)
    } else if sinu.n != 1. {
        aasin(sin(y) / sinu.n)
    } else {
        y
    });
    p.set_lam(p.x() / (sinu.c_x * (sinu.m + cos(y))));
}
