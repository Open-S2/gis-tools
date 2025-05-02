use crate::{
    _msfn, CoordinateStep, MERCATOR, Proj, ProjectCoordinates, TransformCoordinates, WEB_MERCATOR,
    sinhpsi2tanphi,
};
use libm::{asinh, atan, atanh, cos, sin, sinh, tan};

/// Mercator type (spherical or ellipsoidal)
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MercatorType {
    /// Ellipsoidal, mercator
    Ellipsoidal,
    /// Spherical, mercator
    Spherical,
}

/// # Mercator Projection
///
/// The Mercator projection is a cylindrical map projection originating from the 16th century.
/// It is widely recognized as the first regularly used map projection. It is a conformal projection
/// where the equator projects to a straight line at constant scale. A rhumb line, or course of
/// constant heading, projects to a straight line, making it suitable for navigational purposes.
///
/// **Classification**: Conformal cylindrical
///
/// **Available forms**: Forward and Inverse, spherical and ellipsoidal
///
/// **Defined area**: Global, but best used near the equator
///
/// **Alias**: `merc`
///
/// **Domain**: 2D
///
/// **Input type**: Geodetic coordinates
///
/// **Output type**: Projected coordinates
///
/// ## Projection String
/// ```ini
/// +proj=merc
/// ```
///
/// ## Usage
/// The Mercator projection is often used for equatorial regions and navigational charts. It is not
/// suitable for world maps due to significant area distortions. For example, Greenland appears
/// larger than South America in the projection, despite Greenland's actual area being approximately
/// one-eighth of South America's.
///
/// **Examples:**
///
/// - Using latitude of true scale:
///   ```bash
///   $ echo 56.35 12.32 | proj +proj=merc +lat_ts=56.5
///   3470306.37    759599.90
///   ```
/// - Using scaling factor:
///   ```bash
///   $ echo 56.35 12.32 | proj +proj=merc +k_0=2
///   12545706.61    2746073.80
///   ```
///
/// **Note**: `+lat_ts` and `+k_0` are mutually exclusive. If both are used, `+lat_ts` takes
/// precedence over `+k_0`.
///
/// ## Parameters
/// - `lat_ts`: Latitude of true scale
/// - `k_0`: Scaling factor
/// - `lon_0`: Longitude of origin
/// - `x_0`: False easting
/// - `y_0`: False northing
/// - `ellps`: Ellipsoid
/// - `R`: Radius of the sphere
///
/// ## Mathematical Definition
///
/// **Spherical Form**
/// - **Forward Projection**:
///   $$x = k_0 \cdot R \cdot \lambda$$
///   $$y = k_0 \cdot R \cdot \psi$$
///   where
///   $$\psi = \ln\left(\tan\left(\frac{\pi}{4} + \frac{\phi}{2}\right)\right)$$
/// - **Inverse Projection**:
///   $$\lambda = x / (k_0 \cdot R)$$
///   $$\psi = y / (k_0 \cdot R)$$
///   $$\phi = \frac{\pi}{2} - 2 \cdot \arctan\left(\exp(-\psi)\right)$$
///
/// **Ellipsoidal Form**
/// - **Forward Projection**:
///   $$x = k_0 \cdot a \cdot \lambda$$
///   $$y = k_0 \cdot a \cdot \psi$$
///   where
///   $$\psi = \ln\left(\tan\left(\frac{\pi}{4} + \frac{\phi}{2}\right)\right) - 0.5 \cdot e \cdot \ln\left(\frac{1 + e \cdot \sin(\phi)}{1 - e \cdot \sin(\phi)}\right)$$
/// - **Inverse Projection**:
///   $$\lambda = x / (k_0 \cdot a)$$
///   $$\psi = y / (k_0 \cdot a)$$
///   $$\phi = \arctan(\tau)$$
///   where
///   $$\tau = \tan(\phi)$$
///
/// ## Further Reading
/// - [Wikipedia: Mercator Projection](https://en.wikipedia.org/wiki/Mercator_projection)
/// - [Wolfram Mathworld: Mercator Projection](http://mathworld.wolfram.com/MercatorProjection.html)
///
/// ![Mercator Projection](https://github.com/Open-S2/gis-tools/blob/master/assets/proj4/projections/images/merc.png?raw=true)
#[derive(Debug)]
pub struct MercatorProjection {
    conv_case: MercatorType,
}
impl ProjectCoordinates for MercatorProjection {
    fn code(&self) -> u32 {
        MERCATOR
    }
    fn name(&self) -> &'static str {
        "Mercator"
    }
    fn names() -> &'static [&'static str] {
        &[
            "Mercator",
            "Popular Visualisation Pseudo Mercator",
            "Mercator_1SP",
            "Mercator_Auxiliary_Sphere",
            "merc",
        ]
    }
}
impl CoordinateStep for MercatorProjection {
    fn new(proj: &mut Proj) -> Self {
        let phits = 0.0;
        let is_phits: bool = false;
        let conv_case: MercatorType;

        // TODO:
        // if ((is_phits = pj_param(proj.ctx, proj.params, "tlat_ts").i)) {
        //     phits = fabs(pj_param(proj.ctx, proj.params, "rlat_ts").f);
        // }
        if phits >= core::f64::consts::FRAC_PI_2 {
            panic!("Invalid value for lat_ts: |lat_ts| should be <= 90°");
        }

        if proj.es != 0.0 {
            // ellipsoid case
            conv_case = MercatorType::Ellipsoidal;
            if is_phits {
                proj.k0 = _msfn(sin(phits), cos(phits), proj.es);
            }
        } else {
            // sphere case
            conv_case = MercatorType::Spherical;
            if is_phits {
                proj.k0 = cos(phits);
            }
        }
        MercatorProjection { conv_case }
    }
    fn forward<P: TransformCoordinates>(&self, proj: &Proj, p: &mut P) {
        if self.conv_case == MercatorType::Spherical {
            merc_s_forward(proj, p);
        } else {
            merc_e_forward(proj, p);
        }
    }
    fn inverse<P: TransformCoordinates>(&self, proj: &Proj, p: &mut P) {
        if self.conv_case == MercatorType::Spherical {
            merc_s_inverse(proj, p);
        } else {
            merc_e_inverse(proj, p);
        }
    }
}

