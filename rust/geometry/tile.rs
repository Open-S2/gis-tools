use crate::{
    data_structures::HasLayer,
    geometry::{LonLat, S2CellId, Source, convert, ll_to_px, ll_to_tile, xyz_to_bbox},
};
use alloc::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
};
use core::cmp::Ordering;
use libm::{fabs, floor, pow};
use s2json::{
    BBox, Face, Feature, Geometry, GetXY, JSONCollection, NewXY, Projection, VectorFeature,
    VectorGeometry,
};
use serde::{Deserialize, Serialize};

/// # Trait for getting a tile's metadata attributes
pub trait GetTileID: Debug + Sized + Clone + Eq + Ord {
    /// Create a new Tile ID
    fn new(face: Option<Face>, zoom: u8, x: u32, y: u32) -> Self;
    /// Get the tile's face
    fn face(&self) -> Option<Face>;
    /// Get the tile's zoom
    fn zoom(&self) -> u8;
    /// Get the tile's x coordinate
    fn x(&self) -> u32;
    /// Get the tile's y coordinate
    fn y(&self) -> u32;
    /// Get the face, zoom, x, and y
    fn fzxy(&self) -> (Option<Face>, u8, u32, u32) {
        (self.face(), self.zoom(), self.x(), self.y())
    }
    /// Convert the S2TileID to a CellID
    fn to_id(&self) -> S2CellId;
    /// Convert to a bbox in WGS84 Lon-Lat coordinates. TMS style is optional and only applicable if Web Mercator (WM)
    fn to_bbox(&self, tms_style: Option<bool>) -> BBox;
    /// Find the center lon-lat of the tile. TMS style is optional and only applicable if Web Mercator (WM)
    fn to_center_lon_lat<P: NewXY>(&self, tms_style: Option<bool>) -> P;
    /// Find the neighbors of the tile
    fn neighbors(&self) -> Vec<Self>;
    /// Get the children of the tile
    fn children(&self) -> Vec<Self>;
    /// Get the parent of the tile
    fn parent(&self) -> Option<Self>;
    /// Merge a list of tiles into a smaller list of higher zooms when possible
    fn merge<I>(tiles: I) -> Vec<Self>
    where
        I: IntoIterator<Item = Self>,
    {
        let mut maxzoom = 0;
        let mut tile_set = BTreeMap::<u8, BTreeSet<Self>>::new();

        for tile in tiles {
            maxzoom = maxzoom.max(tile.zoom());
            tile_set.entry(tile.zoom()).or_default().insert(tile);
        }

        for zoom in (1..=maxzoom).rev() {
            // Remove the set to avoid borrowing issues and cloning
            let Some(curr_zoom_tiles) = tile_set.remove(&zoom) else {
                continue;
            };
            let mut parent_zoom_tiles = tile_set.remove(&(zoom - 1)).unwrap_or_default();

            let mut has_parent = BTreeSet::new();
            let mut potential_parents = BTreeSet::new();
            // Add all potential parents
            for tile in curr_zoom_tiles.iter() {
                if let Some(parent) = tile.parent() {
                    potential_parents.insert(parent);
                }
            }

            // Check if all parents are in the set
            for parent in potential_parents {
                let children = parent.children();
                // Check if all siblings exist in the current zoom set
                if children.iter().all(|c| curr_zoom_tiles.contains(c)) {
                    for child in children {
                        has_parent.insert(child);
                    }
                    // Add the parent to the set
                    parent_zoom_tiles.insert(parent);
                }
            }

            // Put back only the tiles that weren't merged into a parent
            let remaining: BTreeSet<Self> =
                curr_zoom_tiles.into_iter().filter(|t| !has_parent.contains(t)).collect();
            tile_set.insert(zoom, remaining);
            tile_set.entry(zoom - 1).or_default().extend(parent_zoom_tiles);
        }

        tile_set.into_values().flatten().collect()
    }
}

