use libm::{atan, cos, exp, floor, log, pow, sin, tan};

use crate::space::EARTH_CIRCUMFERENCE;

use core::f64::consts::PI;

/// 900913 (Web Mercator) constant
pub const A: f64 = 6_378_137.0;
/// 900913 (Web Mercator) max extent
pub const MAXEXTENT: f64 = 20_037_508.342789244;
/// 900913 (Web Mercator) maximum latitude
pub const MAXLAT: f64 = 85.0511287798;

/// The source of the coordinate inputs
#[derive(Debug, Clone, PartialEq)]
pub enum Source {
    /// The WGS84 projection
    WGS84,
    /// The Google (900913) projection
    Google,
}

/// Given a zoom and tilesize, build mercator positional attributes
pub fn get_zoom_size(zoom: u8, tile_size: f64) -> (f64, f64, f64, f64) {
    let size = tile_size * pow(2., zoom as f64);
    (size / 360., size / (2. * PI), size / 2., size)
}

/// Convert Longitude and Latitude to a mercator pixel coordinate
/// Return the mercator pixel coordinate
pub fn ll_to_px(
    lonlat: (f64, f64),
    zoom: u8,
    anti_meridian: Option<bool>,
    tile_size: Option<f64>,
) -> (f64, f64) {
    let anti_meridian = anti_meridian.unwrap_or(false);
    let tile_size = tile_size.unwrap_or(512.);

    let (bc, cc, zc, ac) = get_zoom_size(zoom, tile_size);
    let expansion = if anti_meridian { 2. } else { 1. };
    let d = zc;
    let f = sin((lonlat.1).to_radians()).clamp(-0.999999999999, 0.999999999999);
    let mut x = d + lonlat.0 * bc;
    let mut y = d + 0.5 * log((1. + f) / (1. - f)) * -cc;
    if x > ac * expansion {
        x = ac * expansion;
    }
    if y > ac {
        y = ac;
    }

    (x, y)
}

/// Convert mercator pixel coordinates to Longitude and Latitude
/// Return the Longitude and Latitude
pub fn px_to_ll(xy: (f64, f64), zoom: u8, tile_size: Option<f64>) -> (f64, f64) {
    let tile_size = tile_size.unwrap_or(512.);
    let (bc, cc, zc, _) = get_zoom_size(zoom, tile_size);
    let g = (xy.1 - zc) / -cc;
    let lon = (xy.0 - zc) / bc;
    let lat = 2. * atan(exp(g)).to_degrees() - 0.5 * PI;

    (lon, lat)
}

/// Convert Longitude and Latitude to a mercator x-y coordinates
/// Return the mercator x-y coordinates
pub fn ll_to_merc(lonlan: (f64, f64)) -> (f64, f64) {
    let mut x = (A * lonlan.0).to_radians();
    let mut y = A * log(tan(PI * 0.25 + (0.5 * lonlan.1).to_radians()));
    // if xy value is beyond maxextent (e.g. poles), return maxextent.
    x = x.clamp(-MAXEXTENT, MAXEXTENT);
    y = y.clamp(-MAXEXTENT, MAXEXTENT);

    (x, y)
}

/// Convert mercator x-y coordinates to Longitude and Latitude
/// Return the Longitude and Latitude
pub fn merc_to_ll(merc: (f64, f64)) -> (f64, f64) {
    let x = (merc.0 / A).to_degrees();
    let y = (0.5 * PI - 2. * atan(exp(-merc.1 / A))).to_degrees();
    (x, y)
}

/// Convert a pixel coordinate to a tile x-y coordinate
/// Return the tile x-y
pub fn px_to_tile(px: (f64, f64), tile_size: Option<f64>) -> (u32, u32) {
    let tile_size = tile_size.unwrap_or(512.);
    (floor(px.0 / tile_size) as u32, floor(px.1 / tile_size) as u32)
}

