// // import { chordAngFromS2Points } from '../geometry/s1/chordAngle';
// // import { PointIndex, PointShape, Tile } from '..';
// // import {
// //   pointAddMut as addMut,
// //   pointDivMutScalar as divMutScalar,
// //   pointFromST as fromST,
// //   pointMulScalar as mulScalar,
// //   pointNormalize as normalize,
// //   pointToST as toST,
// // } from '../geometry/s2/point';
// // import { convert, idFromFacePosLevel, idGetVertices, idLevel, idRange } from '../geometry';

// // import type { FeatureIterator } from '..';
// // import type { S1ChordAngle } from '../geometry/s1/chordAngle';
// // import type {
// //   Face,
// //   JSONCollection,
// //   MValue,
// //   Projection,
// //   Properties,
// //   S2CellId,
// //   VectorPoint,
// //   VectorPointM,
// // } from '../geometry';

// // import type { VectorStore, VectorStoreConstructor } from '../dataStore/vector';

// // /** The kind of input required to store a point for proper indexing */
// // export type ClusterStore<M extends MValue = Properties> = VectorStoreConstructor<
// //   PointShape<Cluster<M>>
// // >;

// use crate::data_store::vector::{Vector, VectorStore};

// use super::PointIndex;
// use alloc::{collections::BTreeMap, string::String};
// use s2json::{MValue, MValueCompatible, Projection, VectorPoint};
// use serde::{de::DeserializeOwned, Deserialize, Serialize};

// /// The type of search to use
// #[derive(Debug)]
// pub enum ClusterSearch {
//     /// Search for points within a radius
//     Radial,
//     /// Search for points within an S2CellId
//     Cell,
// }

// // /** Options for point clustering */
// // export interface ClusterOptions<M extends MValue = Properties> {
// //   /** type of store to use. Defaults to an in memory store */
// //   store?: ClusterStore<M>;
// //   /** projection to use */
// //   projection?: Projection;
// //   /** Name of the layer to build when requesting a tile */
// //   layer_name?: string;
// //   /** min zoom to generate clusters on */
// //   minzoom?: number;
// //   /** max zoom level to cluster the points on */
// //   maxzoom?: number;
// //   /** cluster radius in pixels relative to a 512x512 pixel tile */
// //   radius?: number;
// // }

// /// A cluster is a storage device to maintain groups of information in a cluster
// #[derive(Debug, Default, Clone, Serialize, Deserialize, MValueCompatible)]
// pub struct Cluster<M: MValueCompatible = MValue> {
//     /// The data of the cluster
//     pub data: M,
//     /// The number of points in the cluster
//     pub count: usize,
//     visited: bool,
// }
// impl<M: MValueCompatible> Cluster<M> {
//     /// Create a cluster given the value (cluster size) and data
//     pub fn new(data: M, count: usize) -> Self {
//         Cluster { data, count, visited: false }
//     }
// }

// /// Compare two data items, return true to merge data
// pub type ClusterDataComparitor<M> = fn(a: M, b: M) -> bool;

// /// # Point Cluster
// ///
// /// ## Description
// /// A cluster store to index points at each zoom level
// pub struct PointCluster<
//     M: MValueCompatible + Serialize + DeserializeOwned,
//     S: VectorStore<VectorPoint<Cluster<M>>> = Vector<Cluster<M>>,
// > where
//     Cluster<M>: MValueCompatible,
// {
//     projection: Projection,
//     layer_name: String,
//     minzoom: u8,
//     maxzoom: u8,
//     radius: f64,
//     grid_size: u32, // a default is a 512x512 pixel tile
//     indexes: BTreeMap<u8, PointIndex<Cluster<M>, S>>, // zoom => index
// }
// //   /**
// //    * @param data - if provided, the data to index
// //    * @param options - cluster options on how to build the cluster
// //    * @param maxzoomStore - the store to use for the maxzoom index
// //    */
// //   constructor(
// //     data?: JSONCollection<unknown, M, M>,
// //     options?: ClusterOptions<M>,
// //     maxzoomStore?: VectorStore<PointShape<Cluster<M>>>,
// //   ) {
// //     this.projection = options?.projection ?? 'S2';
// //     this.layer_name = options?.layer_name ?? 'default';
// //     this.minzoom = Math.max(options?.minzoom ?? 0, 0);
// //     this.maxzoom = Math.min(options?.maxzoom ?? 16, 29);
// //     this.radius = options?.radius ?? 40;
// //     // one extra zoom incase its a cell search system (bottom zoom isn't clustered to a cell)
// //     for (let zoom = this.minzoom; zoom <= this.maxzoom + 1; zoom++) {
// //       this.indexes.set(zoom, new PointIndex<Cluster<M>>(options?.store, this.projection));
// //     }
// //     if (maxzoomStore !== undefined) {
// //       const maxzoomIndex = this.indexes.get(this.maxzoom);
// //       maxzoomIndex?.setStore(maxzoomStore);
// //     }
// //     // convert features if provided
// //     if (data !== undefined) this.insertFeature(data);
// //   }

