import TileWorker from './worker/tileWorker';
import { DrawType, MetadataBuilder } from 's2-tilejson';

import type { Extents } from 'open-vector-tile';
import type {
  Attribution,
  Encoding,
  Extensions,
  ImageExtensions,
  LayerMetaData,
  Scheme,
  SourceType,
} from 's2-tilejson';
import type {
  ClusterOptions,
  FeatureIterator,
  MValue,
  Properties,
  RGBA,
  TileStoreOptions,
  TileWriter,
  VectorFeatures,
  VectorPoint,
} from '../..';
import type { FeatureMessage, InitMessage } from './worker/tileWorker';

// NOTE: IF using workers in threads, it MUST be for local use filesytem. If using a buffer system it's single threaded

/**
 * Before tiling the data, you can mutate it here. It can also act as a filter if you return undefined
 */
export type OnFeature<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
> = (feature: VectorFeatures<M, D, P>) => VectorFeatures<M, D, P> | undefined;

/** Function to get the value of a point. Used by grid layers */
export type GetPointValue<T extends MValue = Properties> = (point: VectorPoint<T>) => number;

/** No matter the type of layer you want to build, these are default properties to include */
export interface BaseLayer<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
> {
  /** Name of the source */
  sourceName: string;
  /** Name of the layer */
  layerName: string;
  /** Components of how the layer is built and stored */
  metadata: LayerMetaData;
  /** If provided, you can mutate the feature. If you return nothing it's the same as filtering the feature */
  onFeature?: OnFeature<M, D, P>;
}

/** Guide to building Raster layer data */
export interface RasterLayer<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
> extends BaseLayer<M, D, P> {
  /** describes how the image will be stored */
  outputType: ImageExtensions;
  /**
   * The type of interpolation to use for downsampling to parent tiles from children.
   * Options:
   * - average: average of the reference points
   * - nearest: nearest neighbor interpolation
   * - idw: Inverse Distance Weighted interpolation
   * - lanczos: Lanczos 3 interpolation
   * [default: lanczos]
   */
  interpolation?: 'average' | 'nearest' | 'idw' | 'lanczos';
}
/** Guide to building Raster layer data where the onFeature is stringified to ship to workers */
export interface StringifiedRasterLayer extends Omit<RasterLayer, 'onFeature'> {
  /** Stringified version of the onFeature used by the source so it can be shipped to a worker. */
  onFeature?: string;
}

/** Guide to building Grid layer data */
export interface GridLayer<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
> extends BaseLayer<M, D, P> {
  /** Method of accessing the value of interest. Defaults to the point's "z" value */
  getValue?: GetPointValue;
  /**
   * The type of interpolation to use for downsampling to parent tiles from children.
   * Options:
   * - average: average of the reference points
   * - nearest: nearest neighbor interpolation
   * - idw: Inverse Distance Weighted interpolation
   * - lanczos: Lanczos 3 interpolation
   * [default: lanczos]
   */
  interpolation?: 'average' | 'nearest' | 'idw' | 'lanczos';
  /** Specify the tile buffer size in pixels. [default: 1] */
  bufferSize?: number;
  /** Extent at which the layer is storing its data */
  extent: Extents;
}
/** Guide to building Grid layer data where the onFeature is stringified to ship to workers */
export interface StringifiedGridLayer extends Omit<GridLayer, 'onFeature' | 'getValue'> {
  /** Stringified version of the onFeature used by the source so it can be shipped to a worker. */
  onFeature?: string;
  /** Stringified version of the getValue used by the source so it can be shipped to a worker. */
  getValue?: string;
}

/** Guide to building Cluster layer data */
export interface ClusterLayer<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
> extends BaseLayer<M, D, P> {
  /** If options are provided, the assumption is the point data is clustered */
  clusterGuide: ClusterOptions;
  /** Extent at which the layer is storing its data */
  extent: Extents;
}
/** Guide to building Cluster layer data where the onFeature is stringified to ship to workers */
export interface StringifiedClusterLayer extends Omit<ClusterLayer, 'onFeature'> {
  /** Stringified version of the onFeature used by the source so it can be shipped to a worker. */
  onFeature?: string;
}

/** Guide to building Vector layer data */
export interface VectorLayer<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
> extends BaseLayer<M, D, P> {
  /** Guide on how to splice the data into vector tiles */
  tileGuide: TileStoreOptions;
  /** Extent at which the layer is storing its data */
  extent: Extents;
}
/** Guide to building Vector layer data where the onFeature is stringified to ship to workers */
export interface StringifiedVectorLayer extends Omit<VectorLayer, 'onFeature'> {
  /** Stringified version of the onFeature used by the source so it can be shipped to a worker. */
  onFeature?: string;
}

