import { convert } from '../../geometry/tools/convert.js';

import type { FeatureIterator, VectorFeatures, Writer } from '../../index.js';

/** User defined options on how to store the features */
export interface ToCSVOptions {
  /** The delimiter to use to separate lines [Default=','] */
  delimiter?: string;
  /** The lineDelimiter to use to separate lines [Default='\n'] */
  lineDelimiter?: string;
  /** If provided the lookup of the longitude [Default='lon'] */
  lonKey?: string;
  /** If provided the lookup of the latitude [Default='lat'] */
  latKey?: string;
  /** If provided the lookup for the height value [Default=undefined] */
  heightKey?: string;
  /** List of parameters to include in the feature */
  properties?: string[];
  /** handle each feature */
  onFeature?: (feature: VectorFeatures) => VectorFeatures | undefined;
}

/**
 * # CSV Writer
 *
 * ## Description
 * Given a writer and an array of iterators, write the input features to the writer as a CSV data
 *
 * ## Usage
 * ```ts
 * import { toCSV, JSONReader } from 'gis-tools-ts';
 * import { FileReader, FileWriter } from 'gis-tools-ts/file';
 * // or use mmap reader if using bun
 * // import { MMapReader } from 'gis-tools-ts/mmap';
 *
 * const fileReader = new FileReader(`${__dirname}/fixtures/points.geojson`);
 * const jsonReader = new JSONReader(fileReader);
 * const bufWriter = new FileWriter(`${__dirname}/fixtures/points.csv`);
 *
 * // store to singular output
 * await toCSV(bufWriter, [jsonReader], { properties: ['name'] });
 * ```
 *
 * @param writer - the writer to append strings to
 * @param iterators - the collection of iterators to write
 * @param opts - user defined options [optional]
 */
export async function toCSV(
  writer: Writer,
  iterators: FeatureIterator[],
  opts?: ToCSVOptions,
): Promise<void> {
  const onFeature = opts?.onFeature ?? ((feature) => feature);
  const delimiter = opts?.delimiter ?? ',';
  const lineDelimiter = opts?.lineDelimiter ?? '\n';
  const lonKey = opts?.lonKey ?? 'lon';
  const latKey = opts?.latKey ?? 'lat';
  const heightKey = opts?.heightKey ?? undefined;
  const props = opts?.properties ?? [];

  // setup the CSV first descripter line
  let startString = `${lonKey}${delimiter}${latKey}`;
  if (heightKey !== undefined) startString += `${delimiter}${heightKey}`;
  for (const property of props) startString += `${delimiter}${property}`;
  await writer.appendString(startString + lineDelimiter);

  for (const iterator of iterators) {
    for await (const feature of iterator) {
      const convertedFeatures = convert('WG', feature, false);
      for (const convertedFeature of convertedFeatures) {
        const userFeature = onFeature(convertedFeature);
        if (userFeature === undefined) continue;
        const { geometry, properties } = userFeature;
        const { type: geoType, coordinates } = geometry;
        const points =
          geoType === 'Point' ? [coordinates] : geoType === 'MultiPoint' ? coordinates : [];

        for (const point of points) {
          // write each point to CSV file
          let outputString = `${point.x}${delimiter}${point.y}`;
          if (heightKey !== undefined) outputString += `${delimiter}${point.z ?? ''}`;
          for (const property of props) outputString += `${delimiter}${properties[property]}`;
          await writer.appendString(outputString + lineDelimiter);
        }
      }
    }
  }
}