/// Convert a tile x-y-z to a bbox of the form `[w, s, e, n]`
/// Return the bbox
pub fn tile_to_bbox(tile: (u8, u32, u32), tile_size: Option<f64>) -> (f64, f64, f64, f64) {
    let tile_size = tile_size.unwrap_or(512.);
    let (_zoom, x, y) = tile;
    let min_x = x as f64 * tile_size;
    let min_y = y as f64 * tile_size;
    let max_x = min_x + tile_size;
    let max_y = min_y + tile_size;

    (min_x, min_y, max_x, max_y)
}

/// Convert a lat-lon and zoom to the tile's x-y coordinates
/// Return the tile x-y
pub fn ll_to_tile(lonlat: (f64, f64), zoom: u8, tile_size: Option<f64>) -> (u32, u32) {
    let px = ll_to_px(lonlat, zoom, Some(false), tile_size);
    px_to_tile(px, tile_size)
}

/// given a lon-lat and tile, find the offset in pixels
/// return the tile xy pixel
pub fn ll_to_tile_px(
    lonlat: (f64, f64),
    tile: (u8, u32, u32),
    tile_size: Option<f64>,
) -> (f64, f64) {
    let (zoom, x, y) = tile;
    let tile_size = tile_size.unwrap_or(512.);
    let px = ll_to_px(lonlat, zoom, Some(false), Some(tile_size));
    let tile_x_start = x as f64 * tile_size;
    let tile_y_start = y as f64 * tile_size;

    ((px.0 - tile_x_start) / tile_size, (px.1 - tile_y_start) / tile_size)
}

/// Convert a bbox of the form `[w, s, e, n]` to a bbox of the form `[w, s, e, n]`
/// The result can be in lon-lat (WGS84) or WebMercator (900913)
pub fn convert_bbox(bbox: &(f64, f64, f64, f64), source: Source) -> (f64, f64, f64, f64) {
    let low: (f64, f64);
    let high: (f64, f64);
    match source {
        Source::WGS84 => {
            low = merc_to_ll((bbox.0, bbox.1));
            high = merc_to_ll((bbox.2, bbox.3));
        }
        Source::Google => {
            low = ll_to_merc((bbox.0, bbox.1));
            high = ll_to_merc((bbox.2, bbox.3));
        }
    };
    (low.0, low.1, high.0, high.1)
}

/// Convert a tile x-y-z to a bbox of the form `[w, s, e, n]`
/// The result can be in lon-lat (WGS84) or WebMercator (900913)
/// The default result is in WebMercator (900913)
pub fn xyz_to_bbox(
    x: f64,
    y: f64,
    zoom: u8,
    tms_style: Option<bool>,
    source: Option<Source>,
    tile_size: Option<f64>,
) -> (f64, f64, f64, f64) {
    let tms_style = tms_style.unwrap_or(true);
    let source = source.unwrap_or(Source::Google);
    let tile_size = tile_size.unwrap_or(512.0);
    let mut y = y;
    // Convert xyz into bbox with srs WGS84
    // if tmsStyle, the y is inverted
    if tms_style {
        y = pow(2., zoom as f64) - 1. - y;
    }
    // Use +y to make sure it's a number to avoid inadvertent concatenation.
    let bl: (f64, f64) = (x * tile_size, (y + 1.) * tile_size);
    // Use +x to make sure it's a number to avoid inadvertent concatenation.
    let tr: (f64, f64) = ((x + 1.) * tile_size, y * tile_size);
    // to pixel-coordinates
    let px_bl = px_to_ll(bl, zoom, Some(tile_size));
    let px_tr = px_to_ll(tr, zoom, Some(tile_size));

    match source {
        Source::Google => {
            let ll_bl = ll_to_merc(px_bl);
            let ll_tr = ll_to_merc(px_tr);
            (ll_bl.0, ll_bl.1, ll_tr.0, ll_tr.1)
        }
        _ => (px_bl.0, px_bl.1, px_tr.0, px_tr.1),
    }
}

