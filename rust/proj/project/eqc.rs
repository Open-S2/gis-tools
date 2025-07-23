use crate::proj::{
    CoordinateStep, EQUIDISTANT_CYLINDRICAL, LATITUDE_STD_PARALLEL, Proj, ProjValue,
    ProjectCoordinates, TransformCoordinates,
};
use alloc::rc::Rc;
use core::cell::RefCell;
use libm::cos;

/// Equidistant Cylindrical variables
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Eqc {
    rc: f64,
}

/// # Equidistant Cylindrical (Plate Carrée)
///
/// **Classification**: Conformal cylindrical
///
/// **Available forms**: Forward and inverse
///
/// **Defined area**: Global, but best used near the equator
///
/// **Alias**: eqc, plate_carrée, simple_cylindrical
///
/// **Domain**: 2D
///
/// **Input type**: Geodetic coordinates
///
/// **Output type**: Projected coordinates
///
/// ## Projection String
/// ```ini
/// +proj=eqc
/// ```
///
/// ## Usage
///
/// Because of the distortions introduced by this projection, it has little use in navigation or
/// cadastral mapping and finds its main use in thematic mapping.
/// In particular, the Plate Carrée has become a standard for global raster datasets, such as
/// Celestia and NASA World Wind, because of the particularly simple relationship between the
/// position of an image pixel on the map and its corresponding geographic location on Earth.
///
/// ### Special Cases of Cylindrical Equidistant Projection:
///
/// - Plain/Plane Chart: $0°$
/// - Simple Cylindrical: $0°$
/// - Plate Carrée: $0°$
/// - Ronald Miller—minimum overall scale distortion: $37°30'$
/// - E. Grafarend and A. Niermann: $42°$
/// - Ronald Miller—minimum continental scale distortion: $43°30'$
/// - Gall Isographic: $45°$
/// - Ronald Miller Equirectangular: $50°30'$
/// - E. Grafarend and A. Niermann minimum linear distortion: $61°7'$
///
/// ## Example
///
/// Example using EPSG 32662 (WGS84 Plate Carrée):
/// ```bash
/// echo 2 47 | proj +proj=eqc +ellps=WGS84
/// ```
/// Output: 222638.98 5232016.07
///
/// Example using Plate Carrée projection with true scale at latitude 30° and central meridian 90°W:
/// ```bash
/// echo -88 30 | proj +proj=eqc +lat_ts=30 +lon_0=90w
/// ```
/// Output: 192811.01 3339584.72
///
/// ## Parameters
///
/// - `+lon_0` (Central meridian)
/// - `+lat_0` (Latitude of origin)
/// - `+lat_ts` (Latitude of true scale)
/// - `+x_0` (False easting)
/// - `+y_0` (False northing)
/// - `+ellps` (Ellipsoid name)
/// - `+R` (Radius of the sphere)
///
/// ## Mathematical Definition
///
/// ### Forward projection:
/// $$x = \lambda \cos(\phi_{ts})$$
/// $$y = \phi - \phi_0$$
///
/// ### Inverse projection:
/// $$\lambda = x / \cos(\phi_{ts})$$
/// $$\phi = y + \phi_0$$
///
/// ## Further Reading
///
/// - [Wikipedia](https://en.wikipedia.org/wiki/Equirectangular_projection)
/// - [Wolfram Mathworld](http://mathworld.wolfram.com/CylindricalEquidistantProjection.html)
///
/// ![Equidistant Cylindrical](https://github.com/Open-S2/gis-tools/blob/master/assets/proj4/projections/images/eqc.png?raw=true)
#[derive(Debug, Clone, PartialEq)]
pub struct EquidistantCylindricalProjection {
    proj: Rc<RefCell<Proj>>,
    store: RefCell<Eqc>,
}
impl ProjectCoordinates for EquidistantCylindricalProjection {
    fn code(&self) -> i64 {
        EQUIDISTANT_CYLINDRICAL
    }
    fn name(&self) -> &'static str {
        "Equidistant Cylindrical"
    }
    fn names() -> &'static [&'static str] {
        &[
            "Equidistant Cylindrical",
            "EquidistantCylindrical",
            "Equidistant Cylindrical (Plate Carree)",
            "eqc",
        ]
    }
}
impl CoordinateStep for EquidistantCylindricalProjection {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        let mut store = Eqc::default();
        {
            let proj = &mut proj.borrow_mut();
            let lat_ts = proj
                .params
                .get(&LATITUDE_STD_PARALLEL) // (lat_ts)
                .unwrap_or(&ProjValue::default())
                .f64()
                .to_radians();
            if cos(lat_ts) <= 0. {
                panic!("Invalid value for lat_ts: |lat_ts| should be <= 90°");
            }
            store.rc = lat_ts;
            proj.es = 0.;
        }
        EquidistantCylindricalProjection { proj, store: store.into() }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        eqc_s_forward(&self.store.borrow(), &self.proj.borrow(), p);
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        eqc_s_inverse(&self.store.borrow(), &self.proj.borrow(), p);
    }
}

/// Equidistant Cylindrical Spheroidal forward project
pub fn eqc_s_forward<P: TransformCoordinates>(eqc: &Eqc, proj: &Proj, p: &mut P) {
    p.set_x(eqc.rc * p.lam());
    p.set_y(p.phi() - proj.phi0);
}

/// Equidistant Cylindrical Spheroidal inverse project
pub fn eqc_s_inverse<P: TransformCoordinates>(eqc: &Eqc, proj: &Proj, p: &mut P) {
    p.set_lam(p.x() / eqc.rc);
    p.set_phi(p.y() + proj.phi0);
}
