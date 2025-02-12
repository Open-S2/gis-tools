import { toReader } from '..';
import { toVector } from '../..';

import type { FeatureIterator, Reader, ReaderInputs } from '..';
import type { Features, JSONCollection, MValue, Properties, VectorFeatures } from '../../geometry';

/**
 * # JSON Buffer Reader
 *
 * ## Description
 * Standard Buffer Reader for (Geo|S2)JSON
 * implements the {@link FeatureIterator} interface
 *
 * ## Usage
 * ```ts
 * import { BufferJSONReader } from 'gis-tools-ts';
 *
 * const reader = new BufferJSONReader('{ type: 'FeatureCollection', features: [...] }');
 * // OR
 * const reader = new BufferJSONReader({ type: 'FeatureCollection', features: [...] });
 *
 * // read the features
 * for await (const feature of reader) {
 *   console.log(feature);
 * }
 * ```
 */
export class BufferJSONReader<
  M = Record<string, unknown>,
  D extends MValue = MValue,
  P extends Properties = Properties,
> implements FeatureIterator<M, D, P>
{
  data: JSONCollection<M, D, P>;

  /** @param data - the JSON data to parase */
  constructor(data: string | JSONCollection<M, D, P>) {
    if (typeof data === 'string') {
      this.data = JSON.parse(data);
    } else {
      this.data = data;
    }
  }

  /**
   * Generator to iterate over each (Geo|S2)JSON object in the file
   * @yields {VectorFeatures}
   */
  async *[Symbol.asyncIterator](): AsyncGenerator<VectorFeatures<M, D, P>> {
    const { type } = this.data;

    if (type === 'FeatureCollection') {
      for (const feature of this.data.features) {
        if (feature.type === 'VectorFeature') yield feature;
        else yield toVector(feature, true);
      }
    } else if (type === 'Feature') {
      yield toVector(this.data, true);
    } else if (type === 'VectorFeature') {
      yield this.data;
    } else if (type === 'S2FeatureCollection') {
      for (const feature of this.data.features) {
        yield feature;
      }
    } else if (type === 'S2Feature') {
      yield this.data;
    }
  }
}

/**
 * # NewLine Delimited JSON Reader
 *
 * ## Description
 * Parse (Geo|S2)JSON from a file that is in a newline-delimited format
 * Implements the {@link FeatureIterator} interface
 *
 * ## Usage
 * ```ts
 * import { NewLineDelimitedJSONReader } from 'gis-tools-ts';
 * import { FileReader } from 'gis-tools-ts/file';
 *
 * const reader = new NewLineDelimitedJSONReader(new FileReader('./data.geojsonld'));
 * // read the features
 * for await (const feature of reader) {
 *   console.log(feature);
 * }
 * ```
 */
export class NewLineDelimitedJSONReader<
  M = Record<string, unknown>,
  D extends MValue = MValue,
  P extends Properties = Properties,
> implements FeatureIterator<M, D, P>
{
  reader: Reader;
  /**
   * @param input - the input to parse from
   * @param seperator - the newline delimiter. Default is "\n" but can be "\r\n" or "\r"
   */
  constructor(
    input: ReaderInputs,
    private seperator = '\n',
  ) {
    this.reader = toReader(input);
  }

  /**
   * Generator to iterate over each (Geo|S2)JSON object in the file
   * @yields {VectorFeatures}
   */
  async *[Symbol.asyncIterator](): AsyncGenerator<VectorFeatures<M, D, P>> {
    const { reader } = this;
    let cursor = 0;
    let offset = 0;
    let partialLine = '';

    while (offset < reader.byteLength) {
      const length = Math.min(65_536, reader.byteLength - cursor);
      // Prepend any partial line to the new chunk
      const chunk = partialLine + reader.parseString(offset, length);
      partialLine = chunk.endsWith(this.seperator) ? this.seperator : '';
      // Split the chunk by newlines and yield each complete line
      const lines = chunk.split(this.seperator).filter((line) => line.length > 0);
      for (let i = 0; i < lines.length - 1; i++) {
        const feature: Features<M, D, P> = JSON.parse(lines[i]);
        if (feature.type === 'Feature') yield toVector(feature, true);
        else yield feature;
      }
      // Store the remaining partial line for the next iteration
      partialLine = lines[lines.length - 1] + partialLine;
      // Update the cursor and offset
      offset += length;
      cursor += length;
    }

    // Yield any remaining partial line after the loop
    if (partialLine.length > 1) {
      const feature: Features<M, D, P> = JSON.parse(partialLine);
      if (feature.type === 'Feature') yield toVector(feature, true);
      else yield feature;
    }
  }
}

