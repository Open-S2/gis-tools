use crate::{
    geometry::{GetTileID, TileID, WMTileID},
    parsers::{Buffer, ImageData},
};
use alloc::rc::Rc;

/// A tile grid guide on how to build a tile given source data
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TileGridGuide {
    /// Final grid position to start at (x, y)
    pub dest_offsets: (usize, usize),
    /// Where in the tile to start (x, y)
    pub src_offsets: (usize, usize),
    /// Write size (width, height)
    pub write_size: (usize, usize),
    /// What tile to use
    pub tile: TileID,
    /// Notify that we have a clamp
    pub clamp: Option<bool>,
    /// Image data (added later)
    pub image: Option<Rc<ImageData>>,
}

/// Given the WebMercator source tile and the padding, build a grid of tiles to render
///
/// ## Parameters
/// - `tile`: metadata for the position in the quad-tree
/// - `padding`: amount of padding to add to the tile
/// - `size`: the size of each tile (size x size)
/// - `wanted_size`: the size of the rendered tile. So if the source tiles are 256x256, you can set this to 512
/// - `is_tms`: if the tile scheme is TMS
///
/// ## Returns
/// The grid design to build/render a resultant tile
pub fn build_tile_grid_wm(
    tile: &TileID,
    padding: usize,
    size: usize,
    wanted_size: usize,
    is_tms: bool,
) -> Vec<TileGridGuide> {
    // if wanted_size % size != 0 {
    if !wanted_size.is_multiple_of(size) {
        panic!("wanted_size must be a multiple of size");
    }

    let scale = wanted_size / size;
    let depth_shift = scale.ilog2() as usize;
    let modulo = |n: isize, m: isize| ((n % m) + m) % m;

    // Increment the zoom level by the size shift
    let z_hi = tile.zoom() as usize + depth_shift;
    let zoom_tile_size = 1 << z_hi;

    // In TMS, Y=0 is bottom. In WM, Y=0 is top.
    // When subdividing, the "top" index changes based on the scheme.
    let x_hi_start = tile.x() as usize * scale;
    let y_hi_origin =
        if is_tms { tile.y() as usize * scale + (scale - 1) } else { tile.y() as usize * scale };

    let mut horizontal_strip: Vec<TileGridGuide> = vec![];

    // Center tiles of the strip
    for i in 0..scale {
        horizontal_strip.push(TileGridGuide {
            dest_offsets: (padding + i * size, 0), // Y adjusted later
            src_offsets: (0, 0),
            write_size: (size, size),
            tile: (WMTileID {
                zoom: z_hi as u8,
                x: modulo((x_hi_start + i) as isize, zoom_tile_size) as u32,
                y: y_hi_origin as u32,
            })
            .into(),
            clamp: None,
            image: None,
        });
    }

    // Left padding
    let mut remaining_left = padding as isize;
    let mut current_x_left = x_hi_start as isize;
    let mut current_offset_left = padding as isize;
    while remaining_left > 0 {
        current_x_left = modulo(current_x_left - 1, zoom_tile_size);
        let write_width = isize::min(remaining_left, size as isize);
        current_offset_left -= write_width;
        horizontal_strip.push(TileGridGuide {
            dest_offsets: (current_offset_left as usize, 0),
            src_offsets: ((size as isize - write_width) as usize, 0),
            write_size: (write_width as usize, size),
            tile: (WMTileID { zoom: z_hi as u8, x: current_x_left as u32, y: y_hi_origin as u32 })
                .into(),
            clamp: None,
            image: None,
        });
        remaining_left -= write_width;
    }

    // Right padding
    let mut remaining_right = padding as isize;
    let mut current_x_right = (x_hi_start + (scale - 1)) as isize;
    let mut current_offset_right = (padding + wanted_size) as isize;
    while remaining_right > 0 {
        current_x_right = modulo(current_x_right + 1, zoom_tile_size);
        let write_width = isize::min(remaining_right, size as isize);
        horizontal_strip.push(TileGridGuide {
            dest_offsets: (current_offset_right as usize, 0),
            src_offsets: (0, 0),
            write_size: (write_width as usize, size),
            tile: (WMTileID { zoom: z_hi as u8, x: current_x_right as u32, y: y_hi_origin as u32 })
                .into(),
            clamp: None,
            image: None,
        });
        current_offset_right += write_width;
        remaining_right -= write_width;
    }

    // Expand the horizontal strip vertically
    let mut final_grid: Vec<TileGridGuide> = vec![];

    for h in &horizontal_strip {
        let h_x = h.tile.x();
        let h_dest_x = h.dest_offsets.0;
        let h_src_x = h.src_offsets.0;
        let h_write_w = h.write_size.0;

        // Vertical: The "Wanted" Center Rows
        for i in 0..scale {
            // In TMS, as we go "down" the screen, Y decreases.
            let current_y = if is_tms { y_hi_origin - i } else { y_hi_origin + i };
            final_grid.push(TileGridGuide {
                dest_offsets: (h_dest_x, padding + i * size),
                src_offsets: (h_src_x, 0),
                write_size: (h_write_w, size),
                tile: (WMTileID { zoom: z_hi as u8, x: h_x, y: current_y as u32 }).into(),
                clamp: None,
                image: None,
            });
        }

        // Vertical: Padding Up
        let mut remaining_top = padding as isize;
        // let mut current_y_top = (if is_tms { y_hi_origin } else { y_hi_origin }) as isize;
        let mut current_y_top = y_hi_origin as isize;
        let mut current_offset_top = padding as isize;
        while remaining_top > 0 {
            let write_height = isize::min(remaining_top, size as isize);
            current_offset_top -= write_height;

            let next_y = if is_tms { current_y_top + 1 } else { current_y_top - 1 };
            let is_oob = if is_tms { next_y >= zoom_tile_size } else { next_y < 0 };
            if !is_oob {
                current_y_top = next_y;
            }

            final_grid.push(TileGridGuide {
                dest_offsets: (h_dest_x, current_offset_top as usize),
                src_offsets: (
                    h_src_x,
                    if is_oob {
                        if is_tms { size - 1 } else { 0 }
                    } else {
                        (size as isize - write_height) as usize
                    },
                ),
                write_size: (
                    h_write_w,
                    (if is_oob { remaining_top } else { write_height }) as usize,
                ),
                tile: (WMTileID { zoom: z_hi as u8, x: h_x, y: current_y_top as u32 }).into(),
                clamp: Some(is_oob),
                image: None,
            });
            remaining_top -= write_height;
            if is_oob {
                remaining_top = 0;
            }
        }

        // Vertical: Padding Down
        let mut remaining_bottom = padding as isize;
        // Bottom edge of the center block
        let mut current_y_bottom = if is_tms {
            y_hi_origin as isize - (scale as isize - 1)
        } else {
            y_hi_origin as isize + (scale as isize - 1)
        };
        let mut current_offset_bottom = (padding + wanted_size) as isize;
        while remaining_bottom > 0 {
            let write_height = isize::min(remaining_bottom, size as isize);

            let next_y: isize = if is_tms { current_y_bottom - 1 } else { current_y_bottom + 1 };
            let is_oob = if is_tms { next_y < 0 } else { next_y >= zoom_tile_size };
            if !is_oob {
                current_y_bottom = next_y;
            }

            final_grid.push(TileGridGuide {
                dest_offsets: (h_dest_x, current_offset_bottom as usize),
                src_offsets: (h_src_x, if is_oob { if is_tms { 0 } else { size - 1 } } else { 0 }),
                write_size: (
                    h_write_w,
                    (if is_oob { remaining_bottom } else { write_height }) as usize,
                ),
                tile: (WMTileID { zoom: z_hi as u8, x: h_x, y: current_y_bottom as u32 }).into(),
                clamp: Some(is_oob),
                image: None,
            });
            current_offset_bottom += write_height;
            remaining_bottom -= write_height;
            if is_oob {
                remaining_bottom = 0;
            }
        }
    }

    final_grid
}

