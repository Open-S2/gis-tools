import { MetadataBuilder } from 's2-tilejson';

import type { Reader } from '../../readers';
import type { VectorFeatures } from '../../geometry';
import type { Attribution, Encoding, LayerMetaData, Scheme } from 's2-tilejson';
import type { TileWriter, Writer } from '../../writers';

import type { ClusterOptions } from '../../dataStructures/pointCluster';

/** A layer defines the exact mechanics of what data to parse and how the data is stored */
export interface Layer {
  /** Name of the layer */
  name: string;
  /** Components of how the layer is built and stored */
  metadata: LayerMetaData;
}

/** Sources are the blueprints of what data to fetch and how to store it */
export interface Source {
  /** The reader to parse the data from */
  data: Reader;
  /** If options are provided, the assumption is the point data is clustered */
  cluster?: ClusterOptions;
  /** Before tiling the data, you can mutate it here. It can also act as a filter if you return undefined */
  onFeature?: (feature: VectorFeatures) => VectorFeatures | undefined;
  /** The layers to construct and organize the data around for this source */
  layers: Layer[];
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
  /** The sources that the tile is built from and how the layers are to be stored */
  sources: Source[];
  /** The attribution of the data. Store as [key: presentation name]: [value: href link] */
  attribution?: Attribution;
  /**
   * The vector format if applicable helps define how the vector data is stored.
   * - The more modern vector format is the 'open-v2' which supports things like m-values
   * and 3D geometries.
   * - The legacy vector format is the 'open-v1' which only supports 2D geometries and works on
   * older map engines like Mapbox-gl-js.
   */
  vectorFormat?: 'open-v2' | 'open-v1';
  /**
   * The data created will be stored in either a folder structure or a pmtiles file
   * Folder structure is either '{face}/{zoom}/{x}/{y}.pbf' or '{zoom}/{x}/{y}.pbf'.
   * PMTiles store all data in a single data file.
   */
  tileWriter: TileWriter;
  /** Explain to the module what kind of writer to use (A buffer or file writer) */
  writer: Writer;
}

/**
 * Build vector tiles give a guide on what sources to parse data from and how to store it
 * @param buildGuide - the user defined guide on building the vector tiles
 */
export function toVectorTiles(buildGuide: BuildGuide): void {
  const { tileWriter } = buildGuide;

  // first setup our metadata builder
  const metaBuilder = new MetadataBuilder();
  updateBuilder(metaBuilder, buildGuide);

  // TODO: iterate features and have workers split/store them

  // TODO: have workers build tiles

  // FINISH:
  const metadata = metaBuilder.commit();
  tileWriter.commit(metadata);
}

/**
 * @param metaBuilder - the metadata builder to update
 * @param buildGuide - the user defined guide on building the vector tiles
 */
function updateBuilder(metaBuilder: MetadataBuilder, buildGuide: BuildGuide): void {
  const { name, description, version, scheme, encoding, attribution, sources } = buildGuide;

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
  for (const { layers } of sources) {
    for (const layer of layers) {
      metaBuilder.addLayer(layer.name, layer.metadata);
    }
  }
}

// TODO:
// - step 1: ship individual features to workers
// - - step 1a: splite the features into tiles, requesting all tiles from range min->max of the layer
// - - step 1b: store all those features into a multimap where the key is the tile-id and the value is the features
// - step 2: build tiles from each worker
// - - step 2a: given a tile-id, retrieve the features from the multimap
// - - step 2b: build the tile from the features, gzip, etc. then ship the buffer and metadata to main thread
// - - step 2c: store metadata into metaBuilder and the buffer into the store
// finish
