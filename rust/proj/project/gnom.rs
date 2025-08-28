use crate::proj::{
    CoordinateStep, EPS10, GeodGeodesic, GeodGeodesicline, GeodMask, Proj, ProjMethod, ProjMode,
    ProjectCoordinates, TransformCoordinates, geod_geninverse, geod_genposition, geod_init,
    geod_lineinit,
};
use alloc::rc::Rc;
use core::{cell::RefCell, f64::consts::FRAC_PI_2};
use libm::{asin, atan, atan2, cos, fabs, hypot, sin, sqrt};

/// Gnomonic Variables
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GnomData {
    sinph0: f64,
    cosph0: f64,
    mode: ProjMode,
    g: GeodGeodesic,
}

/// # Gnomonic (gnom)
///
/// For a sphere, the gnomonic projection is a projection from the center of
/// the sphere onto a plane tangent to the center point of the projection.
/// This projects great circles to straight lines.  For an ellipsoid, it is
/// the limit of a doubly azimuthal projection, a projection where the
/// azimuths from 2 points are preserved, as the two points merge into the
/// center point.  In this case, geodesics project to approximately straight
/// lines (these are exactly straight if the geodesic includes the center
/// point).  For details, see Section 8 of :cite:`Karney2013`.
///
/// **Classification**: Azimuthal
///
/// **Available forms**: Forward and inverse, spherical and ellipsoidal
///
/// **Defined area**: Within a quarter circumference of the center point
///
/// **Alias**: gnom
///
/// **Domain**: 2D
///
/// **Input type**: Geodetic coordinates
///
/// **Output type**: Projected coordinates
///
/// ## Projection String
/// ```ini
/// +proj=gnom +lat_0=90 +lon_0=-50 +R=6.4e6
/// ```
///
/// ## Required Parameters
/// - None, all parameters are optional for this projection.
///
/// ## Optional Parameters
/// - `+lon_0`: Longitude of origin (central meridian).
/// - `+lat_0`: Latitude of origin.
/// - `+x_0`: False easting.
/// - `+y_0`: False northing.
/// - `+ellps`: Ellipsoid.
/// - `+R`: Earth radius.
///
/// Reference:
/// Wolfram Mathworld "Gnomonic Projection"
/// <http://mathworld.wolfram.com/GnomonicProjection.html>
/// Accessed: 12th November 2009
///
/// ![Gnomonic](https://github.com/Open-S2/gis-tools/blob/master/assets/proj4/projections/images/gnom.png?raw=true)
#[derive(Debug, Clone, PartialEq)]
pub struct GnomonicProjection {
    proj: Rc<RefCell<Proj>>,
    store: RefCell<GnomData>,
    method: ProjMethod,
}
impl ProjectCoordinates for GnomonicProjection {
    fn code(&self) -> i64 {
        -1
    }
    fn name(&self) -> &'static str {
        "Gnomonic"
    }
    fn names() -> &'static [&'static str] {
        &["Gnomonic", "gnom"]
    }
}
impl CoordinateStep for GnomonicProjection {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        let mut store = GnomData::default();
        let method: ProjMethod;
        {
            let proj = &mut proj.borrow_mut();

            method = if proj.es == 0. {
                if fabs(fabs(proj.phi0) - FRAC_PI_2) < EPS10 {
                    store.mode = if proj.phi0 < 0. { ProjMode::SPole } else { ProjMode::NPole };
                } else if fabs(proj.phi0) < EPS10 {
                    store.mode = ProjMode::Equit;
                } else {
                    store.mode = ProjMode::Obliq;
                    store.sinph0 = sin(proj.phi0);
                    store.cosph0 = cos(proj.phi0);
                }
                ProjMethod::Spheroidal
            } else {
                geod_init(&mut store.g, 1., proj.f);
                ProjMethod::Ellipsoidal
            };
            proj.es = 0.;
        }
        GnomonicProjection { proj, store: store.into(), method }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        if self.method == ProjMethod::Ellipsoidal {
            gnom_e_forward(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
        } else {
            gnom_s_forward(&mut self.store.borrow_mut(), p);
        }
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        if self.method == ProjMethod::Ellipsoidal {
            gnom_e_inverse(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
        } else {
            gnom_s_inverse(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
        }
    }
}

/// Gnomonic Spheroidal Forward
pub fn gnom_s_forward<P: TransformCoordinates>(gnom: &mut GnomData, p: &mut P) {
    let mut y;

    let sinphi = sin(p.phi());
    let cosphi = cos(p.phi());
    let mut coslam = cos(p.lam());

    match gnom.mode {
        ProjMode::Equit => {
            y = cosphi * coslam;
        }
        ProjMode::Obliq => {
            y = gnom.sinph0 * sinphi + gnom.cosph0 * cosphi * coslam;
        }
        ProjMode::SPole => {
            y = -sinphi;
        }
        ProjMode::NPole => {
            y = sinphi;
        }
    }

    if y <= EPS10 {
        panic!("Coordinate outside projection domain");
    }

    y = 1. / y;
    let x = (y) * cosphi * sin(p.lam());
    match gnom.mode {
        ProjMode::Equit => {
            y *= sinphi;
        }
        ProjMode::Obliq => {
            y *= gnom.cosph0 * sinphi - gnom.sinph0 * cosphi * coslam;
        }
        ProjMode::NPole => {
            coslam = -coslam;
            y *= cosphi * coslam;
        }
        ProjMode::SPole => {
            y *= cosphi * coslam;
        }
    }

    p.set_x(x);
    p.set_y(y);
}

/// Gnomonic Spheroidal inverse
pub fn gnom_s_inverse<P: TransformCoordinates>(gnom: &mut GnomData, proj: &Proj, p: &mut P) {
    let mut x = p.x();
    let mut y = p.y();

    let rh = hypot(x, y);
    let mut phi = atan(rh);
    let lam;
    let sinz = sin(phi);
    let cosz = sqrt(1. - sinz * sinz);

    if fabs(rh) <= EPS10 {
        phi = proj.phi0;
        lam = 0.;
    } else {
        match gnom.mode {
            ProjMode::Obliq => {
                phi = cosz * gnom.sinph0 + y * sinz * gnom.cosph0 / rh;
                if fabs(phi) >= 1. {
                    phi = if phi > 0. { FRAC_PI_2 } else { -FRAC_PI_2 };
                } else {
                    phi = asin(phi);
                }
                y = (cosz - gnom.sinph0 * sin(phi)) * rh;
                x *= sinz * gnom.cosph0;
            }
            ProjMode::Equit => {
                phi = y * sinz / rh;
                if fabs(phi) >= 1. {
                    phi = if phi > 0. { FRAC_PI_2 } else { -FRAC_PI_2 };
                } else {
                    phi = asin(phi);
                }
                y = cosz * rh;
                x *= sinz;
            }
            ProjMode::SPole => {
                phi -= FRAC_PI_2;
            }
            ProjMode::NPole => {
                phi = FRAC_PI_2 - phi;
                y = -y;
            }
        }
        lam = atan2(x, y);
    }

    p.set_phi(phi);
    p.set_lam(lam);
}

/// Gnomonic Ellipsoidal Forward
pub fn gnom_e_forward<P: TransformCoordinates>(gnom: &mut GnomData, proj: &Proj, p: &mut P) {
    let lat0 = proj.phi0.to_degrees();
    let lon0 = 0.;
    let lat1 = p.phi().to_degrees();
    let lon1 = p.lam().to_degrees();
    let mut azi0 = 0.;
    let mut m = 0.;
    let mut _m = 0.;

    geod_geninverse(
        &mut gnom.g,
        lat0,
        lon0,
        lat1,
        lon1,
        &mut 0.,
        &mut azi0,
        &mut 0.,
        &mut m,
        &mut _m,
        &mut 0.,
        &mut 0.,
    );
    if _m <= 0. {
        panic!("Coordinate outside projection domain {_m}");
    } else {
        let rho = m / _m;
        azi0 = azi0.to_radians();
        p.set_x(rho * sin(azi0));
        p.set_y(rho * cos(azi0));
    }
}

/// Gnomonic Ellipsoidal inverse
pub fn gnom_e_inverse<P: TransformCoordinates>(gnom: &mut GnomData, proj: &Proj, p: &mut P) {
    let numit_ = 10;
    let eps_ = 0.01 * sqrt(f64::EPSILON);
    let lat0 = proj.phi0.to_degrees();
    let lon0 = 0.;
    let azi0 = atan2(p.x(), p.y()).to_degrees();
    let mut rho = hypot(p.x(), p.y());
    let mut s = atan(rho);
    let little = rho <= 1.;
    if !little {
        rho = 1. / rho;
    }
    let mut l = GeodGeodesicline::default();
    geod_lineinit(
        &mut l,
        &gnom.g,
        lat0,
        lon0,
        azi0,
        GeodMask::GeodLatitude as u32
            | GeodMask::GeodLongitude as u32
            | GeodMask::GeodDistanceIn as u32
            | GeodMask::GeodReducedlength as u32
            | GeodMask::GeodGeodesicScale as u32,
    );

    let mut lat1 = 0.;
    let mut lon1 = 0.;
    let mut count = numit_;
    let mut trip = 0;
    while count != 0 {
        count -= 1;
        // double m, M;
        let mut m = 0.;
        let mut _m = 0.;
        geod_genposition(
            &l, 0, s, &mut lat1, &mut lon1, &mut 0., &mut s, &mut m, &mut _m, &mut 0., &mut 0.,
        );
        if trip != 0 {
            break;
        }
        // If little, solve rho(s) = rho with drho(s)/ds = 1/M^2
        // else solve 1/rho(s) = 1/rho with d(1/rho(s))/ds = -1/m^2
        let ds = if little { (m - rho * _m) * _m } else { (rho * m - _m) * m };
        s -= ds;
        // Reversed test to allow escape with NaNs
        if fabs(ds) < eps_ {
            trip += 1;
        }
    }
    if trip != 0 {
        p.set_phi(lat1.to_radians());
        p.set_lam(lon1.to_radians());
    } else {
        panic!("Coordinate outside projection domain");
    }
}
