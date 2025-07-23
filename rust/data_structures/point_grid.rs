use super::{IndexPoint, PointIndex};
use crate::{
    data_store::{KV, KVStore, Vector, VectorStore},
    geometry::{LonLat, S2CellId, S2Point},
    parsers::{FeatureReader, RGBA},
    util::{
        GetInterpolateValue, Interpolatable, InterpolationFunction, InterpolationMethod,
        get_interpolation,
    },
};
use alloc::{collections::BTreeSet, fmt::Debug, string::String, vec, vec::Vec};
use libm::{floor, log2};
use s2json::{BBox, Face, GetXY, JSONCollection, Projection};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

/// Options for grid clustering
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GridOptions<V: Clone + Debug> {
    /// projection to use
    pub projection: Option<Projection>,
    /// Name of the layer to build when requesting a tile
    pub layer_name: Option<String>,
    /// min zoom to generate clusters on
    pub minzoom: Option<u8>,
    /// max zoom level to cluster the points on
    pub maxzoom: Option<u8>,
    /// Used by cell search to specify the type of interpolation to use.
    /// The recommendation is IDW as you want to prioritize closest data points. [default: 'idw']
    #[serde(skip)]
    pub maxzoom_interpolation: Option<InterpolationMethod>,
    /// Used by cell search to specify the type of interpolation to use.
    /// From experimentation, lanczos is a fast algorithm that maintains the quality of the data
    /// [default: 'lanczos']
    #[serde(skip)]
    pub interpolation: Option<InterpolationMethod>,
    /// Used by cell search to specify the interpolation function to use [default: the Default of the Value type]
    #[serde(skip)]
    pub get_value: Option<GetInterpolateValue<IndexPoint<V>, V>>,
    /// Grid size, assumed pixel ratio.
    pub grid_size: Option<u32>,
    /// Used by the cell search to specify the tile buffer size in pixels. [default: 0]
    pub buffer_size: Option<u32>,
}

/// An export of the data as a grid
#[derive(Debug)]
pub struct TileGrid<'a, V> {
    /// name of the layer
    pub name: String,
    /// size of the grid including the buffer
    pub size: u32,
    /// flattened array of number or RGBA.
    /// The size of the array is grid_size * grid_size
    /// Access the position as `grid_size * y + x`
    pub data: &'a Vec<V>,
}

/// A Point Grid that uses locally allocated data to store the data
pub type LocalPointGrid<V = RGBA> =
    PointGrid<V, Vector<S2CellId, IndexPoint<V>>, KV<S2CellId, Vec<V>>>;

/// # Grid Cluster
///
/// ## Description
/// A cluster store to build grid data of grid_size x grid_size. The resultant tiles are filled.
/// Useful for building raster tiles or other grid like data (temperature, precipitation, wind, etc).
#[derive(Debug)]
pub struct PointGrid<
    V: Interpolatable + Serialize + DeserializeOwned + Debug = RGBA,
    S: VectorStore<S2CellId, IndexPoint<V>> = Vector<S2CellId, IndexPoint<V>>,
    G: KVStore<S2CellId, Vec<V>> = KV<S2CellId, Vec<V>>,
> {
    projection: Projection,
    layer_name: String,
    minzoom: u8,
    maxzoom: u8,
    buffer_size: u32,
    maxzoom_interpolation: InterpolationFunction<S2Point, IndexPoint<V>, V>,
    interpolation: InterpolationFunction<S2Point, IndexPoint<V>, V>,
    get_value: GetInterpolateValue<IndexPoint<V>, V>,
    grid_size: u32, // a default is a 512x512 pixel tile
    point_index: PointIndex<V, S>,
    grid_tile_store: G,
}
impl<
    V: Interpolatable + Serialize + DeserializeOwned + Debug,
    S: VectorStore<S2CellId, IndexPoint<V>>,
    G: KVStore<S2CellId, Vec<V>>,
