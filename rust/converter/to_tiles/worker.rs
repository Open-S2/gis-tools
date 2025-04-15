use super::{
    BuildGuide, FormatOutput, LayerGuide, LayerHandler, MVectorFeature, ToTileMetadata,
    VectorLayerGuide,
};
use crate::{
    data_store::{MultiMap, MultiMapStore},
    data_structures::{HasLayer, PointCluster, PointGrid, TileStore, TileStoreOptions},
    geometry::S2CellId,
    readers::RGBA,
    util::{CompressionFormat, compress_data},
};
use alloc::{collections::BTreeMap, string::String, vec, vec::Vec};
use core::mem::take;
use earclip::{earclip, tesselate};
use libm::{floor, fmax, fmin};
use open_vector_tile::{
    base::{BaseVectorLayer, BaseVectorTile, s2json_to_base},
    mapbox, write_tile,
};
use s2_tilejson::DrawType;
use s2json::{
    Face, JSONCollection, MValue, MValueCompatible, Projection, VectorFeature, VectorGeometry,
    VectorPolygon,
};

/// A built tile that is ready to be written to the filesystem
#[derive(Debug)]
pub struct BuiltTile {
    /// Face of the tile
    pub face: Face,
    /// zoom of the tile
    pub zoom: u8,
    /// x tile coordinate
    pub x: u32,
    /// y tile coordinate
    pub y: u32,
    /// compressed data
    pub data: Vec<u8>,
}

/// Convert a vector feature to a collection of tiles and store each tile feature
#[derive(Debug)]
pub struct TileWorker {
    /// Id of the worker (useful for threads)
    pub id: usize,
    /// total minzoom
    pub minzoom: u8,
    /// total maxzoom
    pub maxzoom: u8,
    /// Description of how to build data
    build_guide: BuildGuide,
    /// Store Vector Features whose key are Tile IDs.
    pub vector_store: MultiMap<S2CellId, MVectorFeature>,
    /// Unique store for each layer that describes itself as a cluster source
    pub cluster_stores: BTreeMap<String, PointCluster<MValue>>, /* { [layerName: string]: PointCluster } = {}; */
    /// Unique store for each layer that describes itself as a raster source
    pub raster_stores: BTreeMap<String, PointGrid<RGBA>>, /* { [layerName: string]: PointGrid<RGBA> } = {}; */
    /// Unique store for each layer that describes itself as a grid source
    pub grid_stores: BTreeMap<String, PointGrid<f64>>, // { [layerName: string]: PointGrid } = {};
}
impl TileWorker {
    /// Create a new TileWorker
    pub fn new(id: usize, build_guide: BuildGuide) -> Self {
        let (minzoom, maxzoom) = get_zooms(&build_guide.layer_guides);
        Self {
            id,
            minzoom,
            maxzoom,
            build_guide,
            vector_store: MultiMap::<S2CellId, MVectorFeature>::new(None),
            cluster_stores: BTreeMap::new(),
            raster_stores: BTreeMap::new(),
            grid_stores: BTreeMap::new(),
        }
    }
    /// Iterate through all the stores and sort/cluster as needed
    pub fn sort(&mut self) {
        for cluster in self.cluster_stores.values_mut() {
            cluster.build_clusters(None);
        }
        for raster in self.raster_stores.values_mut() {
            raster.build_clusters();
        }
        for grid in self.grid_stores.values_mut() {
            grid.build_clusters();
        }
    }

    /// Store a feature to the appropriate store
    pub fn store_feature<M: Clone + HasLayer, P: MValueCompatible, D: MValueCompatible>(
        &mut self,
        mut feature: VectorFeature<M, P, D>,
        source_name: String,
        on_layer_feature: Option<&Vec<LayerHandler<M, P, D>>>,
    ) {
        // First find all build guide layers that use this source
        let layer_guides = self
            .build_guide
            .layer_guides
            .iter()
            .filter(|lg| lg.has_source(&source_name))
            .cloned()
            .collect::<Vec<_>>();
        // iterate through each layer and store the feature
        for layer_guide in layer_guides {
            // find the layer handler that matches layer_guide's layer_name and mutate if needed
            if let Some(on_layer_feature) = &on_layer_feature {
                if let Some(layer_handler) =
                    on_layer_feature.iter().find(|lh| lh.layer_name == layer_guide.layer_name())
                {
                    if let Some(new_feature) = (layer_handler.on_feature)(take(&mut feature)) {
                        feature = new_feature;
                    }
                }
            }
            match layer_guide {
                LayerGuide::Vector(vlg) => {
                    self.store_vector_feature(
                        feature.to_m_vector_feature(|_| {
                            Some(ToTileMetadata::new(vlg.base.layer_name.clone()))
                        }),
                        vlg,
                    );
                }
                _ => {
                    unimplemented!()
                }
            }
        }
    }

