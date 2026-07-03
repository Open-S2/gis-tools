use crate::{
    geometry::{GetTileID, TileID},
    parsers::Buffer,
    tools::{ElevationConverter, IsoGrid, build_isobands, get_elevation_grid},
};
use alloc::collections::BTreeMap;
use core::f64::consts::PI;
use libm::{atan, atan2, cos, pow, sin, sqrt};
use s2json::{FeatureCollection, Features, MValue, Point, VectorFeature, VectorGeometry};
use serde::{Deserialize, Serialize};

/// Contour Properties
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, MValue)]
pub struct HillshadeProperties {
    /// hillshade type as a user-defined name
    pub hillshade: String,
}

// https://pro.arcgis.com/en/pro-app/latest/tool-reference/3d-analyst/how-hillshade-works.htm

/// Result of generating greyscale hillshade data
pub struct HillshadeResult {
    /// The width of the tile
    pub width: usize,
    /// The height of the tile
    pub height: usize,
    /// The raw hillshade data as f64 but scaled to 0-255 (greyscale u8s)
    pub hillshade: Vec<f64>,
}

/// # Vectorize hillshade data
///
/// ## Description
/// Generate vectorized hillshade data with lights and darks
///
/// ## Example
/// ```rust
/// use std::fs;
/// use gistools::{geometry::TileID, parsers::Buffer, tools::vectorize_hillshade};
///
/// let elevation_image = fs::read("tests/tools/elevation/fixtures/13_1544_3085.webp").unwrap();
/// let vector_hillshade = vectorize_hillshade(
///     &Buffer::new(elevation_image),
///     TileID::new_wm(13, 1544, 3085),
///     None,
///     None,
///     None,
///     None,
///     None,
///     None,
///     None,
///     None,
///     None,
/// );
/// ```
///
/// ## Links
/// - <https://pro.arcgis.com/en/pro-app/latest/tool-reference/3d-analyst/how-hillshade-works.htm>
///
/// ## Parameters
/// - `image_data`: the raw RGB(A) image data
/// - `tile`: The zoom, x, and y of the tile
/// - `elevation_converter`: the conversion function to convert the pixels to elevation
/// - `tms_style`: if true, the y position will be inverted
/// - `azimuth`: The azimuth of the sun
/// - `altitude`: The altitude of the sun
/// - `z_factor`: The zFactor effects the weight of the sun's light ouput
/// - `thresholds`: The thresholds for the lights and darks to generate
/// - `weights`: The weights of the azimuths. The first azimuth is the sun's azimuth value-eter. Each subsequent azimuth is the angle away from the sun.
/// - `padding`: The padding to add to the image. This is needed to account for the edge of the tile.
/// - `tolerance`: The tolerance of the simplification of the lines using the Ramer-Douglas-Peucker algorithm
///
/// ## Returns
/// An image/array of grayscale values
#[allow(clippy::too_many_arguments)]
pub fn vectorize_hillshade(
    image_data: &Buffer,
    tile: TileID,
    elevation_converter: Option<ElevationConverter>,
    tms_style: Option<bool>,
    azimuth: Option<f64>,
    altitude: Option<f64>,
    z_factor: Option<f64>,
    thresholds: Option<BTreeMap<u8, String>>,
    weights: Option<[f64; 4]>,
    padding: Option<usize>,
    tolerance: Option<f64>,
) -> FeatureCollection {
    let padding = padding.unwrap_or(0);
    let tolerance = tolerance.unwrap_or(1. / 2_096.);
    let z_factor = z_factor.unwrap_or(1.5);
    let HillshadeResult { width, height, hillshade } = generate_hillshade(
        image_data,
        tile,
        elevation_converter,
        tms_style,
        azimuth,
        altitude,
        Some(z_factor),
        weights,
        Some(true),
    );
    let hillshade: Vec<f64> = hillshade.to_vec();
    let hillshade_invert: Vec<f64> = hillshade.iter().map(|x| 255. - x).collect();
    let thresholds = thresholds.unwrap_or(BTreeMap::from([
        (220, "dark".into()),
        (235, "darker".into()),
        (60, "light".into()),
        (40, "lighter".into()),
        (15, "lightest".into()),
    ]));

    let mut features: Vec<VectorFeature> = vec![];
    for (elev, name) in thresholds {
        let elev = elev as f64;
        let hill = if elev >= 127.5 { &hillshade } else { &hillshade_invert };
        let threshold = if elev >= 127.5 { elev } else { 255.0 - elev };
        let polygons = build_isobands(hill, threshold, width, height, padding, Some(tolerance));
        features.push(VectorFeature::new_wm(
            None,
            (HillshadeProperties { hillshade: name }).into(),
            VectorGeometry::new_multipolygon(polygons, None),
            None,
        ));
    }

    let mut feature_collection = FeatureCollection::new(None);
    feature_collection.features = features.into_iter().map(Features::VectorFeature).collect();
    feature_collection
}

