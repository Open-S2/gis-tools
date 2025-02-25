import { encodingToCompression } from '../..';
import { xyzToBBOX } from '../../geometry/wm/coords';
import { DrawType, MetadataBuilder } from 's2-tilejson';
import TileWorker, { getMaxzoom, getMinzoom } from './worker/tileWorker';

import type { BuildGuide, LayerGuide, StringifiedLayerGuide } from './types';
import type { Extensions, LayerMetaData, SourceType } from 's2-tilejson';
import type { FeatureIterator, VectorFeatures } from '../..';
import type { FeatureMessage, InitMessage } from './worker/tileWorker';

export * from './types';

/**
 * Build vector tiles give a guide on what sources to parse data from and how to store it
 * @param buildGuide - the user defined guide on building the vector tiles
 */
export async function toTiles(buildGuide: BuildGuide): Promise<void> {
  const { tileWriter, extension, projection, encoding } = buildGuide;
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
  await tileWriter.commit(metaBuilder.commit(), encodingToCompression(encoding ?? 'none'));
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
  const { vectorSources, rasterSources, gridSources, layerGuides } = buildGuide;
  const { projection, encoding, format, buildIndices } = buildGuide;
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
    buildIndices,
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
  metaBuilder.setDescription(description ?? 'Built by GIS-Tools');
  metaBuilder.setVersion(version ?? '1.0.0');
  // NOTE: For now we only support xyz and fzxy
  metaBuilder.setScheme(projection === 'WG' ? 'xyz' : 'fzxy'); // 'fzxy' | 'tfzxy' | 'xyz' | 'txyz' | 'tms'
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
  if ('vectorGuide' in layer) {
    drawTypes = layer.drawTypes;
  } else if ('clusterGuide' in layer) {
    drawTypes = [DrawType.Points];
  } else if ('rasterGuide' in layer) {
    drawTypes = [DrawType.Raster];
  } else if ('gridGuide' in layer) {
    drawTypes = [DrawType.Grid];
  }
  const minzoom = getMinzoom([layer]);
  const maxzoom = getMaxzoom([layer]);
  const shape = 'shape' in layer ? layer.shape : {};
  const mShape = 'mShape' in layer ? layer.mShape : undefined;
  return { description, minzoom, maxzoom, drawTypes, shape, mShape };
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
