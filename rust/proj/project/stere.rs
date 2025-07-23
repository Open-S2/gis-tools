use crate::proj::{
    CoordinateStep, EPS10, LATITUDE_STD_PARALLEL, POLAR_STEREOGRAPHIC_VARIANT_A,
    POLAR_STEREOGRAPHIC_VARIANT_B, POLAR_STEREOGRAPHIC_VARIANT_C, Proj, ProjMethod, ProjMode,
    ProjectCoordinates, SOUTH, TransformCoordinates, tsfn,
};
use alloc::rc::Rc;
use core::{
    cell::RefCell,
    f64::consts::{FRAC_PI_2, FRAC_PI_4},
};
use libm::{asin, atan, atan2, cos, fabs, hypot, pow, sin, sqrt, tan};

/// Stereographic Variables
#[derive(Debug, Default, Clone, PartialEq)]
pub struct StereData {
    phits: f64,
    sin_x1: f64,
    cos_x1: f64,
    akm1: f64,
    mode: ProjMode,
}

const TOL: f64 = 1e-8;
const NITER: usize = 8;
const CONV: f64 = 1e-10;

fn ssfn_(phit: f64, mut sinphi: f64, eccen: f64) -> f64 {
    sinphi *= eccen;
    tan(0.5 * (FRAC_PI_2 + phit)) * pow((1. - sinphi) / (1. + sinphi), 0.5 * eccen)
}

/// general stereographic initialization
fn stere_setup(proj: &mut Proj, store: &mut StereData) -> ProjMethod {
    let t = fabs(proj.phi0);
    if fabs(t - FRAC_PI_2) < EPS10 {
        store.mode = if proj.phi0 < 0. { ProjMode::SPole } else { ProjMode::NPole };
    } else {
        store.mode = if t > EPS10 { ProjMode::Obliq } else { ProjMode::Equit };
    }
    store.phits = fabs(store.phits);

    if proj.es != 0.0 {
        match store.mode {
            ProjMode::NPole | ProjMode::SPole => {
                if fabs(store.phits - FRAC_PI_2) < EPS10 {
                    store.akm1 = 2. * proj.k0
                        / sqrt(pow(1. + proj.e, 1. + proj.e) * pow(1. - proj.e, 1. - proj.e));
                } else {
                    let mut t = sin(store.phits);
                    store.akm1 = cos(store.phits) / tsfn(store.phits, t, proj.e);
                    t *= proj.e;
                    store.akm1 /= sqrt(1. - t * t);
                }
            }
            ProjMode::Equit | ProjMode::Obliq => {
                let mut t = sin(proj.phi0);
                let x = 2. * atan(ssfn_(proj.phi0, t, proj.e)) - FRAC_PI_2;
                t *= proj.e;
                store.akm1 = 2. * proj.k0 * cos(proj.phi0) / sqrt(1. - t * t);
                store.sin_x1 = sin(x);
                store.cos_x1 = cos(x);
            }
        }
        ProjMethod::Ellipsoidal
    } else {
        match store.mode {
            ProjMode::Obliq => {
                store.sin_x1 = sin(proj.phi0);
                store.cos_x1 = cos(proj.phi0);
                store.akm1 = 2. * proj.k0;
            }
            ProjMode::Equit => {
                store.akm1 = 2. * proj.k0;
            }
            ProjMode::SPole | ProjMode::NPole => {
                store.akm1 = if fabs(store.phits - FRAC_PI_2) >= EPS10 {
                    cos(store.phits) / tan(FRAC_PI_4 - 0.5 * store.phits)
                } else {
                    2. * proj.k0
                };
            }
        }

        ProjMethod::Spheroidal
    }
}

