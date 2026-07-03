use super::{
    GetRasterTileValue, TileFetcher, TileID, TileReader,
    grid::{build_tile_grid_wm, merge_tile_grid_wm},
};
use crate::{
    geometry::{
        GetTileID, S2Point, S2TileID, Source, WMTileID, ll_to_px, merc_to_ll, px_to_tile,
        tile_xy_from_st_zoom, xyz_to_bbox,
    },
    parsers::{FeatureReader, ImageData},
};
use alloc::{format, rc::Rc, string::String, vec, vec::Vec};
use core::marker::PhantomData;
use image::RgbaImage;
use libm::{floor, pow};
use s2_tilejson::{Metadata, Scheme, UnknownMetadata};
use s2json::{
    BBox, BBox3D, Face, NewXY, Point, Point3D, VectorFeature, VectorFeatureType, VectorGeometry,
    VectorMultiPoint, VectorPoint,
};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

/// # Raster Tiles Fetcher
///
/// ## Description
/// Read an entire archive of raster tiles, where the max zoom data is iterated upon
///
/// Supports reading either RGB(A) data, RGB(A) encoded elevation data, or build your own structure.
///
/// Implements the [`FeatureReader`] and [`TileFetcher`] traits
///
/// ## Usage
///
/// The methods you have access to:
/// - [`RasterTileFetcher::new`]: Create a new RasterTileFetcher
/// - [`RasterTileFetcher::get_metadata`]: Get the metadata of the tileset
/// - [`RasterTileFetcher::has_tile_wm`]: Check if it is WM tile
/// - [`RasterTileFetcher::has_tile_s2`]: Check if it is S2 tile
/// - [`RasterTileFetcher::get_tile_wm`]: Get an WM tile
/// - [`RasterTileFetcher::get_tile_s2`]: Get an S2 tile
/// - [`RasterTileFetcher::get_tile_with_padding_wm`]: Get an WM tile with pixel padding (added from adjacent tiles if applicable & available)
/// - [`RasterTileFetcher::iter`]: Iterate over the tiles
///
/// ```rust
/// use gistools::{parsers::{RGBA, FeatureReader}, readers::{TileFetcher, RasterTileFetcher}};
/// use std::path::PathBuf;
///
/// let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
/// path.push("tests/readers/tile/fixtures/wm/satellite");
///
/// // read the RGBA data of each tile. Each pixel is stored as a VectorPoint
/// let reader = RasterTileFetcher::<RGBA>::new(path, Some(1));
/// let tiles: Vec<_> = reader.iter().collect();
/// assert_eq!(tiles.len(), 4);
/// ```
///
/// ## Links
/// - <https://satakagi.github.io/mapsForWebWS2020-docs/QuadTreeCompositeTilingAndVectorTileStandard.html>
/// - <https://cesium.com/blog/2015/04/07/quadtree-cheatseet/>
#[derive(Debug, Clone)]
pub struct RasterTileFetcher<D: Clone + Default + GetRasterTileValue> {
    path: PathBuf,
    threshold: Option<u8>,
    metadata: Metadata,
    phantom: PhantomData<D>,
}
impl<D: Clone + Default + GetRasterTileValue> RasterTileFetcher<D> {
    /// Grab the tile at the given zoom-x-y coordinates.
    ///
    /// This function adds the ability to pull from surrounding images and add them as padding
    ///
    /// This function is also useful for just expanding the zoom level up. So if the image is 256x256,
    /// you can use this function to get a 512x512 image
    ///
    /// ## Parameters
    /// - `zoom`: the zoom level of the tile
    /// - `x`: the x coordinate of the tile
    /// - `y`: the y coordinate of the tile
    /// - `padding`: the amount of padding to add to each side of the tile
    /// - `size`: the size of each tile width and height.
    /// - `wanted_size`: the size of the rendered center tile. For example if you want a 512x512 tile, but the source is 256x256, you can set this to 512.
    ///
    /// ## Returns
    /// The tile with the added padding
    pub fn get_tile_with_padding_wm(
        &self,
        zoom: u8,
        x: u32,
        y: u32,
        padding: usize,
        size: Option<usize>,
        wanted_size: Option<usize>,
    ) -> Option<RasterTileReader<D>> {
        let size = size.unwrap_or(512);
        let wanted_size = wanted_size.unwrap_or(size);
        let metadata = self.get_metadata();
        let Metadata { scheme, .. } = metadata;
        let is_tms = *scheme == Scheme::Tms;
        let tile: TileID = (WMTileID { zoom, x, y }).into();
        // Setup a grid
        let mut grid = build_tile_grid_wm(&tile, padding, size, wanted_size, is_tms);
        if grid.is_empty() {
            return None;
        }
        // track the tiles we've already fetched so we don't keep fetching
        let mut fetch_map = BTreeMap::<TileID, Rc<ImageData>>::new();
        for tile_guide in grid.iter_mut() {
            // if the tile has already been fetched, skip
            if tile_guide.image.is_some() {
                continue;
            }
            // if we have the tile in the cache, use it
            if let Some(image) = fetch_map.get(&tile_guide.tile) {
                tile_guide.image = Some(image.clone());
                continue;
            }
            // last case: fetch the tile
            let tile =
                self.get_tile_wm(tile_guide.tile.zoom(), tile_guide.tile.x(), tile_guide.tile.y());
            let Some(tile_image) = &tile.image else { continue };
            let image: Rc<ImageData> = Rc::new(tile_image.into());
            fetch_map.insert(tile_guide.tile, image.clone());
            tile_guide.image = Some(image);
        }
        // Now merge the images into a single image
        let merged = merge_tile_grid_wm(&grid, wanted_size, padding);

        Some(RasterTileReader::<D> {
            metadata: tile,
            image: Some((&merged).into()),
            is_s2: self.is_s2(),
            tms_style: is_tms,
            _phantom: PhantomData,
        })
    }
}
impl<D: Clone + Default + GetRasterTileValue> TileFetcher<D, D, RasterTileReader<D>>
    for RasterTileFetcher<D>
{
    fn new<P: AsRef<Path>>(path: P, threshold: Option<u8>) -> RasterTileFetcher<D> {
        let path = path.as_ref().to_path_buf();
        let metadata_path = path.join("metadata.json");
        let meta_string: String = fs::read_to_string(&metadata_path).unwrap();
        let unknown_meta: UnknownMetadata = serde_json::from_str(&meta_string).unwrap();

        RasterTileFetcher {
            path,
            threshold,
            metadata: unknown_meta.to_metadata(),
            phantom: PhantomData,
        }
    }
    fn get_metadata(&self) -> &Metadata {
        &self.metadata
    }

    fn has_tile_wm(&self, zoom: u8, x: u32, y: u32) -> bool {
        let Metadata { extension, .. } = self.get_metadata();
        let tile_path = self.path.join(format!("{zoom}/{x}/{y}.{extension}"));

        tile_path.exists()
    }

    fn has_tile_s2(&self, face: Face, zoom: u8, x: u32, y: u32) -> bool {
        let Metadata { extension, .. } = self.get_metadata();
        let tile_path =
            self.path.join(format!("{}/{}/{}/{}.{}", u8::from(face), zoom, x, y, extension));

        tile_path.exists()
    }

    fn get_tile_wm(&self, zoom: u8, x: u32, y: u32) -> RasterTileReader<D> {
        RasterTileReader::new(self.path.clone(), self.get_metadata(), 0.into(), zoom, x, y, false)
    }

    fn get_tile_s2(&self, face: Face, zoom: u8, x: u32, y: u32) -> RasterTileReader<D> {
        RasterTileReader::new(self.path.clone(), self.get_metadata(), face, zoom, x, y, true)
    }

    fn get_tile_value_wm(&self, zoom: u8, lon: f64, lat: f64, tile_size: Option<u64>) -> Option<D> {
        let tile_size = tile_size.unwrap_or(512);
        let tile_size_f64 = tile_size as f64;
        let zoom_f64 = zoom as f64;
        // get the tile coordinates
        let Point(x, y) = ll_to_px(&Point(lon, lat), zoom_f64, Some(false), Some(tile_size));
        let (tile_x, tile_y) = px_to_tile(&Point(x, y), Some(tile_size));
        // get the tile
        let tile = self.get_tile_wm(zoom, tile_x as u32, tile_y as u32);
        let Some(tile_image) = &tile.image else { return None };
        // get the pixel
        let local_x = modulo(x, tile_size_f64);
        let local_y = modulo(y, tile_size_f64);
        let pixel_x = floor(local_x);
        // If TMS style, invert the y position
        let pixel_y =
            if tile.tms_style { floor(tile_size_f64 - 1.0 - local_y) } else { floor(local_y) };
        let pixel = tile_image.get_pixel(pixel_x as u32, pixel_y as u32);

        Some(D::get_raster_tile_value(pixel.0[0], pixel.0[1], pixel.0[2], Some(pixel.0[3])))
    }

    fn get_tile_value_s2(&self, zoom: u8, lon: f64, lat: f64, tile_size: Option<u64>) -> Option<D> {
        let tile_size = tile_size.unwrap_or(512);
        let tile_size_f64 = tile_size as f64;
        let zoom_f64 = zoom as f64;
        // get the tile coordinates
        let xyz = S2Point::from_lon_lat(&Point3D::new_xy(lon, lat));
        let (face, s, t) = xyz.to_face_st();
        let (tile_x, tile_y) = tile_xy_from_st_zoom(s, t, zoom);
        // get the tile
        let tile = self.get_tile_s2(face.into(), zoom, tile_x as u32, tile_y as u32);
        let Some(tile_image) = &tile.image else { return None };
        // get the pixel
        let zoom_size = tile_size_f64 * pow(2., zoom_f64);
        let pixel_x = floor(modulo(zoom_size * s, tile_size_f64));
        let pixel_y = floor(modulo(zoom_size * t, tile_size_f64));
        let pixel = tile_image.get_pixel(pixel_x as u32, pixel_y as u32);

        Some(D::get_raster_tile_value(pixel.0[0], pixel.0[1], pixel.0[2], Some(pixel.0[3])))
    }
}
impl<D: Clone + Default + GetRasterTileValue> FeatureReader<TileID, D, D> for RasterTileFetcher<D> {
    type FeatureIterator<'a>
        = RasterIterator<'a, D>
    where
        Self: 'a;

