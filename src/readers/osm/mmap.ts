import { OSMReader } from '.';
import { MMapKV, MMapReader } from '../../mmap';

import type { OsmReaderOptions } from '.';
import type { FeatureIterator, OSMMetadata, OSMProperties, Properties } from '../..';

/**
 * # OSM MMap Reader
 *
 * ## Description
 * Parses OSM PBF files
 * Implements the {@link FeatureIterator} interface
 *
 * ## Usage
 * ```ts
 * import { OSMMMapReader } from 'gis-tools-ts/mmap';
 *
 * const reader = new OSMMMapReader('./data.osm.pbf');
 *
 * // pull out the header
 * const header = reader.getHeader();
 *
 * // read the features
 * for (const feature of reader) {
 *   console.log(feature);
 * }
 *
 * // close the reader when done
 * reader.close();
 * ```
 *
 * ## Links
 * - https://wiki.openstreetmap.org/wiki/PBF_Format
 * - https://github.com/openstreetmap/pbf/blob/master/OSM-binary.md
 */
export class OSMMMapReader
  extends OSMReader
  implements FeatureIterator<OSMMetadata, Properties, OSMProperties>
{
  /**
   * @param input - The input (may be a local memory filter or file reader)
   * @param options - User defined options to apply when reading the OSM file
   */
  constructor(
    input: string,
    public options?: OsmReaderOptions,
  ) {
    const fileReader = new MMapReader(input);
    super(fileReader, options);
    this.nodeGeometry = new MMapKV();
    this.nodes = new MMapKV();
    this.wayGeometry = new MMapKV();
    this.ways = new MMapKV();
    this.relations = new MMapKV();
    this.nodeRelationPairs = new MMapKV();
  }
}
