import { OSMReader } from '.';
import { FileKV, FileReader } from '../../file';

import type { OsmReaderOptions } from '.';
import type { FeatureIterator, OSMMetadata, OSMProperties, Properties } from '../..';

/**
 * # OSM File Reader
 *
 * ## Description
 * Parses OSM PBF files
 * Implements the {@link FeatureIterator} interface
 *
 * ## Usage
 * ```ts
 * import { OSMFileReader } from 'gis-tools-ts/file';
 *
 * const reader = new OSMFileReader('./data.osm.pbf');
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
export class OSMFileReader
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
    const fileReader = new FileReader(input);
    super(fileReader, options);
    this.nodeGeometry = new FileKV();
    this.nodes = new FileKV();
    this.wayGeometry = new FileKV();
    this.ways = new FileKV();
    this.relations = new FileKV();
    this.nodeRelationPairs = new FileKV();
  }
}
