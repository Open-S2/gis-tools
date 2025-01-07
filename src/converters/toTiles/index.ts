import { MetadataBuilder } from 's2-tilejson';
import VectorTileWorker from './worker/tileWorker';

import type { ClusterOptions } from '../../dataStructures/pointCluster';
import type { Extents } from 'open-vector-tile';
import type { FeatureIterator } from '../../readers';
import type { Features } from '../../geometry';
import type { TileStoreOptions } from '../../dataStructures/tile';
import type { TileWriter } from '../../writers';
import type { Attribution, Encoding, LayerMetaData, Scheme } from 's2-tilejson';
import type { FeatureMessage, InitMessage } from './worker/tileWorker';

// NOTE: IF using workers in threads, it MUST be for local use filesytem. If using a buffer system it's single threaded

/**
 * Before tiling the data, you can mutate it here. It can also act as a filter if you return undefined
 */
export type OnFeature = (feature: Features) => Features | undefined;

/** No matter the type of layer you want to build, these are default properties to include */
export interface BaseLayer {
  /** Name of the source */
  sourceName: string;
  /** Name of the layer */
  layerName: string;
  /** Components of how the layer is built and stored */
  metadata: LayerMetaData;
  /** Stringified version of the onFeature used by the source so it can be shipped to a worker. */
  onFeature?: OnFeature;
}

/** Guide to building Raster layer data */
export interface RasterLayer extends BaseLayer {
  /** describes how the image will be stored */
  outputType: 'webp' | 'png' | 'jpeg' | 'avif';
}
/** Guide to building Raster layer data where the onFeature & filter is stringified to ship to workers */
export interface StringifiedRasterLayer extends Omit<RasterLayer, 'onFeature' | 'filter'> {
  /** Stringified version of the onFeature used by the source so it can be shipped to a worker. */
  onFeature?: string;
}

/** Guide to building Cluster layer data */
export interface ClusterLayer extends BaseLayer {
  /** If options are provided, the assumption is the point data is clustered */
  clusterGuide: ClusterOptions;
  /** Extent at which the layer is storing its data */
  extent: Extents;
}
/** Guide to building Cluster layer data where the onFeature & filter is stringified to ship to workers */
export interface StringifiedClusterLayer extends Omit<ClusterLayer, 'onFeature' | 'filter'> {
  /** Stringified version of the onFeature used by the source so it can be shipped to a worker. */
  onFeature?: string;
}

/** Guide to building Vector layer data */
export interface VectorLayer extends BaseLayer {
  /** Guide on how to splice the data into vector tiles */
  tileGuide: TileStoreOptions;
  /** Extent at which the layer is storing its data */
  extent: Extents;
}
/** Guide to building Vector layer data where the onFeature & filter is stringified to ship to workers */
export interface StringifiedVectorLayer extends Omit<VectorLayer, 'onFeature' | 'filter'> {
  /** Stringified version of the onFeature used by the source so it can be shipped to a worker. */
  onFeature?: string;
}

/** List of user defined guides to build layers */
export type LayerGuide = RasterLayer | ClusterLayer | VectorLayer;

/** List of user defined guides to build layers where the onFeature is stringified to ship to workers */
export type StringifiedLayerGuide =
  | StringifiedRasterLayer
  | StringifiedClusterLayer
  | StringifiedVectorLayer;