/**
 * # GeoJSON Text Sequence Reader
 *
 * ## Description
 * Parse GeoJSON from a file that is in the `geojson-text-sequences` format.
 * Implements the {@link FeatureIterator} interface.
 *
 * ## Usage
 * ```ts
 * import { SequenceJSONReader } from 'gis-tools-ts';
 * import { FileReader } from 'gis-tools-ts/file';
 *
 * const reader = new SequenceJSONReader(new FileReader('./data.geojsons'));
 * // read the features
 * for await (const feature of reader) {
 *   console.log(feature);
 * }
 * ```
 */
export class SequenceJSONReader<
  M = Record<string, unknown>,
  D extends MValue = MValue,
  P extends Properties = Properties,
> extends NewLineDelimitedJSONReader<M, D, P> {
  /** @param input - the input to parse from */
  constructor(input: ReaderInputs) {
    super(input, '␞');
  }
}

const LEFT_BRACE = 0x7b;
const RIGHT_BRACE = 0x7d;
const BACKSLASH = 0x5c;
const STRING = 0x22;

/**
 * # JSON Reader
 *
 * ## Description
 * Parse (Geo|S2)JSON. Can handle millions of features.
 * Implements the {@link FeatureIterator} interface
 *
 * ## Usage
 * ```ts
 * import { JSONReader } from 'gis-tools-ts';
 * import { FileReader } from 'gis-tools-ts/file';
 *
 * const reader = new JSONReader(new FileReader('./data.geojsonld'));
 * // read the features
 * for await (const feature of reader) {
 *   console.log(feature);
 * }
 * ```
 */
export class JSONReader<
  M = Record<string, unknown>,
  D extends MValue = MValue,
  P extends Properties = Properties,