    fn iter(&self) -> Self::FeatureIterator<'_> {
        let Metadata { minzoom, maxzoom, .. } = self.get_metadata();
        let mut stack = vec![(0.into(), 0, 0, 0)];
        if self.is_s2() {
            for face in [Face::Face1, Face::Face2, Face::Face3, Face::Face4, Face::Face5] {
                stack.push((face, 0, 0, 0));
            }
        }
        let threshold = self.threshold.unwrap_or(*maxzoom);
        RasterIterator {
            container: self,
            stack,
            minzoom: *minzoom,
            threshold,
            pool_size: 1,
            thread_id: 0,
            index: 0,
        }
    }

    #[cfg(feature = "std")]
    fn par_iter(&self, pool_size: usize, thread_id: usize) -> Self::FeatureIterator<'_> {
        let Metadata { minzoom, maxzoom, .. } = self.get_metadata();
        let mut stack = vec![(0.into(), 0, 0, 0)];
        if self.is_s2() {
            for face in [Face::Face1, Face::Face2, Face::Face3, Face::Face4, Face::Face5] {
                stack.push((face, 0, 0, 0));
            }
        }
        let threshold = self.threshold.unwrap_or(*maxzoom);
        RasterIterator {
            container: self,
            stack,
            minzoom: *minzoom,
            threshold,
            pool_size: pool_size as u64,
            thread_id: thread_id as u64,
            index: 0,
        }
    }
}