/// # Stereographic
///
/// **Classification**: Azimuthal
///
/// **Available forms**: Forward and inverse, spherical and ellipsoidal
///
/// **Defined area**: Global
///
/// **Alias**: stere
///
/// **Domain**: 2D
///
/// **Input type**: Geodetic coordinates
///
/// **Output type**: Projected coordinates
///
/// ## Projection String
/// ```ini
/// +proj=stere +lat_0=90 +latTs=75
/// ```
///
/// Note:
/// This projection method gives different results than the :ref:`sterea`
/// method in the non-polar cases (i.e. the oblique and equatorial case). The later
/// projection method is the one referenced by EPSG as "Oblique Stereographic".
///
/// ## Required Parameters
/// - None
///
/// ## Optional Parameters
/// - `+lat_0=<value>`: Latitude of origin.
/// - `+latTs=<value>`: Latitude where scale is not distorted.
/// - `+k_0=<value>`: Scale factor.
/// - `+lon_0=<value>`: Central meridian.
/// - `+ellps=<value>`: Ellipsoid used.
/// - `+R=<value>`: Radius of the projection sphere.
/// - `+x_0=<value>`: False easting.
/// - `+y_0=<value>`: False northing.
///
/// ![Stereographic](https://github.com/Open-S2/gis-tools/blob/master/assets/proj4/projections/images/stere.png?raw=true)
#[derive(Debug, Clone, PartialEq)]
pub struct StereographicProjection {
    proj: Rc<RefCell<Proj>>,
    store: RefCell<StereData>,
    method: ProjMethod,
}
impl ProjectCoordinates for StereographicProjection {
    fn code(&self) -> i64 {
        -1
    }
    fn name(&self) -> &'static str {
        "Stereographic"
    }
    fn names() -> &'static [&'static str] {
        &[
            "Stereographic",
            "Polar_Stereographic",
            "StereographicSouthPole",
            "Stereographic_South_Pole",
            "Stereographic South Pole",
            "Polar Stereographic (variant B)",
            "stere",
        ]
    }
}
impl CoordinateStep for StereographicProjection {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        let mut store = StereData::default();
        let method: ProjMethod;
        {
            let proj = &mut proj.borrow_mut();
            store.phits = if let Some(lat_ts) = proj.params.get(&LATITUDE_STD_PARALLEL) {
                lat_ts.f64()
            } else {
                FRAC_PI_2
            };

            method = stere_setup(proj, &mut store);
        }
        StereographicProjection { proj, store: store.into(), method }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        if self.method == ProjMethod::Spheroidal {
            stere_s_forward(&mut self.store.borrow_mut(), p);
        } else {
            stere_e_forward(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
        }
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        if self.method == ProjMethod::Spheroidal {
            stere_s_inverse(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
        } else {
            stere_e_inverse(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
        }
    }
}

/// Polar Stereographic Variant A Projection
pub type PolarStereographicVariantAProjection =
    UniversalPolarStereographicProjection<POLAR_STEREOGRAPHIC_VARIANT_A>;
/// Polar Stereographic Variant B Projection
pub type PolarStereographicVariantBProjection =
    UniversalPolarStereographicProjection<POLAR_STEREOGRAPHIC_VARIANT_B>;
/// Polar Stereographic Variant C Projection
pub type PolarStereographicVariantCProjection =
    UniversalPolarStereographicProjection<POLAR_STEREOGRAPHIC_VARIANT_C>;

/// Stereographic Projection
#[derive(Debug, Clone, PartialEq)]
pub struct UniversalPolarStereographicProjection<const C: i64> {
    proj: Rc<RefCell<Proj>>,
    store: RefCell<StereData>,
    method: ProjMethod,
}
impl<const C: i64> ProjectCoordinates for UniversalPolarStereographicProjection<C> {
    fn code(&self) -> i64 {
        C
    }
    fn name(&self) -> &'static str {
        "Universal Polar Stereographic"
    }
    fn names() -> &'static [&'static str] {
        &[
            "Polar Stereographic",
            "Universal Polar Stereographic",
            "Polar Stereographic (variant A)",
            "Polar Stereographic (variant B)",
            "Polar Stereographic (variant C)",
        ]
    }
}
impl<const C: i64> CoordinateStep for UniversalPolarStereographicProjection<C> {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        let mut store = StereData::default();
        let method: ProjMethod;
        {
            let proj = &mut proj.borrow_mut();
            proj.phi0 = if proj.params.contains_key(&SOUTH) { -FRAC_PI_2 } else { FRAC_PI_2 };
            if proj.es == 0.0 {
                panic!("Invalid value for es: only ellipsoidal formulation supported");
            }
            proj.k0 = 0.994;
            proj.x0 = 2000000.;
            proj.y0 = 2000000.;
            store.phits = FRAC_PI_2;
            proj.lam0 = 0.;

            method = stere_setup(proj, &mut store);
        }
        UniversalPolarStereographicProjection { proj, store: store.into(), method }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        if self.method == ProjMethod::Spheroidal {
            stere_s_forward(&mut self.store.borrow_mut(), p);
        } else {
            stere_e_forward(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
        }
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        if self.method == ProjMethod::Spheroidal {
            stere_s_inverse(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
        } else {
            stere_e_inverse(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
        }
    }
}

/// Stereographic Ellipsoidal forward project
pub fn stere_e_forward<P: TransformCoordinates>(stere: &mut StereData, proj: &Proj, p: &mut P) {
    let mut x;
    let y;
    let mut coslam = cos(p.lam());
    let sinlam = sin(p.lam());
    let mut sinphi = sin(p.phi());
    let mut sin_x = 0.;
    let mut cos_x = 0.;
    if stere.mode == ProjMode::Obliq || stere.mode == ProjMode::Equit {
        let x = 2. * atan(ssfn_(p.phi(), sinphi, proj.e)) - FRAC_PI_2;
        sin_x = sin(x);
        cos_x = cos(x);
    }

    match stere.mode {
        ProjMode::Obliq => {
            let denom = stere.cos_x1 * (1. + stere.sin_x1 * sin_x + stere.cos_x1 * cos_x * coslam);
            if denom == 0. {
                panic!("Coordinate outside projection domain");
            }
            let a = stere.akm1 / denom;
            y = a * (stere.cos_x1 * sin_x - stere.sin_x1 * cos_x * coslam);
            x = a * cos_x;
        }
        ProjMode::Equit => {
            // avoid zero division
            let mut a = 0.;
            if 1. + cos_x * coslam == 0.0 {
                y = f64::MAX;
            } else {
                a = stere.akm1 / (1. + cos_x * coslam);
                y = a * sin_x;
            }
            x = a * cos_x;
        }
        ProjMode::SPole => {
            p.set_phi(-p.phi());
            coslam = -coslam;
            sinphi = -sinphi;
            if fabs(p.phi() - FRAC_PI_2) < 1e-15 {
                x = 0.;
            } else {
                x = stere.akm1 * tsfn(p.phi(), sinphi, proj.e);
            }
            y = -x * coslam;
        }
        ProjMode::NPole => {
            if fabs(p.phi() - FRAC_PI_2) < 1e-15 {
                x = 0.;
            } else {
                x = stere.akm1 * tsfn(p.phi(), sinphi, proj.e);
            }
            y = -x * coslam;
        }
    }

    x *= sinlam;
    p.set_x(x);
    p.set_y(y);
}

/// Stereographic Spheroidal forward project
pub fn stere_s_forward<P: TransformCoordinates>(stere: &mut StereData, p: &mut P) {
    let sinphi = sin(p.phi());
    let cosphi = cos(p.phi());
    let mut coslam = cos(p.lam());
    let sinlam = sin(p.lam());
    let x;
    let mut y;

    match stere.mode {
        ProjMode::Equit => {
            y = 1. + cosphi * coslam;
            if y <= EPS10 {
                panic!("Coordinate outside projection domain");
            }
            y = stere.akm1 / y;
            x = y * cosphi * sinlam;
            y *= if stere.mode == ProjMode::Equit {
                sinphi
            } else {
                stere.cos_x1 * sinphi - stere.sin_x1 * cosphi * coslam
            };
        }
        ProjMode::Obliq => {
            y = 1. + stere.sin_x1 * sinphi + stere.cos_x1 * cosphi * coslam;
            if y <= EPS10 {
                panic!("Coordinate outside projection domain");
            }
            y = stere.akm1 / y;
            x = y * cosphi * sinlam;
            y *= if stere.mode == ProjMode::Equit {
                sinphi
            } else {
                stere.cos_x1 * sinphi - stere.sin_x1 * cosphi * coslam
            };
        }
        ProjMode::NPole => {
            coslam = -coslam;
            p.set_phi(-p.phi());
            if fabs(p.phi() - FRAC_PI_2) < TOL {
                panic!("Coordinate outside projection domain");
            }
            y = stere.akm1 * tan(FRAC_PI_4 + 0.5 * p.phi());
            x = sinlam * y;
            y *= coslam;
        }
        ProjMode::SPole => {
            if fabs(p.phi() - FRAC_PI_2) < TOL {
                panic!("Coordinate outside projection domain");
            }
            y = stere.akm1 * tan(FRAC_PI_4 + 0.5 * p.phi());
            x = sinlam * y;
            y *= coslam;
        }
    }

    p.set_x(x);
    p.set_y(y);
}

/// Stereographic Ellipsoidal inverse project
pub fn stere_e_inverse<P: TransformCoordinates>(stere: &mut StereData, proj: &Proj, p: &mut P) {
    let mut x = p.x();
    let mut y = p.y();
    let rho = hypot(p.x(), p.y());
    let mut phi_l;
    let mut tp;
    let halfpi;
    let halfe;

    match stere.mode {
        ProjMode::Obliq | ProjMode::Equit => {
            tp = 2. * atan2(rho * stere.cos_x1, stere.akm1);
            let cosphi = cos(tp);
            let sinphi = sin(tp);
            if rho == 0.0 {
                phi_l = asin(cosphi * stere.sin_x1);
            } else {
                phi_l = asin(cosphi * stere.sin_x1 + (y * sinphi * stere.cos_x1 / rho));
            }

            tp = tan(0.5 * (FRAC_PI_2 + phi_l));
            x *= sinphi;
            y = rho * stere.cos_x1 * cosphi - y * stere.sin_x1 * sinphi;
            halfpi = FRAC_PI_2;
            halfe = 0.5 * proj.e;
        }
        ProjMode::NPole => {
            y = -y;
            tp = -rho / stere.akm1;
            phi_l = FRAC_PI_2 - 2. * atan(tp);
            halfpi = -FRAC_PI_2;
            halfe = -0.5 * proj.e;
        }
        ProjMode::SPole => {
            tp = -rho / stere.akm1;
            phi_l = FRAC_PI_2 - 2. * atan(tp);
            halfpi = -FRAC_PI_2;
            halfe = -0.5 * proj.e;
        }
    }

    let mut i = NITER;
    while i > 0 {
        let sinphi = proj.e * sin(phi_l);
        p.set_phi(2. * atan(tp * pow((1. + sinphi) / (1. - sinphi), halfe)) - halfpi);
        if fabs(phi_l - p.phi()) < CONV {
            if stere.mode == ProjMode::SPole {
                p.set_phi(-p.phi());
            }
            p.set_lam(if x == 0. && y == 0. { 0. } else { atan2(x, y) });
            return;
        }
        phi_l = p.phi();
        i -= 1;
    }

    panic!("Coordinate outside projection domain");
}

/// Stereographic Spheroidal inverse project
pub fn stere_s_inverse<P: TransformCoordinates>(stere: &mut StereData, proj: &Proj, p: &mut P) {
    let rh = hypot(p.x(), p.y());
    let mut c = 2. * atan(rh / stere.akm1);
    let sinc = sin(c);
    let cosc = cos(c);
    let mut lam = 0.;
    let phi;

    match stere.mode {
        ProjMode::Equit => {
            if fabs(rh) <= EPS10 {
                phi = 0.;
            } else {
                phi = asin(p.y() * sinc / rh);
            }
            if cosc != 0. || p.x() != 0. {
                lam = atan2(p.x() * sinc, cosc * rh);
            }
        }
        ProjMode::Obliq => {
            if fabs(rh) <= EPS10 {
                phi = proj.phi0;
            } else {
                phi = asin(cosc * stere.sin_x1 + p.y() * sinc * stere.cos_x1 / rh);
            }
            c = cosc - stere.sin_x1 * sin(phi);
            if c != 0. || p.x() != 0. {
                lam = atan2(p.x() * sinc * stere.cos_x1, c * rh);
            }
        }
        ProjMode::NPole => {
            p.set_y(-p.y());
            if fabs(rh) <= EPS10 {
                phi = proj.phi0;
            } else {
                phi = asin(if stere.mode == ProjMode::SPole { -cosc } else { cosc });
            }
            lam = if p.x() == 0. && p.y() == 0. { 0. } else { atan2(p.x(), p.y()) };
        }
        ProjMode::SPole => {
            if fabs(rh) <= EPS10 {
                phi = proj.phi0;
            } else {
                phi = asin(if stere.mode == ProjMode::SPole { -cosc } else { cosc });
            }
            lam = if p.x() == 0. && p.y() == 0. { 0. } else { atan2(p.x(), p.y()) };
        }
    }

    p.set_lam(lam);
    p.set_phi(phi);
}
