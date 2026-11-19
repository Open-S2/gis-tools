import { toReader } from '../index.js';

import type { FeatureIterator, Reader, ReaderInputs } from '../index.js';
import type { MValue, Properties, VectorFeature, VectorPoint } from '../../geometry/index.js';

/** User defined options on how to parse the CSV file */
export interface CSVReaderOptions {
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
}

/**
 * # CSV Reader
 *
 * ## Description
 * Parse (Geo|S2)JSON from a file that is in the CSV format
 * Implements the {@link FeatureIterator} interface
 *
 * ## Usage
 * ```ts
 * import { CSVReader } from 'gis-tools-ts';
 * import { FileReader } from 'gis-tools-ts/file';
 *
 * const fileReader = new FileReader(`${__dirname}/fixtures/basic3D.csv`);
 * const csvReader = new CSVReader(fileReader, {
 *  // set options, completely optional, but the values you see are the default.
 *  delimiter: ',', // how the CSV document separates values
 *  lineDelimiter: '\n', // the line delimiter, sometimes "\r\n" or some specializ unicode character
 *  lonKey: 'Longitude', // the key to use to store the longitude
 *  latKey: 'Latitude', // the key to use to store the latitude
 *  heightKey: 'height', // the key to use for the height or "z-value"
 * });
 *
 * // read the features
 * for await (const feature of csvReader) {
 *   console.log(feature);
 * }
 * ```
 *
 * ## Links
 * - https://en.wikipedia.org/wiki/Comma-separated_values
 * - https://cesium.com/blog/2015/04/07/quadtree-cheatseet/
 */
export class CSVReader<
  M = Record<string, unknown>,
  D extends MValue = MValue,
  P extends Properties = Properties,
> implements FeatureIterator<M, D, P> {
  reader: Reader;
  #delimiter: string;
  #lineDelimiter: string;
  #lonKey = 'lon';
  #latKey = 'lat';
  #heightKey?: string;
  #firstLine = true;
  #fields: string[] = [];
  /**
   * @param input - the input data to parse from
   * @param options - user defined options on how to parse the CSV file
   */
  constructor(input: ReaderInputs, options?: CSVReaderOptions) {
    this.reader = toReader(input);
    this.#delimiter = options?.delimiter ?? ',';
    this.#lineDelimiter = options?.lineDelimiter ?? '\n';
    this.#lonKey = options?.lonKey ?? 'lon';
    this.#latKey = options?.latKey ?? 'lat';
    this.#heightKey = options?.heightKey;
  }

  /**
   * Generator to iterate over each (Geo|S2)JSON object in the file
   * @yields {VectorFeature}
   */
  async *[Symbol.asyncIterator](): AsyncGenerator<VectorFeature<M, D, P>> {
    const { reader } = this;
    let offset = 0;
    let partialLine = '';

    while (offset < reader.byteLength) {
      const length = Math.min(65_536, reader.byteLength - offset);
      // Prepend any partial line to the new chunk
      const chunk = partialLine + reader.parseString(offset, length);
      partialLine = '';
      // Split the chunk by newlines and yield each complete line
      const lines = chunk.split(this.#lineDelimiter);
      for (let i = 0; i < lines.length - 1; i++) {
        const line = lines[i].trim();
        if (line.length === 0 || line.startsWith('#')) continue;
        if (this.#firstLine) {
          this.#parseFirstLine(line);
          this.#firstLine = false;
        } else {
          yield this.#parseLine(line);
        }
      }
      // Store the remaining partial line for the next iteration
      partialLine = lines[lines.length - 1];
      // Update the offset
      offset += length;
    }

    // Yield any remaining partial line after the loop
    if (partialLine.length > 0) yield this.#parseLine(partialLine);
  }

  /**
   * @param line - the values mapped to the first lines fields
   * @returns - a GeoJSON Vector Feature
   */
  #parseLine(line: string): VectorFeature<M, D, P> {
    const values = parseCSVLine(line, this.#delimiter);

    let is3D = false;
    const properties: Properties = {};
    const coordinates: VectorPoint<D> = { x: 0, y: 0 };

    for (let i = 0; i < this.#fields.length; i++) {
      const field = this.#fields[i];
      const value = values[i];
      if (field.length === 0 || value === undefined || value.trim().length === 0) continue;

      if (field === this.#lonKey) coordinates.x = parseFloat(value);
      else if (field === this.#latKey) coordinates.y = parseFloat(value);
      else if (this.#heightKey !== undefined && field === this.#heightKey) {
        is3D = true;
        coordinates.z = parseFloat(value);
      } else properties[field] = value;
    }
    if (isNaN(coordinates.x) || isNaN(coordinates.y))
      throw new Error('coordinates must be finite numbers');

    return {
      type: 'VectorFeature',
      geometry: {
        type: 'Point',
        is3D,
        coordinates,
      },
      properties: properties as P,
    };
  }

  #parseFirstLine(line: string): void {
    this.#fields = parseCSVLine(line, this.#delimiter);
  }
}

/**
 * Parse CSV data into a record
 * @param source - the source of the CSV data
 * @param delimiter - the delimiter to use (defaults to ',')
 * @param lineDelimiter - the line delimiter to use (defaults to '\n')
 * @returns - an array of records parsed from the CSV rows
 */
export function parseCSVAsRecord<T = Record<string, string>>(
  source: string,
  delimiter = ',',
  lineDelimiter = '\n',
): T[] {
  const res: T[] = [];
  const lines = source.split(lineDelimiter);
  if (lines.length === 0) return res;

  const header = parseCSVLine(lines[0], delimiter);

  for (const rawLine of lines.slice(1)) {
    const line = rawLine.trim();
    if (line.length === 0) continue;

    const record: Record<string, string | undefined> = {};
    const values = parseCSVLine(line, delimiter);

    const limit = Math.min(values.length, header.length);
    for (let i = 0; i < limit; i++) {
      const value = values[i];
      record[header[i]] = value === undefined || value.trim().length === 0 ? undefined : value;
    }

    res.push(record as T);
  }

  return res;
}

/**
 * @param line - a line of a CSV file
 * @param delimiter - the delimiter to use
 * @returns - the values split by the delimiter
 */
function parseCSVLine(line: string, delimiter = ','): string[] {
  const result: string[] = [];
  let current = '';
  let inQuotes = false;
  let quoteChar = '';

  for (let i = 0; i < line.length; i++) {
    const ch = line[i];

    // If we encounter a quote and we aren't in quoted text yet
    if ((ch === '"' || ch === "'") && !inQuotes) {
      inQuotes = true;
      quoteChar = ch;
    }
    // If we see the same quote again, it might be the closing quote
    else if (ch === quoteChar && inQuotes) {
      // Check if it's an escaped quote by looking at the next character
      if (i < line.length - 1 && line[i + 1] === quoteChar) {
        current += quoteChar;
        i++;
      } else {
        inQuotes = false;
      }
    }
    // Split by delimiter only if not inside quotes
    else if (ch === delimiter && !inQuotes) {
      result.push(current.trim());
      current = '';
    } else {
      current += ch;
    }
  }

  // Push the final field
  result.push(current.trim());

  return result;
}
