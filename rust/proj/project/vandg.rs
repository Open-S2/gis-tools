use crate::proj::{CoordinateStep, Proj, ProjectCoordinates, TransformCoordinates};
use alloc::rc::Rc;
use core::{
    cell::RefCell,
    f64::consts::{FRAC_PI_2, PI, TAU},
};
use libm::{acos, asin, cos, fabs, sqrt, tan};

// Changes to handle +over are: Copyright 2011-2014 Morelli Informatik
const TOL: f64 = 1e-10;
const THIRD: f64 = 0.333_333_333_333_333_3;
const C2_27: f64 = 0.074_074_074_074_074_07; // 2/27
const PI4_3: f64 = 4.188_790_204_786_391; // 4*pi/3
const PISQ: f64 = 9.869_604_401_089_358; // pi^2
const TPISQ: f64 = 19.739_208_802_178_716; // 2*pi^2
const HPISQ: f64 = 4.934_802_200_544_679; // pi^2/2

/// Van der Grinten (I) Projection
#[derive(Debug, Clone, PartialEq)]
pub struct VanDerGrintenIProjection {
    proj: Rc<RefCell<Proj>>,
}
impl ProjectCoordinates for VanDerGrintenIProjection {
    fn code(&self) -> i64 {
        -1
    }
    fn name(&self) -> &'static str {
        "van der Grinten (I)"
    }
    fn names() -> &'static [&'static str] {
        &["van der Grinten", "VanDerGrinten", "Van_der_Grinten_I", "van der Grinten (I)", "vandg"]
    }
}
impl CoordinateStep for VanDerGrintenIProjection {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        {
            let proj = &mut proj.borrow_mut();
            proj.es = 0.;
        }
        VanDerGrintenIProjection { proj }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        vandg_s_forward(&self.proj.borrow(), p);
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        vandg_s_inverse(p);
    }
}

/// Van der Grinten (I) Spheroidal forward project
pub fn vandg_s_forward<P: TransformCoordinates>(proj: &Proj, p: &mut P) {
    // Comments tie this formulation to Snyder (1987), p. 241.
    let mut x;
    let mut y;
    let mut p2 = fabs(p.phi() / FRAC_PI_2); // sin(theta) from (29-6)
    if (p2 - TOL) > 1. {
        panic!("Coordinate outside projection domain");
    }
    let mut sign = 1;
    if proj.over && fabs(p.lam()) > PI {
        sign = -1;
    }
    if p2 > 1. {
        p2 = 1.;
    }
    if fabs(p.phi()) <= TOL {
        x = p.lam();
        y = 0.;
    } else if fabs(p.lam()) <= TOL || fabs(p2 - 1.) < TOL {
        x = 0.;
        y = PI * tan(0.5 * asin(p2));
        if p.phi() < 0. {
            y = -y;
        }
    } else {
        let al = 0.5 * (sign as f64) * fabs(PI / p.lam() - p.lam() / PI); // A from (29-3)
        let al2 = al * al; // A^2
        let mut g = sqrt(1. - p2 * p2); // cos(theta)
        g = g / (p2 + g - 1.); // G from (29-4)
        let g2 = g * g; // G^2
        p2 = g * (2. / p2 - 1.); // P from (29-5)
        p2 = p2 * p2; // P^2
        x = g - p2; // G - P^2
        g = p2 + al2; // P^2 + A^2
        // (29-1)
        x = PI * fabs(al * x + sqrt(al2 * x * x - g * (g2 - p2))) / g;
        if p.lam() < 0. {
            x = -x;
        }
        y = fabs(x / PI);
        // y from (29-2) has been expressed in terms of x here
        y = 1. - y * (y + 2. * al);
        if y < -TOL {
            panic!("Coordinate outside projection domain");
        }
        if y < 0. {
            y = 0.;
        } else {
            y = sqrt(y) * (if p.phi() < 0. { -PI } else { PI });
        }
    }

    p.set_x(x);
    p.set_y(y);
}

/// Van der Grinten (I) Spheroidal forward project
pub fn vandg_s_inverse<P: TransformCoordinates>(p: &mut P) {
    // static PJ_LP vandg_s_inverse(PJ_XY xy, PJ *P) { /* Spheroidal, inverse */
    //     PJ_LP lp = {0.0, 0.0};
    //     double t, c0, c1, c2, c3, al, r2, r, m, d, ay, x2, y2;
    // Comments tie this formulation to Snyder (1987), p. 242.
    let x2 = p.x() * p.x(); // pi^2 * X^2
    let ay = fabs(p.y());
    if ay < TOL {
        p.set_phi(0.);
        let t = x2 * x2 + TPISQ * (x2 + HPISQ);
        p.set_lam(if fabs(p.x()) <= TOL { 0. } else { 0.5 * (x2 - PISQ + sqrt(t)) / p.x() });
        return;
    }
    let y2 = p.y() * p.y(); // pi^2 * Y^2
    let r = x2 + y2; // pi^2 * (X^2+Y^2)
    let r2 = r * r; // pi^4 * (X^2+Y^2)^2
    let c1 = -PI * ay * (r + PISQ); // pi^4 * c1 (29-11)
    // pi^4 * c3 (29-13)
    let c3 = r2 + TAU * (ay * r + PI * (y2 + PI * (ay + FRAC_PI_2)));
    let mut c2 = c1 + PISQ * (r - 3. * y2); // pi^4 * c2 (29-12)
    let c0 = PI * ay; // pi^2 * Y
    c2 /= c3; // c2/c3
    let al = c1 / c3 - THIRD * c2 * c2; // a1 (29-15)
    let m = 2. * sqrt(-THIRD * al); // m1 (29-16)
    let d = C2_27 * c2 * c2 * c2 + (c0 * c0 - THIRD * c2 * c1) / c3; // d (29-14)
    let al_mul_m = al * m; // a1*m1
    if fabs(al_mul_m) < 1e-16 {
        panic!("Coordinate outside projection domain");
    }
    let mut d = 3. * d / al_mul_m; // cos(3*theta1) (29-17)
    let mut t = fabs(d);
    if (t - TOL) <= 1. {
        d = if t > 1. { if d > 0. { 0. } else { PI } } else { acos(d) }; // 3*theta1 (29-17)
        if r > PISQ {
            // This code path is triggered for coordinates generated in the
            // forward path when |long|>180deg and +over
            d = TAU - d;
        }
        // (29-18) but change pi/3 to 4*pi/3 to flip sign of cos
        p.set_phi(PI * (m * cos(d * THIRD + PI4_3) - THIRD * c2));
        if p.y() < 0. {
            p.set_phi(-p.phi());
        }
        t = r2 + TPISQ * (x2 - y2 + HPISQ);
        p.set_lam(if fabs(p.x()) <= TOL {
            0.
        } else {
            0.5 * (r - PISQ + (if t <= 0. { 0. } else { sqrt(t) })) / p.x()
        });
    } else {
        panic!("Coordinate outside projection domain");
    }
}
