use super::TransformCoordinates;
use crate::proj::{ProjJSON, ProjectionTransform, check_not_wgs84, datum_transform};
use alloc::fmt::Debug;
use alloc::{collections::BTreeMap, string::String};

/// # PROJ Transformer
///
/// ## Description
///
/// A Transformer class contains all projections necessary for converting coordinates from one
/// projection to another. This is a modular class that can be extended to add new projections
/// as needed to reduce code size and improve performance.
/// Both forward and inverse projections are default set to wgs84.
///
/// See the NadGridStore
///
/// ## Usage
///
/// ### Full Example
///
/// ```ts
/// // TODO
/// ```
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Transformer {
    epsgs: BTreeMap<String, String>,
    src: ProjectionTransform,
    dest: ProjectionTransform,
}
impl Transformer {
    /// Create a new Transformer
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            epsgs: BTreeMap::new(),
            src: ProjectionTransform::wgs84(),
            dest: ProjectionTransform::wgs84(),
        }
    }
    /// Move forward from source projection to destination projection
    pub fn forward<P: TransformCoordinates + Debug>(&self, p: &P) -> P {
        let mut res = p.clone();
        transform_point(&self.src, &self.dest, &mut res);
        res
    }
    /// Move forward from source projection to destination projection in place
    pub fn forward_mut<P: TransformCoordinates + Debug>(&self, p: &mut P) {
        transform_point(&self.src, &self.dest, p);
    }
    /// Move backward from destination projection to source projection
    pub fn inverse<P: TransformCoordinates + Debug>(&self, p: &P) -> P {
        let mut res = p.clone();
        transform_point(&self.dest, &self.src, &mut res);
        res
    }
    /// Move backward from destination projection to source projection in place
    pub fn inverse_mut<P: TransformCoordinates + Debug>(&self, p: &mut P) {
        transform_point(&self.dest, &self.src, p);
    }

    /// Insert an EPSG code definition
    /// ```ts
    /// // TODO
    /// ```
    /// @param code - EPSG code to insert e.g. "EPSG_4326" (uses underscore instead of colon)
    /// @param value - the EPSG definition which is either a WKT string object or proj4 encoded string
    pub fn insert_epsg_code(&mut self, code: String, value: String) {
        self.epsgs.insert(code, value);
    }

    /// Set the source projection
    /// @param code - can be a name or a json/wkt coded definition
    pub fn set_source(&mut self, code: String) {
        self.src = self.build_transformer(code);
    }

    /// Set the source projection
    /// @param def - transform definition
    pub fn set_source_def(&mut self, def: ProjectionTransform) {
        self.src = def;
    }

    /// Set the destination projection
    /// @param code - can be a name or a coded definition
    pub fn set_destination(&mut self, code: String) {
        self.dest = self.build_transformer(code);
    }

    /// Set the source projection
    /// @param def - transform definition
    pub fn set_destination_def(&mut self, def: ProjectionTransform) {
        self.dest = def;
    }

    /// Build a ProjectionTransform
    /// @param code - can be a WKT object or proj4 encoded string
    /// @returns - A ready to use ProjectionTransform
    fn build_transformer(&mut self, mut code: String) -> ProjectionTransform {
        if let Some(epsg) = self.epsgs.get(&code) {
            code = epsg.clone();
        }
        // TRY JSON
        if let Ok(json) = serde_json::from_str::<ProjJSON>(&code) {
            return json.to_projection_transform();
        }
        // TRY WKT
        ProjJSON::parse_wkt(&code).to_projection_transform()
    }

    /// Get access to an epsg code
    pub fn get_epsg_code(&self, code: String) -> Option<String> {
        self.epsgs.get(&code).cloned()
    }
}

/// Transforms a point from one projection to another
/// @param src - source projection
/// @param dest - destination projection
/// @param point - point to mutate
pub fn transform_point<P: TransformCoordinates + Debug>(
    src: &ProjectionTransform,
    dest: &ProjectionTransform,
    point: &mut P,
) {
    // Check if there are any steps
    if (src.is_empty() && dest.is_empty()) || src == dest || (src.is_wgs84 && dest.is_wgs84) {
        return;
    }
    let has_z = point.has_z();

    // STEP 0: If the datums are incompatible, be sure to use the intermediate wgs84 transform
    if check_not_wgs84(src, dest) {
        transform_point(src, &ProjectionTransform::wgs84(), point);
    }

    // STEP 1 A: ADJUST ORGIN
    let a = src.proj.borrow().a;
    if a != 0. {
        point.set_x(point.x() - src.proj.borrow().x0);
        point.set_y(point.y() - src.proj.borrow().y0);
    }
    // STEP 1 B: SOURCE -> WGS84
    for step in src.steps.iter().rev() {
        step.inverse(point);
    }
    // STEP 1 C: adjust for meters if necessary
    let mut to_meter = src.proj.borrow().to_meter;
    point.set_x(point.x() * to_meter);
    point.set_y(point.y() * to_meter);
    // STEP 1 D: Adjust for the prime meridian if necessary
    point.set_x(point.x() + src.proj.borrow().from_greenwich);

    // STEP 2: MID-POINT. Convert datums if needed, and if possible.
    datum_transform(point, &src.proj.borrow(), &dest.proj.borrow());

    // STEP 3 A: Adjust for the prime meridian if necessary
    point.set_x(point.x() - dest.proj.borrow().from_greenwich);
    // STEP 3 B: WGS84 in Radians -> DEST
    for step in &dest.steps {
        step.forward(point);
    }
    // STEP 3 C: adjust for meters and scale if necessary
    let a = dest.proj.borrow().a;
    if a != 0. {
        point.set_x(point.x() * a);
        point.set_y(point.y() * a);
    }
    to_meter = dest.proj.borrow().to_meter;
    point.set_x(point.x() / to_meter);
    point.set_y(point.y() / to_meter);

    if !has_z {
        point.set_z(0.0);
    }
}
