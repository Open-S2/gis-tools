import { MetadataBuilder } from 's2-tilejson';

import type { Reader } from '../../readers';
import type { TileWriter } from '../../writers';
import type { VectorFeatures } from '../../geometry';
import type { Attribution, Encoding, LayerMetaData, Scheme } from 's2-tilejson';

import type { ClusterOptions } from '../../dataStructures/pointCluster';
import type { TileStoreOptions } from '../../dataStructures/tile';

/** A layer defines the exact mechanics of what data to parse and how the data is stored */
export interface SourceLayer {
  /** Name of the layer */
  name: string;
  /** Components of how the layer is built and stored */
  metadata: LayerMetaData;
}

/**
 * Before tiling the data, you can mutate it here. It can also act as a filter if you return undefined
 */
export type OnFeature = (feature: VectorFeatures) => VectorFeatures | undefined;

/** Sources are the blueprints of what data to fetch and how to store it */
export interface BaseSource {
  /** The name of the source */
  name: string;
  /** The reader to parse the data from */
  data: Reader;
}

/** Vector source */
export interface VectorSource extends BaseSource {
  /** The layers to construct and organize the data around for this source */
  layers: SourceLayer[];
  /** If options are provided, the assumption is the point data is clustered */
  cluster?: ClusterOptions;
  /** tile buffer on each side so lines and polygons don't get clipped */
  buffer?: number;
  /** whether to build the bounding box for each tile feature */
  buildBBox?: boolean;
  /** Before tiling the data, you can mutate it here. It can also act as a filter if you return undefined */
  onFeature?: OnFeature;
}

/** Raster source */
export interface RasterSource extends BaseSource {
  description?: string;
}

/** A layerguide is a minified version of a layer used by workers to build tiles */
export interface LayerGuide {
  /** Components of how the layer is built and stored */
  metadata: LayerMetaData;
  /** Stringified version of the onFeature used by the source so it can be shipped to a worker. */
  onFeature?: string;
  /** Guide on how to splice the data into vector tiles */
  tileGuide: TileStoreOptions;
}

/** A parsed LayerGuide where onFeature is parsed back into a function */
export interface ParsedLayerGuide extends Omit<LayerGuide, 'onFeature'> {
  onFeature?: OnFeature;
}

/** A Source Guide is a minified version of a source used by workers to build tiles */
export interface SourceGuide {
  /** The name of the source */
  [sourceName: string]: {
    /** Name of the associated layer */
    [layerName: string]: LayerGuide;
  };
}

/** A parsed SourceGuide where onFeature is parsed back into a function */
export interface ParsedSourceGuide {
  /** The name of the source */
  [sourceName: string]: {
    /** Name of the associated layer */
    [layerName: string]: ParsedLayerGuide;
  };
}

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
  /** The vector sources that the tile is built from and how the layers are to be stored */
  vectorSources?: VectorSource[];
  /** The raster sources that will be conjoined into a single rgba pixel index for tile extraction */
  rasterSources?: RasterSource[];
  /** The attribution of the data. Store as { key: 'presentation name' }. [value: href link] */
  attribution?: Attribution;
  /**
   * The vector format if applicable helps define how the vector data is stored.
   * - The more modern vector format is the 'open-v2' which supports things like m-values
   * and 3D geometries.
   * - The legacy vector format is the 'open-v1' which only supports 2D geometries and works on
   * older map engines like Mapbox-gl-js.
   * [Default: 'open-v2']
   */
  vectorFormat?: 'open-v1' | 'open-v2';
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
  const { tileWriter } = buildGuide;

  // TODO: Prepare init messages for workers

  // TODO: iterate features and have workers split/store them

  // TODO: externalSort on features at this point.

  // TODO: have workers build tiles

  // build metadata based on the guide
  const metaBuilder = new MetadataBuilder();
  updateBuilder(metaBuilder, buildGuide);
  const metadata = metaBuilder.commit();

  // FINISH
  await tileWriter.commit(metadata);
}

/**
 * @param metaBuilder - the metadata builder to update
 * @param buildGuide - the user defined guide on building the vector tiles
 */
function updateBuilder(metaBuilder: MetadataBuilder, buildGuide: BuildGuide): void {
  const { name, description, version, scheme, encoding, attribution, vectorSources } = buildGuide;

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
  for (const { layers } of vectorSources ?? []) {
    for (const layer of layers) metaBuilder.addLayer(layer.name, layer.metadata);
  }
}

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