/// # Build greyscale hillshade data
///
/// ## Description
/// Builds an array of grayscale values for a given tile.
///
/// ## Example
///
/// ```rust
/// use gistools::{
///     geometry::TileID,
///     parsers::Buffer,
///     tools::generate_hillshade,
/// };
/// use std::fs;
///
/// let elevation_image = fs::read("tests/tools/elevation/fixtures/13_1544_3085.webp").unwrap();
/// let res = generate_hillshade(
///     &Buffer::new(elevation_image),
///     TileID::new_wm(13, 1544, 3085),
///     None,
///     None,
///     None,
///     None,
///     None,
///     None,
///     None,
/// );
///
/// assert_eq!(res.width, 514);
/// assert_eq!(res.height, 514);
/// assert_eq!(res.hillshade.len(), 514 * 514);
/// ```
///
/// ## Parameters
/// - `image_data`: the raw RGB(A) image data
/// - `tile`: The zoom, x, and y of the tile
/// - `elevation_converter`: the conversion function to convert the pixels to elevation
/// - `tms_style`: if true, the y position will be inverted
/// - `azimuth`: The azimuth of the sun
/// - `altitude`: The altitude of the sun
/// - `z_factor`: The zFactor effects the weight of the sun's light ouput
/// - `weights`: The weights of the azimuths. The first azimuth is the sun's azimuth value parameter. Each subsequent azimuth is the angle away from the sun.
///
/// ## Returns
/// An image/array of grayscale values
#[allow(clippy::too_many_arguments)]
pub fn generate_hillshade(
    image_data: &Buffer,
    tile: TileID,
    elevation_converter: Option<ElevationConverter>,
    tms_style: Option<bool>,
    azimuth: Option<f64>,
    altitude: Option<f64>,
    z_factor: Option<f64>,
    weights: Option<[f64; 4]>,
    smooth: Option<bool>,
) -> HillshadeResult {
    let azimuth = azimuth.unwrap_or(315.0);
    let altitude = altitude.unwrap_or(45.0);
    let z_factor = z_factor.unwrap_or(1.0);
    let weights = weights.unwrap_or([0.65, 0.15, 0.15, 0.05]);
    // create the elevation grid
    let elevation_grid = get_elevation_grid(image_data, elevation_converter, tms_style);
    let IsoGrid { width, height, elevations, .. } = elevation_grid;
    // remove padding pixels for future calculations
    let corrected_width = width - (width % 256);
    let mut hillshade = vec![0.; width * height];

    let zenith_rad = (90.0 - altitude).to_radians();
    let azimuth_sources = [
        azimuth,       // Primary
        azimuth - 45., // Secondary 1
        azimuth + 45., // Secondary 2
        azimuth + 90., // Secondary 3
    ]
    .map(|a| {
        let math_a = (360. - a + 90.) % 360.;
        math_a.to_radians()
    });

    let Point(_, lat) = tile.to_center_lon_lat(tms_style);
    let lat_rad = lat.to_radians();
    let cell_size =
        (40075016.686 * cos(lat_rad)) / (pow(2., tile.zoom() as f64) * corrected_width as f64);

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            // 1. Get 3x3 window
            let a = elevations[(y - 1) * width + (x - 1)];
            let b = elevations[(y - 1) * width + x];
            let c = elevations[(y - 1) * width + (x + 1)];
            let d = elevations[y * width + (x - 1)];
            // let e = elevations[y * width + x];
            let f = elevations[y * width + (x + 1)];
            let g = elevations[(y + 1) * width + (x - 1)];
            let h = elevations[(y + 1) * width + x];
            let i = elevations[(y + 1) * width + (x + 1)];

            // 2. Calculate dz/dx and dz/dy (Horn's Method)
            // [dz/dx] = ((c + 2f + i) - (a + 2d + g)) / (8 * cellsize)
            let dzdx = (z_factor * (c + 2. * f + i - (a + 2. * d + g))) / (8. * cell_size);
            // [dz/dy] = ((g + 2h + i) - (a + 2b + c)) / (8 * cellsize)
            let dzdy = (z_factor * (g + 2. * h + i - (a + 2. * b + c))) / (8. * cell_size);

            // 3. Calculate Aspect and Slope
            let rise_run = sqrt(dzdx * dzdx + dzdy * dzdy);
            let slope_rad = atan(rise_run);

            let mut aspect_rad = 0.;
            // Only calculate aspect if the slope is not perfectly flat
            if rise_run > 0.0001 {
                aspect_rad = atan2(dzdy, -dzdx);
                if aspect_rad < 0. {
                    aspect_rad += 2. * PI;
                }
            }

            // 4. Calculate Multi-directional Illumination
            let mut total_illumination = 0.;

            for i in 0..azimuth_sources.len() {
                let source_azimuth_rad = azimuth_sources[i];
                let weight = weights[i];

                let illumination = cos(zenith_rad) * cos(slope_rad)
                    + sin(zenith_rad) * sin(slope_rad) * cos(source_azimuth_rad - aspect_rad);

                // Weighted sum
                total_illumination += f64::max(0., illumination) * weight;
            }

            let val = total_illumination * 255.;
            hillshade[y * width + x] = val;
        }
    }

    // Pad top and bottom rows
    for i in 0..width {
        hillshade[i] = hillshade[width + i];
        hillshade[(height - 1) * width + i] = hillshade[(height - 2) * width + i];
    }
    // Pad left and right columns
    for j in 0..height {
        hillshade[j * width] = hillshade[j * width + 1];
        hillshade[j * width + (width - 1)] = hillshade[j * width + (width - 2)];
    }

    if smooth == Some(true) {
        hillshade = smooth_hillshade(&hillshade, width, height);
    }

    HillshadeResult { width, height, hillshade }
}

// Simple 3x3 Mean Filter to "clean up" the hillshade a bit. works a lot like a gaussian blur
fn smooth_hillshade(data: &[f64], width: usize, height: usize) -> Vec<f64> {
    let mut smoothed = vec![0.; width * height];
    // only smooth inner cells
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let mut sum: f64 = 0.;
            for ky in -1..=1 {
                for kx in -1..=1 {
                    sum +=
                        data[((y as isize + ky) as usize) * width + ((x as isize + kx) as usize)];
                }
            }
            smoothed[y * width + x] = sum / 9.;
        }
    }
    // refill top and bottom rows:
    for i in 0..width {
        smoothed[i] = smoothed[width + i];
        smoothed[(height - 1) * width + i] = smoothed[(height - 2) * width + i];
    }
    // refill left and right columns:
    for j in 0..height {
        smoothed[j * width] = smoothed[j * width + 1];
        smoothed[j * width + (width - 1)] = smoothed[j * width + (width - 2)];
    }

    smoothed
}