    /// Store a vector feature across all appropriate zooms
    pub fn store_vector_feature(&mut self, feature: MVectorFeature, layer: VectorLayerGuide) {
        let BuildGuide { projection, .. } = &self.build_guide;
        // skip features who are not using the layer guide's
        if layer.draw_types.contains(&to_draw_type(&feature)) {
            return;
        }
        let minzoom = layer.vector_guide.minzoom.unwrap_or(0);
        // Setup a tile_cache and dive down. Store the 4 children if data is found while storing data as we go
        let mut tile_store = TileStore::new(
            JSONCollection::VectorFeature(feature),
            TileStoreOptions { projection: Some(*projection), ..layer.vector_guide },
        );
        let mut tile_cache = vec![S2CellId::from_face(0)];
        if *projection == Projection::S2 {
            tile_cache.extend([
                S2CellId::from_face(1),
                S2CellId::from_face(2),
                S2CellId::from_face(3),
                S2CellId::from_face(4),
                S2CellId::from_face(5),
            ]);
        }
        while let Some(id) = tile_cache.pop() {
            let (_, zoom, _, _) = id.to_face_ij();
            let tile = tile_store.get_tile(id);
            if minzoom > zoom {
                // if we haven't reached the data yet, we store children
                tile_cache.extend(id.children(None));
            } else if let Some(tile) = tile
                && !tile.is_empty()
            {
                // store feature with the associated layername
                for layer in tile.layers.values() {
                    for feature in &layer.features {
                        self.vector_store.set(id, feature.clone());
                    }
                }
                // store 4 children tiles to ask for
                tile_cache.extend(id.children(None));
            }
        }
    }

    /// Get vector/cluster features for a tile
    fn get_vector_vile(&mut self, id: S2CellId) -> Option<BaseVectorTile> {
        let BuildGuide { layer_guides, format, build_indices, .. } = &self.build_guide;
        if *format == FormatOutput::Raster {
            return None;
        }
        let mut res = BaseVectorTile::default();
        let zoom = id.level();
        // store vector features
        if let Some(vector_features) = self.vector_store.get(id).cloned() {
            for mut feature in vector_features {
                let layer_name = feature.metadata.as_ref().unwrap().layer_name.clone();
                let layer = layer_guides
                    .iter()
                    .find(|lg| lg.layer_name() == layer_name)
                    .unwrap()
                    .to_vector()
                    .unwrap();
                if *build_indices {
                    // pre-earclip, since we are still working with floats, the extent of 1 is fine! :D
                    earclip_polygons(&mut feature, zoom);
                }
                let layer = res.layers.entry(layer_name.clone()).or_insert(BaseVectorLayer::new(
                    layer_name,
                    layer.extent,
                    vec![],
                    layer.shape,
                    layer.m_shape,
                ));
                layer.add_feature(s2json_to_base(&feature, layer.extent));
            }
        }
        // // store all cluster features
        // for (const [layerName, cluster] of Object.entries(this.cluster_stores)) {
        //   const layerClusterFeatures = await cluster.getTile(id);
        //   if (layerClusterFeatures === undefined) continue;
        //   for (const layer of Object.values(layerClusterFeatures.layers)) {
        //     for (const feature of layer.features) tile.addFeature(feature, layerName);
        //   }
        // }

        if res.layers.is_empty() { None } else { Some(res) }
    }