/// # Web Mercator / Pseudo Mercator Projection
///
/// The Web Mercator / Pseudo Mercator projection is a cylindrical map projection.
/// This is a variant of the regular [Mercator](crate::projections::mercator) projection,
/// except that the computation is done on a sphere, using the semi-major axis of the ellipsoid.
///
/// From [Wikipedia](https://en.wikipedia.org/wiki/Web_Mercator):
///
/// > This projection is widely used by the Web Mercator, Google Web Mercator,
/// > Spherical Mercator, WGS 84 Web Mercator[1] or WGS 84/Pseudo-Mercator is a
/// > variant of the Mercator projection and is the de facto standard for Web
/// > mapping applications. [...]
/// > It is used by virtually all major online map providers [...]
/// > Its official EPSG identifier is EPSG:3857, although others have been used
/// > historically.
///
/// **Classification**: Cylindrical (non-conformal if used with an ellipsoid)
///
/// **Available forms**: Forward and Inverse
///
/// **Defined area**: Global
///
/// **Alias**: `webmerc`
///
/// **Domain**: 2D
///
/// **Input type**: Geodetic coordinates
///
/// **Output type**: Projected coordinates
///
/// ## Usage
///
/// ```bash
/// $ echo 2 49 | proj +proj=webmerc +datum=WGS84
/// 222638.98       6274861.39
/// ```
///
/// ## Parameters
///
/// **Note**: All parameters for the projection are optional, except the ellipsoid
/// definition, which is WGS84 for the typical use case of EPSG:3857.
/// In which case, the other parameters are set to their default 0 value.
///
/// - `ellps`: Ellipsoid
/// - `lon_0`: Longitude of origin
/// - `x_0`: False easting
/// - `y_0`: False northing
///
/// ## Mathematical Definition
///
/// The formulas describing the Mercator projection are adapted from G. Evenden's libproj manuals.
///
/// **Forward Projection**:
/// ```latex
/// x = λ
/// y = ln(tan(π/4 + φ/2))
/// ```
///
/// **Inverse Projection**:
/// ```latex
/// λ = x
/// φ = π/2 - 2 * atan(exp(-y))
/// ```
///
/// ## Further Reading
///
/// - [Wikipedia: Web Mercator](https://en.wikipedia.org/wiki/Web_Mercator)
///
/// ![Web Mercator Projection](https://github.com/Open-S2/gis-tools/blob/master/assets/proj4/projections/images/merc.png?raw=true)
#[derive(Debug)]
pub struct WebMercatorProjection {}
impl ProjectCoordinates for WebMercatorProjection {
    fn code(&self) -> u32 {
        WEB_MERCATOR
    }
    fn name(&self) -> &'static str {
        "Web Mercator"
    }
    fn names() -> &'static [&'static str] {
        &["Web Mercator", "Pseudo Mercator", "webmerc"]
    }
}
impl CoordinateStep for WebMercatorProjection {
    fn new(_proj: &mut Proj) -> Self {
        WebMercatorProjection {}
    }
    fn forward<P: TransformCoordinates>(&self, proj: &Proj, p: &mut P) {
        merc_s_forward(proj, p);
    }
    fn inverse<P: TransformCoordinates>(&self, proj: &Proj, p: &mut P) {
        merc_s_inverse(proj, p);
    }
}

/// Ellipsoidal, mercator forward projection
pub fn merc_e_forward<P: TransformCoordinates>(proj: &Proj, p: &mut P) {
    p.set_x(proj.k0 * p.get_lam());
    // Instead of calling tan and sin, call sin and cos which the compiler
    // optimizes to a single call to sincos.
    let phi = p.get_phi();
    let sphi = phi.sin();
    let cphi = phi.cos();
    p.set_y(proj.k0 * (asinh(sphi / cphi) - proj.e * atanh(proj.e * sphi)));
}

/// Spherical, mercator forward
pub fn merc_s_forward<P: TransformCoordinates>(proj: &Proj, p: &mut P) {
    p.set_x(proj.k0 * p.get_lam());
    p.set_y(proj.k0 * asinh(tan(p.get_phi())));
}

/// Ellipsoidal, mercator inverse
pub fn merc_e_inverse<P: TransformCoordinates>(proj: &Proj, p: &mut P) {
    p.set_phi(atan(sinhpsi2tanphi(sinh(p.get_y() / proj.k0), proj.e)));
}

/// Spherical, mercator inverse
pub fn merc_s_inverse<P: TransformCoordinates>(proj: &Proj, p: &mut P) {
    p.set_phi(atan(sinh(p.get_y() / proj.k0)));
    p.set_lam(p.get_x() / proj.k0);
}
