// https://docs.ogc.org/is/19-008r4/19-008r4.html#_requirements_class_tiff
import '../../util/polyfills/dataview';
import { GeoTIFFHeaderReader } from './header';
import { GeoTIFFImage } from './image';
import { toReader } from '..';

import type { GeoTIFFMetadata, RGBA } from './image';
import type { Properties, VectorFeature, VectorMultiPointGeometry } from '../../geometry';
import type { Reader, ReaderInputs } from '..';

export * from './color';
export * from './constants';
export * from './decoder';
export * from './header';
export * from './image';
export * from './imageUtil';
export * from './predictor';

/** A grid reader object */
export interface GridReader {
  key: string;
  reader: Reader;
}

/**
 * GeoTIFF Reader
 */
export class GeoTIFFReader extends GeoTIFFHeaderReader {
  gridStore: GridReader[] = [];
  /** @param input - the geotiff input to parse data from */
  constructor(input: ReaderInputs) {
    const reader = toReader(input);
    super(reader);
  }

  /**
   * Add a grid reader
   * @param key - the key or name of the grid
   * @param input - the input data to parse
   */
  addGridReader(key: string, input: ReaderInputs): void {
    const reader = toReader(input);
    this.gridStore.push({ key, reader });
  }

  /**
   * Get the n-th internal subfile of an image. By default, the first is returned.
   * @param index - the index of the image to get [Default=0]
   * @returns - the image at the given index
   */
  getImage(index = 0): GeoTIFFImage {
    if (index >= this.length) throw new Error('Index out of bounds.');
    return new GeoTIFFImage(
      this.reader,
      this.imageDirectories[index],
      this.littleEndian,
      this.gridStore,
    );
  }

  /**
   * Iterate through each image and return a vector feature
   * @yields {VectorFeature}
   */
  async *[Symbol.asyncIterator](): AsyncGenerator<
    VectorFeature<GeoTIFFMetadata, RGBA, Properties, VectorMultiPointGeometry>
  > {
    for (let i = 0; i < this.length; i++) {
      const { geometry, width, height, alpha } = await this.getImage(i).getMultiPointVector();
      yield {
        type: 'VectorFeature',
        geometry,
        properties: {},
        metadata: { width, height, alpha },
      };
    }
  }
}
