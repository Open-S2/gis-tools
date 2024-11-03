import { MultiMap } from '../../../dataStore';

import type { MultiMapStore } from '../../../dataStore';
import type { VectorFeature } from '../../../geometry';
import type { OnFeature, ParsedSourceGuide, SourceGuide } from '../..';

/** Take in options that will be used to create a tiled data correctly */
export interface InitMessage {
  /** Message type */
  type: 'init';
  /** The sources that will be used to create the tile */
  sources: SourceGuide;
}

/** Take in a feature that will be added to the tile */
export interface FeatureMessage {
  /** Message type */
  type: 'feature';
  /** The feature to add to the tile */
  feature: VectorFeature;
}

/** Convert a vector feature to a collection of tiles and store each tile feature */
export default class VectorTileWorker {
  sources: ParsedSourceGuide = {};
  writer: MultiMapStore<VectorFeature> = new MultiMap<VectorFeature>();
  /**
   * Tile-ize input vector features and store them
   * @param event - the init message or a feature message
   */
  onmessage(event: Bun.MessageEvent<InitMessage | FeatureMessage>): void {
    this.handleMessage(event.data);
    self.postMessage({ type: 'ready' });
  }

  /**
   * Tile-ize input vector features and store them
   * @param message - the init message or a feature message
   */
  handleMessage(message: InitMessage | FeatureMessage): void {
    const { type } = message;
    if (type === 'init') {
      this.sources = parseSourceGuide(message.sources);
    } else {
      // TODO:
    }
  }
}

/**
 * Convert a source guide to a parsed source guide (where onFeature is parsed back into a function)
 * @param sourceGuide - the source guide to parse
 * @returns the parsed source guide
 */
function parseSourceGuide(sourceGuide: SourceGuide): ParsedSourceGuide {
  const res: ParsedSourceGuide = {};

  for (const [sourceName, source] of Object.entries(sourceGuide)) {
    for (const [layerName, layer] of Object.entries(source)) {
      res[sourceName][layerName] = {
        ...layer,
        onFeature:
          layer.onFeature !== undefined
            ? (new Function(layer.onFeature)() as OnFeature)
            : undefined,
      };
    }
  }

  return res;
}
