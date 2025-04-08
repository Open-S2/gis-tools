use super::{
    GetRasterTileValue, S2TileMetadata, TileFetcher, TileMetadata, TileReader, WMTileMetadata,
};
use crate::{
    geometry::{merc_to_ll, xyz_to_bbox, Source},
    readers::FeatureReader,
};
use alloc::{format, string::String, vec, vec::Vec};
use core::marker::PhantomData;
use image::RgbaImage;
use s2_tilejson::{Metadata, Scheme, UnknownMetadata};
use s2json::{
    BBox, BBox3D, Face, VectorFeature, VectorFeatureType, VectorGeometry, VectorMultiPoint,
    VectorPoint,
};
use std::{
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
/// ## Links
/// - https://satakagi.github.io/mapsForWebWS2020-docs/QuadTreeCompositeTilingAndVectorTileStandard.html
/// - https://cesium.com/blog/2015/04/07/quadtree-cheatseet/
pub struct RasterTileFetcher<D: Clone + Default + GetRasterTileValue> {
    path: PathBuf,
    threshold: Option<u8>,
    metadata: Metadata,
    phantom: PhantomData<D>,
}
impl<D: Clone + Default + GetRasterTileValue> TileFetcher<(), D, RasterTileReader<D>>
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
        let tile_path = self.path.join(format!("{}/{}/{}.{}", zoom, x, y, extension));

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
}
impl<D: Clone + Default + GetRasterTileValue> FeatureReader<TileMetadata, (), D>
    for RasterTileFetcher<D>
{
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
        RasterIterator { container: self, stack, minzoom: *minzoom, threshold }
    }
}

/// Iterator for the S2 Raster Tile Fetcher
pub struct RasterIterator<'a, D: Clone + Default + GetRasterTileValue> {
    container: &'a RasterTileFetcher<D>,
    stack: Vec<(Face, u8, u32, u32)>,
    minzoom: u8,
    threshold: u8,
}
impl<D: Clone + Default + GetRasterTileValue> Iterator for RasterIterator<'_, D> {
    type Item = VectorFeature<TileMetadata, (), D>;
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

/// Raster Tile Reader
pub struct RasterTileReader<D: Clone + Default + GetRasterTileValue> {
    /// Tile Metadata
    pub metadata: TileMetadata,
    /// Tile Image
    pub image: RgbaImage,
    /// If the tile is S2 or WM
    pub is_s2: bool,
    /// Tile's scheme is TMS
    pub tms_style: bool,
    /// Help D pass
    _phantom: PhantomData<D>,
}
impl<D: Clone + Default + GetRasterTileValue> TileReader<(), D> for RasterTileReader<D> {
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
            path.join(format!("{}/{}/{}.{}", zoom, x, y, extension))
        };
        let image = image::open(&tile_path).unwrap().to_rgba8();
        let metadata = if is_s2 {
            TileMetadata::S2(S2TileMetadata { face, zoom, x, y })
        } else {
            TileMetadata::WM(super::WMTileMetadata { zoom, x, y })
        };

        RasterTileReader {
            metadata,
            image,
            is_s2,
            tms_style: *scheme == Scheme::Tms,
            _phantom: PhantomData,
        }
    }

    fn build_feature(&self) -> VectorFeature<TileMetadata, (), D> {
        match &self.metadata {
            TileMetadata::S2(s2) => self.build_feature_s2(s2),
            TileMetadata::WM(wm) => self.build_feature_wm(wm),
        }
    }
}
impl<D: Clone + Default + GetRasterTileValue> RasterTileReader<D> {
    fn build_feature_s2(&self, metadata: &S2TileMetadata) -> VectorFeature<TileMetadata, (), D> {
        let S2TileMetadata { x, y, zoom, face } = metadata;
        let tile_size = self.image.width() as usize;
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
                let pixel = self.image.get_pixel(px as u32, py as u32);
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
            metadata: Some(self.metadata.clone()),
            geometry: VectorGeometry::new_multipoint(coordinates, Some(bbox)),
            ..Default::default()
        }
    }

    fn build_feature_wm(&self, metadata: &WMTileMetadata) -> VectorFeature<TileMetadata, (), D> {
        let WMTileMetadata { x, y, zoom } = metadata;
        let tile_size = self.image.width() as usize;
        let mut bbox = BBox3D::default();
        // Get the bounding box of the tile in lon-lat
        let (west, south, east, north) = xyz_to_bbox(
            *x,
            *y,
            *zoom,
            Some(self.tms_style),
            Some(Source::Google),
            Some(tile_size as u16),
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
                let (lon, lat) = merc_to_ll((x_pos, y_pos));
                let pixel = self.image.get_pixel(px as u32, py as u32);
                let m_value =
                    D::get_raster_tile_value(pixel.0[0], pixel.0[1], pixel.0[2], Some(pixel.0[3]));

                let vp = VectorPoint::new_xy(lon, lat, Some(m_value));
                bbox.extend_from_point(&vp);
                coordinates.push(vp);
            }
        }

        VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            metadata: Some(self.metadata.clone()),
            geometry: VectorGeometry::new_multipoint(coordinates, Some(bbox)),
            ..Default::default()
        }
    }
}
