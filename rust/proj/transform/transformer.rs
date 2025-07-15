use super::TransformCoordinates;
use crate::proj::{
    Direction, IoUnits, ProjJSON, ProjectionTransform, adjlon, check_not_wgs84, datum_transform,
    geocentric_latitude,
};
use alloc::{collections::BTreeMap, fmt::Debug, string::String};
use core::f64::consts::FRAC_PI_2;

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
    /// @param code - EPSG code to insert e.g. "4326": "WKT_STRING"
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
    if src == dest || (src.is_wgs84 && dest.is_wgs84) {
        return;
    }
    let has_z = point.has_z();

    // TODO: We ideally drop this when cart and cart_wgs84 are merged
    // STEP 0: If the datums are incompatible, be sure to use the intermediate wgs84 transform
    if check_not_wgs84(src, dest) {
        transform_point(src, &ProjectionTransform::wgs84(), point);
    }

    // STEP 1 INVERSE
    transform_inv(src, point);
    // STEP 2: MID-POINT. Convert datums if needed, and if possible.
    datum_transform(point, &src.proj.borrow(), &dest.proj.borrow());
    // STEP 3: FORWARD
    transform_fwd(dest, point);

    if !has_z {
        point.set_z(0.0);
    }
}

fn transform_inv<P: TransformCoordinates + Debug>(
    proj_trans: &ProjectionTransform,
    coords: &mut P,
) {
    transform_inv_prepare(proj_trans, coords);
    proj_trans.method.inverse(coords);
    transform_inv_finalize(proj_trans, coords);
}

fn transform_inv_prepare<P: TransformCoordinates + Debug>(
    proj_trans: &ProjectionTransform,
    coords: &mut P,
) {
    let proj = &proj_trans.proj.borrow();
    // The helmert datum shift will choke unless it gets a sensible 4D coordinate
    if f64::INFINITY == coords.z() && proj_trans.helmert.is_some() {
        coords.set_z(0.0);
    }
    if f64::INFINITY == coords.t() && proj_trans.helmert.is_some() {
        coords.set_t(0.0);
    }

    if let Some(axisswap) = &proj_trans.axisswap {
        axisswap.method.inverse(coords);
    }

    // Handle remaining possible input types
    match proj.right {
        // de-scale and de-offset
        IoUnits::CARTESIAN => {
            let to_meter = proj.to_meter;
            coords.set_x(coords.x() * to_meter);
            coords.set_y(coords.y() * to_meter);
            coords.set_z(coords.z() * to_meter);
            if proj.is_geocent
                && let Some(cart) = &proj_trans.cart
            {
                transform_inv(cart, coords);
            }
        }

        IoUnits::PROJECTED | IoUnits::CLASSIC => {
            let to_meter = proj.to_meter;
            let vto_meter = proj.vto_meter;
            coords.set_x(coords.x() * to_meter - proj.x0);
            // 440720.0 - 400000
            coords.set_y(coords.y() * to_meter - proj.y0);
            coords.set_z(coords.z() * vto_meter - proj.z0);
            if proj.right == IoUnits::PROJECTED {
                return;
            }

            // Classic proj.4 functions expect plane coordinates in units of the semimajor axis
            // Multiplying by ra, rather than dividing by a because the CalCOFI projection
            // stomps on a and hence (apparently) depends on this to roundtrip correctly
            // (CalCOFI avoids further scaling by stomping - but a better solution is possible)
            if proj.ra != 0. {
                coords.set_x(coords.x() * proj.ra);
                coords.set_y(coords.y() * proj.ra);
            }
        }

        IoUnits::RADIANS => {
            coords.set_z(coords.z() * proj.vto_meter - proj.z0);
        }
        _ => {}
    }
}

fn transform_inv_finalize<P: TransformCoordinates + Debug>(
    proj_trans: &ProjectionTransform,
    coords: &mut P,
) {
    let proj = &proj_trans.proj.borrow();
    if proj.left == IoUnits::RADIANS {
        // Distance from central meridian, taking system zero meridian into account
        coords.set_lam(coords.lam() + proj.from_greenwich + proj.lam0);

        // adjust longitude to central meridian
        if !proj.over {
            coords.set_lam(adjlon(coords.lam()));
        }

        if let Some(vgridshift) = &proj_trans.vgridshift {
            // Go geometric from orthometric
            transform_inv(vgridshift, coords);
        }
        if let Some(hgridshift) = &proj_trans.hgridshift {
            // Go geometric from orthometric
            transform_fwd(hgridshift, coords);
        } else if (proj_trans.cart_wgs84.is_some() && proj_trans.cart.is_some())
            || proj_trans.helmert.is_some()
        {
            // Go cartesian in local frame
            if let Some(cart) = &proj_trans.cart {
                transform_fwd(cart, coords);
            }
            // Step into WGS84
            if let Some(helmert) = &proj_trans.helmert {
                transform_fwd(helmert, coords);
            }
            // Go back to angular using WGS84 ellps
            if let Some(cart_wgs84) = &proj_trans.cart_wgs84 {
                transform_inv(cart_wgs84, coords);
            }
        }

        // If input latitude was geocentrical, convert back to geocentrical
        if proj.geoc {
            geocentric_latitude(proj, Direction::FWD, coords);
        }
    }
}