/** List of user defined guides to build layers */
export type LayerGuide = RasterLayer | GridLayer | ClusterLayer | VectorLayer;

/** List of user defined guides to build layers where the onFeature is stringified to ship to workers */
export type StringifiedLayerGuide =
  | StringifiedRasterLayer
  | StringifiedGridLayer
  | StringifiedClusterLayer
  | StringifiedVectorLayer;

/**
 * The vector format if applicable helps define how the vector data is stored.
 * - The more modern vector format is the 'open-s2' which supports things like m-values
 * and 3D geometries.
 * - The new vector format is the 'open-s2' which only supports 2D & 3D geometries, supports M-Values,
 * properties and M-Values can have nested properties and/or arrays, and is decently fast to parse.
 * - The basic vector format is the 'flat-open-s2' which only supports 2D geometries and works on
 * older map engines like Mapbox-gl-js, is faster to parse and often lighter in size.
 * - The older vector format is the 'mapbox' which is the legacy format used by Mapbox and slow to parse.
 * - The `raster` format is used speciially for raster ONLY data. Ensures the data is stored as a raster
 * [Default: 'open-s2']
 */
export type FormatOutput = 'mapbox' | 'flat-open-s2' | 'open-s2' | 'raster';

/** A user defined guide on building the vector tiles */
export interface BuildGuide {
  /** The name of the data */
  name: string;
  /** The description of the data */
  description?: string;
  /** User defined versioning for their data */
  version?: string;
  /**
   * Specify the image type. e.g. 'pbf', 'png', 'jpg', 'webp', etc.
   * [Default: 'pbf']
   */
  extension?: Extensions;
  /**
   * What kind of output format should be used. Used for describing either S2 or WM
   * projections. [Default: 'fzxy']
   */
  scheme?: Scheme;
  /** The encoding format. Can be either 'gz', 'br', 'zstd' or 'none'. [Default: 'gz'] */
  encoding?: Encoding;
  /** The attribution of the data. Store as `{ 'presentation name': 'href' }`. */
  attribution?: Attribution;
  /**
   * The vector format if applicable helps define how the vector data is stored.
   * - The more modern vector format is the 'open-s2' which supports things like m-values
   * and 3D geometries.
   * - The new vector format is the 'open-s2' which only supports 2D & 3D geometries, supports M-Values,
   * properties and M-Values can have nested properties and/or arrays, and is decently fast to parse.
   * - The basic vector format is the 'flat-open-s2' which only supports 2D geometries and works on
   * older map engines like Mapbox-gl-js, is faster to parse and often lighter in size.
   * - The older vector format is the 'mapbox' which is the legacy format used by Mapbox and slow to parse.
   * - The `raster` format is used speciially for raster ONLY data. Ensures the data is stored as a raster
   * [Default: 'open-s2']
   */
  format?: FormatOutput;
  /**
   * The vector sources that the tile is built from and how the layers are to be stored.
   * Created using `{ [sourceName: string]: FeatureIterator }`
   * See: {@link FeatureIterator}
   */
  vectorSources?: Record<string, FeatureIterator>;
  /** The raster sources that will be conjoined into a single rgba pixel index for tile extraction */
  rasterSources?: Record<string, FeatureIterator<Record<string, unknown>, RGBA, Properties>>;
  /** The grid sources that will be conjoined into a single grid index for tile extraction */
  gridSources?: Record<string, FeatureIterator>;
  /**
   * The guides on how to build the various data
   * See: {@link LayerGuide}
   */
  layerGuides: LayerGuide[];
  /**
   * The data created will be stored in either a folder structure or a pmtiles file
   * Folder structure is either '{face}/{zoom}/{x}/{y}.pbf' or '{zoom}/{x}/{y}.pbf'.
   * PMTiles store all data in a single data file.
   * See: {@link TileWriter}
   */
  tileWriter: TileWriter;
  /** Set the number of threads to use. [Default: 1] */
  threads?: number;
}

/**
 * List of user defined guides to build raster data where the onFeature is stringified to ship to workers
 */
export interface RasterBuildGuide
  extends Omit<
    BuildGuide,
    'extension' | 'layerGuides' | 'format' | 'vectorSources' | 'gridSources' | 'encoding'
  > {
  /** Specify the image type. e.g. 'png', 'jpg', 'webp', etc. */
  extension: ImageExtensions;
  /** The description of the data */
  description?: string;
  /** User defined minimum zoom level */
  minzoom: number;
  /** User defined maximum zoom level */
  maxzoom: number;
  /** Stringified version of the onFeature used by the source so it can be shipped to a worker. */
  onFeature?: OnFeature;
}

