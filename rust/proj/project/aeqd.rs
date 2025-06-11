use crate::proj::{
    AZIMUTHAL_EQUIDISTANT, CoordinateStep, EPS10, GUAM, GeodGeodesic, Proj, ProjMethod, ProjMode,
    ProjectCoordinates, TransformCoordinates, aasin, enfn, geod_direct, geod_init, geod_inverse,
    inv_mlfn, mlfn,
};
use alloc::vec::Vec;
use core::{
    cell::RefCell,
    f64::consts::{FRAC_PI_2, PI},
};
use libm::{acos, atan2, cos, fabs, hypot, sin, sqrt, tan};

// /******************************************************************************
//  * Project:  PROJ.4
//  * Purpose:  Implementation of the aeqd (Azimuthal Equidistant) projection.
//  * Author:   Gerald Evenden
//  *
//  ******************************************************************************
//  * Copyright (c) 1995, Gerald Evenden
//  *
//  * Permission is hereby granted, free of charge, to any person obtaining a
//  * copy of this software and associated documentation files (the "Software"),
//  * to deal in the Software without restriction, including without limitation
//  * the rights to use, copy, modify, merge, publish, distribute, sublicense,
//  * and/or sell copies of the Software, and to permit persons to whom the
//  * Software is furnished to do so, subject to the following conditions:
//  *
//  * The above copyright notice and this permission notice shall be included
//  * in all copies or substantial portions of the Software.
//  *
//  * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
//  * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//  * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
//  * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//  * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
//  * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
//  * DEALINGS IN THE SOFTWARE.
//  *****************************************************************************/
/// Azimuthal Equidistant Variables
#[derive(Debug, Default, Clone, PartialEq)]
pub struct AeqdData {
    sinph0: f64,
    cosph0: f64,
    en: Vec<f64>,
    m1: f64,
    n1: f64,
    mp: f64,
    he: f64,
    _g: f64,
    mode: ProjMode,
    g: GeodGeodesic,
}

const TOL: f64 = 1e-14;