/// Convert a bbox of the form `[w, s, e, n]` to a tile's bounding box
/// in the form of { minX, maxX, minY, maxY }
/// The bbox can be in lon-lat (WGS84) or WebMercator (900913)
/// The default expectation is in WebMercator (900913)
/// returns the tile's bounding box
pub fn bbox_to_xyz_bounds(
    bbox: (f64, f64, f64, f64),
    zoom: u8,
    tms_style: Option<bool>,
    source: Option<Source>,
    tile_size: Option<f64>,
) -> (f64, f64, f64, f64) {
    let tms_style = tms_style.unwrap_or(true);
    let source = source.unwrap_or(Source::Google);
    let tile_size = tile_size.unwrap_or(512.0);

    let mut bl = (bbox.0, bbox.1); // bottom left
    let mut tr = (bbox.2, bbox.3); // top right

    if source == Source::Google {
        bl = ll_to_merc(bl);
        tr = ll_to_merc(tr);
    }
    let px_bl = ll_to_px(bl, zoom, Some(false), Some(tile_size));
    let px_tr = ll_to_px(tr, zoom, Some(false), Some(tile_size));
    let x = (floor(px_bl.0) / tile_size, floor((px_tr.0 - 1.0) / tile_size));
    let y = (floor(px_tr.1) / tile_size, floor((px_bl.1 - 1.0) / tile_size));

    let mut bounds =
        (f64::min(x.0, x.1), f64::min(y.0, y.1), f64::max(x.0, x.1), f64::max(y.0, y.1));
    if bounds.0 < 0. {
        bounds.0 = 0.
    }
    if bounds.1 < 0. {
        bounds.1 = 0.
    }

    if tms_style {
        let zoom_diff = pow(2., zoom as f64) - 1.;
        bounds.1 = zoom_diff - bounds.3;
        bounds.3 = zoom_diff - bounds.1;
    }

    bounds
}

/// The circumference at a line of latitude in meters.
pub fn circumference_at_latitude(latitude: f64) -> f64 {
    EARTH_CIRCUMFERENCE * cos(latitude.to_radians())
}

/// Convert longitude to mercator projection X-Value
/// returns the X-Value
pub fn lng_to_mercator_x(lng: f64) -> f64 {
    (180.0 + lng) / 360.0
}

/// Convert latitude to mercator projection Y-Value
/// returns the Y-Value
pub fn lat_to_mercator_y(lat: f64) -> f64 {
    (180. - (180. / PI) * log(tan(PI / 4. + (lat * PI) / 360.))) / 360.
}

/// Convert altitude to mercator projection Z-Value
/// returns the Z-Value
pub fn altitude_to_mercator_z(altitude: f64, lat: f64) -> f64 {
    altitude / circumference_at_latitude(lat)
}

/// Convert mercator projection's X-Value to longitude
/// returns the longitude
pub fn lng_from_mercator_x(x: f64) -> f64 {
    x * 360. - 180.
}

/// Convert mercator projection's Y-Value to latitude
/// returns the latitude
pub fn lat_from_mercator_y(y: f64) -> f64 {
    let y2 = 180. - y * 360.;
    (360. / PI) * atan(exp((y2 * PI) / 180.)) - 90.
}

/// Convert mercator projection's Z-Value to altitude
/// returns the altitude
pub fn altitude_from_mercator_z(z: f64, y: f64) -> f64 {
    z * circumference_at_latitude(lat_from_mercator_y(y))
}

/// Determine the Mercator scale factor for a given latitude, see
/// https://en.wikipedia.org/wiki/Mercator_projection#Scale_factor
///
/// At the equator the scale factor will be 1, which increases at higher latitudes.
/// returns the scale factor
pub fn mercator_lat_scale(lat: f64) -> f64 {
    1. / cos((lat * PI) / 180.)
}