/// # S2 Tile's metadata
///
/// Implements the [`GetTileID`] trait
///
/// ## Usage
///
/// Methods that are available:
/// - [`S2TileID::new`]: Create a new [`S2TileID`]
/// - [`S2TileID::face`]: Get the tile's face
/// - [`S2TileID::zoom`]: Get the tile's zoom
/// - [`S2TileID::x`]: Get the tile's x coordinate
/// - [`S2TileID::y`]: Get the tile's y coordinate
/// - [`S2TileID::fzxy`]: Get the face, zoom, x, and y from the tile
/// - [`S2TileID::to_id`]: Convert to an [`S2CellId`]
/// - [`S2TileID::from_id`]: Convert from a [`S2CellId`]
/// - [`S2TileID::to_bbox`]: Covnert to a [`BBox`]
/// - [`S2TileID::to_center_lon_lat`]: Find the center lon-lat of the tile
/// - [`S2TileID::children`]: Get the children of the tile
/// - [`S2TileID::parent`]: Get the parent of the tile
/// - [`S2TileID::neighbors`]: Find the neighbors of the tile
/// - [`S2TileID::from_point`]: Get the tile from S2's S-T coordinate space
/// - [`S2TileID::from_multipoint`]: Get a collection of tiles that cover a multipoint from S2's S-T coordinate space
/// - [`S2TileID::from_linestring`]: Get a collection of tiles that cover a linestring from S2's S-T coordinate space
/// - [`S2TileID::from_multilinestring`]: Get a collection of tiles that cover a multilinestring from S2's S-T coordinate space
/// - [`S2TileID::from_polygon`]: Get a collection of tiles that cover a polygon from S2's S-T coordinate space
/// - [`S2TileID::from_multipolygon`]: Get a collection of tiles that cover a multipolygon from S2's S-T coordinate space
/// - [`S2TileID::from_vector_geometry`]: Get a collection of tiles that cover a [`VectorGeometry`] that is in S2's S-T coordinate space
/// - [`S2TileID::from_vector_feature`]: Get a collection of tiles that cover a [`VectorFeature`] that is in S2's S-T coordinate space
/// - [`S2TileID::from_json`]: Get a collection of tiles that cover an input JSON object in any coordinate space
///
/// Into relations:
/// - [`S2CellId`]
/// - [`TileID`]
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct S2TileID {
    /// S2 Face
    pub face: Face,
    /// S2 Zoom
    pub zoom: u8,
    /// S2 X Tile Coordinate
    pub x: u32,
    /// S2 Y Tile Coordinate
    pub y: u32,
}
impl GetTileID for S2TileID {
    fn new(face: Option<Face>, zoom: u8, x: u32, y: u32) -> Self {
        Self { face: face.unwrap_or_default(), zoom, x, y }
    }
    fn face(&self) -> Option<Face> {
        Some(self.face)
    }
    fn zoom(&self) -> u8 {
        self.zoom
    }
    fn x(&self) -> u32 {
        self.x
    }
    fn y(&self) -> u32 {
        self.y
    }
    fn to_id(&self) -> S2CellId {
        S2CellId::from_face_ij(self.face as u8, self.x, self.y, Some(self.zoom))
    }
    fn to_bbox(&self, _tms_style: Option<bool>) -> BBox {
        s2_bounds(*self)
    }
    fn to_center_lon_lat<P: NewXY>(&self, tms_style: Option<bool>) -> P {
        let BBox { left, bottom, right, top } = self.to_bbox(tms_style);
        P::new_xy((left + right) / 2., (top + bottom) / 2.)
    }
    fn neighbors(&self) -> Vec<Self> {
        let id = self.to_id();
        let neighbors = id.neighbors();
        neighbors.into_iter().map(S2TileID::from_id).collect()
    }
    fn children(&self) -> Vec<Self> {
        let id = self.to_id();
        let children = id.children(None);
        children.into_iter().map(S2TileID::from_id).collect()
    }
    fn parent(&self) -> Option<Self> {
        if self.zoom == 0 {
            None
        } else {
            let id = self.to_id();
            Some(id.parent(None).into())
        }
    }
}
impl From<S2CellId> for S2TileID {
    fn from(id: S2CellId) -> Self {
        S2TileID::from_id(id)
    }
}
impl From<S2TileID> for S2CellId {
    fn from(id: S2TileID) -> Self {
        id.to_id()
    }
}
impl S2TileID {
    /// Create a new S2TileID
    pub fn new(face: Face, zoom: u8, x: u32, y: u32) -> Self {
        Self { face, zoom, x, y }
    }
    /// Create a new S2TileID from an S2CellId
    pub fn from_id(id: S2CellId) -> Self {
        let (face, zoom, i, j) = id.to_face_ij();
        Self { face: face.into(), zoom, x: i, y: j }
    }
    /// Get the tile from S2's S-T coordinate space
    pub fn from_point<P: GetXY + NewXY>(face: Face, point: &P, zoom: u8) -> Self {
        let (s, t) = point.xy();
        let mut cell = S2CellId::from_face_st(face as u8, s, t, None);
        cell = cell.parent(Some(zoom));
        Self::from_id(cell)
    }
    /// Get the tiles from S2's S-T coordinate space
    pub fn from_multipoint<P: GetXY + NewXY>(face: Face, points: &[P], zoom: u8) -> Vec<Self> {
        points.iter().map(|p| Self::from_point(face, p, zoom)).collect()
    }
    /// Get a collection of tiles that cover a linestring that is in S2's S-T coordinate space
    pub fn from_linestring<P: GetXY + NewXY>(face: Face, linestring: &[P], zoom: u8) -> Vec<Self> {
        let mut tiles = BTreeSet::new();
        line_cover(&mut tiles, linestring, zoom, &mut vec![], Some(s2_to_px));
        GetTileID::merge(
            tiles.into_iter().map(|wm_t| S2TileID::new(face, wm_t.zoom, wm_t.x, wm_t.y)),
        )
    }
    /// Get a collection of tiles that cover a multilinestring that is in S2's S-T coordinate space
    pub fn from_multilinestring<P: GetXY + NewXY>(
        face: Face,
        multilinestring: &[Vec<P>],
        zoom: u8,
    ) -> Vec<Self> {
        let mut tiles = BTreeSet::new();
        for line in multilinestring {
            line_cover(&mut tiles, line, zoom, &mut vec![], Some(s2_to_px));
        }
        GetTileID::merge(
            tiles.into_iter().map(|wm_t| S2TileID::new(face, wm_t.zoom, wm_t.x, wm_t.y)),
        )
    }
    /// Get a collection of tiles that cover a polygon that is in S2's S-T coordinate space
    pub fn from_polygon<P: GetXY + NewXY>(face: Face, polygon: &[Vec<P>], zoom: u8) -> Vec<Self> {
        let mut tiles = BTreeSet::new();
        poly_cover(&mut tiles, polygon, zoom, Some(s2_to_px));
        GetTileID::merge(
            tiles.into_iter().map(|wm_t| S2TileID::new(face, wm_t.zoom, wm_t.x, wm_t.y)),
        )
    }
    /// Get a collection of tiles that cover a multipolygon that is in S2's S-T coordinate space
    pub fn from_multipolygon<P: GetXY + NewXY>(
        face: Face,
        polygons: &[Vec<Vec<P>>],
        zoom: u8,
    ) -> Vec<Self> {
        let mut tiles = BTreeSet::new();
        for polygon in polygons {
            poly_cover(&mut tiles, polygon, zoom, Some(s2_to_px));
        }
        GetTileID::merge(
            tiles.into_iter().map(|wm_t| S2TileID::new(face, wm_t.zoom, wm_t.x, wm_t.y)),
        )
    }
    /// Get a collection of tiles that cover a [`VectorGeometry`] that is in S2's S-T coordinate space
    pub fn from_vector_geometry<M: Clone + Default>(
        geometry: &VectorGeometry<M>,
        face: Face,
        zoom: u8,
    ) -> Vec<Self> {
        match geometry {
            VectorGeometry::Point(g) => vec![S2TileID::from_point(face, &g.coordinates, zoom)],
            VectorGeometry::MultiPoint(g) => S2TileID::from_multipoint(face, &g.coordinates, zoom),
            VectorGeometry::LineString(g) => S2TileID::from_linestring(face, &g.coordinates, zoom),
            VectorGeometry::MultiLineString(g) => {
                S2TileID::from_multilinestring(face, &g.coordinates, zoom)
            }
            VectorGeometry::Polygon(g) => S2TileID::from_polygon(face, &g.coordinates, zoom),
            VectorGeometry::MultiPolygon(g) => {
                S2TileID::from_multipolygon(face, &g.coordinates, zoom)
            }
        }
    }
    /// Get a collection of tiles that cover a [`VectorFeeature`] that is in S2's S-T coordinate space
    pub fn from_vector_feature<M: Clone + Default, P: Clone + Default, D: Clone + Default>(
        feature: &VectorFeature<M, P, D>,
        zoom: u8,
    ) -> Vec<Self> {
        S2TileID::from_vector_geometry(&feature.geometry, feature.face, zoom)
    }
    /// Get a collection of tiles that cover an input JSON object in any coordinate space
    pub fn from_json<M: Clone + Default, P: Clone + Default, D: Clone + Default>(
        json: &JSONCollection<M, P, D>,
        zoom: u8,
    ) -> Vec<Self> {
        let features = convert(Projection::S2, json, None, None);
        let mut tiles = BTreeSet::new();
        for feature in &features {
            let feature_tiles = S2TileID::from_vector_feature(feature, zoom);
            tiles.extend(feature_tiles);
        }
        GetTileID::merge(tiles)
    }
}

