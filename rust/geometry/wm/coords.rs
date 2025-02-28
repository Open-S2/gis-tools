use libm::{atan, cos, exp, floor, fmax, fmin, log, pow, sin, tan};

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
fn get_zoom_size(zoom: u8, tile_size: f64) -> (f64, f64, f64, f64) {
    let size = tile_size * pow(2., zoom as f64);
    (size / 360., size / (2. * PI), size / 2., size)
}

/// Convert Longitude and Latitude to a mercator pixel coordinate
/// Return the mercator pixel coordinate
pub fn ll_to_px(
    lonlat: (f64, f64),
    zoom: u8,
    anti_meridian: Option<bool>,
    tile_size: Option<u16>,
) -> (f64, f64) {
    let anti_meridian = anti_meridian.unwrap_or(false);
    let tile_size = tile_size.unwrap_or(512) as f64;

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
pub fn px_to_ll(xy: (f64, f64), zoom: u8, tile_size: Option<u16>) -> (f64, f64) {
    let tile_size = tile_size.unwrap_or(512) as f64;
    let (bc, cc, zc, _) = get_zoom_size(zoom, tile_size);
    let g = (xy.1 - zc) / -cc;
    let lon = (xy.0 - zc) / bc;
    let lat = (2. * atan(exp(g)) - 0.5 * PI).to_degrees();

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
pub fn px_to_tile(px: (f64, f64), tile_size: Option<u16>) -> (u32, u32) {
    let tile_size = tile_size.unwrap_or(512) as f64;
    (floor(px.0 / tile_size) as u32, floor(px.1 / tile_size) as u32)
}

/// Convert a tile x-y-z to a bbox of the form `[w, s, e, n]`
/// Return the bbox
pub fn tile_to_bbox(tile: (u8, u32, u32), tile_size: Option<u16>) -> (u32, u32, u32, u32) {
    let tile_size = tile_size.unwrap_or(512) as u32;
    let (_zoom, x, y) = tile;
    let min_x = x * tile_size;
    let min_y = y * tile_size;
    let max_x = min_x + tile_size;
    let max_y = min_y + tile_size;

    (min_x, min_y, max_x, max_y)
}

/// Convert a lat-lon and zoom to the tile's x-y coordinates
/// Return the tile x-y
pub fn ll_to_tile(lonlat: (f64, f64), zoom: u8, tile_size: Option<u16>) -> (u32, u32) {
    let px = ll_to_px(lonlat, zoom, Some(false), tile_size);
    px_to_tile(px, tile_size)
}

/// given a lon-lat and tile, find the offset in pixels
/// return the tile xy pixel
pub fn ll_to_tile_px(
    lonlat: (f64, f64),
    tile: (u8, u32, u32),
    tile_size: Option<u16>,
) -> (f64, f64) {
    let (zoom, x, y) = tile;
    let tile_size = tile_size.unwrap_or(512);
    let tile_size_f = tile_size as f64;
    let px = ll_to_px(lonlat, zoom, Some(false), Some(tile_size));
    let tile_x_start = x as f64 * tile_size_f;
    let tile_y_start = y as f64 * tile_size_f;

    ((px.0 - tile_x_start) / tile_size_f, (px.1 - tile_y_start) / tile_size_f)
}

/// Convert a bbox of the form `[w, s, e, n]` to a bbox of the form `[w, s, e, n]`
/// The result can be in lon-lat (WGS84) or WebMercator (900913)
pub fn convert_bbox(bbox: (f64, f64, f64, f64), source: Source) -> (f64, f64, f64, f64) {
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
    x: u32,
    y: u32,
    zoom: u8,
    tms_style: Option<bool>,
    source: Option<Source>,
    tile_size: Option<u16>,
) -> (f64, f64, f64, f64) {
    let x = x as f64;
    let mut y = y as f64;
    let tms_style = tms_style.unwrap_or(true);
    let source = source.unwrap_or(Source::Google);
    let tile_size = tile_size.unwrap_or(512);
    let tile_size_f = tile_size as f64;
    // Convert xyz into bbox with srs WGS84
    // if tmsStyle, the y is inverted
    if tms_style {
        y = pow(2., zoom as f64) - 1. - y;
    }
    // Use +y to make sure it's a number to avoid inadvertent concatenation.
    let bl: (f64, f64) = (x * tile_size_f, (y + 1.) * tile_size_f);
    // Use +x to make sure it's a number to avoid inadvertent concatenation.
    let tr: (f64, f64) = ((x + 1.) * tile_size_f, y * tile_size_f);
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
    tile_size: Option<u16>,
) -> (u32, u32, u32, u32) {
    let tms_style = tms_style.unwrap_or(true);
    let source = source.unwrap_or(Source::WGS84);
    let tile_size = tile_size.unwrap_or(512);
    let tile_size_f: f64 = tile_size as f64;

    let mut bl = (bbox.0, bbox.1); // bottom left
    let mut tr = (bbox.2, bbox.3); // top right

    if source == Source::Google {
        bl = ll_to_merc(bl);
        tr = ll_to_merc(tr);
    }
    let px_bl = ll_to_px(bl, zoom, Some(false), Some(tile_size));
    let px_tr = ll_to_px(tr, zoom, Some(false), Some(tile_size));
    let x = (floor(px_bl.0 / tile_size_f), floor((px_tr.0 - 1.0) / tile_size_f));
    let y = (floor(px_tr.1 / tile_size_f), floor((px_bl.1 - 1.0) / tile_size_f));

    let mut bounds =
        (f64::min(x.0, x.1), f64::min(y.0, y.1), f64::max(x.0, x.1), f64::max(y.0, y.1));

    if tms_style {
        let zoom_diff = pow(2., zoom as f64) - 1.;
        bounds.1 = zoom_diff - bounds.3;
        bounds.3 = zoom_diff - bounds.1;
    }

    let min_x = fmax(bounds.0, 0.) as u32;
    let min_y = fmax(bounds.1, 0.) as u32;
    let max_x = fmin(bounds.2, pow(2., zoom as f64) - 1.) as u32;
    let max_y = fmin(bounds.3, pow(2., zoom as f64) - 1.) as u32;

    (min_x, min_y, max_x, max_y)
}

/// The circumference at a line of latitude in meters.
pub fn circumference_at_latitude(latitude: f64, circumference: Option<f64>) -> f64 {
    let circumference = circumference.unwrap_or(EARTH_CIRCUMFERENCE);
    circumference * cos(latitude.to_radians())
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
pub fn altitude_to_mercator_z(altitude: f64, lat: f64, circumference: Option<f64>) -> f64 {
    altitude / circumference_at_latitude(lat, circumference)
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
pub fn altitude_from_mercator_z(z: f64, y: f64, circumference: Option<f64>) -> f64 {
    z * circumference_at_latitude(lat_from_mercator_y(y), circumference)
}

/// Determine the Mercator scale factor for a given latitude, see
/// https://en.wikipedia.org/wiki/Mercator_projection#Scale_factor
///
/// At the equator the scale factor will be 1, which increases at higher latitudes.
/// returns the scale factor
pub fn mercator_lat_scale(lat: f64) -> f64 {
    1. / cos((lat * PI) / 180.)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ll_to_px() {
        assert_eq!(
            ll_to_px((-179.0, 85.0), 9, Some(true), Some(256)),
            (364.0888888888876, 214.68476683766494),
            // "PX with int zoom value converts when antiMeridian=true"
        );
        assert_eq!(
            ll_to_px((-179.0, 85.0), 9, Some(false), Some(256)),
            (364.0888888888876, 214.68476683766494),
            // "PX with int zoom value converts when antiMeridian=false"
        );
        assert_eq!(
            ll_to_px((250.0, 3.0), 4, Some(false), Some(256)),
            (4096.0, 2013.8510595566413),
            // "Clamps PX by default when lon >180 when antiMeridian=false"
        );
        assert_eq!(
            ll_to_px((250.0, 3.0), 4, Some(true), Some(256)),
            (4892.444444444444, 2013.8510595566413),
            // "PX with lon > 180 converts when antimeridian=true"
        );
        assert_eq!(
            ll_to_px((400.0, 3.0), 4, Some(true), Some(256)),
            (6599.111111111111, 2013.8510595566413),
            // "Clamps PX when lon >360 and antimeridian=true"
        );
        assert_eq!(
            ll_to_px((400.0, 3.0), 4, Some(false), Some(256)),
            (4096.0, 2013.8510595566413),
            // "Clamps PX when lon >360 and antimeridian=false"
        );
    }

    #[test]
    fn test_px_to_ll() {
        assert_eq!(
            px_to_ll((200.0, 200.0), 9, Some(256)),
            (-179.45068359375, 85.00351401304403),
            // "LL with int zoom value converts when antiMeridian=true"
        );
    }

    #[test]
    fn test_ll_to_merc() {
        assert_eq!(
            ll_to_merc((0.0, 0.0)),
            (0.0, -7.081154551613622e-10),
            // "LL with int zoom value converts when antiMeridian=true"
        );
        assert_eq!(
            ll_to_merc((-180.0, 90.0)),
            (-20037508.342789244, 20037508.342789244),
            // "LL with int zoom value converts when antiMeridian=true"
        );
        assert_eq!(
            ll_to_merc((180.0, -90.0)),
            (20037508.342789244, -20037508.342789244),
            // "LL with int zoom value converts when antiMeridian=true"
        );
    }

    #[test]
    fn test_merc_to_ll() {
        assert_eq!(
            merc_to_ll((0.0, -7.081154551613622e-10)),
            (0.0, 0.0),
            // "LL with int zoom value converts when antiMeridian=true"
        );
        assert_eq!(
            merc_to_ll((-20037508.34278924, 20037508.342789244)),
            (-179.99999999999997, 85.0511287798066),
            // "LL with int zoom value converts when antiMeridian=true"
        );
        assert_eq!(
            merc_to_ll((20037508.34278924, -20037508.342789244)),
            (179.99999999999997, -85.0511287798066),
            // "LL with int zoom value converts when antiMeridian=true"
        );
    }

    #[test]
    fn test_px_to_tile() {
        assert_eq!(
            px_to_tile((0.0, 0.0), Some(512)),
            (0, 0),
            // "LL with int zoom value converts when antiMeridian=true"
        );
        assert_eq!(
            px_to_tile((600.0, 2000.0), Some(512)),
            (1, 3),
            // "LL with int zoom value converts when antiMeridian=true"
        );
    }

    #[test]
    fn test_xyz_to_bbox() {
        assert_eq!(
            xyz_to_bbox(0, 0, 0, Some(true), Some(Source::WGS84), Some(256)),
            (-180.0, -85.05112877980659, 180.0, 85.0511287798066),
            // "LL with int zoom value converts when antiMeridian=true"
        );

        assert_eq!(
            xyz_to_bbox(0, 0, 0, Some(true), Some(Source::Google), Some(256)),
            (-20037508.342789244, -20037508.342789236, 20037508.342789244, 20037508.342789244),
            // "LL with int zoom value converts when antiMeridian=true & Source::Google"
        );

        assert_eq!(
            xyz_to_bbox(0, 0, 0, Some(true), Some(Source::WGS84), Some(256)),
            (-180.0, -85.05112877980659, 180.0, 85.0511287798066),
            // "LL with int zoom value converts when antiMeridian=true"
        );

        assert_eq!(
            xyz_to_bbox(0, 0, 1, Some(true), Some(Source::WGS84), Some(256)),
            (-180.0, -85.05112877980659, 0.0, 0.0),
            // "LL with int zoom value converts when antiMeridian=true"
        );

        assert_eq!(
            xyz_to_bbox(0, 0, 1, Some(true), Some(Source::WGS84), Some(256)),
            (-180.0, -85.05112877980659, 0.0, 0.0),
            // "LL with int zoom value converts when antiMeridian=true"
        );
    }

    #[test]
    fn test_convert_bbox() {
        assert_eq!(
            convert_bbox(
                (-20037508.342789244, -20037508.342789244, 20037508.342789244, 20037508.342789244),
                Source::WGS84
            ),
            (-180.00000000000003, -85.0511287798066, 180.00000000000003, 85.0511287798066),
        );
        assert_eq!(
            convert_bbox(
                (-180.00000000000003, -85.0511287798066, 180.00000000000003, 85.0511287798066),
                Source::Google
            ),
            (-20037508.342789244, -20037508.342789244, 20037508.342789244, 20037508.342789244),
        );

        assert_eq!(
            convert_bbox((-240., -90., 240., 90.), Source::Google),
            (-20037508.342789244, -20037508.342789244, 20037508.342789244, 20037508.342789244)
        );

        assert_eq!(
            bbox_to_xyz_bounds(
                (-240., -90., 240., 90.),
                4,
                Some(true),
                Some(Source::WGS84),
                Some(256)
            ),
            (0, 0, 15, 15)
        );
        assert_eq!(
            bbox_to_xyz_bounds(
                (-20037508.342789244, -20037508.342789244, 20037508.342789244, 20037508.342789244),
                4,
                Some(true),
                Some(Source::Google),
                Some(256)
            ),
            (0, 0, 15, 15)
        );
    }

    #[test]
    fn test_tile_to_bbox() {
        assert_eq!(tile_to_bbox((0, 0, 0), None), (0, 0, 512, 512));
        assert_eq!(tile_to_bbox((1, 0, 0), None), (0, 0, 512, 512));
        assert_eq!(tile_to_bbox((1, 1, 0), None), (512, 0, 1024, 512));
        assert_eq!(tile_to_bbox((2, 2, 2), None), (1024, 1024, 1536, 1536));
    }

    #[test]
    fn test_ll_to_tile() {
        assert_eq!(ll_to_tile((0.0, 0.0), 0, Some(512)), (0, 0));
        assert_eq!(ll_to_tile((-180.0, 85.05), 0, Some(512)), (0, 0));
        assert_eq!(ll_to_tile((0.0, 0.0), 1, Some(512)), (1, 1));
        assert_eq!(ll_to_tile((-180.0, 85.05), 1, Some(512)), (0, 0));
    }

    #[test]
    fn test_ll_to_tile_px() {
        assert_eq!(
            ll_to_tile_px((0.0, 0.0), (0, 0, 0), Some(512)),
            (0.5, 0.5),
            // "0-0-0: center point"
        );
        assert_eq!(
            ll_to_tile_px((0.0, 0.0), (2, 3, 3), Some(512)),
            (-1.0, -1.0),
            // "2-3-3: center point"
        );
        assert_eq!(
            ll_to_tile_px((0.0, 0.0), (0, 2, 0), Some(512)),
            (-1.5, 0.5),
            // "0-1-0: out of bounds tile with center point (used for world wrapping)"
        );
        assert_eq!(
            ll_to_tile_px((-180.0, 85.05), (0, 0, 0), Some(512)),
            (0.0, 0.00003634242909722474),
            // "0-0-0: top left"
        );
        assert_eq!(
            ll_to_tile_px((180.0, 85.05), (0, 0, 0), Some(512)),
            (1.0, 0.00003634242909722474),
            // "0-0-0: top right"
        );
        assert_eq!(
            ll_to_tile_px((180.0, -85.05), (0, 0, 0), Some(512)),
            (1.0, 0.9999636575709028),
            // "0-0-0: bottom right"
        );
        assert_eq!(
            ll_to_tile_px((-180.0, -85.05), (0, 0, 0), Some(512)),
            (0.0, 0.9999636575709028),
            // "0-0-0: bottom left"
        );
        assert_eq!(
            ll_to_tile_px((0.0, 0.0), (1, 0, 0), Some(512)),
            (1.0, 1.0),
            // "center for zoom 1 tiles"
        );
        assert_eq!(
            ll_to_tile_px((0.0, 0.0), (1, 1, 0), Some(512)),
            (0.0, 1.0),
            // "center for zoom 1 tiles"
        );
        assert_eq!(
            ll_to_tile_px((0.0, 0.0), (1, 0, 1), Some(512)),
            (1.0, 0.0),
            // "center for zoom 1 tiles"
        );
        assert_eq!(
            ll_to_tile_px((0.0, 0.0), (1, 1, 1), Some(512)),
            (0.0, 0.0),
            // "center for zoom 1 tiles"
        );
    }

    #[test]
    fn test_lng_from_mercator_x() {
        assert_eq!(lng_from_mercator_x(0.5), 0.);
        assert_eq!(lng_from_mercator_x(0.), -180.);
        assert_eq!(lng_from_mercator_x(1.), 180.);
    }

    #[test]
    fn test_lat_from_mercator_y() {
        assert_eq!(lat_from_mercator_y(0.5), 0.);
        assert_eq!(lat_from_mercator_y(1.), -85.05112877980659);
        assert_eq!(lat_from_mercator_y(0.0), 85.05112877980659);
        // out of bounds numbers
        assert_eq!(lat_from_mercator_y(2.), -89.99075251648904);
        assert_eq!(lat_from_mercator_y(-1.), 89.99075251648904);
    }

    #[test]
    fn test_lng_to_mercator_x() {
        assert_eq!(lng_to_mercator_x(0.0), 0.5);
        assert_eq!(lng_to_mercator_x(-180.), 0.);
        assert_eq!(lng_to_mercator_x(180.), 1.);
    }

    #[test]
    fn test_lat_to_mercator_y() {
        //         expect(mercatorYfromLat(0)).toEqual(0.5);
        //   expect(mercatorYfromLat(-85.05112877980659)).toEqual(0.9999999999999999);
        //   expect(mercatorYfromLat(85.05112877980659)).toEqual(-7.894919286223336e-17);
        //   // out of bounds numbers
        //   expect(mercatorYfromLat(90)).toEqual(-5.441549447954536);
        //   expect(mercatorYfromLat(-90)).toEqual(Infinity);
        assert_eq!(lat_to_mercator_y(0.0), 0.5);
        assert_eq!(lat_to_mercator_y(-85.05112877980659), 0.9999999999999999);
        assert_eq!(lat_to_mercator_y(85.05112877980659), -7.894919286223336e-17);
        // out of bounds numbers
        assert_eq!(lat_to_mercator_y(90.0), -5.441549447954536);
        assert_eq!(lat_to_mercator_y(-90.0), f64::INFINITY);
    }

    #[test]
    fn test_altitude_to_mercator_z() {
        assert_eq!(altitude_to_mercator_z(0., 0., None), 0.0);
        assert_eq!(altitude_to_mercator_z(1_000_000., 0., None), 0.0249811212145705);
        assert_eq!(altitude_to_mercator_z(1_000_000., 60., None), 0.04996224242914099);
    }

    #[test]
    fn test_altitude_from_mercator_z() {
        assert_eq!(altitude_from_mercator_z(0., 0., None), 0.);
        assert_eq!(altitude_from_mercator_z(0.0249811212145705, 0., None), 86_266.73833405455);
        assert_eq!(altitude_from_mercator_z(0.04996224242914099, 60., None), 1.224646799147353e-10);
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn test_mercator_lat_scale() {
        assert_eq!(mercator_lat_scale(0.0), 1.0);
        assert_eq!(mercator_lat_scale(45.0), 1.414213562373095);
        assert_eq!(mercator_lat_scale(-45.0), 1.414213562373095);
        assert_eq!(mercator_lat_scale(85.0), 11.47371324566986);
        assert_eq!(mercator_lat_scale(-85.0), 11.47371324566986);
    }
}