> PointGrid<V, S, G>
{
    /// Create a new point grid
    pub fn new(options: Option<GridOptions<V>>) -> Self {
        let options = options.unwrap_or_default();
        let maxzoom_interpolation =
            options.maxzoom_interpolation.unwrap_or(InterpolationMethod::IDW);
        let interpolation = options.interpolation.unwrap_or(InterpolationMethod::Lanczos);
        let projection = options.projection.unwrap_or(Projection::S2);
        PointGrid {
            projection,
            maxzoom_interpolation: get_interpolation(maxzoom_interpolation),
            interpolation: get_interpolation(interpolation),
            get_value: options.get_value.unwrap_or(|v| v.data),
            buffer_size: options.buffer_size.unwrap_or(0),
            minzoom: options.minzoom.unwrap_or(0),
            maxzoom: options.maxzoom.unwrap_or(16),
            layer_name: options.layer_name.unwrap_or(String::from("default")),
            grid_size: options.grid_size.unwrap_or(512),
            point_index: PointIndex::new(None, Some(projection)),
            grid_tile_store: G::new(None),
        }
    }

    // /// Insert a cell with the point and its corresponding data to the index
    // pub fn insert(&mut self, cell: S2CellId, point: S2Point, data: Option<V>) {
    //     self.point_index.insert(cell, point, data);
    // }

    /// Add a lon-lat pair as with any shape
    pub fn insert_point<T: GetXY>(&mut self, point: T, data: Option<V>) {
        self.point_index.insert_point(point, data);
    }

    /// Add a lon-lat pair to the cluster
    pub fn insert_lon_lat(&mut self, ll: LonLat<V>) {
        self.point_index.insert_lon_lat(ll);
    }

    /// Insert an STPoint to the index
    pub fn insert_face_st(&mut self, face: Face, s: f64, t: f64, data: V) {
        self.point_index.insert_face_st(face, s, t, data);
    }

    /// Add all points from a reader. It will try to use the M-value first, but if it doesn't exist
    /// it will use the feature properties data
    pub fn insert_reader<L: Clone, F: FeatureReader<L, V, V>>(&mut self, reader: &F) {
        self.point_index.insert_reader::<L, F>(reader);
    }

    /// Add a vector feature. It will try to use the M-value first, but if it doesn't exist
    /// it will use the feature properties data
    pub fn insert_feature<L: Clone>(&mut self, data: JSONCollection<L, V, V>) {
        self.point_index.insert_feature::<L>(data);
    }

    ///  Get the point data as a grid of a tile
    pub fn get_tile(&self, id: S2CellId) -> Option<TileGrid<'_, V>> {
        let Self { layer_name, grid_size, buffer_size, .. } = self;
        let data = self.grid_tile_store.get(id);

        if let Some(data) = data {
            return Some(TileGrid {
                name: layer_name.clone(),
                size: grid_size + buffer_size * 2,
                data,
            });
        }

        None
    }

    /// Build the grid cluster tiles
    pub fn build_clusters(&mut self) {
        // build tiles at maxzoom
        let mut parents = self.cluster_maxzoom();
        // work upwards, take the 4 children and merge them
        for zoom in (self.minzoom..self.maxzoom).rev() {
            parents = self.custer_zoom(zoom, &parents);
        }
    }

    /// Using the point index, build grids at maxzoom by doing searches for each gridpoint.
    /// return the parent cells
    fn cluster_maxzoom(&mut self) -> BTreeSet<S2CellId> {
        let mut parents = BTreeSet::<S2CellId>::new();
        let grid_length = (self.grid_size + self.buffer_size * 2) as usize;
        let buffer_size_f64 = self.buffer_size as f64;
        let grid_size_f64: f64 = self.grid_size as f64;
        // if the grid is 512 x 512, log2 is 9, meaning the quadtree must split 9 times to analyze
        // each individual pixel. Make sure we don't dive past 30 levels as that's the limit of the spec.
        let zoom_grid_level =
            f64::min(self.maxzoom as f64 + floor(log2(grid_size_f64)) - 1., 30.) as u8;

        self.point_index.sort();
        for (cell, _) in self.point_index.iter() {
            let maxzoom_id = cell.parent(Some(self.maxzoom)); // idParent(cell, maxzoom);
            // if maxzoom_id grid tile already exists, skip
            if self.grid_tile_store.has(maxzoom_id) {
                continue;
            }
            // prep variables and grid result
            let face = cell.face();
            let BBox { left: s_min, bottom: t_min, right: s_max, top: t_max } =
                maxzoom_id.bounds_st(Some(self.maxzoom));
            let s_pixel = (s_max - s_min) / grid_size_f64;
            let t_pixel = (t_max - t_min) / grid_size_f64;
            let s_start = s_min - s_pixel * buffer_size_f64;
            let t_start = t_min - t_pixel * buffer_size_f64;
            let mut grid: Vec<V> = vec![V::default(); grid_length * grid_length];
            // iterate through the grid and do searches for each position. Interpolate the data to the
            // position and store the result in the grid.
            for y in 0..grid_length {
                for x in 0..grid_length {
                    let t = t_start + (y as f64) * t_pixel;
                    let mut s = s_start + (x as f64) * s_pixel;
                    if self.projection == Projection::WG {
                        s = (s + 1.) % 1.;
                    } // ensure within 0-1 range via wrapping to the other side
                    // search for points within a reasonable cell size
                    let mut grid_level_search = zoom_grid_level;
                    let mut point_shapes: Vec<IndexPoint<V>>;
                    let fst_point = S2CellId::from_face_st(face, s, t);
                    let mut st_cell = fst_point.parent(Some(zoom_grid_level));
                    loop {
                        point_shapes = self
                            .point_index
                            .search_range(st_cell, None, None)
                            .into_iter()
                            .map(|(_, point)| point.clone())
                            .collect();
                        if !point_shapes.is_empty()
                            || grid_level_search <= zoom_grid_level - 3
                            || grid_level_search == 0
                        {
                            break;
                        }
                        grid_level_search -= 1;
                        st_cell = st_cell.parent(Some(grid_level_search));
                    }
                    if point_shapes.is_empty() {
                        continue;
                    }
                    let point: S2Point = fst_point.to_point();
                    grid[y * grid_length + x] =
                        (self.maxzoom_interpolation)(&point, &point_shapes, self.get_value);
                }
            }
            // store the grid and add the parent cell for future upscaling
            self.grid_tile_store.set(maxzoom_id, grid);
            if self.maxzoom != 0 {
                parents.insert(maxzoom_id.parent(Some(self.maxzoom - 1)));
            }
        }

        parents
    }

    /// Build the parent cells. We simply search for the children of the cell and merge/downsample.
    ///
    /// ## Parameters
    /// - `zoom`: the current zoom we are upscaling to
    /// - `cells`: the cells to build grids for
    ///
    /// ## Returns
    /// returns the parent cells for the next round of upscaling
    fn custer_zoom(&mut self, zoom: u8, cells: &BTreeSet<S2CellId>) -> BTreeSet<S2CellId> {
        let mut parents = BTreeSet::<S2CellId>::new();
        let grid_length = (self.grid_size + self.buffer_size * 2) as usize;
        let half_grid_length = grid_length / 2;

        for cell in cells {
            let mut grid: Vec<V> = vec![V::default(); grid_length * grid_length];
            let (face, cell_zoom, i, j) = cell.to_face_ij();
            let [bl_id, br_id, tl_id, tr_id] = S2CellId::children_ij(face, cell_zoom, i, j);
            // for each child, downsample into the result grid
            self.downsample_grid(bl_id, &mut grid, 0, 0);
            self.downsample_grid(br_id, &mut grid, half_grid_length, 0);
            self.downsample_grid(tl_id, &mut grid, 0, half_grid_length);
            self.downsample_grid(tr_id, &mut grid, half_grid_length, half_grid_length);
            // store the grid and add the parent cell for future upscaling
            self.grid_tile_store.set(*cell, grid);
            if zoom != 0 {
                parents.insert(cell.parent(Some(zoom - 1)));
            }
        }

        parents
    }

    /// Upscale a grid into the target grid at x,y position
    fn downsample_grid(&self, cell_id: S2CellId, target: &mut [V], x: usize, y: usize) {
        let grid = if let Some(grid) = self.grid_tile_store.get(cell_id) {
            grid
        } else {
            return;
        };

        let Self { grid_size, buffer_size, .. } = self;
        let grid_length = (grid_size + buffer_size * 2) as usize;
        let half_grid_length = grid_length / 2;
        let mid_point = S2Point::new(0.5, 0.5, 0.);
        let null_val = V::default();

        for j in 0..half_grid_length {
            for i in 0..half_grid_length {
                let base_j = j * 2;
                let base_i = i * 2;

                let source_points: Vec<IndexPoint<V>> = [
                    ((0.0, 0.0), base_j * grid_length + base_i),
                    ((1.0, 0.0), base_j * grid_length + base_i + 1),
                    ((0.0, 1.0), (base_j + 1) * grid_length + base_i),
                    ((1.0, 1.0), (base_j + 1) * grid_length + base_i + 1),
                ]
                .into_iter()
                .filter_map(|((x, y), idx)| {
                    let value = grid[idx];
                    if value != null_val {
                        Some(IndexPoint::new(S2Point::new(x, y, 0.0), value))
                    } else {
                        None
                    }
                })
                .collect();

                if source_points.is_empty() {
                    continue;
                }

                target[(j + y) * grid_length + (i + x)] =
                    (self.interpolation)(&mid_point, &source_points, self.get_value);
            }
        }
    }
}