/// # WM Tile's metadata
///
/// Implements the [`GetTileID`] trait
///
/// ## Usage
///
/// Methods that are available:
/// - [`WMTileID::new`]: Create a new [`WMTileID`]
/// - [`WMTileID::face`]: Get the tile's face
/// - [`WMTileID::zoom`]: Get the tile's zoom
/// - [`WMTileID::x`]: Get the tile's x coordinate
/// - [`WMTileID::y`]: Get the tile's y coordinate
/// - [`WMTileID::fzxy`]: Get the face, zoom, x, and y from the tile
/// - [`WMTileID::to_id`]: Convert to an [`S2CellId`]
/// - [`WMTileID::from_id`]: Convert from a [`S2CellId`]
/// - [`WMTileID::to_bbox`]: Covnert to a [`BBox`]
/// - [`WMTileID::to_center_lon_lat`]: Find the center lon-lat of the tile
/// - [`WMTileID::children`]: Get the children of the tile
/// - [`WMTileID::parent`]: Get the parent of the tile
/// - [`WMTileID::neighbors`]: Find the neighbors of the tile
/// - [`WMTileID::from_point`]: Get the tile from a Lon-Lat WGS84 coordinate
/// - [`WMTileID::from_linestring`]: Get a collection of tiles that cover a linestring from Lon-Lat WGS84 coordinates
/// - [`WMTileID::from_multilinestring`]: Get a collection of tiles that cover a multilinestring from Lon-Lat WGS84 coordinates
/// - [`WMTileID::from_polygon`]: Get a collection of tiles that cover a polygon from Lon-Lat WGS84 coordinates
/// - [`WMTileID::from_multipolygon`]: Get a collection of tiles that cover a multipolygon from Lon-Lat WGS84 coordinates
/// - [`WMTileID::from_geometry`]: Get a collection of tiles that cover a geometry from Lon-Lat WGS84 coordinates
/// - [`WMTileID::from_feature`]: Get a collection of tiles that cover a feature from Lon-Lat WGS84 coordinates
/// - [`WMTileID::from_vector_geometry`]: Get a collection of tiles that cover a vector geometry from Lon-Lat WGS84 coordinates
/// - [`WMTileID::from_vector_feature`]: Get a collection of tiles that cover a vector feature from Lon-Lat WGS84 coordinates
/// - [`WMTileID::from_json`]: Get a collection of tiles that cover a JSON object in any coordinate space
///
/// Into relations:
/// - [`S2CellId`]
/// - [`TileID`]
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct WMTileID {
    /// Zoom level
    pub zoom: u8,
    /// X tile coordinate
    pub x: u32,
    /// Y tile coordinate
    pub y: u32,
}
impl GetTileID for WMTileID {
    fn new(_face: Option<Face>, zoom: u8, x: u32, y: u32) -> Self {
        Self { zoom, x, y }
    }
    fn face(&self) -> Option<Face> {
        None
    }
    fn zoom(&self) -> u8 {
        self.zoom
    }
    fn x(&self) -> u32 {
        self.x
    }
    fn y(&self) -> u32 {
        self.y
    }
    fn to_id(&self) -> S2CellId {
        S2CellId::from_face_ij(0, self.x, self.y, Some(self.zoom))
    }
    fn to_bbox(&self, tms_style: Option<bool>) -> BBox {
        xyz_to_bbox(self.x as i64, self.y as i64, self.zoom as f64, tms_style, Some(Source::WGS84))
            .into()
    }
    fn to_center_lon_lat<P: NewXY>(&self, tms_style: Option<bool>) -> P {
        let BBox { left, bottom, right, top } = self.to_bbox(tms_style);
        P::new_xy((left + right) / 2., (top + bottom) / 2.)
    }
    fn parent(&self) -> Option<WMTileID> {
        if self.zoom == 0 {
            None
        } else {
            Some(WMTileID::new(self.zoom - 1, self.x >> 1, self.y >> 1))
        }
    }
    fn neighbors(&self) -> Vec<WMTileID> {
        if self.zoom == 0 {
            return vec![];
        }
        // allow wrapping horizontally
        let Self { zoom, x, y } = *self;
        let max_grid = (1 << zoom) - 1;
        let mut neighbors = BTreeSet::<WMTileID>::new();
        // wrap X tiles
        if x + 1 <= max_grid {
            neighbors.insert(WMTileID::new(zoom, x + 1, y));
        } else {
            neighbors.insert(WMTileID::new(zoom, 0, y));
        }
        if (x as i32) - 1 >= 0 {
            neighbors.insert(WMTileID::new(zoom, x - 1, y));
        } else {
            neighbors.insert(WMTileID::new(zoom, max_grid, y));
        }
        // vertical Y has a max and min
        if y + 1 <= max_grid {
            neighbors.insert(WMTileID::new(zoom, x, y + 1));
        }
        if (y as i32) - 1 >= 0 {
            neighbors.insert(WMTileID::new(zoom, x, y - 1));
        }

        neighbors.into_iter().collect()
    }
    fn children(&self) -> Vec<Self> {
        let Self { zoom, x, y } = self;
        vec![
            WMTileID::new(zoom + 1, x * 2, y * 2),
            WMTileID::new(zoom + 1, x * 2, y * 2 + 1),
            WMTileID::new(zoom + 1, x * 2 + 1, y * 2),
            WMTileID::new(zoom + 1, x * 2 + 1, y * 2 + 1),
        ]
    }
}
impl From<S2CellId> for WMTileID {
    fn from(id: S2CellId) -> Self {
        WMTileID::from_id(id)
    }
}
impl From<WMTileID> for S2CellId {
    fn from(id: WMTileID) -> Self {
        id.to_id()
    }
}
impl WMTileID {
    /// Create a new WMTileID
    pub fn new(zoom: u8, x: u32, y: u32) -> Self {
        Self { zoom, x, y }
    }
    /// Convert a CellID to a WMTileID
    pub fn from_id(id: S2CellId) -> Self {
        let (_, zoom, i, j) = id.to_face_ij();
        Self { zoom, x: i, y: j }
    }
    /// Get the tile from a Lon-Lat WGS84 coordinate
    pub fn from_point<P: GetXY + NewXY>(point: &P, zoom: u8) -> Self {
        let (x, y) = ll_to_tile(point, zoom as f64);
        Self { zoom, x: x as u32, y: y as u32 }
    }
    /// Get the tile from Lon-Lat WGS84 coordinates
    pub fn from_multipoint<P: GetXY + NewXY>(points: &[P], zoom: u8) -> Vec<Self> {
        points.iter().map(|p| Self::from_point(p, zoom)).collect()
    }
    /// Get a collection of tiles that cover a linestring from Lon-Lat WGS84 coordinates
    pub fn from_linestring<P: GetXY + NewXY>(linestring: &[P], zoom: u8) -> Vec<Self> {
        let mut tiles = BTreeSet::new();
        line_cover(&mut tiles, linestring, zoom, &mut vec![], None);
        GetTileID::merge(tiles)
    }
    /// Get a collection of tiles that cover a multilinestring from Lon-Lat WGS84 coordinates
    pub fn from_multilinestring<P: GetXY + NewXY>(
        multilinestring: &[Vec<P>],
        zoom: u8,
    ) -> Vec<Self> {
        let mut tiles = BTreeSet::new();
        for line in multilinestring {
            line_cover(&mut tiles, line, zoom, &mut vec![], None);
        }
        GetTileID::merge(tiles)
    }
    /// Get a collection of tiles that cover a polygon from Lon-Lat WGS84 coordinates
    pub fn from_polygon<P: GetXY + NewXY>(polygon: &[Vec<P>], zoom: u8) -> Vec<Self> {
        let mut tiles = BTreeSet::new();
        poly_cover(&mut tiles, polygon, zoom, None);
        GetTileID::merge(tiles)
    }
    /// Get a collection of tiles that cover a multipolygon from Lon-Lat WGS84 coordinates
    pub fn from_multipolygon<P: GetXY + NewXY>(polygons: &[Vec<Vec<P>>], zoom: u8) -> Vec<Self> {
        let mut tiles = BTreeSet::new();
        for polygon in polygons {
            poly_cover(&mut tiles, polygon, zoom, None);
        }
        GetTileID::merge(tiles)
    }
    /// Get a collection of tiles that cover an [`Geometry`] from Lon-Lat WGS84 coordinates
    pub fn from_geometry<M: Clone + Default>(geometry: &Geometry<M>, zoom: u8) -> Vec<Self> {
        match geometry {
            Geometry::Point(g) => vec![WMTileID::from_point(&g.coordinates, zoom)],
            Geometry::MultiPoint(g) => WMTileID::from_multipoint(&g.coordinates, zoom),
            Geometry::LineString(g) => WMTileID::from_linestring(&g.coordinates, zoom),
            Geometry::MultiLineString(g) => WMTileID::from_multilinestring(&g.coordinates, zoom),
            Geometry::Polygon(g) => WMTileID::from_polygon(&g.coordinates, zoom),
            Geometry::MultiPolygon(g) => WMTileID::from_multipolygon(&g.coordinates, zoom),
            Geometry::Point3D(g) => vec![WMTileID::from_point(&g.coordinates, zoom)],
            Geometry::MultiPoint3D(g) => WMTileID::from_multipoint(&g.coordinates, zoom),
            Geometry::LineString3D(g) => WMTileID::from_linestring(&g.coordinates, zoom),
            Geometry::MultiLineString3D(g) => WMTileID::from_multilinestring(&g.coordinates, zoom),
            Geometry::Polygon3D(g) => WMTileID::from_polygon(&g.coordinates, zoom),
            Geometry::MultiPolygon3D(g) => WMTileID::from_multipolygon(&g.coordinates, zoom),
        }
    }
    /// Get a collection of tiles that cover an [`VectorGeometry`] from Lon-Lat WGS84 coordinates
    pub fn from_vector_geometry<M: Clone + Default>(
        vector_geometry: &VectorGeometry<M>,
        zoom: u8,
    ) -> Vec<Self> {
        match vector_geometry {
            VectorGeometry::Point(g) => vec![WMTileID::from_point(&g.coordinates, zoom)],
            VectorGeometry::MultiPoint(g) => WMTileID::from_multipoint(&g.coordinates, zoom),
            VectorGeometry::LineString(g) => WMTileID::from_linestring(&g.coordinates, zoom),
            VectorGeometry::MultiLineString(g) => {
                WMTileID::from_multilinestring(&g.coordinates, zoom)
            }
            VectorGeometry::Polygon(g) => WMTileID::from_polygon(&g.coordinates, zoom),
            VectorGeometry::MultiPolygon(g) => WMTileID::from_multipolygon(&g.coordinates, zoom),
        }
    }
    /// Get a collection of tiles that cover a [`Feature`] from Lon-Lat WGS84 coordinates
    pub fn from_feature<M: Clone + Default, P: Clone + Default, D: Clone + Default>(
        feature: &Feature<M, P, D>,
        zoom: u8,
    ) -> Vec<Self> {
        WMTileID::from_geometry(&feature.geometry, zoom)
    }
    /// Get a collection of tiles that cover a [`VectorFeature`] from Lon-Lat WGS84 coordinates
    pub fn from_vector_feature<M: Clone + Default, P: Clone + Default, D: Clone + Default>(
        vector_feature: &VectorFeature<M, P, D>,
        zoom: u8,
    ) -> Vec<Self> {
        WMTileID::from_vector_geometry(&vector_feature.geometry, zoom)
    }
    /// Get a collection of tiles that cover an input JSON object in any coordinate space
    pub fn from_json<M: Clone + Default, P: Clone + Default, D: Clone + Default>(
        json: &JSONCollection<M, P, D>,
        zoom: u8,
    ) -> Vec<Self> {
        let features = convert(Projection::WG, json, None, None);
        let mut tiles = BTreeSet::new();
        for feature in &features {
            let feature_tiles = WMTileID::from_vector_feature(feature, zoom);
            tiles.extend(feature_tiles);
        }
        GetTileID::merge(tiles)
    }
}

