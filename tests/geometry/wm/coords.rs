#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::geometry::{
        Source, altitude_from_mercator_z, altitude_to_mercator_z, bbox_to_xyz_bounds, convert_bbox,
        lat_from_mercator_y, lat_to_mercator_y, ll_to_merc, ll_to_px, ll_to_tile, ll_to_tile_px,
        lng_from_mercator_x, lng_to_mercator_x, merc_to_ll, mercator_lat_scale, px_to_ll,
        px_to_tile, tile_to_bbox, xyz_to_bbox,
    };

    #[test]
    fn test_ll_to_px() {
        assert_eq!(
            ll_to_px(&(-179.0, 85.0), 9., Some(true), Some(256)),
            (364.0888888888876, 214.68476683766494),
            // "PX with int zoom value converts when antiMeridian=true"
        );
        assert_eq!(
            ll_to_px(&(-179.0, 85.0), 9., Some(false), Some(256)),
            (364.0888888888876, 214.68476683766494),
            // "PX with int zoom value converts when antiMeridian=false"
        );
        assert_eq!(
            ll_to_px(&(250.0, 3.0), 4., Some(false), Some(256)),
            (4096.0, 2013.8510595566413),
            // "Clamps PX by default when lon >180 when antiMeridian=false"
        );
        assert_eq!(
            ll_to_px(&(250.0, 3.0), 4., Some(true), Some(256)),
            (4892.444444444444, 2013.8510595566413),
            // "PX with lon > 180 converts when antimeridian=true"
        );
        assert_eq!(
            ll_to_px(&(400.0, 3.0), 4., Some(true), Some(256)),
            (6599.111111111111, 2013.8510595566413),
            // "Clamps PX when lon >360 and antimeridian=true"
        );
        assert_eq!(
            ll_to_px(&(400.0, 3.0), 4., Some(false), Some(256)),
            (4096.0, 2013.8510595566413),
            // "Clamps PX when lon >360 and antimeridian=false"
        );
    }

    #[test]
    fn test_px_to_ll() {
        assert_eq!(
            px_to_ll(&(200.0, 200.0), 9., Some(256)),
            (-179.45068359375, 85.00351401304403),
            // "LL with int zoom value converts when antiMeridian=true"
        );
    }

    #[test]
    fn test_ll_to_merc() {
        assert_eq!(
            ll_to_merc(&(0.0, 0.0)),
            (0.0, -7.081154551613622e-10),
            // "LL with int zoom value converts when antiMeridian=true"
        );
        assert_eq!(
            ll_to_merc(&(-180.0, 90.0)),
            (-20037508.342789244, 20037508.342789244),
            // "LL with int zoom value converts when antiMeridian=true"
        );
        assert_eq!(
            ll_to_merc(&(180.0, -90.0)),
            (20037508.342789244, -20037508.342789244),
            // "LL with int zoom value converts when antiMeridian=true"
        );
    }

    #[test]
    fn test_merc_to_ll() {
        assert_eq!(
            merc_to_ll(&(0.0, -7.081154551613622e-10)),
            (0.0, 0.0),
            // "LL with int zoom value converts when antiMeridian=true"
        );
        assert_eq!(
            merc_to_ll(&(-20037508.34278924, 20037508.342789244)),
            (-179.99999999999997, 85.0511287798066),
            // "LL with int zoom value converts when antiMeridian=true"
        );
        assert_eq!(
            merc_to_ll(&(20037508.34278924, -20037508.342789244)),
            (179.99999999999997, -85.0511287798066),
            // "LL with int zoom value converts when antiMeridian=true"
        );
    }

    #[test]
    fn test_px_to_tile() {
        assert_eq!(
            px_to_tile(&(0.0, 0.0), Some(512)),
            (0, 0),
            // "LL with int zoom value converts when antiMeridian=true"
        );
        assert_eq!(
            px_to_tile(&(600.0, 2000.0), Some(512)),
            (1, 3),
            // "LL with int zoom value converts when antiMeridian=true"
        );
    }

    #[test]
    fn test_xyz_to_bbox() {
        assert_eq!(
            xyz_to_bbox(0, 0, 0., Some(true), Some(Source::WGS84), Some(256)),
            (-180.0, -85.05112877980659, 180.0, 85.0511287798066),
            // "LL with int zoom value converts when antiMeridian=true"
        );

        assert_eq!(
            xyz_to_bbox(0, 0, 0., Some(true), Some(Source::Google), Some(256)),
            (-20037508.342789244, -20037508.342789236, 20037508.342789244, 20037508.342789244),
            // "LL with int zoom value converts when antiMeridian=true & Source::Google"
        );

        assert_eq!(
            xyz_to_bbox(0, 0, 0., Some(true), Some(Source::WGS84), Some(256)),
            (-180.0, -85.05112877980659, 180.0, 85.0511287798066),
            // "LL with int zoom value converts when antiMeridian=true"
        );

        assert_eq!(
            xyz_to_bbox(0, 0, 1., Some(true), Some(Source::WGS84), Some(256)),
            (-180.0, -85.05112877980659, 0.0, 0.0),
            // "LL with int zoom value converts when antiMeridian=true"
        );

        assert_eq!(
            xyz_to_bbox(0, 0, 1., Some(true), Some(Source::WGS84), Some(256)),
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
                4.,
                Some(true),
                Some(Source::WGS84),
                Some(256)
            ),
            (0, 0, 15, 15)
        );
        assert_eq!(
            bbox_to_xyz_bounds(
                (-20037508.342789244, -20037508.342789244, 20037508.342789244, 20037508.342789244),
                4.,
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
        assert_eq!(ll_to_tile(&(0.0, 0.0), 0., Some(512)), (0, 0));
        assert_eq!(ll_to_tile(&(-180.0, 85.05), 0., Some(512)), (0, 0));
        assert_eq!(ll_to_tile(&(0.0, 0.0), 1., Some(512)), (1, 1));
        assert_eq!(ll_to_tile(&(-180.0, 85.05), 1., Some(512)), (0, 0));
    }

    #[test]
    fn test_ll_to_tile_px() {
        assert_eq!(
            ll_to_tile_px(&(0.0, 0.0), (0, 0, 0), Some(512)),
            (0.5, 0.5),
            // "0-0-0: center point"
        );
        assert_eq!(
            ll_to_tile_px(&(0.0, 0.0), (2, 3, 3), Some(512)),
            (-1.0, -1.0),
            // "2-3-3: center point"
        );
        assert_eq!(
            ll_to_tile_px(&(0.0, 0.0), (0, 2, 0), Some(512)),
            (-1.5, 0.5),
            // "0-1-0: out of bounds tile with center point (used for world wrapping)"
        );
        assert_eq!(
            ll_to_tile_px(&(-180.0, 85.05), (0, 0, 0), Some(512)),
            (0.0, 0.00003634242909722474),
            // "0-0-0: top left"
        );
        assert_eq!(
            ll_to_tile_px(&(180.0, 85.05), (0, 0, 0), Some(512)),
            (1.0, 0.00003634242909722474),
            // "0-0-0: top right"
        );
        assert_eq!(
            ll_to_tile_px(&(180.0, -85.05), (0, 0, 0), Some(512)),
            (1.0, 0.9999636575709028),
            // "0-0-0: bottom right"
        );
        assert_eq!(
            ll_to_tile_px(&(-180.0, -85.05), (0, 0, 0), Some(512)),
            (0.0, 0.9999636575709028),
            // "0-0-0: bottom left"
        );
        assert_eq!(
            ll_to_tile_px(&(0.0, 0.0), (1, 0, 0), Some(512)),
            (1.0, 1.0),
            // "center for zoom 1 tiles"
        );
        assert_eq!(
            ll_to_tile_px(&(0.0, 0.0), (1, 1, 0), Some(512)),
            (0.0, 1.0),
            // "center for zoom 1 tiles"
        );
        assert_eq!(
            ll_to_tile_px(&(0.0, 0.0), (1, 0, 1), Some(512)),
            (1.0, 0.0),
            // "center for zoom 1 tiles"
        );
        assert_eq!(
            ll_to_tile_px(&(0.0, 0.0), (1, 1, 1), Some(512)),
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
