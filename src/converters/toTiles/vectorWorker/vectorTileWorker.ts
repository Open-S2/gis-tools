import { DrawType } from 's2-tilejson';
import { MultiMap } from '../../../dataStore';
import { PointCluster, TileStore } from '../../../dataStructures';
import { childrenIJ, fromFace } from '../../../geometry';

import type { MultiMapStore } from '../../../dataStore';
import type { Features, VectorFeature } from '../../../geometry';
import type { LayerGuide, OnFeature, StringifiedLayerGuide, VectorLayer } from '../..';

/** Take in options that will be used to create a tiled data correctly */
export interface InitMessage {
  /** Message type */
  type: 'init';
  /** id of the worker */
  id: number;
  /** The sources that will be used to create the tile */
  layerGuides: StringifiedLayerGuide[];
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
export interface FeatureMetadata {
  layerName: string;
}

/** Convert a vector feature to a collection of tiles and store each tile feature */
export default class VectorTileWorker {
  id = 0;
  layerGuides: LayerGuide[] = [];
  // TODO: We have vectorSources, clusterSources, and rasterSources
  // TODO: that way each will have their own sorting.
  vectorStore: MultiMapStore<VectorFeature<FeatureMetadata>> = new MultiMap<
    VectorFeature<FeatureMetadata>
  >();
  // Unique store for each layer that describes itself as a cluster source
  clusterStores: { [layerName: string]: PointCluster } = {};
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
      self.postMessage({ type: 'ready' });
    } else {
      this.storeFeature(message);
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
    }
  }

  /**
   * Store a vector feature across all appropriate zooms
   * @param feature - the feature to store
   * @param vectorLayer - the layer guide to describe how to store the feature
   */
  #storeVectorFeature(feature: Features, vectorLayer: VectorLayer): void {
    // TODO: tileGuide should be modifed to match metadata minzoom, maxzoom, and projection
    // minzoom and maxzoom can be left alone if they already exist, but projection MUST match the
    // output projection.
    const { tileGuide, layerName } = vectorLayer;

    // three directions we can build data
    const tileStore = new TileStore(feature, tileGuide);
    const tileCache = [fromFace(0)];
    if (tileStore.projection === 'S2')
      tileCache.push(fromFace(1), fromFace(2), fromFace(3), fromFace(4), fromFace(5));
    while (tileCache.length > 0) {
      const id = tileCache.pop()!;
      const tile = tileStore.getTile(id);
      if (tile !== undefined && !tile.isEmpty()) {
        // store feature with the associated layername
        feature.metadata = { layerName };
        this.vectorStore.set(id, feature as VectorFeature<FeatureMetadata>);

        // store 4 children tiles to ask for
        tileCache.push(...childrenIJ(tile.face, tile.zoom, tile.i, tile.j));
      }
    }
  }
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