/**
 * Build vector tiles give a guide on what sources to parse data from and how to store it
 * @param buildGuide - the user defined guide on building the vector tiles
 */
export async function toVectorTiles(buildGuide: BuildGuide): Promise<void> {
  const { tileWriter, extension, scheme } = buildGuide;
  const vectorWorker = new TileWorker();

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
  const type = buildGuide.format === 'raster' ? 'raster' : 'vector';
  updateBuilder(metaBuilder, buildGuide, type, extension ?? 'pbf');
  const metadata = metaBuilder.commit();

  // STEP 5: Commit the metadata
  await tileWriter.commit(metadata);
}

/**
 * Build vector tiles give a guide on what sources to parse data from and how to store it
 * @param rasterBuildGuide - the user defined guide on building the vector tiles
 */
export async function toRasterTiles(rasterBuildGuide: RasterBuildGuide): Promise<void> {
  const { description, minzoom, maxzoom, extension, onFeature } = rasterBuildGuide;
  // setup worker and layer
  const rasterLayer: RasterLayer = {
    sourceName: 'raster',
    layerName: 'default',
    outputType: extension,
    metadata: {
      minzoom,
      maxzoom,
      description,
      drawTypes: [DrawType.Raster],
      shape: {},
    },
    onFeature,
  };
  const buildGuide: BuildGuide = {
    ...rasterBuildGuide,
    format: 'raster',
    encoding: 'none',
    layerGuides: [rasterLayer],
  };
  await toVectorTiles(buildGuide);
}

/**
 * STEP 1: Convert all features to tile slices of said features.
 * @param buildGuide - the user defined guide on building the vector tiles
 * @param vectorWorker - the vector tile worker to use
 */
async function toVectorTilesSliceFeatures(
  buildGuide: BuildGuide,
  vectorWorker: TileWorker,
): Promise<void> {
  const { vectorSources, rasterSources, layerGuides, scheme, encoding, format } = buildGuide;
  const featuresIterator = getFeature([vectorSources, rasterSources]);

  // Prepare workers with init messages
  const stringifiedLayerGuides = prepareLayerGuides(layerGuides);
  const initMessage: InitMessage = {
    type: 'init',
    id: 0,
    scheme,
    encoding,
    format,
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
      getValue:
        'getValue' in layer && layer.getValue !== undefined
          ? JSON.stringify(layer.getValue)
          : undefined,
      onFeature: layer.onFeature !== undefined ? JSON.stringify(layer.onFeature) : undefined,
    };
  });
}

/** A result of a feature iterator */
export interface FeatureIterateResult {
  sourceName: string;
  feature: VectorFeatures;
}

/**
 * Get the features that will be stored in the tile
 * @param sources - the vector/raster sources. Either:
 * - the vector sources that the tile is built from and how the layers are to be stored.
 * - the raster sources that will be conjoined into a single rgba pixel index for tile extraction
 * @yields - a features
 */
async function* getFeature(
  sources: (Record<string, FeatureIterator> | undefined)[],
): AsyncGenerator<FeatureIterateResult> {
  for (const sourceObj of sources) {
    if (sourceObj === undefined) continue;
    for (const [sourceName, source] of Object.entries(sourceObj)) {
      for await (const feature of source) yield { sourceName, feature };
    }
  }
}

/**
 * @param metaBuilder - the metadata builder to update
 * @param buildGuide - the user defined guide on building the vector tiles
 * @param type - the type of source
 * @param extensions - the extension name
 */
function updateBuilder(
  metaBuilder: MetadataBuilder,
  buildGuide: BuildGuide | RasterBuildGuide,
  type: SourceType,
  extensions: Extensions,
): void {
  const { name, description, version, scheme, attribution } = buildGuide;
  const encoding = 'encoding' in buildGuide ? buildGuide.encoding : 'gz';
  const layerGuides = 'layerGuides' in buildGuide ? buildGuide.layerGuides : [];

  metaBuilder.setName(name);
  metaBuilder.setExtension(extensions);
  metaBuilder.setDescription(description ?? 'Built by S2-Tools');
  metaBuilder.setVersion(version ?? '1.0.0');
  metaBuilder.setScheme(scheme ?? 'fzxy'); // 'fzxy' | 'tfzxy' | 'xyz' | 'txyz' | 'tms'
  metaBuilder.setType(type);
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
