use crate::{
    parsers::Buffer,
    tools::{ElevationConverter, get_elevation_grid},
};

/// The resultant mesh created from elevation data
pub struct TerrainMesh {
    /// The size of the grid
    pub grid_size: usize,
    /// The elevation data
    pub terrain: Vec<f64>,
    /// The vertices of the mesh
    pub vertices: Vec<u32>,
    /// The triangles of the mesh
    pub triangles: Vec<u32>,
}

/// # Build Terrain Mesh
///
/// ## Description
///
/// Builds a triangular mesh from elevation data. Useful for rendering elevation data as a 3D model.
///
/// This is a port of the [martini](https://github.com/mapbox/martini) codebase to be compatible with this library.
///
/// NOTE: Defaults to the Mapbox elevation data converter `convertMapboxElevationData`. However,
/// to use the Terrarium elevation data converter, use `convertTerrariumElevationData`.
///
/// NOTE: This algorithm is limited to a GRID, meaning both the width and height must be equal and a power of 2.
///
/// ## Examples
///
/// ```rust
/// use std::{fs, path::PathBuf};
/// use gistools::{
///     parsers::Buffer,
///     tools::build_terrain_mesh,
/// };
/// let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
/// path.push(format!("tests/tools/elevation/fixtures/fuji.png"));
/// let elevation_image = fs::read(path).unwrap();
/// let image_buffer = Buffer::from(elevation_image);
///
/// let res_mesh = build_terrain_mesh(&image_buffer, Some(500.0), None, None);
/// ```
///
/// ## Links
/// - <https://github.com/mapbox/martini/tree/main>
///
/// ## Parameters
/// `image`: the raw RGB(A) image data
/// `max_error`: The maximum error allowed in the mesh in meters
/// `elevation_converter`: the conversion function to convert the pixels to elevation
/// `tms_style`: if true, the y position will be inverted
///
/// ## Returns
/// The terrain mesh. The elevation values.
pub fn build_terrain_mesh(
    image: &Buffer,
    max_error: Option<f64>,
    elevation_converter: Option<ElevationConverter>,
    tms_style: Option<bool>,
) -> TerrainMesh {
    let max_error = max_error.unwrap_or(0.0);
    let grid = get_elevation_grid(image, elevation_converter, tms_style);

    // PREPARE MESH //
    let grid_size = grid.width + 1;
    let tile_size = grid.width;
    if (tile_size & (tile_size - 1)) != 0 {
        panic!("Expected grid size to be 2^n+1, got {grid_size}.");
    }
    let num_triangles = tile_size * tile_size * 2 - 2;
    let num_parent_triangles = num_triangles - tile_size * tile_size;

    // clone the elevation grid over to the terrain grid without the backfill
    let mut terrain: Vec<f64> = vec![0.0; grid_size * grid_size];
    for y in 0..tile_size {
        for x in 0..tile_size {
            terrain[y * grid_size + x] = grid.elevations[y * tile_size + x];
        }
    }
    // backfill right and bottom borders
    for x in 0..grid_size - 1 {
        terrain[grid_size * (grid_size - 1) + x] = terrain[grid_size * (grid_size - 2) + x];
    }
    for y in 0..grid_size {
        terrain[grid_size * y + grid_size - 1] = terrain[grid_size * y + grid_size - 2];
    }

    // coordinates for all possible triangles in an RTIN tile
    let mut errors = vec![0.0; terrain.len()];
    let mut coords: Vec<usize> = vec![0; num_triangles * 4];
    let mut indices: Vec<usize> = vec![0; grid_size * grid_size];

    // get triangle coordinates from its index in an implicit binary tree
    for i in 0..num_triangles {
        let mut id = i + 2;
        let mut ax = 0;
        let mut ay = 0;
        let mut bx = 0;
        let mut by = 0;
        let mut cx = 0;
        let mut cy = 0;
        if (id & 1) != 0 {
            // bottom-left triangle
            cx = tile_size;
            by = cx;
            bx = cx;
        } else {
            // top-right triangle
            cy = tile_size;
            ay = cy;
            ax = cy;
        }
        while {
            id >>= 1;
            id > 1
        } {
            let mx = (ax + bx) >> 1;
            let my = (ay + by) >> 1;

            if (id & 1) != 0 {
                // left half
                bx = ax;
                by = ay;
                ax = cx;
                ay = cy;
            } else {
                // right half
                ax = bx;
                ay = by;
                bx = cx;
                by = cy;
            }
            cx = mx;
            cy = my;
        }
        let k = i * 4;
        coords[k] = ax;
        coords[k + 1] = ay;
        coords[k + 2] = bx;
        coords[k + 3] = by;
    }

    // UPDATE //

    // iterate over all possible triangles, starting from the smallest level
    for i in (0..num_triangles).rev() {
        let k = i * 4;
        let ax = coords[k];
        let ay = coords[k + 1];
        let bx = coords[k + 2];
        let by = coords[k + 3];
        let mx = (ax + bx) >> 1;
        let my = (ay + by) >> 1;
        let cx = mx + my - ay;
        let cy = my + ax - mx;

        // calculate error in the middle of the long edge of the triangle
        let interpolated_height =
            (terrain[ay * grid_size + ax] + terrain[by * grid_size + bx]) / 2.;
        let middle_index = my * grid_size + mx;
        let middle_error = f64::abs(interpolated_height - terrain[middle_index]);

        errors[middle_index] = f64::max(errors[middle_index], middle_error);

        if i < num_parent_triangles {
            // bigger triangles; accumulate error with children
            let left_child_index = ((ay + cy) >> 1) * grid_size + ((ax + cx) >> 1);
            let right_child_index = ((by + cy) >> 1) * grid_size + ((bx + cx) >> 1);
            errors[middle_index] = f64::max(
                f64::max(errors[middle_index], errors[left_child_index]),
                errors[right_child_index],
            );
        }
    }

    // CREATE MESH //

    let mut num_vertices = 0;
    let mut res_triangles = 0;
    let igrid_size = grid_size as isize;
    let max_size = igrid_size - 1;

    // retrieve mesh in two stages that both traverse the error map:
    // - count_elements: find used vertices (and assign each an index), and count triangles (for minimum allocation)
    // - process_triangle: fill the allocated vertices & triangles typed arrays

    count_elements(
        0,
        0,
        max_size,
        max_size,
        max_size,
        0,
        igrid_size,
        max_error,
        &errors,
        &mut indices,
        &mut num_vertices,
        &mut res_triangles,
    );
    count_elements(
        max_size,
        max_size,
        0,
        0,
        0,
        max_size,
        igrid_size,
        max_error,
        &errors,
        &mut indices,
        &mut num_vertices,
        &mut res_triangles,
    );

    let mut vertices: Vec<u32> = vec![0; num_vertices * 2];
    let mut triangles: Vec<u32> = vec![0; res_triangles * 3];
    let mut tri_index = 0;

    process_triangle(
        0,
        0,
        max_size,
        max_size,
        max_size,
        0,
        igrid_size,
        max_error,
        &errors,
        &mut indices,
        &mut vertices,
        &mut triangles,
        &mut tri_index,
    );
    process_triangle(
        max_size,
        max_size,
        0,
        0,
        0,
        max_size,
        igrid_size,
        max_error,
        &errors,
        &mut indices,
        &mut vertices,
        &mut triangles,
        &mut tri_index,
    );

    TerrainMesh { grid_size, terrain, vertices, triangles }
}

