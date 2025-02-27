declare let self: Worker;

import { DrawType } from 's2-tilejson';
import { MultiMap } from '../../../dataStore';
import { compressStream } from '../../../util';
import { BaseVectorTile, writeMVTile, writeOVTile } from 'open-vector-tile';
import { PointCluster, PointGrid, Tile, TileStore } from '../../../dataStructures';
import { earclip, tesselate } from 'earclip';
import { idChildrenIJ, idFromFace, idToFaceIJ } from '../../../geometry';

import type { Encoding } from 's2-tilejson';
import type { MultiMapStore } from '../../../dataStore';
import type { RGBA } from '../../..';
import type {
  Face,
  Projection,
  Properties,
  VectorFeatures,
  VectorMultiPolygon,
} from '../../../geometry';
import type {
  FormatOutput,
  GetPointValue,
  GridLayer,
  LayerGuide,
  OnFeature,
  StringifiedLayerGuide,
  VectorLayer,
} from '../types';
import type { GridInput, ImageDataInput, S2JSONLayerMap } from 'open-vector-tile';

/** Take in options that will be used to create a tiled data correctly */
export interface InitMessage {
  /** Message type */
  type: 'init';
  /** id of the worker */
  id: number;
  /** The sources that will be used to create the tile */
  layerGuides: StringifiedLayerGuide[];
  /** The scheme that will be used to decide the projection and store method */
  projection?: Projection;
  /** The encoding that will be used to compress the tile */
  encoding?: Encoding;
  /** The output format */
  format?: FormatOutput;
  /** If we should build indices into the tile or not */
  buildIndices?: boolean;
}

/** Take in a feature that will be added to the tile */
export interface FeatureMessage {
  /** Message type */
  type: 'feature';
  /** The name of the source that the feature is from */
  sourceName: string;
  /** The feature to add to the tile */
  feature: VectorFeatures;
}

/** We want to track the associated layer for each feature */
export interface FeatureMetadata extends Properties {
  layerName: string;
}

/** A built tile that is ready to be written to the filesystem */
export interface BuiltTile {
  face: Face;
  zoom: number;
  x: number;
  y: number;
  data: Uint8Array;
}

/** Convert a vector feature to a collection of tiles and store each tile feature */
export default class TileWorker {
  id = 0;
  layerGuides: LayerGuide[] = [];
  projection: Projection = 'S2';
  encoding: Encoding = 'none';
  format: FormatOutput = 'open-s2';
  buildIndices = true;
  vectorStore: MultiMapStore<VectorFeatures<FeatureMetadata>> = new MultiMap<
    VectorFeatures<FeatureMetadata>
  >();
  // Unique store for each layer that describes itself as a cluster source
  clusterStores: { [layerName: string]: PointCluster } = {};
  rasterStores: { [layerName: string]: PointGrid<RGBA> } = {};
  gridStores: { [layerName: string]: PointGrid } = {};

  /**
   * Tile-ize input vector features and store them
   * @param event - the init message or a feature message
   */
  onmessage(event: Bun.MessageEvent<InitMessage | FeatureMessage>): void {
    this.handleMessage(event.data);
  }

  /**
   * Tile-ize input vector features and store them
   * @param message - the init message or a feature message
   */
  handleMessage(message: InitMessage | FeatureMessage): void {
    const { type } = message;
    if (type === 'init') {
      this.id = message.id;
      if (message.projection !== undefined) this.projection = message.projection;
      if (message.encoding !== undefined) this.encoding = message.encoding;
      if (message.format !== undefined) this.format = message.format;
      if (message.buildIndices !== undefined) this.buildIndices = message.buildIndices;
      this.#parseLayerGuides(message.layerGuides);
      self.postMessage({ type: 'ready' });
    } else {
      this.storeFeature(message);
    }
  }

  /** Iterate through all the stores and sort/cluster as needed */
  async sort(): Promise<void> {
    for (const cluster of Object.values(this.clusterStores)) await cluster.buildClusters();
    for (const raster of Object.values(this.rasterStores)) await raster.buildClusters();
    for (const grid of Object.values(this.gridStores)) await grid.buildClusters();
  }

