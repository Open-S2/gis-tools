use crate::proj::{
    AZIMUTH_PROJECTION_CENTRE, CoordinateStep, Coords, EPS10, ORTHOGRAPHIC, Proj, ProjMethod,
    ProjMode, ProjValue, ProjectCoordinates, TransformCoordinates, adjlon,
};
use alloc::rc::Rc;
use core::{
    cell::RefCell,
    f64::consts::{FRAC_PI_2, PI},
};
use libm::{acos, asin, atan2, cos, fabs, hypot, sin, sqrt};

/// Orthographic variables
#[derive(Debug, Default, Clone, PartialEq)]
pub struct OrthoData {
    sinph0: f64,
    cosph0: f64,
    nu0: f64,
    y_shift: f64,
    y_scale: f64,
    mode: ProjMode,
    sinalpha: f64,
    cosalpha: f64,
}

/// Orthographic Projection
#[derive(Debug, Clone, PartialEq)]
pub struct OrthographicProjection {
    proj: Rc<RefCell<Proj>>,
    store: RefCell<OrthoData>,
    method: ProjMethod,
}
impl ProjectCoordinates for OrthographicProjection {
    fn code(&self) -> i64 {
        ORTHOGRAPHIC
    }
    fn name(&self) -> &'static str {
        "Orthographic"
    }
    fn names() -> &'static [&'static str] {
        &["Orthographic", "ortho"]
    }
}
impl CoordinateStep for OrthographicProjection {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        let mut store = OrthoData::default();
        let method: ProjMethod;
        {
            let proj = &mut proj.borrow_mut();
            store.sinph0 = sin(proj.phi0);
            store.cosph0 = cos(proj.phi0);
            if fabs(fabs(proj.phi0) - FRAC_PI_2) <= EPS10 {
                store.mode = if proj.phi0 < 0. { ProjMode::SPole } else { ProjMode::NPole };
            } else if fabs(proj.phi0) > EPS10 {
                store.mode = ProjMode::Obliq;
            } else {
                store.mode = ProjMode::Equit;
            }
            method = if proj.es == 0. {
                ProjMethod::Spheroidal
            } else {
                store.nu0 = 1.0 / sqrt(1.0 - proj.es * store.sinph0 * store.sinph0);
                store.y_shift = proj.es * store.nu0 * store.sinph0 * store.cosph0;
                store.y_scale = 1.0 / sqrt(1.0 - proj.es * store.cosph0 * store.cosph0);
                ProjMethod::Ellipsoidal
            };

            let alpha =
                proj.params.get(&AZIMUTH_PROJECTION_CENTRE).unwrap_or(&ProjValue::default()).f64();
            store.sinalpha = sin(alpha);
            store.cosalpha = cos(alpha);
        }
        OrthographicProjection { proj, store: store.into(), method }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        if self.method == ProjMethod::Spheroidal {
            ortho_s_forward(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
        } else {
            ortho_e_forward(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
        }
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        if self.method == ProjMethod::Spheroidal {
            ortho_s_inverse(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
        } else {
            ortho_e_inverse(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
        }
    }
}

fn throw_error() {
    panic!("Coordinate outside projection domain");
}

/// Equal Earth Spheroidal forward project
pub fn ortho_s_forward<P: TransformCoordinates>(ortho: &mut OrthoData, proj: &Proj, p: &mut P) {
    let mut y;

    let cosphi = cos(p.phi());
    let mut coslam = cos(p.lam());
    match ortho.mode {
        ProjMode::Equit => {
            if cosphi * coslam < -EPS10 {
                throw_error();
            }
            y = sin(p.phi());
        }
        ProjMode::Obliq => {
            let sinphi = sin(p.phi());
            // Is the point visible from the projection plane ?
            // From
            // https://lists.osgeo.org/pipermail/proj/2020-September/009831.html
            // this is the dot product of the normal of the ellipsoid at the center
            // of the projection and at the point considered for projection.
            // [cos(phi)*cos(lambda), cos(phi)*sin(lambda), sin(phi)]
            // Also from Snyder's Map Projection - A working manual, equation (5-3),
            // page 149
            if ortho.sinph0 * sinphi + ortho.cosph0 * cosphi * coslam < -EPS10 {
                throw_error();
            }
            y = ortho.cosph0 * sinphi - ortho.sinph0 * cosphi * coslam;
        }
        ProjMode::NPole => {
            coslam = -coslam;
            if fabs(p.phi() - proj.phi0) - EPS10 > FRAC_PI_2 {
                throw_error();
            }
            y = cosphi * coslam;
        }
        ProjMode::SPole => {
            if fabs(p.phi() - proj.phi0) - EPS10 > FRAC_PI_2 {
                throw_error();
            }
            y = cosphi * coslam;
        }
    }
    let mut x = cosphi * sin(p.lam());

    let xp = x;
    let yp = y;
    x = (xp * ortho.cosalpha - yp * ortho.sinalpha) * proj.k0;
    y = (xp * ortho.sinalpha + yp * ortho.cosalpha) * proj.k0;

    p.set_x(x);
    p.set_y(y);
}

/// Equal Earth Spheroidal inverse project
pub fn ortho_s_inverse<P: TransformCoordinates>(ortho: &mut OrthoData, proj: &Proj, p: &mut P) {
    let lam;
    let mut phi;

    let xf = p.x();
    let yf = p.y();
    let mut x = (ortho.cosalpha * xf + ortho.sinalpha * yf) / proj.k0;
    let mut y = (-ortho.sinalpha * xf + ortho.cosalpha * yf) / proj.k0;

    let rh = hypot(x, y);
    let mut sinc = rh;
    if sinc > 1. {
        if (sinc - 1.) > EPS10 {
            throw_error();
        }
        sinc = 1.;
    }
    let cosc = sqrt(1. - sinc * sinc); /* in this range OK */
    if fabs(rh) <= EPS10 {
        phi = proj.phi0;
        lam = 0.0;
    } else {
        match ortho.mode {
            ProjMode::NPole => {
                y = -y;
                phi = acos(sinc);
            }
            ProjMode::SPole => {
                phi = -acos(sinc);
            }
            ProjMode::Equit => {
                phi = y * sinc / rh;
                x *= sinc;
                y = cosc * rh;
                // goto sinchk;
                if fabs(phi) >= 1. {
                    phi = if phi < 0. { -FRAC_PI_2 } else { FRAC_PI_2 };
                } else {
                    phi = asin(phi);
                }
            }
            ProjMode::Obliq => {
                phi = cosc * ortho.sinph0 + y * sinc * ortho.cosph0 / rh;
                y = (cosc - ortho.sinph0 * phi) * rh;
                x *= sinc * ortho.cosph0;
                // goto sinchk;
                if fabs(phi) >= 1. {
                    phi = if phi < 0. { -FRAC_PI_2 } else { FRAC_PI_2 };
                } else {
                    phi = asin(phi);
                }
            }
        }
        lam = if y == 0. && (ortho.mode == ProjMode::Obliq || ortho.mode == ProjMode::Equit) {
            if x == 0. {
                0.
            } else if x < 0. {
                -FRAC_PI_2
            } else {
                FRAC_PI_2
            }
        } else {
            atan2(x, y)
        };
    }

    p.set_lam(lam);
    p.set_phi(phi);
}

/// Equal Earth Ellipsoidal forward project
pub fn ortho_e_forward<P: TransformCoordinates>(ortho: &mut OrthoData, proj: &Proj, p: &mut P) {
    // From EPSG guidance note 7.2, March 2020, §3.3.5 Orthographic
    let cosphi = cos(p.phi());
    let sinphi = sin(p.phi());
    let coslam = cos(p.lam());
    let sinlam = sin(p.lam());

    // Is the point visible from the projection plane ?
    // Same condition as in spherical case
    if ortho.sinph0 * sinphi + ortho.cosph0 * cosphi * coslam < -EPS10 {
        throw_error();
    }

    let nu = 1.0 / sqrt(1.0 - proj.es * sinphi * sinphi);
    let xp = nu * cosphi * sinlam;
    let yp = nu * (sinphi * ortho.cosph0 - cosphi * ortho.sinph0 * coslam)
        + proj.es * (ortho.nu0 * ortho.sinph0 - nu * sinphi) * ortho.cosph0;
    p.set_x((ortho.cosalpha * xp - ortho.sinalpha * yp) * proj.k0);
    p.set_y((ortho.sinalpha * xp + ortho.cosalpha * yp) * proj.k0);
}

/// Equal Earth Ellipsoidal inverse project
pub fn ortho_e_inverse<P: TransformCoordinates>(ortho: &mut OrthoData, proj: &Proj, p: &mut P) {
    let sq = |x: f64| -> f64 { x * x };

    let xf = p.x();
    let yf = p.y();
    let x = (ortho.cosalpha * xf + ortho.sinalpha * yf) / proj.k0;
    let y = (-ortho.sinalpha * xf + ortho.cosalpha * yf) / proj.k0;

    if ortho.mode == ProjMode::NPole || ortho.mode == ProjMode::SPole {
        // Polar case. Forward case equations can be simplified as:
        // x = nu * cosphi * sinlam
        // y = nu * -cosphi * coslam * sign(phi0)
        // ==> lam = atan2(x, -y * sign(phi0))
        // ==> x^2 + y^2 = nu^2 * cosphi^2
        //                rh^2 = cosphi^2 / (1 - es * sinphi^2)
        // ==>  cosphi^2 = rh^2 * (1 - es) / (1 - es * rh^2)

        let rh2 = sq(x) + sq(y);
        if rh2 >= 1. - 1e-15 {
            if (rh2 - 1.) > EPS10 {
                throw_error();
            }
            p.set_phi(0.);
        } else {
            p.set_phi(
                acos(sqrt(rh2 * proj.one_es / (1. - proj.es * rh2)))
                    * (if ortho.mode == ProjMode::NPole { 1. } else { -1. }),
            );
        }
        p.set_lam(atan2(x, y * (if ortho.mode == ProjMode::NPole { -1. } else { 1. })));
        return;
    }

    if ortho.mode == ProjMode::Equit {
        // Equatorial case. Forward case equations can be simplified as:
        // x = nu * cosphi * sinlam
        // y  = nu * sinphi * (1 - proj.es)
        // x^2 * (1 - es * sinphi^2) = (1 - sinphi^2) * sinlam^2
        // y^2 / ((1 - es)^2 + y^2 * es) = sinphi^2

        // Equation of the ellipse
        if sq(x) + sq(y * (proj.a / proj.b)) > 1. + 1.0e-11 {
            throw_error();
        }

        let sinphi2 = if y == 0. { 0. } else { 1.0 / (sq((1. - proj.es) / y) + proj.es) };
        if sinphi2 > 1. - 1e-11 {
            p.set_phi(FRAC_PI_2 * (if y > 0. { 1. } else { -1. }));
            p.set_lam(0.);
            return;
        }
        p.set_phi(asin(sqrt(sinphi2)) * (if y > 0. { 1. } else { -1. }));
        let sinlam = x * sqrt((1. - proj.es * sinphi2) / (1. - sinphi2));
        if fabs(sinlam) - 1. > -1e-15 {
            p.set_lam(FRAC_PI_2 * (if x > 0. { 1. } else { -1. }));
        } else {
            p.set_lam(asin(sinlam));
        }
        return;
    }

    // Using ortho.sinph0 * sinphi + ortho.cosph0 * cosphi * coslam == 0 (visibity
    // condition of the forward case) in the forward equations, and a lot of
    // substitution games...
    // PJ_XY xy_recentered;
    let mut xy_recentered = Coords::default();
    xy_recentered.set_x(x);
    xy_recentered.set_y((y - ortho.y_shift) / ortho.y_scale);
    if sq(x) + sq(xy_recentered.y()) > 1. + 1e-11 {
        throw_error();
    }

    // From EPSG guidance note 7.2, March 2020, §3.3.5 Orthographic

    // It suggests as initial guess:
    // lp.lam = 0;
    // lp.phi = proj.phi0;
    // But for poles, this will not converge well. Better use:
    ortho_s_inverse(ortho, proj, &mut xy_recentered);
    p.set_x(xy_recentered.x());
    p.set_y(xy_recentered.y() * ortho.y_scale + ortho.y_shift);

    for _ in 0..20 {
        let cosphi = cos(p.phi());
        let sinphi = sin(p.phi());
        let coslam = cos(p.lam());
        let sinlam = sin(p.lam());
        let one_minus_es_sinphi2 = 1.0 - proj.es * sinphi * sinphi;
        let nu = 1.0 / sqrt(one_minus_es_sinphi2);
        let mut xy_new = Coords::default();
        xy_new.set_x(nu * cosphi * sinlam);
        xy_new.set_y(
            nu * (sinphi * ortho.cosph0 - cosphi * ortho.sinph0 * coslam)
                + proj.es * (ortho.nu0 * ortho.sinph0 - nu * sinphi) * ortho.cosph0,
        );
        let rho = (1.0 - proj.es) * nu / one_minus_es_sinphi2;
        let j11 = -rho * sinphi * sinlam;
        let j12 = nu * cosphi * coslam;
        let j21 = rho * (cosphi * ortho.cosph0 + sinphi * ortho.sinph0 * coslam);
        let j22 = nu * ortho.sinph0 * cosphi * sinlam;
        let d = j11 * j22 - j12 * j21;
        let dx = x - xy_new.x();
        let dy = y - xy_new.y();
        let dphi = (j22 * dx - j12 * dy) / d;
        let dlam = (-j21 * dx + j11 * dy) / d;
        p.set_phi(p.phi() + dphi);
        if p.phi() > FRAC_PI_2 {
            p.set_phi(FRAC_PI_2 - (p.phi() - FRAC_PI_2));
            p.set_lam(adjlon(p.lam() + PI));
        } else if p.phi() < -FRAC_PI_2 {
            p.set_phi(-FRAC_PI_2 + (-FRAC_PI_2 - p.phi()));
            p.set_lam(adjlon(p.lam() + PI));
        }
        p.set_lam(p.lam() + dlam);
        if fabs(dphi) < 1e-12 && fabs(dlam) < 1e-12 {
            return;
        }
    }
    throw_error();
}