    /// Iterate through all the stores and sort/cluster as needed and build tiles
    pub fn build_tiles(&mut self) -> TileWorkerTileBuilder {
        let mut tile_stack = vec![S2CellId::from_face(0)];
        if self.build_guide.projection == Projection::S2 {
            tile_stack.extend([
                S2CellId::from_face(1),
                S2CellId::from_face(2),
                S2CellId::from_face(3),
                S2CellId::from_face(4),
                S2CellId::from_face(5),
            ]);
        }
        TileWorkerTileBuilder { worker: self, tile_stack }
    }
}
/// Iterate through the stores and build tiles, compressing as we go if required
#[derive(Debug)]
pub struct TileWorkerTileBuilder<'a> {
    worker: &'a mut TileWorker,
    tile_stack: Vec<S2CellId>,
}
impl Iterator for TileWorkerTileBuilder<'_> {
    type Item = BuiltTile;
    fn next(&mut self) -> Option<Self::Item> {
        let build_guide = &self.worker.build_guide;
        let format = build_guide.format.clone();
        let encoding: CompressionFormat = (&build_guide.encoding).into();
        while let Some(id) = self.tile_stack.pop() {
            // if the current id is less than our target zoom, we add children to the stack and continue
            let (face, zoom, x, y) = id.to_face_ij();
            if zoom < self.worker.maxzoom {
                self.tile_stack.extend(id.children(None));
                continue;
            }
            let mut vector_tile = self.worker.get_vector_vile(id);
            // otherwise, we build the tile
            if format != FormatOutput::Raster {
                let mut data = if format == FormatOutput::OpenS2 {
                    write_tile(vector_tile.as_mut(), None, None)
                } else if let Some(mut vector_tile) = vector_tile {
                    mapbox::vector_tile::write_tile(
                        &mut vector_tile,
                        format == FormatOutput::Mapbox,
                    )
                } else {
                    vec![]
                };
                if data.is_empty() {
                    continue;
                } else {
                    data = compress_data(data, encoding).unwrap();
                    return Some(BuiltTile { face: face.into(), zoom, x, y, data });
                }
            }
        }
        None
    }
}

//   /**
//    * Iterate through the stores and build tiles, compressing as we go if required
//    * @yields - a built tile
//    */
//   async *buildTiles(): AsyncGenerator<BuiltTile> {
//     const { format, layer_guides, projection, encoding } = this;
//     const minzoom = getMinzoom(layer_guides);

//     // three directions we can build data
//     const tile_cache = [idFromFace(0)];
//     if (projection === 'S2')
//       tile_cache.push(idFromFace(1), idFromFace(2), idFromFace(3), idFromFace(4), idFromFace(5));
//     while (tile_cache.length > 0) {
//       const id = tile_cache.pop()!;
//       const tile = new Tile(id);
//       const { face, zoom, i: x, j: y } = tile;

//       const vectorTile = await this.#get_vector_vile(id, tile);
//       const rasterData = await this.#getRasterTile(id);
//       const gridData = await this.#getGridTile(id);
//       if (format === 'raster') {
//         // RASTER CASE
//         if (rasterData !== undefined) {
//           const data = new Uint8Array(rasterData[0].image);
//           yield { face, zoom, x, y, data };
//           // store 4 children tiles to ask for children features
//           tile_cache.push(...idChildrenIJ(face, zoom, x, y));
//         } else {
//           // if we haven't reached the data yet, we store children
//           if (minzoom > tile.zoom) tile_cache.push(...idChildrenIJ(face, zoom, x, y));
//         }
//       } else {
//         // VECTOR CASE
//         if (vectorTile === undefined && rasterData === undefined && gridData === undefined) {
//           // if we haven't reached the data yet, we store children
//           if (minzoom > tile.zoom) tile_cache.push(...idChildrenIJ(face, zoom, x, y));
//         } else {
//           // write to a buffer using the open-vector-tile spec
//           let data =
//             format === 'open-s2'
//               ? writeOVTile(vectorTile, rasterData, gridData)
//               : writeMVTile(vectorTile!, format === 'mapbox');
//           // gzip if necessary
//           if (encoding === 'gz') data = await compressStream(data, 'gzip');
//           // yield the buffer
//           yield { face, zoom, x, y, data };
//           // store 4 children tiles to ask for children features
//           tile_cache.push(...idChildrenIJ(face, zoom, x, y));
//         }
//       }
//     }
//   }

//   /**
//    * Get raster data for a tile
//    * @param id - the tile id
//    * @returns - a collection of GridInputs
//    */
//   async #getRasterTile(id: S2CellId): Promise<ImageDataInput[] | undefined> {
//     const res: ImageDataInput[] = [];
//     // store all cluster features
//     for (const raster of Object.values(this.raster_stores)) {
//       const layerGrid = await raster.getTile(id);
//       if (layerGrid === undefined) continue;
//       const { name, size, data } = layerGrid;
//       const image = (data as RGBA[]).flatMap(({ r, g, b, a }) => [r, g, b, a]);
//       res.push({
//         name,
//         type: 'raw',
//         width: size,
//         height: size,
//         image: new Uint8Array(image),
//       });
//     }

