// https://docs.ogc.org/is/19-008r4/19-008r4.html#_requirements_class_tiff
import '../../util/polyfills/dataview';
import { GeoTIFFHeaderReader } from './header';
import { GeoTIFFImage } from './image';

import type { Reader } from '..';
import type { GeoTIFFMetadata, RGBA } from './image';
import type { Properties, VectorFeature, VectorMultiPointGeometry } from 's2-tools/geometry';

export * from './color';
export * from './constants';
export * from './decoder';
export * from './header';
export * from './image';
export * from './imageUtil';
export * from './predictor';

/**
 *
 */
export interface GridReader {
  key: string;
  reader: Reader;
}

/**
 *
 */
export class GeoTIFFReader extends GeoTIFFHeaderReader {
  gridStore: GridReader[] = [];
  /** @param reader - the geotiff reader to parse data from */
  constructor(reader: Reader) {
    super(reader);
  }

  /**
   * @param key
   * @param reader
   */
  addGridReader(key: string, reader: Reader): void {
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
