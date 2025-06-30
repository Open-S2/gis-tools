use crate::proj::{CoordinateStep, Proj, ProjectCoordinates, TransformCoordinates};
use alloc::rc::Rc;
use core::{cell::RefCell, f64::consts::FRAC_PI_4};
use libm::{atan, exp, log, tan};

/**
 * # Miller Cylindrical
 *
 * The Miller cylindrical projection is a modified Mercator projection, proposed by
 * Osborn Maitland Miller in 1942.
 *
 * **Classification**: Neither conformal nor equal area cylindrical
 *
 * **Available forms**: Forward and inverse spherical
 *
 * **Defined area**: Global, but best used near the equator
 *
 * **Alias**: mill
 *
 * **Domain**: 2D
 *
 * **Input type**: Geodetic coordinates
 *
 * **Output type**: Projected coordinates
 *
 * ## Projection String
 * ```ini
 * +proj=mill
 * ```
 *
 * ## Required Parameters
 * - None
 *
 * ## Optional Parameters
 * - `+lon_0`: Longitude of projection center. Defaults to `0`.
 * - `+R`: Radius of the sphere.
 * - `+x_0`: False easting. Defaults to `0`.
 * - `+y_0`: False northing. Defaults to `0`.
 *
 * ## Usage Example
 * Using Central meridian 90°W:
 * ```bash
 * $ echo -100 35 | proj +proj=mill +lon_0=90w
 * -1113194.91      4061217.24
 * ```
 *
 * ## Mathematical Definition
 * ### Forward projection:
 * $$x = \lambda$$
 * $$y = 1.25 * \ln \left[ \tan \left(\frac{\pi}{4} + 0.4 * \phi \right) \right]$$
 * ### Inverse projection:
 * $$\lambda = x$$
 * $$\phi = 2.5 * ( \arctan \left[ e^{0.8 * y} \right] - \frac{\pi}{4} )$$
 *
 * ## Further reading
 * - [Wikipedia on Miller Cylindrical](https://en.wikipedia.org/wiki/Miller_cylindrical_projection)
 * - "New Equal-Area Map Projections for Noncircular Regions", John P. Snyder, The American Cartographer, Vol 15, No. 4, October 1988, pp. 341-355.
 *
 * ![Miller Cylindrical](https://github.com/Open-S2/gis-tools/blob/master/assets/proj4/projections/images/mill.png?raw=true)
 */
#[derive(Debug, Clone, PartialEq)]
pub struct MillerCylindricalProjection {
    proj: Rc<RefCell<Proj>>,
}
impl ProjectCoordinates for MillerCylindricalProjection {
    fn code(&self) -> i64 {
        -1
    }
    fn name(&self) -> &'static str {
        "Miller Cylindrical"
    }
    fn names() -> &'static [&'static str] {
        &["MillerCylindrical", "Miller Cylindrical", "Miller_Cylindrical", "mill"]
    }
}
impl CoordinateStep for MillerCylindricalProjection {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        proj.borrow_mut().es = 0.;
        MillerCylindricalProjection { proj }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        mill_s_forward(p);
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        mill_s_inverse(p);
    }
}

/// Miller Cylindrical Forward Project
pub fn mill_s_forward<P: TransformCoordinates>(p: &mut P) {
    p.set_x(p.lam());
    p.set_y(log(tan(FRAC_PI_4 + p.phi() * 0.4)) * 1.25);
}

/// Miller Cylindrical Inverse Project
pub fn mill_s_inverse<P: TransformCoordinates>(p: &mut P) {
    p.set_lam(p.x());
    p.set_phi(2.5 * (atan(exp(0.8 * p.y())) - FRAC_PI_4));
}
