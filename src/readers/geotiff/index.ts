// https://docs.ogc.org/is/19-008r4/19-008r4.html#_requirements_class_tiff
import { GeoTIFFHeaderReader } from './header';
import { GeoTIFFImage } from './image';
import { toReader } from '..';

import type { GeoTIFFMetadata } from './image';
import type { ProjectionTransformDefinition } from '../../proj4';
import type { FeatureIterator, RGBA, Reader, ReaderInputs } from '..';
import type { Properties, VectorFeature, VectorMultiPointGeometry } from '../../geometry';

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
 * # GeoTIFF Reader
 *
 * ## Description
 * This class reads a GeoTIFF file and returns a list of GeoTIFF images.
 * Implements the {@link FeatureIterator} interface.
 *
 * ## Usage
 * ```ts
 * import { ALL_DEFINITIONS, EPSG_CODES, GeoTIFFReader } from 'gis-tools-ts';
 * import { FileReader } from 'gis-tools-ts/file';
 *
 * const fileReader = new FileReader(`${__dirname}/fixtures/utm.tif`);
 * const geotiffReader = new GeoTIFFReader(fileReader, ALL_DEFINITIONS, EPSG_CODES);
 * ```
 */
export class GeoTIFFReader
  extends GeoTIFFHeaderReader
  implements FeatureIterator<GeoTIFFMetadata, RGBA, Properties>
{
  gridStore: GridReader[] = [];
  /**
   * @param input - the geotiff input to parse data from
   * @param definitions - an array of projection definitions for the transformer if needed
   * @param epsgCodes - a record of EPSG codes to use for the transformer if needed
   * @param gridStores - an array of grid readers if needed
   */
  constructor(
    input: ReaderInputs,
    private definitions: ProjectionTransformDefinition[] = [],
    private epsgCodes: Record<string, string> = {},
    gridStores?: GridReader[],
  ) {
    const reader = toReader(input);
    super(reader);
    if (gridStores !== undefined) this.gridStore = gridStores;
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
      this.definitions,
      this.epsgCodes,
    );
  }

  /**
   * Iterate through each image and return a vector feature
   * @yields {VectorFeature}
   */
  async *[Symbol.asyncIterator](): AsyncGenerator<
    VectorFeature<GeoTIFFMetadata, RGBA, Properties, VectorMultiPointGeometry<RGBA>>
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
