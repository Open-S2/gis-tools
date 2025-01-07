declare let self: Worker;

import { DrawType } from 's2-tilejson';
import { MultiMap } from '../../../dataStore';
import { compressStream } from '../../../util';
import { BaseVectorTile, writeOVTile } from 'open-vector-tile';
import { PointCluster, PointIndex, Tile, TileStore } from '../../../dataStructures';
import { childrenIJ, convert, fromFace, toFaceIJ } from '../../../geometry';

import type { MultiMapStore } from '../../../dataStore';
import type { S2JSONLayerMap } from 'open-vector-tile';
import type {
  ClusterLayer,
  LayerGuide,
  OnFeature,
  StringifiedLayerGuide,
  VectorLayer,
} from '../..';
import type { ElevationPoint, RGBA } from '../../..';
import type { Encoding, Scheme } from 's2-tilejson';
import type { Face, Features, Properties, VectorFeatures } from '../../../geometry';

/** Take in options that will be used to create a tiled data correctly */
export interface InitMessage {
  /** Message type */
  type: 'init';
  /** id of the worker */
  id: number;
  /** The sources that will be used to create the tile */
  layerGuides: StringifiedLayerGuide[];
  /** The scheme that will be used to decide the projection and store method */
  scheme?: Scheme;
  /** The encoding that will be used to compress the tile */
  encoding?: Encoding;
}

/** Take in a feature that will be added to the tile */
export interface FeatureMessage {
  /** Message type */
  type: 'feature';
  /** The name of the source to add the feature to */
  sourceName: string;
  /** The feature to add to the tile */
  feature: Features;
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
export default class VectorTileWorker {
  id = 0;
  layerGuides: LayerGuide[] = [];
  scheme: Scheme = 'fzxy';
  encoding: Encoding = 'none';
  vectorStore: MultiMapStore<VectorFeatures<FeatureMetadata>> = new MultiMap<
    VectorFeatures<FeatureMetadata>
  >();
  // Unique store for each layer that describes itself as a cluster source
  clusterStores: { [layerName: string]: PointCluster } = {};
  rasterStore: PointIndex<RGBA> = new PointIndex<RGBA>();
  elevationStore: PointIndex<ElevationPoint> = new PointIndex<ElevationPoint>();

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
      this.layerGuides = parseLayerGuides(message.layerGuides);
      this.id = message.id;
      if (message.scheme !== undefined) this.scheme = message.scheme;
      if (message.encoding !== undefined) this.encoding = message.encoding;
      self.postMessage({ type: 'ready' });
    } else {
      this.storeFeature(message);
    }
  }

  /** Iterate through all the stores and sort/cluster as needed */
  async sort(): Promise<void> {
    for (const cluster of Object.values(this.clusterStores)) await cluster.buildClusters();
    await this.rasterStore.sort();
    await this.elevationStore.sort();
  }

  /**
   * Iterate through the stores and build tiles, gzip compressing as we go
   * @yields - a built tile
   */
  async *buildTiles(): AsyncGenerator<BuiltTile> {
    const { layerGuides, scheme, encoding } = this;
    const minzoom = getMinzoom(layerGuides);

    // three directions we can build data
    const tileCache = [fromFace(0)];
    if (scheme === 'fzxy')
      tileCache.push(fromFace(1), fromFace(2), fromFace(3), fromFace(4), fromFace(5));
    while (tileCache.length > 0) {
      const id = tileCache.pop()!;
      const tile = new Tile(id);
      // store vector features
      const vectorFeatures = await this.vectorStore.get(id);
      if (vectorFeatures !== undefined) {
        for (const feature of vectorFeatures) tile.addFeature(feature);
      }
      // store all cluster features
      for (const [layerName, cluster] of Object.entries(this.clusterStores)) {
        const layerClusterFeatures = await cluster.getTile(id);
        if (layerClusterFeatures === undefined) continue;
        for (const feature of layerClusterFeatures.layers.default.features) {
          tile.addFeature(feature, layerName);
        }
      }
      // TODO: Request raster tile if it exists
      // if tile is not empty we build a vector tile
      if (!tile.isEmpty()) {
        // build the base vector tile layerguides => S2JSONLayerMap
        const vectorTile = BaseVectorTile.fromS2JSONTile(tile, toLayerMap(layerGuides));
        // write to a buffer using the open-vector-tile spec
        let vectorTileBuffer: Uint8Array<ArrayBufferLike> = writeOVTile(vectorTile);
        // gzip if necessary
        if (encoding === 'gz') {
          vectorTileBuffer = await compressStream(vectorTileBuffer, 'gzip');
        }
        // yield the buffer
        yield { face: tile.face, zoom: tile.zoom, x: tile.i, y: tile.j, data: vectorTileBuffer };
        // store 4 children tiles to ask for children features
        tileCache.push(...childrenIJ(tile.face, tile.zoom, tile.i, tile.j));
      } else if (minzoom > tile.zoom) {
        // if we haven't reached the data yet, we store children
        tileCache.push(...childrenIJ(tile.face, tile.zoom, tile.i, tile.j));
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
      const {
        onFeature,
        metadata: { drawTypes },
      } = layerGuide;
      const parsedFeature = onFeature !== undefined ? onFeature(feature) : feature;
      if (parsedFeature === undefined) return;
      if (drawTypes.length === 0 || drawTypes.includes(toDrawType(feature))) return;

      if ('tileGuide' in layerGuide) this.#storeVectorFeature(parsedFeature, layerGuide);
      else if ('clusterGuide' in layerGuide) this.#storeClusterFeature(parsedFeature, layerGuide);
      // TODO raster source storing
    }
  }

  /**
   * Store a cluster feature in the correct point cluster
   * @param feature - the feature to store
   * @param clusterLayer - the layer guide to describe how to store the feature
   */
  #storeClusterFeature(feature: Features, clusterLayer: ClusterLayer): void {
    if (feature.geometry.type !== 'Point' && feature.geometry.type !== 'MultiPoint') return;
    const { scheme } = this;
    const {
      clusterGuide,
      layerName,
      metadata: { maxzoom },
    } = clusterLayer;
    const projection = clusterGuide.projection ?? (scheme === 'fzxy' ? 'S2' : 'WM');
    if (this.clusterStores[layerName] === undefined) {
      this.clusterStores[layerName] = new PointCluster(undefined, clusterGuide);
    }
    const vectorFeature = convert(projection, feature, false, undefined, maxzoom, false)[0];
    const {
      face,
      geometry: { type, coordinates },
      properties,
    } = vectorFeature;
    if (type === 'Point') {
      const { x, y } = coordinates;
      if (projection === 'S2')
        this.clusterStores[layerName].insertFaceST(face ?? 0, x, y, properties);
      else this.clusterStores[layerName].insertLonLat(x, y, vectorFeature.properties);
    } else if (type === 'MultiPoint') {
      for (const point of coordinates) {
        const { x, y } = point;
        if (projection === 'S2')
          this.clusterStores[layerName].insertFaceST(face ?? 0, x, y, properties);
        else this.clusterStores[layerName].insertLonLat(x, y, vectorFeature.properties);
      }
    }
  }

  /**
   * Store a vector feature across all appropriate zooms
   * @param feature - the feature to store
   * @param vectorLayer - the layer guide to describe how to store the feature
   */
  #storeVectorFeature(feature: Features, vectorLayer: VectorLayer): void {
    const {
      tileGuide,
      layerName,
      metadata: { minzoom },
    } = vectorLayer;
    // NOTE: Don't store above minzoom

    // three directions we can build data
    const tileStore = new TileStore(feature, tileGuide);
    const tileCache = [fromFace(0)];
    if (tileStore.projection === 'S2')
      tileCache.push(fromFace(1), fromFace(2), fromFace(3), fromFace(4), fromFace(5));
    while (tileCache.length > 0) {
      const id = tileCache.pop()!;
      const [face, zoom, i, j] = toFaceIJ(id);
      const tile = tileStore.getTile(id);
      if (minzoom > zoom) {
        // if we haven't reached the data yet, we store children
        tileCache.push(...childrenIJ(face, zoom, i, j));
      } else if (tile !== undefined && !tile.isEmpty()) {
        // store feature with the associated layername
        for (const { features } of Object.values(tile.layers)) {
          for (const feature of features) {
            feature.metadata = { layer: layerName };
            this.vectorStore.set(id, feature as VectorFeatures<FeatureMetadata>);
          }
        }

        // store 4 children tiles to ask for
        tileCache.push(...childrenIJ(tile.face, tile.zoom, tile.i, tile.j));
      }
    }
  }
}

