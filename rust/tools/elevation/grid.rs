use crate::{
    parsers::Buffer,
    tools::{ElevationConverter, convert_mapbox_elevation_data},
};

/// A grid of elevations
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct IsoGrid {
    /// The width of the grid
    pub width: usize,
    /// The height of the grid
    pub height: usize,
    /// The minimum elevation value
    pub min: f64,
    /// The maximum elevation value
    pub max: f64,
    /// The elevations
    pub elevations: Vec<f64>,
}

/// Build a grid from image data and a conversion tool
///
/// NOTE: Defaults to the Mapbox elevation data converter [`convert_mapbox_elevation_data`]. However,
/// to use the Terrarium elevation data converter, use [`crate::readers::convert_terrarium_elevation_data`].
///
/// ## Parameters
/// - `image_data`: image data
/// - `elevation_converter`: the conversion tool
/// - `tms_style`: if true, the y position will be inverted
///
/// ## Returns
/// The elevation grid
#[cfg(feature = "std")]
pub fn get_elevation_grid(
    image_data: &Buffer,
    elevation_converter: Option<ElevationConverter>,
    tms_style: Option<bool>,
) -> IsoGrid {
    use crate::parsers::{ImageData, image_decoder};

    let elevation_converter = elevation_converter.unwrap_or(convert_mapbox_elevation_data);
    let tms_style = tms_style.unwrap_or(false);
    let Ok(ImageData { width, height, data, .. }) = image_decoder(image_data, None) else {
        panic!("Failed to decode image");
    };
    let channels = data.len() / (width * height);
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;

    let mut elevations = vec![0.; width * height];
    for j in 0..height {
        let actual_k = if tms_style { height - j - 1 } else { j };
        let row_offset = actual_k * width * channels;
        let output_row_offset = actual_k * width;
        for i in 0..width {
            let index = row_offset + i * channels;
            let alpha = if channels > 3 { data.get_u8_at(index + 3) } else { 255 };
            let elevation = elevation_converter(
                data.get_u8_at(index),
                data.get_u8_at(index + 1),
                data.get_u8_at(index + 2),
                Some(alpha),
            );
            min = f64::min(min, elevation);
            max = f64::max(max, elevation);
            elevations[output_row_offset + i] = elevation;
        }
    }

    IsoGrid { width, height, min, max, elevations }
}
