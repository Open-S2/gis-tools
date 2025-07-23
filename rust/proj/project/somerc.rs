use crate::proj::{CoordinateStep, Proj, ProjectCoordinates, SOMERC, TransformCoordinates, aasin};
use alloc::rc::Rc;
use core::{
    cell::RefCell,
    f64::consts::{FRAC_PI_2, FRAC_PI_4},
};
use libm::{atan, cos, exp, fabs, log, sin, sqrt, tan};

/// Swiss Oblique Mercator variables
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SomercData {
    k: f64,
    c: f64,
    hlf_e: f64,
    k_r: f64,
    cosp0: f64,
    sinp0: f64,
}

const EPS: f64 = 1.0e-10;
const NITER: usize = 6;

/// # Swiss Oblique Mercator
///
/// **Classification**: Oblique Mercator
///
/// **Available forms**: Forward and inverse, ellipsoidal only
///
/// **Defined area**: Global
///
/// **Alias**: somerc
///
/// **Domain**: 2D
///
/// **Input type**: Geodetic coordinates
///
/// **Output type**: Projected coordinates
///
/// ## Projection String
/// ```ini
/// +proj=somerc
/// ```
///
/// ## Required Parameters
/// - None
///
/// ## Optional Parameters
/// - `+lon_0=<value>`: Central meridian.
/// - `+ellps=<value>`: Ellipsoid used.
/// - `+R=<value>`: Radius of the projection sphere.
/// - `+k_0=<value>`: Scale factor.
/// - `+x_0=<value>`: False easting.
/// - `+y_0=<value>`: False northing.
///
/// ## References:
/// Formules et constantes pour le Calcul pour la
/// projection cylindrique conforme à axe oblique et pour la transformation entre
/// des systèmes de référence.
/// http://www.swisstopo.admin.ch/internet/swisstopo/fr/home/topics/survey/sys/refsys/switzerland.parsysrelated1.31216.downloadList.77004.DownloadFile.tmp/swissprojectionfr.pdf
///
/// ![Swiss Oblique Mercator](https://github.com/Open-S2/gis-tools/blob/master/assets/proj4/projections/images/somerc.png?raw=true)
#[derive(Debug, Clone, PartialEq)]
pub struct SwissOblMercatorProjection {
    proj: Rc<RefCell<Proj>>,
    store: RefCell<SomercData>,
}
impl ProjectCoordinates for SwissOblMercatorProjection {
    fn code(&self) -> i64 {
        SOMERC
    }
    fn name(&self) -> &'static str {
        "Swiss. Obl. Mercator"
    }
    fn names() -> &'static [&'static str] {
        &["Swiss. Obl. Mercator", "somerc"]
    }
}
impl CoordinateStep for SwissOblMercatorProjection {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        let mut store = SomercData::default();
        {
            let proj = &mut proj.borrow_mut();
            store.hlf_e = 0.5 * proj.e;
            let mut cp = cos(proj.phi0);
            cp *= cp;
            store.c = sqrt(1. + proj.es * cp * cp * proj.rone_es);
            let mut sp = sin(proj.phi0);
            store.sinp0 = sp / store.c;
            let phip0 = aasin(store.sinp0);
            store.cosp0 = cos(phip0);
            sp *= proj.e;
            store.k = log(tan(FRAC_PI_4 + 0.5 * phip0))
                - store.c
                    * (log(tan(FRAC_PI_4 + 0.5 * proj.phi0))
                        - store.hlf_e * log((1. + sp) / (1. - sp)));
            store.k_r = proj.k0 * sqrt(proj.one_es) / (1. - sp * sp);
        }
        SwissOblMercatorProjection { proj, store: store.into() }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        somerc_e_forward(&self.store.borrow(), &self.proj.borrow(), p);
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        somerc_e_inverse(&self.store.borrow(), &self.proj.borrow(), p);
    }
}

/// Equal Earth Ellipsoidal forward project
pub fn somerc_e_forward<P: TransformCoordinates>(somerc: &SomercData, proj: &Proj, p: &mut P) {
    let sp = proj.e * sin(p.phi());
    let phip = 2.
        * atan(exp(somerc.c
            * (log(tan(FRAC_PI_4 + 0.5 * p.phi())) - somerc.hlf_e * log((1. + sp) / (1. - sp)))
            + somerc.k))
        - FRAC_PI_2;
    let lamp = somerc.c * p.lam();
    let cp = cos(phip);
    let phipp = aasin(somerc.cosp0 * sin(phip) - somerc.sinp0 * cp * cos(lamp));
    let lampp = aasin(cp * sin(lamp) / cos(phipp));
    p.set_x(somerc.k_r * lampp);
    p.set_y(somerc.k_r * log(tan(FRAC_PI_4 + 0.5 * phipp)));
}

/// Equal Earth Ellipsoidal forward project
pub fn somerc_e_inverse<P: TransformCoordinates>(somerc: &SomercData, proj: &Proj, p: &mut P) {
    let phipp = 2. * (atan(exp(p.y() / somerc.k_r)) - FRAC_PI_4);
    let lampp = p.x() / somerc.k_r;
    let cp = cos(phipp);
    let mut phip = aasin(somerc.cosp0 * sin(phipp) + somerc.sinp0 * cp * cos(lampp));
    let lamp = aasin(cp * sin(lampp) / cos(phip));
    let con = (somerc.k - log(tan(FRAC_PI_4 + 0.5 * phip))) / somerc.c;
    let mut i = NITER;
    while i > 0 {
        let esp = proj.e * sin(phip);
        let delp = (con + log(tan(FRAC_PI_4 + 0.5 * phip))
            - somerc.hlf_e * log((1. + esp) / (1. - esp)))
            * (1. - esp * esp)
            * cos(phip)
            * proj.rone_es;
        phip -= delp;
        if fabs(delp) < EPS {
            break;
        }
        i -= 1;
    }
    if i != 0 {
        p.set_phi(phip);
        p.set_lam(lamp / somerc.c);
    } else {
        panic!("Coordinate outside projection domain");
    }
}