  /**
   * Iterate through the stores and build tiles, compressing as we go if required
   * @yields - a built tile
   */
  async *buildTiles(): AsyncGenerator<BuiltTile> {
    const { format, layerGuides, projection, encoding } = this;
    const minzoom = getMinzoom(layerGuides);

    // three directions we can build data
    const tileCache = [idFromFace(0)];
    if (projection === 'S2')
      tileCache.push(idFromFace(1), idFromFace(2), idFromFace(3), idFromFace(4), idFromFace(5));
    while (tileCache.length > 0) {
      const id = tileCache.pop()!;
      const tile = new Tile(id);
      const { face, zoom, i: x, j: y } = tile;

      const vectorTile = await this.#getVectorTile(id, tile);
      const rasterData = await this.#getRasterTile(id);
      const gridData = await this.#getGridTile(id);
      if (format === 'raster') {
        // RASTER CASE
        if (rasterData !== undefined) {
          const data = new Uint8Array(rasterData[0].image);
          yield { face, zoom, x, y, data };
          // store 4 children tiles to ask for children features
          tileCache.push(...idChildrenIJ(face, zoom, x, y));
        } else {
          // if we haven't reached the data yet, we store children
          if (minzoom > tile.zoom) tileCache.push(...idChildrenIJ(face, zoom, x, y));
        }
      } else {
        // VECTOR CASE
        if (vectorTile === undefined && rasterData === undefined && gridData === undefined) {
          // if we haven't reached the data yet, we store children
          if (minzoom > tile.zoom) tileCache.push(...idChildrenIJ(face, zoom, x, y));
        } else {
          // write to a buffer using the open-vector-tile spec
          let data =
            format === 'open-s2'
              ? writeOVTile(vectorTile, rasterData, gridData)
              : writeMVTile(vectorTile!, format === 'mapbox');
          // gzip if necessary
          if (encoding === 'gz') data = await compressStream(data, 'gzip');
          // yield the buffer
          yield { face, zoom, x, y, data };
          // store 4 children tiles to ask for children features
          tileCache.push(...idChildrenIJ(face, zoom, x, y));
        }
      }
    }
  }

  /**
   * Store a feature across all appropriate zooms
   * @param message - the message to pull the feature and source info from
   */
  storeFeature(message: FeatureMessage): void {
    const { layerGuides } = this;
    const { feature, sourceName } = message;

    for (const layerGuide of layerGuides.filter((layer) => layer.sourceName === sourceName)) {
      const { onFeature, layerName } = layerGuide;
      const parsedFeature = onFeature !== undefined ? onFeature(feature) : feature;
      if (parsedFeature === undefined) return;

      if ('vectorGuide' in layerGuide) this.#storeVectorFeature(parsedFeature, layerGuide);
      else if ('clusterGuide' in layerGuide)
        this.clusterStores[layerName].insertFeature(parsedFeature);
      else if ('rasterGuide' in layerGuide)
        this.rasterStores[layerName].insertFeature(
          parsedFeature as VectorFeatures<Record<string, unknown>, RGBA, RGBA>,
        );
      else this.gridStores[layerName].insertFeature(parsedFeature);
    }
  }

  /**
   * Get vector/cluster features for a tile
   * @param id - the tile id
   * @param tile - the tile object to fill
   * @returns - a BaseVectorTile containing all the features if there are any
   */
  async #getVectorTile(id: bigint, tile: Tile): Promise<BaseVectorTile | undefined> {
    const { layerGuides, format, buildIndices } = this;
    if (format === 'raster') return;
    // store vector features
    const vectorFeatures = await this.vectorStore.get(id);
    if (vectorFeatures !== undefined) {
      if (buildIndices) earclipPolygons(vectorFeatures, tile.zoom, tile.extent);
      for (const feature of vectorFeatures) tile.addFeature(feature, feature.metadata?.layerName);
    }
    // store all cluster features
    for (const [layerName, cluster] of Object.entries(this.clusterStores)) {
      const layerClusterFeatures = await cluster.getTile(id);
      if (layerClusterFeatures === undefined) continue;
      for (const layer of Object.values(layerClusterFeatures.layers)) {
        for (const feature of layer.features) tile.addFeature(feature, layerName);
      }
    }

