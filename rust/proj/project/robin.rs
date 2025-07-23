use crate::proj::{CoordinateStep, Proj, ProjectCoordinates, TransformCoordinates};
use alloc::rc::Rc;
use core::{
    cell::RefCell,
    f64::consts::{FRAC_PI_2, PI},
};
use libm::{fabs, floor, round};

fn v(c: &RobinCoefs, z: f64) -> f64 {
    c.c0 + z * (c.c1 + z * (c.c2 + z * c.c3))
}
fn dv(c: &RobinCoefs, z: f64) -> f64 {
    c.c1 + 2.0 * z * c.c2 + z * z * 3.0 * c.c3
}

// note: following terms based upon 5 deg. intervals in degrees.
//
// Some background on these coefficients is available at:
//
// http://article.gmane.org/gmane.comp.gis.proj-4.devel/6039
// http://trac.osgeo.org/proj/ticket/113

/// Robin coefficients
#[derive(Debug, Default, Clone, PartialEq)]
pub struct RobinCoefs {
    /// First coefficient
    pub c0: f64,
    /// Second coefficient
    pub c1: f64,
    /// Third coefficient
    pub c2: f64,
    /// Fourth coefficient
    pub c3: f64,
}
const X_COEFS: [RobinCoefs; 19] = [
    RobinCoefs { c0: 1.0, c1: 2.2199e-17, c2: -7.15515e-05, c3: 3.1103e-06 },
    RobinCoefs { c0: 0.9986, c1: -0.000482243, c2: -2.4897e-05, c3: -1.3309e-06 },
    RobinCoefs { c0: 0.9954, c1: -0.00083103, c2: -4.48605e-05, c3: -9.86701e-07 },
    RobinCoefs { c0: 0.99, c1: -0.00135364, c2: -5.9661e-05, c3: 3.6777e-06 },
    RobinCoefs { c0: 0.9822, c1: -0.00167442, c2: -4.49547e-06, c3: -5.72411e-06 },
    RobinCoefs { c0: 0.973, c1: -0.00214868, c2: -9.03571e-05, c3: 1.8736e-08 },
    RobinCoefs { c0: 0.96, c1: -0.00305085, c2: -9.00761e-05, c3: 1.64917e-06 },
    RobinCoefs { c0: 0.9427, c1: -0.00382792, c2: -6.53386e-05, c3: -2.6154e-06 },
    RobinCoefs { c0: 0.9216, c1: -0.00467746, c2: -0.00010457, c3: 4.81243e-06 },
    RobinCoefs { c0: 0.8962, c1: -0.00536223, c2: -3.23831e-05, c3: -5.43432e-06 },
    RobinCoefs { c0: 0.8679, c1: -0.00609363, c2: -0.000113898, c3: 3.32484e-06 },
    RobinCoefs { c0: 0.835, c1: -0.00698325, c2: -6.40253e-05, c3: 9.34959e-07 },
    RobinCoefs { c0: 0.7986, c1: -0.00755338, c2: -5.00009e-05, c3: 9.35324e-07 },
    RobinCoefs { c0: 0.7597, c1: -0.00798324, c2: -3.5971e-05, c3: -2.27626e-06 },
    RobinCoefs { c0: 0.7186, c1: -0.00851367, c2: -7.01149e-05, c3: -8.6303e-06 },
    RobinCoefs { c0: 0.6732, c1: -0.00986209, c2: -0.000199569, c3: 1.91974e-05 },
    RobinCoefs { c0: 0.6213, c1: -0.010418, c2: 8.83923e-05, c3: 6.24051e-06 },
    RobinCoefs { c0: 0.5722, c1: -0.00906601, c2: 0.000182, c3: 6.24051e-06 },
    RobinCoefs { c0: 0.5322, c1: -0.00677797, c2: 0.000275608, c3: 6.24051e-06 },
];

const Y_COEFS: [RobinCoefs; 19] = [
    RobinCoefs { c0: -5.20417e-18, c1: 0.0124, c2: 1.21431e-18, c3: -8.45284e-11 },
    RobinCoefs { c0: 0.062, c1: 0.0124, c2: -1.26793e-09, c3: 4.22642e-10 },
    RobinCoefs { c0: 0.124, c1: 0.0124, c2: 5.07171e-09, c3: -1.60604e-09 },
    RobinCoefs { c0: 0.186, c1: 0.0123999, c2: -1.90189e-08, c3: 6.00152e-09 },
    RobinCoefs { c0: 0.248, c1: 0.0124002, c2: 7.10039e-08, c3: -2.24e-08 },
    RobinCoefs { c0: 0.31, c1: 0.0123992, c2: -2.64997e-07, c3: 8.35986e-08 },
    RobinCoefs { c0: 0.372, c1: 0.0124029, c2: 9.88983e-07, c3: -3.11994e-07 },
    RobinCoefs { c0: 0.434, c1: 0.0123893, c2: -3.69093e-06, c3: -4.35621e-07 },
    RobinCoefs { c0: 0.4958, c1: 0.0123198, c2: -1.02252e-05, c3: -3.45523e-07 },
    RobinCoefs { c0: 0.5571, c1: 0.0121916, c2: -1.54081e-05, c3: -5.82288e-07 },
    RobinCoefs { c0: 0.6176, c1: 0.0119938, c2: -2.41424e-05, c3: -5.25327e-07 },
    RobinCoefs { c0: 0.6769, c1: 0.011713, c2: -3.20223e-05, c3: -5.16405e-07 },
    RobinCoefs { c0: 0.7346, c1: 0.0113541, c2: -3.97684e-05, c3: -6.09052e-07 },
    RobinCoefs { c0: 0.7903, c1: 0.0109107, c2: -4.89042e-05, c3: -1.04739e-06 },
    RobinCoefs { c0: 0.8435, c1: 0.0103431, c2: -6.4615e-05, c3: -1.40374e-09 },
    RobinCoefs { c0: 0.8936, c1: 0.00969686, c2: -6.4636e-05, c3: -8.547e-06 },
    RobinCoefs { c0: 0.9394, c1: 0.00840947, c2: -0.000192841, c3: -4.2106e-06 },
    RobinCoefs { c0: 0.9761, c1: 0.00616527, c2: -0.000256, c3: -4.2106e-06 },
    RobinCoefs { c0: 1.0, c1: 0.00328947, c2: -0.000319159, c3: -4.2106e-06 },
];