/** A user defined guide on building the vector tiles */
export interface BuildGuide {
  /** The name of the data */
  name: string;
  /** The description of the data */
  description?: string;
  /** User defined versioning for their data */
  version?: string;
  /**
   * What kind of output format should be used. Used for describing either S2 or WM
   * projections [Default: 'fzxy']
   */
  scheme?: Scheme;
  /** The encoding format. Can be either 'gz', 'br', 'zstd' or 'none' [Default: 'gz'] */
  encoding?: Encoding;
  /** The attribution of the data. Store as { 'presentation name': 'href' }. */
  attribution?: Attribution;
  /**
   * The vector format if applicable helps define how the vector data is stored.
   * - The more modern vector format is the 'open-v2' which supports things like m-values
   * and 3D geometries.
   * - The new vector format is the 'open-v2' which only supports 2D & 3D geometries, supports M-Values,
   * properties and M-Values can have nested properties and/or arrays, and is decently fast to parse.
   * - The legacy vector format is the 'open-v1' which only supports 2D geometries and works on
   * older map engines like Mapbox-gl-js, is faster to parse and often lighter in size.
   * - The older vector format is the 'mapbox' which is the legacy format used by Mapbox and slow to parse.
   * [Default: 'open-v2']
   */
  vectorFormat?: 'mapbox' | 'open-v1' | 'open-v2';
  /**
   * The vector sources that the tile is built from and how the layers are to be stored.
   * Created using `{ [sourceName: string]: FeatureIterator }`
   */
  vectorSources?: Record<string, FeatureIterator>;
  /** The raster sources that will be conjoined into a single rgba pixel index for tile extraction */
  rasterSources?: Record<string, FeatureIterator>;
  /** The elevation sources that will be conjoined into a single elevation index for tile extraction */
  elevationSources?: Record<string, FeatureIterator>;
  /** The guides on how to build the various data */
  layerGuides: LayerGuide[];
  /**
   * The data created will be stored in either a folder structure or a pmtiles file
   * Folder structure is either '{face}/{zoom}/{x}/{y}.pbf' or '{zoom}/{x}/{y}.pbf'.
   * PMTiles store all data in a single data file.
   */
  tileWriter: TileWriter;
  /** Set the number of threads to use. [Default: 1] */
  threads?: number;
}

/**
 * Build vector tiles give a guide on what sources to parse data from and how to store it
 * @param buildGuide - the user defined guide on building the vector tiles
 */
export async function toVectorTiles(buildGuide: BuildGuide): Promise<void> {
  const { tileWriter, scheme } = buildGuide;
  const vectorWorker = new VectorTileWorker();

  // STEP 1: Convert all features to tile slices of said features.
  await toVectorTilesSliceFeatures(buildGuide, vectorWorker);

  // STEP 2: Ensure all data is prepped/sorted for reading/building tiles
  await vectorWorker.sort();

  // STEP 3: collect all existing multimap feature stores and build tiles
  for await (const { face, zoom, x, y, data } of vectorWorker.buildTiles()) {
    if (scheme === 'fzxy') await tileWriter.writeTileS2(face, zoom, x, y, data);
    else await tileWriter.writeTileWM(zoom, x, y, data);
  }

  // STEP 4: build metadata based on the guide
  const metaBuilder = new MetadataBuilder();
  updateBuilder(metaBuilder, buildGuide);
  const metadata = metaBuilder.commit();

  // STEP 5: Commit the metadata
  await tileWriter.commit(metadata);
}

/**
 * STEP 1: Convert all features to tile slices of said features.
 * @param buildGuide - the user defined guide on building the vector tiles
 * @param vectorWorker - the vector tile worker to use
 */
async function toVectorTilesSliceFeatures(
  buildGuide: BuildGuide,
  vectorWorker: VectorTileWorker,
): Promise<void> {
  const { vectorSources, rasterSources, layerGuides, scheme, encoding } = buildGuide;
  const featuresIterator = getFeature(vectorSources, rasterSources);

  // Prepare workers with init messages
  const stringifiedLayerGuides = prepareLayerGuides(layerGuides);
  const initMessage: InitMessage = {
    type: 'init',
    id: 0,
    scheme,
    encoding,
    layerGuides: stringifiedLayerGuides,
  };

  vectorWorker.handleMessage(initMessage);

  for await (const nextFeature of featuresIterator) {
    const { sourceName, feature } = nextFeature;
    const featureMessage: FeatureMessage = { type: 'feature', sourceName, feature };
    vectorWorker.handleMessage(featureMessage);
  }
}

/**
 * Prepare the layer guides for workers to be stringified
 * @param layerGuides - the user defined guide on building the vector tiles
 * @returns the stringified layer guides
 */
function prepareLayerGuides(layerGuides: LayerGuide[]): StringifiedLayerGuide[] {
  return layerGuides.map((layer) => {
    return {
      ...layer,
      onFeature: layer.onFeature !== undefined ? JSON.stringify(layer.onFeature) : undefined,
    };
  });
}