/**
 * Get the absolute minzoom from the layer guides
 * @param layerGuides - the user defined guide on building the vector tiles
 * @returns the absolute minzoom
 */
function getMinzoom(layerGuides: LayerGuide[]): number {
  return Math.min(...layerGuides.map((layer) => layer.metadata.minzoom));
}

/**
 * Convert layer guides to S2JSONLayerMap to store in the open-vector-tile schema
 * @param layerGuides - the user defined guide on building the vector tiles
 * @returns the S2JSONLayerMap
 */
function toLayerMap(layerGuides: LayerGuide[]): S2JSONLayerMap {
  const res: S2JSONLayerMap = {};
  for (const layer of layerGuides) {
    if (!('extent' in layer)) continue;
    const { shape, mShape } = layer.metadata;
    res[layer.layerName] = { extent: layer.extent, shape, mShape };
  }
  return res;
}

/**
 * Convert a source guide to a parsed source guide (where onFeature is parsed back into a function)
 * @param sourceGuide - the source guide to parse
 * @returns the parsed source guide
 */
function parseLayerGuides(sourceGuide: StringifiedLayerGuide[]): LayerGuide[] {
  return sourceGuide.map((guide) => {
    return {
      ...guide,
      onFeature:
        guide.onFeature !== undefined ? (new Function(guide.onFeature)() as OnFeature) : undefined,
    };
  });
}

/**
 * Check if a feature is included by draw types defined by the layer guide
 * @param feature - the feature to find the associating draw type for
 * @returns - the associating draw type for the feature
 */
function toDrawType(feature: Features): DrawType {
  const {
    geometry: { type },
  } = feature;
  if (type === 'Point' || type === 'MultiPoint') return DrawType.Points;
  else if (type === 'Point3D' || type === 'MultiPoint3D') return DrawType.Points3D;
  else if (type === 'LineString' || type === 'MultiLineString') return DrawType.Lines;
  else if (type === 'Polygon' || type === 'MultiPolygon') return DrawType.Polys;
  else if (type === 'LineString3D' || type === 'MultiLineString3D') return DrawType.Lines3D;
  else if (type === 'Polygon3D' || type === 'MultiPolygon3D') return DrawType.Polys3D;
  else return DrawType.Points;
}