// //   /**
// //    * Add a point to the maxzoom index. The point is a Point3D
// //    * @param point - the point to add
// //    */
// //   insert(point: VectorPointM<M>): void {
// //     const { x, y, z, m } = point;
// //     const maxzoomIndex = this.indexes.get(this.maxzoom);
// //     maxzoomIndex?.insert({ x, y, z, m: toCluster<M>(m, 1) });
// //   }

// //   /**
// //    * Add all points from a reader. It will try to use the M-value first, but if it doesn't exist
// //    * it will use the feature properties data
// //    * @param reader - a reader containing the input data
// //    */
// //   async insertReader(reader: FeatureIterator<unknown, M, M>): Promise<void> {
// //     for await (const feature of reader) this.insertFeature(feature);
// //   }

// //   /**
// //    * Add a vector feature. It will try to use the M-value first, but if it doesn't exist
// //    * it will use the feature properties data
// //    * @param data - any source of data like a feature collection or features themselves
// //    */
// //   insertFeature(data: JSONCollection<unknown, M, M>): void {
// //     const features = convert(this.projection, data, undefined, undefined, undefined, true);
// //     for (const { face = 0, geometry, properties } of features) {
// //       const { type, coordinates } = geometry;
// //       if (type === 'Point') {
// //         const { x: s, y: t, m } = coordinates;
// //         this.#insertFaceST(face, s, t, m ?? properties);
// //       } else if (type === 'MultiPoint') {
// //         for (const point of coordinates) {
// //           const { x: s, y: t, m } = point;
// //           this.#insertFaceST(face, s, t, m ?? properties);
// //         }
// //       }
// //     }
// //   }

// //   /**
// //    * Add a lon-lat pair to the cluster
// //    * @param ll - lon-lat vector point in degrees
// //    */
// //   insertLonLat(ll: VectorPoint<M>): void {
// //     this.insertFeature({
// //       type: 'VectorFeature',
// //       properties: ll.m ?? ({} as M),
// //       geometry: { type: 'Point', coordinates: ll, is3D: false },
// //     });
// //   }

// //   /**
// //    * Insert an STPoint to the index
// //    * @param face - the face of the cell
// //    * @param s - the s coordinate
// //    * @param t - the t coordinate
// //    * @param data - the data associated with the point
// //    */
// //   insertFaceST(face: Face, s: number, t: number, data: M): void {
// //     this.insertFeature({
// //       type: 'S2Feature',
// //       face,
// //       properties: data,
// //       geometry: { type: 'Point', coordinates: { x: s, y: t, m: data }, is3D: false },
// //     });
// //   }

// //   /**
// //    * Insert an STPoint to the index
// //    * @param face - the face of the cell
// //    * @param s - the s coordinate
// //    * @param t - the t coordinate
// //    * @param data - the data associated with the point
// //    */
// //   #insertFaceST(face: Face, s: number, t: number, data: M): void {
// //     this.insert(fromST(face, s, t, data) as VectorPointM<M>);
// //   }

// //   /**
// //    * Build the clusters when done adding points
// //    * @param cmp_ - custom compare function
// //    */
// //   async buildClusters(cmp_?: ClusterDataComparitor<M>): Promise<void> {
// //     const { minzoom, maxzoom } = this;
// //     const cmp: ClusterDataComparitor<M> = cmp_ ?? ((_a: M, _b: M) => true);
// //     for (let zoom = maxzoom; zoom >= minzoom; zoom--) {
// //       const curIndex = this.indexes.get(zoom);
// //       const queryIndex = this.indexes.get(zoom + 1);
// //       if (curIndex === undefined || queryIndex === undefined) throw new Error('Index not found');
// //       await this.#clusterRadius(zoom, queryIndex, curIndex, cmp);
// //     }
// //     // ensure all point indexes are sorted
// //     for (const index of this.indexes.values()) await index.sort();
// //   }