/** A result of a feature iterator */
export interface FeatureIterateResult {
  sourceName: string;
  feature: Features;
}

/**
 * Get the features that will be stored in the tile
 * @param vectorSources - the vector sources that the tile is built from and how the layers are to be stored.
 * @param rasterSources - the raster sources that will be conjoined into a single rgba pixel index for tile extraction
 * @yields - a features
 */
async function* getFeature(
  vectorSources?: Record<string, FeatureIterator>,
  rasterSources?: Record<string, FeatureIterator>,
): AsyncGenerator<FeatureIterateResult> {
  if (vectorSources !== undefined) {
    for (const [sourceName, source] of Object.entries(vectorSources)) {
      for await (const feature of source) yield { sourceName, feature };
    }
  }
  if (rasterSources !== undefined) {
    for (const [sourceName, source] of Object.entries(rasterSources)) {
      for await (const feature of source) yield { sourceName, feature };
    }
  }
}

/**
 * @param metaBuilder - the metadata builder to update
 * @param buildGuide - the user defined guide on building the vector tiles
 */
function updateBuilder(metaBuilder: MetadataBuilder, buildGuide: BuildGuide): void {
  const { name, description, version, scheme, encoding, attribution, layerGuides } = buildGuide;

  metaBuilder.setName(name);
  metaBuilder.setExtension('pbf');
  metaBuilder.setDescription(description ?? 'Built by S2-Tools');
  metaBuilder.setVersion(version ?? '1.0.0');
  metaBuilder.setScheme(scheme ?? 'fzxy'); // 'fzxy' | 'tfzxy' | 'xyz' | 'txyz' | 'tms'
  metaBuilder.setType('vector');
  metaBuilder.setEncoding(encoding ?? 'gz'); // 'gz' | 'br' | 'none'
  if (attribution !== undefined) {
    for (const [displayName, href] of Object.entries(attribution)) {
      metaBuilder.addAttribution(displayName, href);
    }
  }
  for (const layer of layerGuides) metaBuilder.addLayer(layer.sourceName, layer.metadata);
}

/**
 * TODO: Find all cases where prepping the data could be done wrong by the user with
 * TODO: explinations of how to correct them.
 * TODO: - metadata must be correct. -
 * Check and display errors
 * @param _layerGuides - the user defined guide on building the vector tiles
 */
function _findErrors(_layerGuides: LayerGuide[]): void {
  // for (const layerGuide of layerGuides) {
  //   // const { metadata } = layerGuide;
  // }
}
// TODO: tileGuide should be modifed to match metadata minzoom, maxzoom, and projection
// minzoom and maxzoom can be left alone if they already exist, but projection MUST match the
// output projection.

// TODO:
// VECTOR:
// - step 1: ship individual features to workers
// - - step 1a: splite the features into tiles, requesting all tiles from range min->max of the layer
// - - step 1b: store all those features into a multimap where the key is the tile-id and the value is the features
// - step 2: build tiles from each worker
// - - step 2a: given a tile-id, retrieve the features from the multimap
// - - step 2b: build the tile from the features, gzip, etc. then ship the buffer and metadata to main thread
// - - step 2c: store metadata into metaBuilder and the buffer into the store
// finish
//
// CLUSTER:
// - step 1: for each point, just add.
// - step 2: cluster.
// - step 3: request tiles as needed.
//
// RASTER:
// - step 1: for every pixel, store to a point index where the m-value is the rgba value
// - step 2: build tiles from the point index
// - - step 2a: start at max zoom - for each pixel in the tile search the range and find average
//              of all the points in that range (remember color is logarithmic so multiply each pixel r-g-b-a by itself first)
//              If too far zoomed in, find the closest pixel within a resonable radius.
// - - step 2b: after finishing max zoom, use https://github.com/rgba-image/lanczos as I move towards minzoom.
// - - sidenote: I think the easiest way to build tiles is start at 0 zoom and dive down everytime we find at least one point, then move back up
// finish