    if (!tile.isEmpty()) {
      // build the base vector tile layerguides => S2JSONLayerMap
      const vectorLayers = layerGuides.filter((layer) => 'vectorGuide' in layer);
      const maxzoom = getMaxzoom(layerGuides);
      tile.transform(0, maxzoom);
      return BaseVectorTile.fromS2JSONTile(tile, toLayerMap(vectorLayers));
    }
  }

  /**
   * Get raster data for a tile
   * @param id - the tile id
   * @returns - a collection of GridInputs
   */
  async #getRasterTile(id: bigint): Promise<ImageDataInput[] | undefined> {
    const res: ImageDataInput[] = [];
    // store all cluster features
    for (const raster of Object.values(this.rasterStores)) {
      const layerGrid = await raster.getTile(id);
      if (layerGrid === undefined) continue;
      const { name, size, data } = layerGrid;
      const image = (data as RGBA[]).flatMap(({ r, g, b, a }) => [r, g, b, a]);
      res.push({
        name,
        type: 'raw',
        width: size,
        height: size,
        image: new Uint8Array(image),
      });
    }

    if (res.length > 0) return res;
  }

  /**
   * Get gridded data for a tile
   * @param id - the tile id
   * @returns - a collection of ImageDataInputs
   */
  async #getGridTile(id: bigint): Promise<GridInput[] | undefined> {
    const res: GridInput[] = [];
    // store all cluster features
    for (const [layerName, grid] of Object.entries(this.gridStores)) {
      const { extent } = this.layerGuides.filter(
        (guide) => guide.layerName === layerName,
      )[0] as GridLayer;
      const layerGrid = await grid.getTile(id);
      if (layerGrid === undefined) continue;
      const { name, size, data } = layerGrid;
      res.push({
        name,
        size,
        data: data as number[],
        extent,
      });
    }

    if (res.length > 0) return res;
  }

  /**
   * Store a vector feature across all appropriate zooms
   * @param feature - the feature to store
   * @param vectorLayer - the layer guide to describe how to store the feature
   */
  #storeVectorFeature(feature: VectorFeatures, vectorLayer: VectorLayer): void {
    const { projection } = this;
    const { vectorGuide, layerName, drawTypes } = vectorLayer;
    const minzoom = vectorGuide.minzoom ?? 0;
    if (!drawTypes.includes(toDrawType(feature))) return;
    // Setup a tileCache and dive down. Store the 4 children if data is found while storing data as we go
    const tileStore = new TileStore(feature, { projection, ...vectorGuide });
    const tileCache = [idFromFace(0)];
    if (projection === 'S2')
      tileCache.push(idFromFace(1), idFromFace(2), idFromFace(3), idFromFace(4), idFromFace(5));
    while (tileCache.length > 0) {
      const id = tileCache.pop()!;
      const [face, zoom, i, j] = idToFaceIJ(id);
      const tile = tileStore.getTile(id);
      if (minzoom > zoom) {
        // if we haven't reached the data yet, we store children
        tileCache.push(...idChildrenIJ(face, zoom, i, j));
      } else if (tile !== undefined && !tile.isEmpty()) {
        // store feature with the associated layername
        for (const { features } of Object.values(tile.layers)) {
          for (const feature of features) {
            feature.metadata = { layer: layerName };
            this.vectorStore.set(id, feature as VectorFeatures<FeatureMetadata>);
          }
        }
        // store 4 children tiles to ask for
        tileCache.push(...idChildrenIJ(face, zoom, i, j));
      }
    }
  }

  /**
   * Convert a source guide to a parsed source guide (where onFeature is parsed back into a function)
   * @param sourceGuide - the source guide to parse
   */
  #parseLayerGuides(sourceGuide: StringifiedLayerGuide[]): void {
    const { projection } = this;
    // setup layerGuides
    this.layerGuides = sourceGuide.map((guide) => {
      return {
        ...guide,
        getValue:
          'getValue' in guide && guide.getValue !== undefined
            ? (new Function(guide.getValue)() as GetPointValue)
            : undefined,
        onFeature:
          guide.onFeature !== undefined
            ? (new Function(guide.onFeature)() as OnFeature)
            : undefined,
      };
    });
    // Setup layer stores
    for (const layer of this.layerGuides) {
      const { layerName } = layer;
      if ('vectorGuide' in layer) continue;
      else if ('clusterGuide' in layer)
        this.clusterStores[layerName] = new PointCluster(undefined, {
          projection,
          layerName,
          ...layer.clusterGuide,
        });
      else if ('rasterGuide' in layer)
        this.rasterStores[layerName] = new PointGrid<RGBA>({
          projection,
          layerName,
          ...layer.rasterGuide,
        });
      else
        this.gridStores[layerName] = new PointGrid({ projection, layerName, ...layer.gridGuide });
    }
  }
}