/// Given input tile grid guide, merge the images into a single image
///
/// ## Parameters
/// - `grid`: the grid guides to merge with
/// - `size`: the size of the final image
/// - `padding`: the amount of padding that was applied
///
/// ## Returns
/// The merged image
pub fn merge_tile_grid_wm(grid: &[TileGridGuide], size: usize, padding: usize) -> ImageData {
    let dest_size = size + padding * 2;
    let mut data: Vec<u8> = vec![0; dest_size * dest_size * 4];

    for guide in grid {
        let TileGridGuide { dest_offsets, src_offsets, write_size, image, clamp, .. } = guide;
        let Some(image) = image else { continue };
        let source_channels = image.data.len() / (image.width * image.height);
        let source_alpha = source_channels >= 4;
        let (width, height) = *write_size;
        let (image_x, image_y) = *src_offsets;
        let image_data = image.data.buf();
        for y in 0..height {
            let source_y = if clamp.unwrap_or(false) { image_y } else { image_y + y };
            let target_y = dest_offsets.1 + y;
            for x in 0..width {
                let source_x = image_x + x;
                let target_x = dest_offsets.0 + x;
                let source_index = (source_y * image.width + source_x) * source_channels;
                let target_index = (target_y * dest_size + target_x) * 4;
                data[target_index] = image_data[source_index];
                data[target_index + 1] = image_data[source_index + 1];
                data[target_index + 2] = image_data[source_index + 2];
                data[target_index + 3] =
                    if source_alpha { image_data[source_index + 3] } else { 255 };
            }
        }
    }

    ImageData { width: dest_size, height: dest_size, data: Buffer::new(data) }
}