//     if (res.length > 0) return res;
//   }

//   /**
//    * Get gridded data for a tile
//    * @param id - the tile id
//    * @returns - a collection of ImageDataInputs
//    */
//   async #getGridTile(id: S2CellId): Promise<GridInput[] | undefined> {
//     const res: GridInput[] = [];
//     // store all cluster features
//     for (const [layerName, grid] of Object.entries(this.grid_stores)) {
//       const { extent } = this.layer_guides.filter(
//         (guide) => guide.layerName === layerName,
//       )[0] as GridLayer;
//       const layerGrid = await grid.getTile(id);
//       if (layerGrid === undefined) continue;
//       const { name, size, data } = layerGrid;
//       res.push({
//         name,
//         size,
//         data: data as number[],
//         extent,
//       });
//     }

//     if (res.length > 0) return res;
//   }

/// Get the absolute maxzoom from the layer guides
/// returns the absolute maxzoom
fn get_zooms(layer_guides: &[LayerGuide]) -> (u8, u8) {
    let mut min: u8 = 30;
    let mut max = 0;

    for layer_guide in layer_guides {
        let (l_min, l_max) = layer_guide.zooms();
        if l_min < min {
            min = l_min;
        }
        if l_max > max {
            max = l_max;
        }
    }

    (min, max)
}

/// Check if a feature is included by draw types defined by the layer guide
fn to_draw_type<M: Clone, P: MValueCompatible, D: MValueCompatible>(
    feature: &VectorFeature<M, P, D>,
) -> DrawType {
    match &feature.geometry {
        VectorGeometry::Point(p) => {
            if p.is_3d {
                DrawType::Points3D
            } else {
                DrawType::Points
            }
        }
        VectorGeometry::MultiPoint(mp) => {
            if mp.is_3d {
                DrawType::Points3D
            } else {
                DrawType::Points
            }
        }
        VectorGeometry::LineString(l) => {
            if l.is_3d {
                DrawType::Lines3D
            } else {
                DrawType::Lines
            }
        }
        VectorGeometry::MultiLineString(ml) => {
            if ml.is_3d {
                DrawType::Lines3D
            } else {
                DrawType::Lines
            }
        }
        VectorGeometry::Polygon(p) => {
            if p.is_3d {
                DrawType::Polygons3D
            } else {
                DrawType::Polygons
            }
        }
        VectorGeometry::MultiPolygon(mp) => {
            if mp.is_3d {
                DrawType::Polygons3D
            } else {
                DrawType::Polygons
            }
        }
    }
}

/// Pre-earclip polygons for faster processing of the tile client side
pub fn earclip_polygons<M: Clone, P: MValueCompatible, D: MValueCompatible>(
    feature: &mut VectorFeature<M, P, D>,
    zoom: u8,
) {
    let mut polys: Vec<&VectorPolygon<D>> = vec![];
    match &feature.geometry {
        VectorGeometry::Polygon(poly) => {
            polys.push(&poly.coordinates);
        }
        VectorGeometry::MultiPolygon(multipoly) => {
            for poly in &multipoly.coordinates {
                polys.push(poly);
            }
        }
        _ => {}
    }

    let mut offset = 0;
    let mut verts = vec![];
    let mut indices = vec![];
    for poly in polys {
        // create triangle mesh
        let (vertices, ind) = earclip(poly, None, Some(offset / 2));
        // update vertex position
        offset += vertices.len();
        verts.extend(vertices);
        // store indices
        indices.extend(ind);
    }
    let tess_pos = verts.len();

    let level = 1 << fmax(fmin(floor(zoom as f64 / 2.), 4.), 0.) as i32;
    let division = (16 / level) as f64;
    if division > 1. {
        tesselate(&mut verts, &mut indices, 1. / division, 2);
    }

    let tess_points = &verts[tess_pos..];

    // store
    feature.geometry.set_indices(indices.into_iter().map(|i| i as u32).collect());
    feature.geometry.set_tess(tess_points.to_vec());
}