/**
 * Get the absolute minzoom from the layer guides
 * @param layerGuides - the user defined guide on building the vector tiles
 * @returns the absolute minzoom
 */
export function getMinzoom(layerGuides: LayerGuide[]): number {
  return Math.min(
    ...layerGuides.map((layer) => {
      if ('vectorGuide' in layer) return layer.vectorGuide.minzoom ?? 0;
      else if ('clusterGuide' in layer) return layer.clusterGuide.minzoom ?? 0;
      else if ('rasterGuide' in layer) return layer.rasterGuide.minzoom ?? 0;
      else return layer.gridGuide.minzoom ?? 0;
    }),
  );
}

/**
 * Get the absolute maxzoom from the layer guides
 * @param layerGuides - the user defined guide on building the vector tiles
 * @returns the absolute maxzoom
 */
export function getMaxzoom(layerGuides: LayerGuide[]): number {
  return Math.max(
    ...layerGuides.map((layer) => {
      if ('vectorGuide' in layer) return layer.vectorGuide.maxzoom ?? 14;
      else if ('clusterGuide' in layer) return layer.clusterGuide.maxzoom ?? 14;
      else if ('rasterGuide' in layer) return layer.rasterGuide.maxzoom ?? 14;
      else return layer.gridGuide.maxzoom ?? 14;
    }),
  );
}

/**
 * Convert vector layer guides to S2JSONLayerMap to store in the open-vector-tile schema
 * @param layerGuides - the user defined guide on building the vector tiles
 * @returns the S2JSONLayerMap
 */
function toLayerMap(layerGuides: VectorLayer[]): S2JSONLayerMap {
  const res: S2JSONLayerMap = {};
  for (const { layerName, extent, shape, mShape } of layerGuides)
    res[layerName] = { extent, shape, mShape };
  return res;
}

/**
 * Check if a feature is included by draw types defined by the layer guide
 * @param feature - the feature to find the associating draw type for
 * @returns - the associating draw type for the feature
 */
function toDrawType(feature: VectorFeatures): DrawType {
  const {
    geometry: { type, is3D },
  } = feature;

  if (type === 'Point' || type === 'MultiPoint') return is3D ? DrawType.Points3D : DrawType.Points;
  else if (type === 'LineString' || type === 'MultiLineString')
    return is3D ? DrawType.Lines3D : DrawType.Lines;
  else if (type === 'Polygon' || type === 'MultiPolygon')
    return is3D ? DrawType.Polys3D : DrawType.Polys;
  else if (type === 'Point3D' || type === 'MultiPoint3D') return DrawType.Points3D;
  else if (type === 'LineString3D' || type === 'MultiLineString3D') return DrawType.Lines3D;
  else if (type === 'Polygon3D' || type === 'MultiPolygon3D') return DrawType.Polys3D;
  else if (type === 'Raster') return DrawType.Raster;
  else if (type === 'Grid') return DrawType.Grid;
  else return DrawType.Points;
}

/**
 * Pre-earclip polygons for faster processing in the tile
 * @param features - the features to be stored in the tile
 * @param zoom - the tile zoom
 * @param extent - the tile extent
 */
function earclipPolygons(features: VectorFeatures[], zoom: number, extent: number): void {
  const { max, min, floor, fround } = Math;

  for (const feature of features) {
    const { geometry } = feature;
    const { type } = geometry;
    if (type !== 'Polygon' && type !== 'MultiPolygon') continue;

    // prep polys
    const polys: VectorMultiPolygon = [];
    // prep for processing
    if (type === 'MultiPolygon') {
      for (const poly of geometry.coordinates) polys.push(poly);
    } else {
      polys.push(geometry.coordinates);
    }

    let offset = 0;
    const verts = [];
    const indices = [];
    for (const poly of polys) {
      // create triangle mesh
      const data = earclip(poly, Infinity, fround(offset / 2));
      // update vertex position
      offset += data.vertices.length;
      verts.push(...data.vertices);
      // store indices
      for (let i = 0, il = data.indices.length; i < il; i++) indices.push(data.indices[i]);
    }
    const tessPos = verts.length;

    const level = 1 << max(min(floor(zoom / 2), 4), 0);
    const division = 16 / level;
    if (division > 1) tesselate(verts, indices, extent / division, 2);

    const tessPoints = verts.slice(tessPos).map((n) => fround(n));

    // store
    geometry.indices = indices;
    geometry.tesselation = tessPoints;
  }
}