/// # Tile's metadata
///
/// ## Usage
///
/// Methods that are available:
/// - [`TileID::new_wm`]: Create a new [`TileID`] that wraps a [`WMTileID`]
/// - [`TileID::new_s2`]: Create a new [`TileID`] that wraps a [`S2TileID`]
/// - [`TileID::face`]: Get the face of the tile
/// - [`TileID::zoom`]: Get the zoom level of the tile
/// - [`TileID::x`]: Get the x position of the tile
/// - [`TileID::y`]: Get the y position of the tile
/// - [`TileID::to_id`]: Convert to an [`S2CellId`]
/// - [`TileID::from_id`]: Convert from a [`S2CellId`]
/// - [`TileID::to_bbox`]: Covnert to a [`BBox`]
/// - [`TileID::to_center_lon_lat`]: Find the center lon-lat of the tile
/// - [`TileID::children`]: Get the children of the tile
/// - [`TileID::parent`]: Get the parent of the tile
/// - [`TileID::neighbors`]: Find the neighbors of the tile
///
/// Into relations:
/// - [`S2CellId`]
/// - [`WMTileID`]
/// - [`S2TileID`]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum TileID {
    /// Web Mercator metadata
    WM(WMTileID),
    /// S2 Mercator metadata
    S2(S2TileID),
}
impl GetTileID for TileID {
    fn new(face: Option<Face>, zoom: u8, x: u32, y: u32) -> Self {
        if let Some(face) = face {
            Self::S2(S2TileID::new(face, zoom, x, y))
        } else {
            Self::WM(WMTileID::new(zoom, x, y))
        }
    }
    fn face(&self) -> Option<Face> {
        match self {
            TileID::WM(_) => None,
            TileID::S2(s2) => Some(s2.face),
        }
    }
    fn zoom(&self) -> u8 {
        match self {
            TileID::WM(wm) => wm.zoom,
            TileID::S2(s2) => s2.zoom,
        }
    }
    fn x(&self) -> u32 {
        match self {
            TileID::WM(wm) => wm.x,
            TileID::S2(s2) => s2.x,
        }
    }
    fn y(&self) -> u32 {
        match self {
            TileID::WM(wm) => wm.y,
            TileID::S2(s2) => s2.y,
        }
    }
    fn to_id(&self) -> S2CellId {
        match self {
            TileID::WM(wm) => wm.to_id(),
            TileID::S2(s2) => s2.to_id(),
        }
    }
    fn to_bbox(&self, tms_style: Option<bool>) -> BBox {
        match self {
            TileID::WM(wm) => wm.to_bbox(tms_style),
            TileID::S2(s2) => s2.to_bbox(tms_style),
        }
    }
    fn to_center_lon_lat<P: NewXY>(&self, tms_style: Option<bool>) -> P {
        match self {
            TileID::WM(wm) => wm.to_center_lon_lat(tms_style),
            TileID::S2(s2) => s2.to_center_lon_lat(tms_style),
        }
    }
    fn parent(&self) -> Option<TileID> {
        match self {
            TileID::WM(wm) => wm.parent().map(Into::into),
            TileID::S2(s2) => s2.parent().map(Into::into),
        }
    }
    fn neighbors(&self) -> Vec<TileID> {
        match self {
            TileID::WM(wm) => wm.neighbors().into_iter().map(Into::into).collect(),
            TileID::S2(s2) => s2.neighbors().into_iter().map(Into::into).collect(),
        }
    }
    fn children(&self) -> Vec<Self> {
        match self {
            TileID::WM(wm) => wm.children().into_iter().map(TileID::from).collect(),
            TileID::S2(s2) => s2.children().into_iter().map(TileID::from).collect(),
        }
    }
}
impl From<WMTileID> for TileID {
    fn from(wm: WMTileID) -> Self {
        Self::WM(wm)
    }
}
impl From<TileID> for WMTileID {
    fn from(tile: TileID) -> Self {
        match tile {
            TileID::WM(wm) => wm,
            TileID::S2(_) => panic!("S2 tile cannot be converted to WM tile"),
        }
    }
}
impl From<S2TileID> for TileID {
    fn from(s2: S2TileID) -> Self {
        Self::S2(s2)
    }
}
impl From<TileID> for S2TileID {
    fn from(tile: TileID) -> Self {
        match tile {
            TileID::WM(_) => panic!("WM tile cannot be converted to S2 tile"),
            TileID::S2(s2) => s2,
        }
    }
}
impl From<TileID> for S2CellId {
    fn from(tile: TileID) -> Self {
        match tile {
            TileID::WM(wm) => wm.into(),
            TileID::S2(s2) => s2.into(),
        }
    }
}
impl TileID {
    /// new wm
    pub fn new_wm(zoom: u8, x: u32, y: u32) -> Self {
        Self::WM(WMTileID::new(zoom, x, y))
    }
    /// new s2
    pub fn new_s2(face: Face, zoom: u8, x: u32, y: u32) -> Self {
        Self::S2(S2TileID::new(face, zoom, x, y))
    }
    /// Convert from an S2CellId into a TileID
    pub fn from_id(id: S2CellId, is_wm: bool) -> Self {
        if is_wm { Self::WM(WMTileID::from_id(id)) } else { Self::S2(S2TileID::from_id(id)) }
    }
}
impl Default for TileID {
    fn default() -> Self {
        Self::WM(WMTileID::default())
    }
}
impl HasLayer for TileID {
    fn get_layer(&self) -> Option<String> {
        None
    }
}