/// Iterator for the S2/WM Raster Tile Fetcher
#[derive(Debug)]
pub struct RasterIterator<'a, D: Clone + Default + GetRasterTileValue> {
    container: &'a RasterTileFetcher<D>,
    stack: Vec<(Face, u8, u32, u32)>,
    minzoom: u8,
    threshold: u8,
    pool_size: u64,
    thread_id: u64,
    index: u64,
}
impl<D: Clone + Default + GetRasterTileValue> Iterator for RasterIterator<'_, D> {
    type Item = VectorFeature<TileID, D, D>;
    fn next(&mut self) -> Option<Self::Item> {
        let is_s2 = self.container.is_s2();
        while let Some((face, zoom, x, y)) = self.stack.pop() {
            // if zoom not reached yet, push children and continue
            let has_tile = if is_s2 {
                self.container.has_tile_s2(face, zoom, x, y)
            } else {
                self.container.has_tile_wm(zoom, x, y)
            };
            if zoom < self.minzoom || (zoom != self.threshold && has_tile) {
                self.stack.extend(vec![
                    (face, zoom + 1, x * 2, y * 2),
                    (face, zoom + 1, x * 2 + 1, y * 2),
                    (face, zoom + 1, x * 2, y * 2 + 1),
                    (face, zoom + 1, x * 2 + 1, y * 2 + 1),
                ]);
                continue;
            } else if zoom == self.threshold && has_tile {
                let idx = self.index;
                self.index += 1;
                if self.pool_size > 1 && idx % self.pool_size != self.thread_id {
                    continue; // skip, belongs to another thread
                }
                let tile = if is_s2 {
                    self.container.get_tile_s2(face, zoom, x, y)
                } else {
                    self.container.get_tile_wm(zoom, x, y)
                };
                return Some(tile.build_feature());
            }
        }
        None
    }
}