const FXC: f64 = 0.8487;
const FYC: f64 = 1.3523;
const C1: f64 = 11.459_155_902_616_464;
const RC1: f64 = 0.087_266_462_599_716_47;
const NODES: usize = 18;
const ONEEPS: f64 = 1.000001;
const EPS: f64 = 1e-10;
// Not sure at all of the appropriate number for MAX_ITER...
const MAX_ITER: usize = 100;

/// # Robinson
///
/// **Classification**: Pseudocylindrical
///
/// **Available forms**: Forward and inverse, spherical projection
///
/// **Defined area**: Global
///
/// **Alias**: robin
///
/// **Domain**: 2D
///
/// **Input type**: Geodetic coordinates
///
/// **Output type**: Projected coordinates
///
/// ## Projection String
/// ```ini
/// +proj=robin
/// ```
///
/// ## Required Parameters
/// - None
///
/// ## Optional Parameters
/// - `+lon_0=<value>`: Central meridian.
/// - `+R=<value>`: Radius of the projection sphere.
/// - `+x_0=<value>`: False easting.
/// - `+y_0=<value>`: False northing.
///
/// ![Robinson](https://github.com/Open-S2/gis-tools/blob/master/assets/proj4/projections/images/robin.png?raw=true)
#[derive(Debug, Clone, PartialEq)]
pub struct RobinsonProjection {
    proj: Rc<RefCell<Proj>>,
}
impl ProjectCoordinates for RobinsonProjection {
    fn code(&self) -> i64 {
        -1
    }
    fn name(&self) -> &'static str {
        "Robinson"
    }
    fn names() -> &'static [&'static str] {
        &["Robinson", "robin"]
    }
}
impl CoordinateStep for RobinsonProjection {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        proj.borrow_mut().es = 0.;
        RobinsonProjection { proj }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        robin_s_forward(p);
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        robin_s_inverse(p);
    }
}

/// Equal Earth Spheroidal forward project
pub fn robin_s_forward<P: TransformCoordinates>(p: &mut P) {
    let mut dphi = fabs(p.phi());
    let mut i = if f64::is_nan(p.phi()) { -1. } else { round(floor(dphi * C1 + 1e-15)) };
    if i < 0. {
        panic!("Coordinate outside projection domain");
    }
    if i >= NODES as f64 {
        i = NODES as f64;
    }
    dphi = (dphi - RC1 * i).to_degrees();
    let x = v(&X_COEFS[i as usize], dphi) * FXC * p.lam();
    let mut y = v(&Y_COEFS[i as usize], dphi) * FYC;
    if p.phi() < 0. {
        y = -y;
    }
    p.set_x(x);
    p.set_y(y);
}

/// Equal Earth Spheroidal inverse project
pub fn robin_s_inverse<P: TransformCoordinates>(p: &mut P) {
    let mut lam = p.x() / FXC;
    let mut phi = fabs(p.y() / FYC);
    if phi >= 1. {
        // simple pathologic cases
        if phi > ONEEPS {
            panic!("Coordinate outside projection domain");
        } else {
            phi = if p.y() < 0. { -FRAC_PI_2 } else { FRAC_PI_2 };
            lam /= X_COEFS[NODES].c0;
        }
    } else {
        // general problem
        // in Y space, reduce to table interval
        let i = if f64::is_nan(phi) { -1. } else { round(floor(phi * NODES as f64)) };
        if i < 0. || i >= NODES as f64 {
            panic!("Coordinate outside projection domain");
        }
        let mut i = i as usize;
        loop {
            if Y_COEFS[i].c0 > phi {
                i -= 1;
            } else if Y_COEFS[i + 1].c0 <= phi {
                i += 1;
            } else {
                break;
            }
        }
        let t_coef = &Y_COEFS[i];
        // first guess, linear interp
        let mut t = 5. * (phi - t_coef.c0) / (Y_COEFS[i + 1].c0 - t_coef.c0);
        let mut iters = MAX_ITER;
        while iters > 0 {
            // Newton-Raphson
            let t1 = (v(t_coef, t) - phi) / dv(t_coef, t);
            t -= t1;
            if fabs(t1) < EPS {
                break;
            }
            iters -= 1;
        }
        if iters == 0 {
            panic!("Coordinate outside projection domain");
        }
        phi = (5. * (i as f64) + t).to_radians();
        if p.y() < 0. {
            phi = -phi;
        }
        lam /= v(&X_COEFS[i], t);
        if fabs(lam) > PI {
            panic!("Coordinate outside projection domain");
        }
    }
    p.set_phi(phi);
    p.set_lam(lam);
}