fn s2_bounds(tile_id: S2TileID) -> BBox {
    let id = S2CellId::from_face_ij(tile_id.face as u8, tile_id.x, tile_id.y, Some(tile_id.zoom));
    let [a, b, c, d] = id.get_vertices();
    let [a, b, c, d]: [LonLat; 4] = [(&a).into(), (&b).into(), (&c).into(), (&d).into()];

    BBox {
        left: f64::min(f64::min(f64::min(a.lon(), b.lon()), c.lon()), d.lon()),
        bottom: f64::min(f64::min(f64::min(a.lat(), b.lat()), c.lat()), d.lat()),
        right: f64::max(f64::max(f64::max(a.lon(), b.lon()), c.lon()), d.lon()),
        top: f64::max(f64::max(f64::max(a.lat(), b.lat()), c.lat()), d.lat()),
    }
}

fn poly_cover<P: GetXY + NewXY>(
    tile_map: &mut BTreeSet<WMTileID>,
    poly: &[Vec<P>],
    zoom: u8,
    coord_to_px: Option<fn(&P, f64, Option<bool>, Option<u64>) -> P>,
) {
    let mut intersections: Vec<(u32, u32)> = vec![];

    for line in poly {
        let mut ring = vec![];
        line_cover(tile_map, line, zoom, &mut ring, coord_to_px);

        let len = ring.len();
        if len < 3 {
            continue;
        }
        for j in 0..len {
            let k = if j == 0 { len - 1 } else { j - 1 };
            let m = (j + 1) % len;

            let y = ring[j].1;
            let y_prev = ring[k].1;
            let y_next = ring[m].1;

            // Correctly check if ring[j] is an intersection point; that is NOT a local extremum
            if (y > y_prev || y > y_next) && // not local minimum
       (y < y_prev || y < y_next) && // not local maximum
       y != y_next
            {
                intersections.push(ring[j]);
            }
        }
    }

    intersections.sort_by(|a, b| {
        (a.1 as i64 - b.1 as i64)
            .partial_cmp(&0)
            .unwrap_or(Ordering::Equal)
            .then_with(|| (a.0 as i64 - b.0 as i64).partial_cmp(&0).unwrap_or(Ordering::Equal))
    });

    // even-odd fill
    for i in (0..intersections.len() - 1).step_by(2) {
        let y = intersections[i].1;
        for x in (intersections[i].0 + 1)..(intersections[i + 1].0) {
            tile_map.insert(WMTileID::new(zoom, x, y));
        }
    }
}

