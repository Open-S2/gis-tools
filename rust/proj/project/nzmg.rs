use crate::proj::{
    Complex, CoordinateStep, Proj, ProjectCoordinates, TransformCoordinates, zpoly1, zpolyd1,
};
use core::cell::RefCell;
use libm::fabs;

// /******************************************************************************
//  * Project:  PROJ.4
//  * Purpose:  Implementation of the nzmg (New Zealand Map Grid) projection.
//  *           Very loosely based upon DMA code by Bradford W. Drew
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
const EPSLN: f64 = 1e-10;
const SEC5_TO_RAD: f64 = 0.484_813_681_109_536;
const RAD_TO_SEC5: f64 = 2.062_648_062_470_963_8;
const N_BF: usize = 5;
const N_TPSI: usize = 9;
const N_TPHI: usize = 8;
const BF: [Complex; 6] = [
    Complex { r: 0.7557853228, i: 0.0 },
    Complex { r: 0.249204646, i: 0.003371507 },
    Complex { r: -0.001541739, i: 0.041058560 },
    Complex { r: -0.10162907, i: 0.01727609 },
    Complex { r: -0.26623489, i: -0.36249218 },
    Complex { r: -0.6870983, i: -1.1651967 },
];

/// New Zealand Map Grid Projection
#[derive(Debug, Clone, PartialEq)]
pub struct NewZealandMapGridProjection {
    proj: RefCell<Proj>,
}
impl ProjectCoordinates for NewZealandMapGridProjection {
    fn code(&self) -> i64 {
        -1
    }
    fn name(&self) -> &'static str {
        "New Zealand Map Grid"
    }
    fn names() -> &'static [&'static str] {
        &["New Zealand Map Grid", "nzmg"]
    }
}
impl CoordinateStep for NewZealandMapGridProjection {
    fn new(proj: RefCell<Proj>) -> Self {
        {
            let proj = &mut proj.borrow_mut();
            // force to International major axis
            proj.a = 6378388.0;
            proj.ra = 1. / proj.a;
            proj.lam0 = 173.0_f64.to_radians();
            proj.phi0 = -41.0_f64.to_radians();
            proj.x0 = 2510000.;
            proj.y0 = 6023150.;
        }
        NewZealandMapGridProjection { proj }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        nzmg_e_forward(&self.proj.borrow(), p);
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        nzmg_e_inverse(&self.proj.borrow(), p);
    }
}

/// New Zealand Map Grid Ellipsoidal forward project
pub fn nzmg_e_forward<P: TransformCoordinates>(proj: &Proj, p: &mut P) {
    let mut c_p = Complex::default();
    let tpsi: [f64; 10] = [
        0.6399175073,
        -0.1358797613,
        0.063294409,
        -0.02526853,
        0.0117879,
        -0.0055161,
        0.0026906,
        -0.001333,
        0.00067,
        -0.00034,
    ];

    p.set_phi((p.phi() - proj.phi0) * RAD_TO_SEC5);
    let mut i = N_TPSI;
    while i > 0 {
        i -= 1;
        c_p.r = tpsi[i] + p.phi() * c_p.r;
    }
    c_p.r *= p.phi();
    c_p.i = p.lam();
    c_p = zpoly1(c_p, &BF, N_BF);
    p.set_x(c_p.i);
    p.set_y(c_p.r);
}

/// New Zealand Map Grid Ellipsoidal inverse project
pub fn nzmg_e_inverse<P: TransformCoordinates>(proj: &Proj, p: &mut P) {
    let mut c_p = Complex::default();
    let mut fp = Complex::default();
    let mut dp = Complex::default();

    let tphi: [f64; 9] = [
        1.5627014243,
        0.5185406398,
        -0.03333098,
        -0.1052906,
        -0.0368594,
        0.007317,
        0.01220,
        0.00394,
        -0.0013,
    ];

    c_p.r = p.y();
    c_p.i = p.x();
    let mut nn: usize = 20;
    while nn > 0 {
        nn -= 1;
        let mut f = zpolyd1(c_p, &BF, N_BF, &mut fp);
        f.r -= p.y();
        f.i -= p.x();
        let den = fp.r * fp.r + fp.i * fp.i;
        dp.r = -(f.r * fp.r + f.i * fp.i) / den;
        dp.i = -(f.i * fp.r - f.r * fp.i) / den;
        c_p.r += dp.r;
        c_p.i += dp.i;
        if (fabs(dp.r) + fabs(dp.i)) <= EPSLN {
            break;
        }
    }
    if nn != 0 {
        p.set_lam(c_p.i);
        let mut phi = 0.0;
        let mut i = N_TPHI;
        while i > 0 {
            i -= 1;
            phi = tphi[i] + c_p.r * phi;
        }
        p.set_phi(proj.phi0 + c_p.r * phi * SEC5_TO_RAD);
    } else {
        p.set_lam(f64::MAX);
        p.set_phi(f64::MAX);
    }
}