> implements FeatureIterator<M, D, P>
{
  reader: Reader;
  #chunkSize = 65_536;
  #buffer: Uint8Array = new Uint8Array();
  #offset = 0;
  #length: number;
  #pos = 0;
  #braceDepth = 0;
  #feature: Uint8Array[] = [];
  #start: null | number = null;
  #end: null | number = null;
  #isObject = true;

  /**
   * @param input - the input to parse from
   * @param chunkSize - the number of bytes to read at a time from the reader. [Default: 65_536]
   */
  constructor(input: ReaderInputs, chunkSize?: number) {
    this.reader = toReader(input);
    if (chunkSize !== undefined) this.#chunkSize = chunkSize;
    this.#length = this.reader.byteLength;
  }

  /**
   * Generator to iterate over each (Geo|S2)JSON object in the reader.
   * @yields {Features}
   */
  async *[Symbol.asyncIterator](): AsyncGenerator<VectorFeatures<M, D, P>> {
    if (this.#length <= this.#chunkSize) {
      const reader = new BufferJSONReader<M, D, P>(this.reader.parseString(0, this.#length));
      yield* reader;
      return;
    }
    // buffer the first chunk
    this.#buffer = new Uint8Array(this.reader.slice(0, this.#chunkSize).buffer);
    // find out starting position
    const set = this.#setStartPosition();
    if (!set) throw Error('File is not geojson or s2json');
    while (true) {
      const feature = this.#nextValue();
      if (feature !== undefined) {
        if (feature.type === 'Feature') yield toVector(feature, true);
        else yield feature;
      } else break;
    }
  }

  /**
   * since we know that a '{' is the start of a feature after we read a '"features"',
   * than we start there to avoid reading in values that are not features.
   * This is a modified Knuth–Morris–Pratt algorithm
   * @returns - true if the start position was found
   */
  #setStartPosition(): boolean {
    const features = Buffer.from('"features":');
    const featuresSize = features.length;

    let k = 0;
    while (this.#pos < this.#chunkSize) {
      if (features[k] === this.#buffer[this.#pos]) {
        k++;
        this.#pos++;
        if (k === featuresSize) {
          return true;
        }
      } else {
        k = 0;
        this.#pos++;
      }
    }
    // if we made it here, we need to read in the next buffer chunk.
    // If we hit the end of the file, return false
    this.#offset += this.#chunkSize;
    if (this.#offset < this.#length) {
      this.#pos = 0;
      if (this.#offset + this.#chunkSize < this.#length) {
        this.#chunkSize = this.#length - this.#offset;
      }
      this.#chunkSize = Math.min(65_536, this.#length - this.#offset);
      this.#buffer = new Uint8Array(
        this.reader.slice(this.#offset, this.#offset + this.#chunkSize).buffer,
      );
      return this.#setStartPosition();
    } else {
      return false;
    }
  }

  /**
   * everytime we see a "{" we start 'recording' the feature. If we see more "{" on our journey, we increment.
   * Once we find the end of the feature, store the "start" and "end" indexes, slice the buffer and send out
   * as a return. If we run out of buffer to read AKA we finish the file, we return a null. If we run
   * out of the buffer, but we still have file left to read, just read into the buffer and continue on
   * @returns - the feature or nothing if we hit the end of the file
   */
  #nextValue(): undefined | Features<M, D, P> {
    // get started
    while (this.#pos < this.#chunkSize) {
      if (this.#buffer[this.#pos] === BACKSLASH) {
        this.#pos++;
      } else if (this.#buffer[this.#pos] === STRING) {
        if (!this.#isObject) this.#isObject = true;
        else this.#isObject = false;
      } else if (this.#buffer[this.#pos] === LEFT_BRACE && this.#isObject) {
        if (this.#braceDepth === 0) this.#start = this.#pos;
        this.#braceDepth++; // first brace is the start of the feature
      } else if (this.#buffer[this.#pos] === RIGHT_BRACE && this.#isObject) {
        this.#braceDepth--; // if this hits zero, we are at the end of the feature
        if (this.#braceDepth === 0) {
          this.#end = this.#pos;
          break;
        }
      }
      this.#pos++;
    }

    // what if the last char in current buffer was a BACKSLASH?
    // we need to make sure in the next buffer we account for increment
    const incrementSpace = this.#pos - this.#chunkSize;

    if (this.#start !== null && this.#end !== null) {
      this.#pos++;
      try {
        this.#feature.push(this.#buffer.subarray(this.#start, this.#end + 1));
        const feature = Buffer.concat(this.#feature);
        // reset variables
        this.#feature = [];
        this.#start = null;
        this.#end = null;
        this.#braceDepth = 0;
        this.#isObject = true;
        // return
        return JSON.parse(feature.toString('utf8'));
      } catch (_err) {
        console.error(new Error('could not parse feature'), this.#feature.toString());
        // reset variables
        this.#feature = [];
        this.#start = null;
        this.#end = null;
        this.#braceDepth = 0;
        this.#isObject = true;
        return;
      }
    } else {
      // if offset isn't at filesize, increment buffer and start again
      if (this.#start !== null) {
        this.#feature.push(this.#buffer.subarray(this.#start));
        this.#start = 0;
      }
      this.#offset += this.#chunkSize;
      if (this.#offset < this.#length) {
        this.#pos = incrementSpace > 0 ? incrementSpace : 0;
        if (this.#offset + this.#chunkSize > this.#length) {
          this.#chunkSize = this.#length - this.#offset;
        }
        this.#chunkSize = Math.min(65_536, this.#length - this.#offset);
        this.#buffer = new Uint8Array(
          this.reader.slice(this.#offset, this.#offset + this.#chunkSize).buffer,
        );
        return this.#nextValue();
      } else {
        return;
      } // end of file
    }
  }
}