fn s2_to_px<P: GetXY + NewXY>(
    pt: &P,
    zoom: f64,
    _anti_meridian: Option<bool>,
    _tile_size: Option<u64>,
) -> P {
    let map_size = 512. * pow(2., zoom);
    P::new_xy(pt.x() * map_size, pt.y() * map_size)
}

// Modified Digital Differential Analyzer algorithm
fn line_cover<P: GetXY + NewXY>(
    tile_map: &mut BTreeSet<WMTileID>,
    coords: &[P],
    zoom: u8,
    ring: &mut Vec<(u32, u32)>,
    coord_to_px: Option<fn(&P, f64, Option<bool>, Option<u64>) -> P>,
) {
    let coord_to_px = coord_to_px.unwrap_or(ll_to_px);
    let mut prev_x = None;
    let mut prev_y = None;

    for i in 0..coords.len() - 1 {
        let a = coord_to_px(&coords[i], zoom as f64, Some(false), Some(512));
        let b = coord_to_px(&coords[i + 1], zoom as f64, Some(false), Some(512));
        let mut x = floor(a.x() / 512.) as u32;
        let mut y = floor(a.y() / 512.) as u32;
        let end_x = floor(b.x() / 512.) as u32;
        let end_y = floor(b.y() / 512.) as u32;
        let dx = b.x() - a.x();
        let dy = b.y() - a.y();
        if dx == 0. && dy == 0. {
            continue;
        }
        let step_x: i32 = if dx > 0. { 1 } else { -1 };
        let step_y: i32 = if dy > 0. { 1 } else { -1 };
        let mut t_max_x = if dx == 0. {
            f64::INFINITY
        } else {
            fabs(((if step_x > 0 { 1. } else { 0. }) * 512. + x as f64 * 512. - a.x()) / dx)
        };
        let mut t_max_y = if dy == 0. {
            f64::INFINITY
        } else {
            fabs(((if step_y > 0 { 1. } else { 0. }) * 512. + y as f64 * 512. - a.y()) / dy)
        };
        let t_delta_x = if dx == 0. { f64::INFINITY } else { fabs(512. / dx) };
        let t_delta_y = if dy == 0. { f64::INFINITY } else { fabs(512. / dy) };

        // Initial tile check
        if Some(x) != prev_x || Some(y) != prev_y {
            tile_map.insert(WMTileID::new(zoom, x, y));
            // Record intersection only if the Y row changed
            if Some(y) != prev_y {
                ring.push((x, y));
            }
            prev_x = Some(x);
            prev_y = Some(y);
        }
        // Main loop
        while x != end_x || y != end_y {
            if t_max_x < t_max_y {
                t_max_x += t_delta_x;
                x = if step_x > 0 { x + 1 } else { x.saturating_sub(1) };
            } else {
                t_max_y += t_delta_y;
                y = if step_y > 0 { y + 1 } else { y.saturating_sub(1) };
            }
            tile_map.insert(WMTileID::new(zoom, x, y));
            // Record intersection only if the Y row changed
            if Some(y) != prev_y {
                ring.push((x, y));
            }
            prev_x = Some(x);
            prev_y = Some(y);
        }
    }

    // If the last intersection recorded is on the same row as the very first
    // intersection, we pop it to maintain parity (closing the loop).
    if ring.len() > 1 && ring[0].1 == ring[ring.len() - 1].1 {
        ring.pop();
    }
}