#[allow(clippy::too_many_arguments)]
fn count_elements(
    ax: isize,
    ay: isize,
    bx: isize,
    by: isize,
    cx: isize,
    cy: isize,
    igrid_size: isize,
    max_error: f64,
    errors: &[f64],
    indices: &mut [usize],
    num_vertices: &mut usize,
    res_triangles: &mut usize,
) {
    let mx = (ax + bx) >> 1;
    let my = (ay + by) >> 1;

    if isize::abs(ax - cx) + isize::abs(ay - cy) > 1
        && errors[(my * igrid_size + mx) as usize] > max_error
    {
        count_elements(
            cx,
            cy,
            ax,
            ay,
            mx,
            my,
            igrid_size,
            max_error,
            errors,
            indices,
            num_vertices,
            res_triangles,
        );
        count_elements(
            bx,
            by,
            cx,
            cy,
            mx,
            my,
            igrid_size,
            max_error,
            errors,
            indices,
            num_vertices,
            res_triangles,
        );
    } else {
        let a_idx = (ay * igrid_size + ax) as usize;
        indices[a_idx] = if indices[a_idx] != 0 {
            indices[a_idx]
        } else {
            *num_vertices += 1;
            *num_vertices
        };
        let b_idx = (by * igrid_size + bx) as usize;
        indices[b_idx] = if indices[b_idx] != 0 {
            indices[b_idx]
        } else {
            *num_vertices += 1;
            *num_vertices
        };
        let c_idx = (cy * igrid_size + cx) as usize;
        indices[c_idx] = if indices[c_idx] != 0 {
            indices[c_idx]
        } else {
            *num_vertices += 1;
            *num_vertices
        };
        *res_triangles += 1;
    }
}

#[allow(clippy::too_many_arguments)]
fn process_triangle(
    ax: isize,
    ay: isize,
    bx: isize,
    by: isize,
    cx: isize,
    cy: isize,
    igrid_size: isize,
    max_error: f64,
    errors: &[f64],
    indices: &mut [usize],
    vertices: &mut [u32],
    triangles: &mut [u32],
    tri_index: &mut usize,
) {
    let mx = (ax + bx) >> 1;
    let my = (ay + by) >> 1;

    if isize::abs(ax - cx) + isize::abs(ay - cy) > 1
        && errors[(my * igrid_size + mx) as usize] > max_error
    {
        // triangle doesn't approximate the surface well enough; drill down further
        process_triangle(
            cx, cy, ax, ay, mx, my, igrid_size, max_error, errors, indices, vertices, triangles,
            tri_index,
        );
        process_triangle(
            bx, by, cx, cy, mx, my, igrid_size, max_error, errors, indices, vertices, triangles,
            tri_index,
        );
    } else {
        // add a triangle
        let a = indices[(ay * igrid_size + ax) as usize] - 1;
        let b = indices[(by * igrid_size + bx) as usize] - 1;
        let c = indices[(cy * igrid_size + cx) as usize] - 1;

        vertices[2 * a] = ax as u32;
        vertices[2 * a + 1] = ay as u32;
        vertices[2 * b] = bx as u32;
        vertices[2 * b + 1] = by as u32;
        vertices[2 * c] = cx as u32;
        vertices[2 * c + 1] = cy as u32;

        triangles[*tri_index] = a as u32;
        *tri_index += 1;
        triangles[*tri_index] = b as u32;
        *tri_index += 1;
        triangles[*tri_index] = c as u32;
        *tri_index += 1;
    }
}