/// Iterator for the S2/WM Raster Tile Fetcher
#[derive(Debug)]
pub struct RasterMetadataIterator<'a, D: Clone + Default + GetRasterTileValue> {
    container: &'a RasterTileFetcher<D>,
    stack: Vec<(Face, u8, u32, u32)>,
    minzoom: u8,
    threshold: u8,
    pool_size: u64,
    thread_id: u64,
    index: u64,
}
impl<D: Clone + Default + GetRasterTileValue> Iterator for RasterMetadataIterator<'_, D> {
    type Item = TileID;
    fn next(&mut self) -> Option<Self::Item> {
        let is_s2 = self.container.is_s2();
        while let Some((face, zoom, x, y)) = self.stack.pop() {
            // check the tile exists
            let has_tile = if is_s2 {
                self.container.has_tile_s2(face, zoom, x, y)
            } else {
                self.container.has_tile_wm(zoom, x, y)
            };
            // add next zoom if max zoom not reached yet
            if zoom < self.minzoom || (zoom != self.threshold && has_tile) {
                self.stack.extend(vec![
                    (face, zoom + 1, x * 2, y * 2),
                    (face, zoom + 1, x * 2 + 1, y * 2),
                    (face, zoom + 1, x * 2, y * 2 + 1),
                    (face, zoom + 1, x * 2 + 1, y * 2 + 1),
                ]);
            }
            // if tile exists, return the metadata for it
            if has_tile {
                let idx = self.index;
                self.index += 1;
                if self.pool_size > 1 && idx % self.pool_size != self.thread_id {
                    continue; // skip, belongs to another thread
                }
                return Some(if is_s2 {
                    (S2TileID { face, zoom, x, y }).into()
                } else {
                    WMTileID { zoom, x, y }.into()
                });
            }
        }
        None
    }
}

/// Raster Tile Reader
#[derive(Debug)]
pub struct RasterTileReader<D: Clone + Default + GetRasterTileValue> {
    /// Tile Metadata
    pub metadata: TileID,
    /// Tile Image
    pub image: Option<RgbaImage>,
    /// If the tile is S2 or WM
    pub is_s2: bool,
    /// Tile's scheme is TMS
    pub tms_style: bool,
    /// Help D pass
    _phantom: PhantomData<D>,
}
impl<D: Clone + Default + GetRasterTileValue> TileReader<D, D> for RasterTileReader<D> {
    fn new(
        path: PathBuf,
        metadata: &Metadata,
        face: Face,
        zoom: u8,
        x: u32,
        y: u32,
        is_s2: bool,
    ) -> Self {
        let Metadata { extension, scheme, .. } = metadata;
        let tile_path = if is_s2 {
            path.join(format!("{}/{}/{}/{}.{}", u8::from(face), zoom, x, y, extension))
        } else {
            path.join(format!("{zoom}/{x}/{y}.{extension}"))
        };
        let image = if !tile_path.exists() {
            None
        } else {
            Some(image::open(&tile_path).unwrap().to_rgba8())
        };
        let metadata = if is_s2 {
            (S2TileID { face, zoom, x, y }).into()
        } else {
            (WMTileID { zoom, x, y }).into()
        };

        RasterTileReader {
            metadata,
            image,
            is_s2,
            tms_style: *scheme == Scheme::Tms,
            _phantom: PhantomData,
        }
    }