fn transform_fwd<P: TransformCoordinates + Debug>(
    proj_trans: &ProjectionTransform,
    coords: &mut P,
) {
    transform_fwd_prepare(proj_trans, coords);
    proj_trans.method.forward(coords);
    transform_fwd_finalize(proj_trans, coords);
}

fn transform_fwd_prepare<P: TransformCoordinates + Debug>(
    proj_trans: &ProjectionTransform,
    coords: &mut P,
) {
    let proj = &proj_trans.proj.borrow();
    // The helmert datum shift will choke unless it gets a sensible 4D coordinate
    if coords.z() == f64::INFINITY && proj_trans.helmert.is_some() {
        coords.set_z(0.0);
    }
    if coords.t() == f64::INFINITY && proj_trans.helmert.is_some() {
        coords.set_t(0.0);
    }

    // Check validity of angular input coordinates
    if proj.left == IoUnits::RADIANS {
        // check for latitude or longitude over-range
        let t = (if coords.phi() < 0. { -coords.phi() } else { coords.phi() }) - FRAC_PI_2;
        if t > 1e-12 || coords.lam() > 10. || coords.lam() < -10. {
            panic!("Invalid latitude");
        }
        // Clamp latitude to -90..90 degree range
        if coords.phi() > FRAC_PI_2 {
            coords.set_phi(FRAC_PI_2);
        }
        if coords.phi() < -FRAC_PI_2 {
            coords.set_phi(-FRAC_PI_2);
        }
        // If input latitude is geocentrical, convert to geographical */
        if proj.geoc {
            geocentric_latitude(proj, Direction::INV, coords);
        }
        // Ensure longitude is in the -pi:pi range
        if !proj.over {
            coords.set_lam(adjlon(coords.lam()));
        }

        if let Some(hgridshift) = &proj_trans.hgridshift {
            transform_inv(hgridshift, coords);
        } else if (proj_trans.cart_wgs84.is_some() && proj_trans.cart.is_some())
            || proj_trans.helmert.is_some()
        {
            // Go cartesian in local frame
            if let Some(cart_wgs84) = &proj_trans.cart_wgs84 {
                transform_fwd(cart_wgs84, coords);
            }
            // Step into WGS84
            if let Some(helmert) = &proj_trans.helmert {
                transform_inv(helmert, coords);
            }
            // Go back to angular using WGS84 ellps
            if let Some(cart) = &proj_trans.cart {
                transform_inv(cart, coords);
            }
        }
        // Go orthometric from geometric
        if let Some(vgridshift) = &proj_trans.vgridshift {
            transform_fwd(vgridshift, coords);
        }
        // Distance from central meridian, taking system zero meridian into account
        coords.set_lam((coords.lam() - proj.from_greenwich) - proj.lam0);

        // Ensure longitude is in the -pi:pi range
        if !proj.over {
            coords.set_lam(adjlon(coords.lam()));
        }

        return;
    }

    // We do not support gridshifts on cartesian input
    if proj.left == IoUnits::CARTESIAN
        && let Some(helmert) = &proj_trans.helmert
    {
        transform_inv(helmert, coords);
    }
}

fn transform_fwd_finalize<P: TransformCoordinates + Debug>(
    proj_trans: &ProjectionTransform,
    coords: &mut P,
) {
    let proj = &proj_trans.proj.borrow();
    match proj.right {
        // Handle false eastings/northings and non-metric linear units
        IoUnits::CARTESIAN => {
            if proj.is_geocent
                && let Some(cart) = &proj_trans.cart
            {
                transform_fwd(cart, coords);
            }
            coords.set_x(coords.x() * proj.fr_meter);
            coords.set_y(coords.y() * proj.fr_meter);
            coords.set_z(coords.z() * proj.vfr_meter);
        }

        // Classic proj.4 functions return plane coordinates in units of the semimajor axis
        IoUnits::CLASSIC => {
            if proj.a != 0. {
                coords.set_x(coords.x() * proj.a);
                coords.set_y(coords.y() * proj.a);
            }
            coords.set_x(proj.fr_meter * (coords.x() + proj.x0));
            coords.set_y(proj.fr_meter * (coords.y() + proj.y0));
            coords.set_z(proj.vfr_meter * (coords.z() + proj.z0));
        }

        // to continue processing in common with IoUnits::PROJECTED
        IoUnits::PROJECTED => {
            coords.set_x(proj.fr_meter * (coords.x() + proj.x0));
            coords.set_y(proj.fr_meter * (coords.y() + proj.y0));
            coords.set_z(proj.vfr_meter * (coords.z() + proj.z0));
        }

        IoUnits::RADIANS => {
            coords.set_z(proj.vfr_meter * (coords.z() + proj.z0));
            // not interested in adding this support. keeping for posterity
            // if (proj.is_long_wrap_set) {
            //     if (coordsz.lam != HUGE_VAL) {
            //         coordsz.lam =
            //             proj.long_wrap_center + adjlon(coordsz.lam - proj.long_wrap_center);
            //     }
            // }
        }
        _ => {}
    }

    if let Some(axisswap) = &proj_trans.axisswap {
        axisswap.method.forward(coords);
    }
}