// //   /**
// //    * Radial clustering
// //    * @param zoom - the zoom level
// //    * @param queryIndex - the index to query
// //    * @param currIndex - the index to insert into
// //    * @param cmp - the compare function
// //    */
// //   async #clusterRadius(
// //     zoom: number,
// //     queryIndex: PointIndex<Cluster<M>>,
// //     currIndex: PointIndex<Cluster<M>>,
// //     cmp: ClusterDataComparitor<M>,
// //   ): Promise<void> {
// //     const radius = this.#getLevelRadius(zoom);
// //     for await (const clusterPoint of queryIndex) {
// //       const { point } = clusterPoint;
// //       const clusterData = point.m;
// //       if (clusterData.visited) continue;
// //       clusterData.visited = true;
// //       // setup a new weighted cluster point
// //       const newClusterPoint = mulScalar(point, clusterData.value as number);
// //       let newNumPoints = clusterData.value as number;
// //       // joining all points found within radius
// //       const points = await queryIndex.searchRadius(point, radius);
// //       for (const { point: foundPoint } of points) {
// //         const foundData = foundPoint.m;
// //         // only add points that match or have not been visited already
// //         if (!cmp(clusterData.data, foundData.data) || foundData.visited) continue;
// //         foundData.visited = true;
// //         // weighted add to newClusterPoint position
// //         addMut(newClusterPoint, mulScalar(foundPoint, foundData.value as number));
// //         newNumPoints += foundData.value as number;
// //       }
// //       // finish position average
// //       divMutScalar(newClusterPoint, newNumPoints);
// //       normalize(newClusterPoint);
// //       // store the new cluster point
// //       const { x, y, z } = newClusterPoint;
// //       currIndex.insert({ x, y, z, m: toCluster(clusterData.data, newNumPoints) });
// //     }
// //   }

// //   /**
// //    * @param id - the cell id
// //    * @returns - the data within the range of the tile id
// //    */
// //   async getCellData(id: S2CellId): Promise<undefined | PointShape<Cluster<M>>[]> {
// //     const { minzoom, maxzoom, indexes } = this;
// //     const zoom = idLevel(id);
// //     if (zoom < minzoom) return;
// //     const [min, max] = idRange(id);
// //     const levelIndex = indexes.get(Math.min(zoom, maxzoom));

// //     return await levelIndex?.searchRange(min, max);
// //   }

// //   /**
// //    * @param id - the id of the vector tile
// //    * @returns - the vector tile
// //    */
// //   async getTile(
// //     id: S2CellId,
// //   ): Promise<undefined | Tile<Record<string, unknown>, { value: number }, M>> {
// //     const data = await this.getCellData(id);
// //     if (data === undefined) return;
// //     const tile = new Tile<Record<string, unknown>, { value: number }, M>(id);
// //     for (const { point } of data) {
// //       const [face, s, t] = toST(point);
// //       const { value, data } = point.m;
// //       tile.addFeature(
// //         {
// //           type: 'VectorFeature',
// //           face,
// //           geometry: { is3D: false, type: 'Point', coordinates: { x: s, y: t, m: { value } } },
// //           properties: data,
// //         },
// //         this.layer_name,
// //       );
// //     }

// //     // transform the geometry to be relative to the tile
// //     tile.transform(0, this.maxzoom);

// //     return tile;
// //   }

// //   /**
// //    * Get a S1ChordAngle relative to a tile zoom level
// //    * @param zoom - the zoom level to build a radius
// //    * @returns - the appropriate radius for the given zoom
// //    */
// //   #getLevelRadius(zoom: number): S1ChordAngle {
// //     const multiplier = this.radius / this.grid_size;
// //     const cell = idFromFacePosLevel(0, 0n, zoom);
// //     const [lo, hi] = idGetVertices(cell);
// //     const angle = chordAngFromS2Points(lo, hi);
// //     return angle * multiplier;
// //   }
// // }