    fn build_feature(&self) -> VectorFeature<TileID, D, D> {
        match &self.metadata {
            TileID::S2(s2) => self.build_feature_s2(s2),
            TileID::WM(wm) => self.build_feature_wm(wm),
        }
    }
}
impl<D: Clone + Default + GetRasterTileValue> RasterTileReader<D> {
    fn build_feature_s2(&self, metadata: &S2TileID) -> VectorFeature<TileID, D, D> {
        let S2TileID { x, y, zoom, face } = metadata;
        let image = self.image.as_ref().unwrap();
        let tile_size = image.width() as usize;
        let mut bbox = BBox3D::default();
        // Get the bounding box of the tile in lon-lat
        let BBox { left: min_s, bottom: min_t, right: max_s, top: max_t } =
            BBox::from_st_zoom(*x as f64, *y as f64, *zoom);
        let s_step = (max_s - min_s) / (tile_size as f64);
        let t_step = (max_t - min_t) / (tile_size as f64);
        let mut coordinates: VectorMultiPoint<D> = vec![];

        for py in 0..tile_size {
            let y = min_s + ((py as f64) + 0.5) * t_step; // Center of the row
            for px in 0..tile_size {
                let x = min_t + ((px as f64) + 0.5) * s_step; // Center of the column
                let pixel = image.get_pixel(px as u32, py as u32);
                let m_value =
                    D::get_raster_tile_value(pixel.0[0], pixel.0[1], pixel.0[2], Some(pixel.0[3]));

                let vp = VectorPoint::new_xy(x, y, Some(m_value));
                bbox.extend_from_point(&vp);
                coordinates.push(vp);
            }
        }

        VectorFeature {
            _type: VectorFeatureType::S2Feature,
            face: *face,
            metadata: Some(self.metadata),
            geometry: VectorGeometry::new_multipoint(coordinates, Some(bbox)),
            ..Default::default()
        }
    }

    fn build_feature_wm(&self, metadata: &WMTileID) -> VectorFeature<TileID, D, D> {
        let WMTileID { x, y, zoom } = metadata;
        let image = self.image.as_ref().unwrap();
        let tile_size = image.width() as usize;
        let mut bbox = BBox3D::default();
        // Get the bounding box of the tile in lon-lat
        let (west, south, east, north) = xyz_to_bbox(
            *x as i64,
            *y as i64,
            *zoom as f64,
            Some(self.tms_style),
            Some(Source::Google),
        );
        let x_step = (east - west) / (tile_size as f64);
        let y_step = (north - south) / (tile_size as f64);
        let mut coordinates: VectorMultiPoint<D> = vec![];

        for py in 0..tile_size {
            let py_f64 = py as f64;
            let y_pos = north - (py_f64 + 0.5) * y_step; // Center of the row
            for px in 0..tile_size {
                let px_f64 = px as f64;
                let x_pos = west + (px_f64 + 0.5) * x_step; // Center of the column
                let (lon, lat) = merc_to_ll(&(x_pos, y_pos));
                let pixel = image.get_pixel(px as u32, py as u32);
                let m_value =
                    D::get_raster_tile_value(pixel.0[0], pixel.0[1], pixel.0[2], Some(pixel.0[3]));

                let vp = VectorPoint::new_xy(lon, lat, Some(m_value));
                bbox.extend_from_point(&vp);
                coordinates.push(vp);
            }
        }

        VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            metadata: Some(self.metadata),
            geometry: VectorGeometry::new_multipoint(coordinates, Some(bbox)),
            ..Default::default()
        }
    }
}

fn modulo(n: f64, m: f64) -> f64 {
    ((n % m) + m) % m
}