/// Azimuthal Equidistant Projection
#[derive(Debug, Clone, PartialEq)]
pub struct AzimuthalEquidistantProjection {
    proj: RefCell<Proj>,
    store: RefCell<AeqdData>,
    method: ProjMethod,
    guam: bool,
}
impl ProjectCoordinates for AzimuthalEquidistantProjection {
    fn code(&self) -> i64 {
        AZIMUTHAL_EQUIDISTANT
    }
    fn name(&self) -> &'static str {
        "Azimuthal Equidistant"
    }
    fn names() -> &'static [&'static str] {
        &["Azimuthal Equidistant", "aeqd", "guam"]
    }
}
impl CoordinateStep for AzimuthalEquidistantProjection {
    fn new(proj: RefCell<Proj>) -> Self {
        let mut store = AeqdData::default();
        let mut method = ProjMethod::Spheroidal;
        let mut guam = false;
        {
            let proj = &mut proj.borrow_mut();

            geod_init(&mut store.g, 1., proj.f);

            if fabs(fabs(proj.phi0) - FRAC_PI_2) < EPS10 {
                store.mode = if proj.phi0 < 0. { ProjMode::SPole } else { ProjMode::NPole };
                store.sinph0 = if proj.phi0 < 0. { -1. } else { 1. };
                store.cosph0 = 0.;
            } else if fabs(proj.phi0) < EPS10 {
                store.mode = ProjMode::Equit;
                store.sinph0 = 0.;
                store.cosph0 = 1.;
            } else {
                store.mode = ProjMode::Obliq;
                store.sinph0 = sin(proj.phi0);
                store.cosph0 = cos(proj.phi0);
            }
            if proj.es == 0.0 {
                method = ProjMethod::Spheroidal;
            } else {
                store.en = enfn(proj.n);
                if proj.params.contains_key(&GUAM) {
                    store.m1 = mlfn(proj.phi0, store.sinph0, store.cosph0, &store.en);
                    guam = true;
                } else {
                    match store.mode {
                        ProjMode::NPole => {
                            store.mp = mlfn(FRAC_PI_2, 1., 0., &store.en);
                        }
                        ProjMode::SPole => {
                            store.mp = mlfn(-FRAC_PI_2, -1., 0., &store.en);
                        }
                        ProjMode::Equit | ProjMode::Obliq => {
                            store.n1 = 1. / sqrt(1. - proj.es * store.sinph0 * store.sinph0);
                            store.he = proj.e / sqrt(proj.one_es);
                            store._g = store.sinph0 * store.he;
                            store.he *= store.cosph0;
                        }
                    }
                    method = ProjMethod::Ellipsoidal;
                }
            }
        }

        AzimuthalEquidistantProjection { proj, store: store.into(), method, guam }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        if self.guam {
            e_guam_fwd(&self.store.borrow(), &self.proj.borrow(), p);
        } else if self.method == ProjMethod::Ellipsoidal {
            aeqd_e_forward(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
        } else {
            aeqd_s_forward(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
        }
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        if self.guam {
            e_guam_inv(&self.store.borrow(), &self.proj.borrow(), p);
        } else if self.method == ProjMethod::Ellipsoidal {
            aeqd_e_inverse(&mut self.store.borrow_mut(), &self.proj.borrow(), p);
        } else {
            aeqd_s_inverse(&self.store.borrow(), &self.proj.borrow(), p);
        }
    }
}

/// Guam Ellipsoidal forward project
pub fn e_guam_fwd<P: TransformCoordinates>(aeqd: &AeqdData, proj: &Proj, p: &mut P) {
    let cosphi = cos(p.phi());
    let sinphi = sin(p.phi());
    let t = 1. / sqrt(1. - proj.es * sinphi * sinphi);
    let x = p.lam() * cosphi * t;
    let y = mlfn(p.phi(), sinphi, cosphi, &aeqd.en) - aeqd.m1
        + 0.5 * p.lam() * p.lam() * cosphi * sinphi * t;

    p.set_x(x);
    p.set_y(y);
}

/// Azimuthal Equidistant Ellipsoidal forward project
pub fn aeqd_e_forward<P: TransformCoordinates>(aeqd: &mut AeqdData, proj: &Proj, p: &mut P) {
    let x;
    let y;
    let mut coslam = cos(p.lam());
    match aeqd.mode {
        ProjMode::NPole => {
            coslam = -coslam;
            let cosphi = cos(p.phi());
            let sinphi = sin(p.phi());
            let rho = fabs(aeqd.mp - mlfn(p.phi(), sinphi, cosphi, &aeqd.en));
            x = rho * sin(p.lam());
            y = rho * coslam;
        }
        ProjMode::SPole => {
            let cosphi = cos(p.phi());
            let sinphi = sin(p.phi());
            let rho = fabs(aeqd.mp - mlfn(p.phi(), sinphi, cosphi, &aeqd.en));
            x = rho * sin(p.lam());
            y = rho * coslam;
        }
        ProjMode::Equit | ProjMode::Obliq => {
            if fabs(p.lam()) < EPS10 && fabs(p.phi() - proj.phi0) < EPS10 {
                x = 0.;
                y = 0.;
            } else {
                let lat1 = proj.phi0.to_degrees();
                let lon1 = 0.;
                let lat2 = p.phi().to_degrees();
                let lon2 = p.lam().to_degrees();

                let mut s12 = 0.0;
                let mut azi1: f64 = 0.;
                let mut azi2 = 0.;
                geod_inverse(&mut aeqd.g, lat1, lon1, lat2, lon2, &mut s12, &mut azi1, &mut azi2);
                azi1 = azi1.to_radians();
                x = s12 * sin(azi1);
                y = s12 * cos(azi1);
            }
        }
    }

    p.set_x(x);
    p.set_y(y);
}

/// Azimuthal Equidistant Spheroidal forward project
pub fn aeqd_s_forward<P: TransformCoordinates>(aeqd: &mut AeqdData, proj: &Proj, p: &mut P) {
    let x;
    let mut y;
    if aeqd.mode == ProjMode::Equit {
        let cosphi = cos(p.phi());
        let sinphi = sin(p.phi());
        let coslam = cos(p.lam());
        let sinlam = sin(p.lam());

        y = cosphi * coslam;

        if fabs(fabs(y) - 1.) < TOL {
            if y < 0. {
                panic!("Coordinate outside projection domain");
            } else {
                aeqd_e_forward(aeqd, proj, p);
                return;
            }
        } else {
            y = acos(y);
            y /= sin(y);
            x = y * cosphi * sinlam;
            y *= sinphi;
        }
    } else if aeqd.mode == ProjMode::Obliq {
        let cosphi = cos(p.phi());
        let sinphi = sin(p.phi());
        let coslam = cos(p.lam());
        let sinlam = sin(p.lam());
        let cosphi_x_coslam = cosphi * coslam;

        y = aeqd.sinph0 * sinphi + aeqd.cosph0 * cosphi_x_coslam;

        if fabs(fabs(y) - 1.) < TOL {
            if y < 0. {
                panic!("Coordinate outside projection domain");
            } else {
                aeqd_e_forward(aeqd, proj, p);
                return;
            }
        } else {
            y = acos(y);
            y /= sin(y);
            x = y * cosphi * sinlam;
            y *= aeqd.cosph0 * sinphi - aeqd.sinph0 * cosphi_x_coslam;
        }
    } else {
        let mut coslam = cos(p.lam());
        let sinlam = sin(p.lam());
        if aeqd.mode == ProjMode::NPole {
            p.set_phi(-p.phi());
            coslam = -coslam;
        }
        if fabs(p.phi() - FRAC_PI_2) < EPS10 {
            panic!("Coordinate outside projection domain");
        }
        y = FRAC_PI_2 + p.phi();
        x = y * sinlam;
        y *= coslam;
    }

    p.set_x(x);
    p.set_y(y);
}

/// Guam Ellipsoidal inverse project
pub fn e_guam_inv<P: TransformCoordinates>(aeqd: &AeqdData, proj: &Proj, p: &mut P) {
    let x = p.x();
    let y = p.y();
    let x2 = 0.5 * x * x;
    let mut t = 0.;
    p.set_phi(proj.phi0);
    for _ in 0..3 {
        t = proj.e * sin(p.phi());
        t = sqrt(1. - t * t);
        p.set_phi(inv_mlfn(aeqd.m1 + y - x2 * tan(p.phi()) * t, &aeqd.en));
    }
    p.set_lam(x * t / cos(p.phi()));
}

/// Azimuthal Equidistant Ellipsoidal inverse project
pub fn aeqd_e_inverse<P: TransformCoordinates>(aeqd: &mut AeqdData, proj: &Proj, p: &mut P) {
    let x = p.x();
    let y = p.y();
    let s12 = hypot(x, y);
    if s12 < EPS10 {
        p.set_phi(proj.phi0);
        p.set_lam(0.);
        return;
    }
    if aeqd.mode == ProjMode::Obliq || aeqd.mode == ProjMode::Equit {
        let lat1 = proj.phi0.to_degrees();
        let lon1 = 0.;
        let azi1 = atan2(x, y).to_degrees(); // Clockwise from north
        let mut lat2 = 0.;
        let mut lon2 = 0.;
        let mut azi2 = 0.;
        geod_direct(&mut aeqd.g, lat1, lon1, azi1, s12, &mut lat2, &mut lon2, &mut azi2);
        p.set_phi(lat2.to_radians());
        p.set_lam(lon2.to_radians());
    } else {
        // Polar
        p.set_phi(inv_mlfn(
            if aeqd.mode == ProjMode::NPole { aeqd.mp - s12 } else { aeqd.mp + s12 },
            &aeqd.en,
        ));
        p.set_lam(atan2(x, if aeqd.mode == ProjMode::NPole { -y } else { y }));
    }
}

/// Azimuthal Equidistant Spheroidal inverse project
pub fn aeqd_s_inverse<P: TransformCoordinates>(aeqd: &AeqdData, proj: &Proj, p: &mut P) {
    let mut x = p.x();
    let mut y = p.y();
    let lam;
    let phi;
    let mut c_rh = hypot(x, y);
    if c_rh > PI {
        if c_rh - EPS10 > PI {
            panic!("Coordinate outside projection domain");
        }
        c_rh = PI;
    } else if c_rh < EPS10 {
        phi = proj.phi0;
        lam = 0.;
        p.set_phi(phi);
        p.set_lam(lam);
        return;
    }
    if aeqd.mode == ProjMode::Obliq || aeqd.mode == ProjMode::Equit {
        let sinc = sin(c_rh);
        let cosc = cos(c_rh);
        if aeqd.mode == ProjMode::Equit {
            phi = aasin(y * sinc / c_rh);
            x *= sinc;
            y = cosc * c_rh;
        } else {
            phi = aasin(cosc * aeqd.sinph0 + y * sinc * aeqd.cosph0 / c_rh);
            y = (cosc - aeqd.sinph0 * sin(phi)) * c_rh;
            x *= sinc * aeqd.cosph0;
        }
        lam = if y == 0. { 0. } else { atan2(x, y) };
    } else if aeqd.mode == ProjMode::NPole {
        phi = FRAC_PI_2 - c_rh;
        lam = atan2(x, -y);
    } else {
        phi = c_rh - FRAC_PI_2;
        lam = atan2(x, y);
    }

    p.set_lam(lam);
    p.set_phi(phi);
}
