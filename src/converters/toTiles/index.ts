import TileWorker from './worker/tileWorker';
import { xyzToBBOX } from '../../geometry/wm/coords';
import { DrawType, MetadataBuilder } from 's2-tilejson';

import type { Extents } from 'open-vector-tile';
import type {
  Attribution,
  Encoding,
  Extensions,
  ImageExtensions,
  LayerMetaData,
  Shape,
  SourceType,
} from 's2-tilejson';
import type {
  ClusterOptions,
  FeatureIterator,
  GridRasterOptions,
  GridValueOptions,
  MValue,
  Projection,
  Properties,
  RGBA,
  // TileReader,
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
  /** Explain what the layer is */
  description?: string;
  /** Name of the source */
  sourceName: string;
  /** Name of the layer */
  layerName: string;
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
  /** Raster clustering guide */
  rasterGuide: GridRasterOptions;
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
  /** Grid clustering guide */
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  gridGuide: GridValueOptions<any>;
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
  vectorGuide: TileStoreOptions;
  /** Extent at which the layer is storing its data */
  extent: Extents;
  /** Shape guide for the vector layer */
  shape: Shape;
  /** M-Value Shape guide for the vector layer */
  mShape?: Shape;
  /**
   * Draw Types (points, lines, polygons, 3D points, 3D lines, 3D polygons).
   * This is a filter mechanic. The source data may have multiple feature/draw types,
   * but the layer you're building only wants to use the points. So you would add:
   * ```ts
   * import { DrawType } from 'gis-tools-ts';
   * const drawTypes = [DrawType.Points];
   * ```
   */
  drawTypes: DrawType[];
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
export interface BuildGuide<V extends MValue = Properties, G extends MValue = Properties> {
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
  projection: Projection;
  /** The encoding format. Can be either 'gz', 'br', 'zstd' or 'none'. [Default: 'none'] */
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
  format: FormatOutput;
  /**
   * The vector sources that the tile is built from and how the layers are to be stored.
   * Created using `{ [sourceName: string]: FeatureIterator }`
   * See: {@link FeatureIterator}
   */
  vectorSources?: Record<string, FeatureIterator<unknown, V, V>>;
  /** The raster sources that will be conjoined into a single rgba pixel index for tile extraction */
  rasterSources?: Record<string, FeatureIterator<unknown, RGBA, RGBA>>;
  /** The grid sources that will be conjoined into a single grid index for tile extraction */
  gridSources?: Record<string, FeatureIterator<unknown, G, G>>;
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
 * Build vector tiles give a guide on what sources to parse data from and how to store it
 * @param buildGuide - the user defined guide on building the vector tiles
 */
export async function toTiles(buildGuide: BuildGuide): Promise<void> {
  const { tileWriter, extension, projection } = buildGuide;
  const worker = new TileWorker();

  // STEP 1: Convert all features to tile slices of said features.
  await toTilesSliceFeatures(buildGuide, worker);
  // STEP 2: Ensure all data is prepped/sorted for reading/building tiles
  await worker.sort();
  // STEP 3: build metadata based on the guide
  const metaBuilder = new MetadataBuilder();
  const type = buildGuide.format === 'raster' ? 'raster' : 'vector';
  updateBuilder(metaBuilder, buildGuide, type, extension ?? 'pbf');
  // STEP 4: collect all existing multimap feature stores and build tiles
  for await (const { face, zoom, x, y, data } of worker.buildTiles()) {
    if (projection === 'S2') {
      await tileWriter.writeTileS2(face, zoom, x, y, data);
      // TODO: handle llbounds correctly
      metaBuilder.addTileS2(face, zoom, x, y, [0, 0, 0, 0]);
    } else {
      await tileWriter.writeTileWM(zoom, x, y, data);
      metaBuilder.addTileWM(zoom, x, y, xyzToBBOX(x, y, zoom, false, 'WGS84'));
    }
  }
  // STEP 5: Commit the metadata
  await tileWriter.commit(metaBuilder.commit());
}

// /**
//  * TODO: Merge tiles together
//  * @param _tileReaders - the tile readers to merge
//  * @param _tileWriter - the tile writer to write to
//  */
// export async function mergeTiles(
//   _tileReaders: TileReader[],
//   _tileWriter: TileWriter,
// ): Promise<void> {}

/**
 * STEP 1: Convert all features to tile slices of said features.
 * @param buildGuide - the user defined guide on building the vector tiles
 * @param worker - the vector tile worker to use
 */
async function toTilesSliceFeatures(buildGuide: BuildGuide, worker: TileWorker): Promise<void> {
  const { vectorSources, rasterSources, gridSources, layerGuides, projection, encoding, format } =
    buildGuide;
  const featuresIterator = getFeature([
    vectorSources as unknown as Record<string, FeatureIterator>,
    rasterSources as unknown as Record<string, FeatureIterator>,
    gridSources as unknown as Record<string, FeatureIterator>,
  ]);

  // Prepare workers with init messages
  const initMessage: InitMessage = {
    type: 'init',
    id: 0,
    projection,
    encoding,
    format,
    layerGuides: prepareLayerGuides(layerGuides),
  };
  worker.handleMessage(initMessage);
  // iterate over all features
  for await (const nextFeature of featuresIterator) {
    const { sourceName, feature } = nextFeature;
    const featureMessage: FeatureMessage = { type: 'feature', sourceName, feature };
    worker.handleMessage(featureMessage);
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
  buildGuide: BuildGuide,
  type: SourceType,
  extensions: Extensions,
): void {
  const { name, description, version, projection, attribution } = buildGuide;
  const encoding = 'encoding' in buildGuide ? buildGuide.encoding : 'none';
  const layerGuides = 'layerGuides' in buildGuide ? buildGuide.layerGuides : [];

  metaBuilder.setName(name);
  metaBuilder.setExtension(extensions);
  metaBuilder.setDescription(description ?? 'Built by S2-Tools');
  metaBuilder.setVersion(version ?? '1.0.0');
  // NOTE: For now we only support xyz and fzxy
  metaBuilder.setScheme(projection === 'WM' ? 'xyz' : 'fzxy'); // 'fzxy' | 'tfzxy' | 'xyz' | 'txyz' | 'tms'
  metaBuilder.setType(type);
  metaBuilder.setEncoding(encoding ?? 'none'); // 'gz' | 'br' | 'none'
  if (attribution !== undefined) {
    for (const [displayName, href] of Object.entries(attribution)) {
      metaBuilder.addAttribution(displayName, href);
    }
  }
  // Build metadata from layerGuides
  for (const layer of layerGuides) metaBuilder.addLayer(layer.layerName, buildLayerMetadata(layer));
}

/**
 * Build layer metadata from layer guide
 * @param layer - the layer guide to parse
 * @returns the layer metadata
 */
function buildLayerMetadata(layer: LayerGuide): LayerMetaData {
  const { description } = layer;
  let drawTypes: DrawType[] = [];
  let minzoom = 0;
  let maxzoom = 14;
  if ('vectorGuide' in layer) {
    drawTypes = layer.drawTypes;
    minzoom = layer.vectorGuide.minzoom ?? 0;
    maxzoom = layer.vectorGuide.maxzoom ?? 14;
  } else if ('clusterGuide' in layer) {
    drawTypes = [DrawType.Points];
    minzoom = layer.clusterGuide.minzoom ?? 0;
    maxzoom = layer.clusterGuide.maxzoom ?? 14;
  } else if ('rasterGuide' in layer) {
    drawTypes = [DrawType.Raster];
    minzoom = layer.rasterGuide.minzoom ?? 0;
    maxzoom = layer.rasterGuide.maxzoom ?? 14;
  } else if ('gridGuide' in layer) {
    drawTypes = [DrawType.Grid];
    minzoom = layer.gridGuide.minzoom ?? 0;
    maxzoom = layer.gridGuide.maxzoom ?? 14;
  }
  const shape = 'shape' in layer ? layer.shape : {};
  const mShape = 'mShape' in layer ? layer.mShape : undefined;
  return {
    description,
    minzoom,
    maxzoom,
    drawTypes,
    shape,
    mShape,
  };
}

// /**
//  * TODO: Find all cases where prepping the data could be done wrong by the user with
//  * TODO: explinations of how to correct them.
//  * TODO: - metadata must be correct. -
//  * Check and display errors
//  * @param _layerGuides - the user defined guide on building the vector tiles
//  */
// function _findErrors(_layerGuides: LayerGuide[]): void {
//   // for (const layerGuide of layerGuides) {
//   //   // const { metadata } = layerGuide;
//   // }
// }
