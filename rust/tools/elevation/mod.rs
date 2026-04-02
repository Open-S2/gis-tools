/// Create isolines and isobands
pub mod contours;
/// Parse elevation grid data from an image
pub mod grid;
/// Generate Hillshade data
pub mod hillshade;
/// Triangular Mesh of elevation data
pub mod mesh;

pub use contours::*;
pub use grid::*;
pub use hillshade::*;
pub use mesh::*;

/// Generic elevation converter function
pub type ElevationConverter = fn(r: u8, g: u8, b: u8, a: Option<u8>) -> f64;

/// Conver a Terrarium tile encoded elevation data into a float precision elevation
/// Terrarium formula: (red * 256 + green + blue / 256) - 32768
pub fn convert_terrarium_elevation_data(r: u8, g: u8, b: u8, _a: Option<u8>) -> f64 {
    (r as f64) * 256.0 + (g as f64) + (b as f64) / 256.0 - 32768.0
}

/// Conver a Mapbox tile encoded elevation data into a float precision elevation
/// Mapbox formula: -10000 + (red * 256 * 256 + green * 256 + blue) * 0.1
pub fn convert_mapbox_elevation_data(r: u8, g: u8, b: u8, _a: Option<u8>) -> f64 {
    -10000. + ((r as f64) * 256. * 256. + (g as f64) * 256. + (b as f64)) * 0.1
}

/// Generic elevation encoder
pub type ElevationEncoder = fn(f64) -> (u8, u8, u8, Option<u8>);

/// Encode a float precision elevation into Terrarium tile encoded elevation data.
/// Terrarium formula: (red * 256 + green + blue / 256) - 32768
pub fn encode_terrarium_elevation_data(elevation: f64) -> (u8, u8, u8, Option<u8>) {
    let scaled_elevation = ((elevation + 32768.0) * 256.0).round() as u32;
    let r = ((scaled_elevation >> 16) & 0xFF) as u8;
    let g = ((scaled_elevation >> 8) & 0xFF) as u8;
    let b = (scaled_elevation & 0xFF) as u8;

    (r, g, b, None)
}

/// Encode a float precision elevation into Mapbox tile encoded elevation data
/// Mapbox formula: -10000 + (red * 256 * 256 + green * 256 + blue) * 0.1
pub fn encode_mapbox_elevation_data(elevation: f64) -> (u8, u8, u8, Option<u8>) {
    let scaled_elevation = ((elevation + 10000.0) * 10.0).round() as u32;
    let r = ((scaled_elevation >> 16) & 0xFF) as u8;
    let g = ((scaled_elevation >> 8) & 0xFF) as u8;
    let b = (scaled_elevation & 0xFF) as u8;

    (r, g, b, None)
}
