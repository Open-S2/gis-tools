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

/** Parse (Geo|S2)JSON from a file that is in the CSV format */
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
export function parseCSVAsRecord(source: string): Record<string, string>[] {
  const res: Record<string, string>[] = [];
  const split = source.split('\n');
  const firstLine = split[0].split(',').map((s) => s.trim());
  for (let line of split.slice(1)) {
    line = line.trim();
    if (line.length === 0) continue;
    const record: Record<string, string> = {};
    const values = line.split(',');
    for (let i = 0; i < firstLine.length; i++) {
      const value: string | undefined = values[i].replaceAll('"', '');
      if (value === '' || value === ' ') continue;
      record[firstLine[i]] = value;
    }
    res.push(record);
  }

  return res;
}
