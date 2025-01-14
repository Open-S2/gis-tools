import { toReader } from '..';

import type { FeatureIterator, Reader, ReaderInputs } from '..';
import type { MValue, Properties, VectorFeature, VectorPoint } from '../../geometry';

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
 *  delimiter: ',',
 *  lineDelimiter: '\n',
 *  lonKey: 'Longitude',
 *  latKey: 'Latitude',
 *  heightKey: 'height',
 * });
 * // read the features
 * for await (const feature of reader) {
 *   console.log(feature);
 * }
 * ```
 */
export class CSVReader<
  M = Record<string, unknown>,
  D extends MValue = MValue,
  P extends Properties = Properties,
> implements FeatureIterator<M, D, P>
{
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
    let cursor = 0;
    let offset = 0;
    let partialLine = '';

    while (offset < reader.byteLength) {
      const length = Math.min(65_536, reader.byteLength - cursor);
      // Prepend any partial line to the new chunk
      const chunk = partialLine + reader.parseString(offset, length);
      partialLine = '';
      // Split the chunk by newlines and yield each complete line
      const lines = chunk.split(this.#lineDelimiter);
      for (let i = 0; i < lines.length - 1; i++) {
        if (this.#firstLine) {
          this.#parseFirstLine(lines[i]);
          this.#firstLine = false;
        } else {
          yield this.#parseLine(lines[i]);
        }
      }
      // Store the remaining partial line for the next iteration
      partialLine = lines[lines.length - 1];
      // Update the cursor and offset
      offset += length;
      cursor += length;
    }

    // Yield any remaining partial line after the loop
    if (partialLine.length > 0) yield this.#parseLine(partialLine);
  }

  /**
   * @param line - the values mapped to the first lines fields
   * @returns - a GeoJSON Vector Feature
   */
  #parseLine(line: string): VectorFeature<M, D, P> {
    const values = line.split(this.#delimiter).map((v) => v.trim());

    let is3D = false;
    const properties: Properties = {};
    const coordinates: VectorPoint<D> = { x: 0, y: 0 };

    for (let i = 0; i < this.#fields.length; i++) {
      const field = this.#fields[i];
      const value = values[i];
      if (field.length === 0 || value.length === 0) continue;

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

  /** @param line - the fields in the first line split by the delimiter */
  #parseFirstLine(line: string): void {
    this.#fields = line.split(this.#delimiter).map((v) => v.trim());
  }
}

/**
 * Parse CSV data into a record
 * @param source - the source of the CSV data
 * @returns - an object with key-value pairs whose keys and values are both strings
 */
export function parseCSVAsRecord<T = Record<string, string>>(source: string): T[] {
  const res: T[] = [];
  const lines = source.split('\n');
  const header = parseCSVLine(lines[0]);

  for (const rawLine of lines.slice(1)) {
    const line = rawLine.trim();
    if (line.length === 0) continue;

    const record: Record<string, string> = {};
    const values = parseCSVLine(line);

    for (let i = 0; i < header.length; i++) {
      const value = values[i];
      if (value === '' || value === ' ') continue;
      record[header[i]] = values[i] ?? '';
    }

    res.push(record as T);
  }

  return res;
}

/**
 * @param line - a line of a CSV file
 * @returns - the values split by the delimiter
 */
function parseCSVLine(line: string): string[] {
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
    // Split by commas only if not inside quotes
    else if (ch === ',' && !inQuotes) {
      result.push(current.trim());
      current = '';
    } else {
      current += ch;
    }
  }

  // Push the final field
  if (current !== undefined) result.push(current.trim());

  return result;
}
