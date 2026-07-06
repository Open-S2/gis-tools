// Implements https://www.esri.com/content/dam/esrisites/sitecore-archive/Files/Pdfs/library/whitepapers/pdfs/shapefile.pdf
import { extendBBox } from '../../geometry/index.js';
import { toReader } from '../index.js';

import type { DataBaseFile } from './dbf.js';
import type { Transformer } from '../../proj4/index.js';
import type {
  BBOX,
  BBox3D,
  MValue,
  Properties,
  VectorFeature,
  VectorFeatureCollection,
  VectorGeometry,
  VectorLineString,
  VectorLineStringGeometry,
  VectorMultiLineString,
  VectorMultiLineStringGeometry,
  VectorMultiPoint,
  VectorMultiPointGeometry,
  VectorPoint,
  VectorPointGeometry,
  VectorPolygonGeometry,
} from '../../geometry/index.js';
import type { FeatureIterator, Reader, ReaderInputs } from '../index.js';

/** A Shapefile Header describing the internal data */
export interface SHPHeader {
  length: number;
  version: number;
  shpCode: number;
  bbox: BBox3D;
}

/** A Shapefile Row explaining how to read the feature */
export interface SHPRow {
  id: number;
  len: number;
  type: number;
  data: DataView;
}

/** M-Value store */
export interface SHPMValue extends MValue {
  value: number;
}

/**
 * # The Shapefile Reader
 *
 * ## Description
 * Reads data from a shapefile implementing the {@link FeatureIterator} interface
 *
 * ## Usage
 *
 * NOTE: It's recommended to not parse the shapefile directly but instead:
 * - `import { shapefileFromURL } from 'gis-tools-ts';`
 * - `import { shapefileFromPath } from 'gis-tools-ts/file';`
 *
 * This ensures the other files paired with the shapefile are loaded to properly handle the
 * projection and properties data.
 *
 * ## Direct Usage
 *
 * ```ts
 * import { ShapeFileReader, DataBaseFile, Transformer } from 'gis-tools-ts';
 * import { FileReader } from 'gis-tools-ts/file';
 * // or use the MMapReader if using Bun:
 * // import { MMapReader } from 'gis-tools-ts/mmap';
 *
 * const transform = new Transformer();
 * const dbf = new DataBaseFile(new FileReader('./data.dbf'), 'utf-8');
 * const reader = new ShapeFileReader(new FileReader('./data.shp'), dbf, transform);
 *
 * // read all the features
 * for await (const feature of reader) {
 *   console.log(feature);
 * }
 * ```
 *
 * ## Links
 * - https://en.wikipedia.org/wiki/Shapefile
 */
export class ShapeFileReader<
  M = Record<string, unknown>,
  P extends Properties = Properties,
> implements FeatureIterator<M, SHPMValue, P> {
  reader: Reader;
  #header!: SHPHeader;
  rows: number[] = [];
  /**
   * @param input - the input data structure to parse
   * @param dbf - the dbf file
   * @param transform - transform mechanics if they exist
   */
  constructor(
    input: ReaderInputs,
    public dbf?: DataBaseFile,
    public transform?: Transformer,
  ) {
    this.reader = toReader(input);
    this.#parseHeader();
    this.#getRows();
  }

  /**
   * Return a shallow copy of the header data
   * @returns - a shallow copy of the header data
   */
  getHeader(): SHPHeader {
    return { ...this.#header };
  }

  /**
   * Return all the features in the shapefile
   * @returns - a collection of VectorFeatures
   */
  async getFeatureCollection(): Promise<VectorFeatureCollection<M, SHPMValue, P>> {
    const featureCollection: VectorFeatureCollection<M, SHPMValue, P> = {
      type: 'FeatureCollection',
      features: [],
      bbox: this.#header.bbox,
    };

    for await (const feature of this) featureCollection.features.push(feature);

    return featureCollection;
  }

  /**
   * Iterate over all features in the shapefile
   * @yields {VectorFeature}
   */
  async *[Symbol.asyncIterator](): AsyncGenerator<VectorFeature<M, SHPMValue, P>> {
    for (let i = 0; i < this.rows.length; i++) {
      const feature = this.#parseRow(this.rows[i], i);
      if (feature !== undefined) yield feature;
    }
  }

  /** Internal parse for the header */
  #parseHeader(): void {
    const { reader } = this;
    this.#header = {
      length: reader.getInt32(6 << 2) << 1,
      version: reader.getInt32(7 << 2, true),
      shpCode: reader.getInt32(8 << 2, true),
      bbox: [
        reader.getFloat64(9 << 2, true),
        reader.getFloat64(11 << 2, true),
        reader.getFloat64(13 << 2, true),
        reader.getFloat64(15 << 2, true),
        reader.getFloat64(17 << 2, true),
        reader.getFloat64(19 << 2, true),
      ],
    };
    if (this.#header.shpCode > 20) {
      this.#header.shpCode -= 20;
    }
  }

  /** Internal parser to build all the row offsets */
  #getRows(): void {
    const { reader, rows } = this;
    let offset = 100;
    const len = reader.byteLength - 8;
    while (offset <= len) {
      const offsetLength = reader.getInt32(offset + 4) << 1;
      const type = reader.getInt32(offset + 8, true);
      if (offsetLength === 0) break;
      if (type !== 0) rows.push(offset);
      offset += 8 + offsetLength;
    }
  }

  /**
   * Get a row
   * @param offset - offset of the row
   * @returns - the row if it exists
   */
  #getRow(offset: number): undefined | SHPRow {
    const { reader } = this;
    const id = reader.getInt32(offset);
    const len = reader.getInt32(offset + 4) << 1;
    if (len === 0 || offset + len + 8 > reader.byteLength) return;
    return {
      id,
      len,
      data: reader.slice(offset + 12, offset + 12 + len - 4),
      type: reader.getInt32(offset + 8, true),
    };
  }

  /**
   * Parse a row
   * @param rowOffset - the row to get and parse
   * @param index - the index of the feature
   * @returns - the parsed feature
   */
  #parseRow(rowOffset: number, index: number): VectorFeature<M, SHPMValue, P> | undefined {
    const row = this.#getRow(rowOffset);
    if (row === undefined) return;
    const { id, type, data } = row;
    const geometry = this.#parseGeometry(type, data);
    if (geometry === undefined) return;

    return {
      id,
      type: 'VectorFeature',
      properties: (this.dbf?.getProperties(index) ?? {}) as P,
      geometry,
    };
  }

  /**
   * Parse a shape geometry
   * @param type - the shape type
   * @param data - the shape data to parse
   * @returns - the parsed geometry if its valid
   */
  #parseGeometry(type: number, data: DataView): undefined | VectorGeometry<SHPMValue> {
    // ESRI Type flags:
    const is3D = type >= 11 && type <= 18;
    const hasM = (type >= 21 && type <= 28) || is3D;

    if (type === 1 || type === 11 || type === 21) {
      const mOffset = type === 21 ? 16 : is3D && data.byteLength >= 32 ? 24 : undefined;
      const coordinates = this.#parsePoint(data, 0, is3D ? 16 : undefined, mOffset);
      return { type: 'Point', is3D, coordinates };
    } else if (type === 8 || type === 18 || type === 28) {
      return this.#parseMultiPoint(data, is3D, hasM);
    } else if (
      type === 3 ||
      type === 5 ||
      type === 13 ||
      type === 15 ||
      type === 23 ||
      type === 25
    ) {
      const isPoly = type === 5 || type === 15 || type === 25;
      return this.#parseMultiLine(data, isPoly, is3D, hasM);
    } else throw new Error('invalid shape type');
  }

  /**
   * Parse a point
   * @param data - the raw data to decode
   * @param offset - the offset of the point to decode
   * @param offset3D - if provided, the offset of the Z value
   * @param offsetM - if provided, the offset of the M value
   * @returns - the decoded point
   */
  #parsePoint(
    data: DataView,
    offset: number,
    offset3D?: number,
    offsetM?: number,
  ): VectorPoint<SHPMValue> {
    const m = offsetM !== undefined ? data.getFloat64(offsetM, true) : undefined;
    const point: VectorPoint<SHPMValue> = {
      x: data.getFloat64(offset, true),
      y: data.getFloat64(offset + 8, true),
      z: offset3D !== undefined ? data.getFloat64(offset3D, true) : undefined,
      m: m !== undefined && m !== -Number.MAX_VALUE ? { value: m } : undefined,
    };
    return this.transform?.forward(point) ?? point;
  }

  /**
   * Parse a multi-point
   * @param data - the raw data to decode
   * @param is3D - is the shape a 3D shape
   * @param hasM - does the shape contain M data
   * @returns - the decoded point or multi-point
   */
  #parseMultiPoint(
    data: DataView,
    is3D = false,
    hasM = false,
  ): undefined | VectorPointGeometry<SHPMValue> | VectorMultiPointGeometry<SHPMValue> {
    const numPoints = data.getInt32(32, true);
    if (numPoints === 0) return;
    let offset = 36;
    let zOffset = 36 + 16 * numPoints;

    // Grab the min-max bounds
    const mins = this.#parsePoint(data, 0);
    const maxs = this.#parsePoint(data, 16);
    let bbox: BBOX = [mins.x, mins.y, maxs.x, maxs.y, 0, 0];

    if (is3D) {
      bbox[4] = data.getFloat64(zOffset, true);
      bbox[5] = data.getFloat64(zOffset + 8, true);
      zOffset += 16;
    }

    // M array block starts cleanly past the XY array and any Z Range/Z Array blocks
    let mOffset = 36 + 16 * numPoints + (is3D ? 16 + 8 * numPoints : 0);
    // Safe guard to check if the optional M block actually exists in this record buffer
    const holdsM = hasM && mOffset + 16 + 8 * numPoints <= data.byteLength;
    if (holdsM) mOffset += 16; // Skip past the 16-byte M Range [minM, maxM] block

    const coordinates: VectorMultiPoint<SHPMValue> = [];
    let index = 0;
    while (index < numPoints) {
      const point = this.#parsePoint(
        data,
        offset,
        is3D ? zOffset : undefined,
        holdsM ? mOffset : undefined,
      );
      coordinates.push(point);
      offset += 16;
      if (is3D) {
        zOffset += 8;
        bbox = extendBBox(bbox, point);
      }
      if (holdsM) mOffset += 8;
      index++;
    }

    if (numPoints === 1) {
      return { type: 'Point', is3D, coordinates: coordinates[0], bbox };
    } else {
      return { type: 'MultiPoint', is3D, coordinates, bbox };
    }
  }

  /**
   * Parse a multi-line
   * @param data - the raw data to decode
   * @param isPoly - is the shape a polygon or line(s)
   * @param is3D - is the shape a 3D shape
   * @param hasM - does the shape contain M data
   * @returns - the decoded point or multi-point
   */
  #parseMultiLine(
    data: DataView,
    isPoly: boolean,
    is3D: boolean,
    hasM = false,
  ):
    | undefined
    | VectorLineStringGeometry<SHPMValue>
    | VectorMultiLineStringGeometry<SHPMValue>
    | VectorPolygonGeometry<SHPMValue> {
    const numParts = data.getInt32(32, true); // The number of rings/parts
    const numPoints = data.getInt32(36, true); // The total number of points
    if (numPoints === 0 || numParts === 0) return;

    // Restore original working offsets
    let offset = 40 + 4 * numParts; // Points array starts exactly after the parts directory
    let zOffset = offset + 16 * numPoints; // Z block starts after all XY points

    // Grab the min-max bounds
    const mins = this.#parsePoint(data, 0);
    const maxs = this.#parsePoint(data, 16);
    let bbox: BBOX = [mins.x, mins.y, maxs.x, maxs.y, 0, 0];

    if (is3D) {
      bbox[4] = data.getFloat64(zOffset, true);
      bbox[5] = data.getFloat64(zOffset + 8, true);
      zOffset += 16; // Skip past the 16-byte Z Range block
    }

    // Compute the exact trailing M block offset
    let mOffset = 40 + 4 * numParts + 16 * numPoints + (is3D ? 16 + 8 * numPoints : 0);
    // Safe guard to verify the optional M block actually exists in this record buffer
    const holdsM = hasM && mOffset + 16 + 8 * numPoints <= data.byteLength;
    if (holdsM) mOffset += 16; // Skip past the 16-byte M Range [minM, maxM] block

    // Parse the parts directory correctly starting at byte 40
    const parts: number[] = [];
    let done = 0;
    while (done < numParts) {
      parts.push(data.getInt32(40 + done * 4, true));
      done++;
    }

    // Build coordinates
    let index = 0;
    const coordinates: VectorMultiLineString<SHPMValue> = [];
    for (let i = 0; i < numParts; i++) {
      const partEnd = parts[i + 1] ?? numPoints;
      const line: VectorLineString<SHPMValue> = [];

      while (index < partEnd) {
        const point = this.#parsePoint(
          data,
          offset,
          is3D ? zOffset : undefined,
          holdsM ? mOffset : undefined,
        );
        line.push(point);
        offset += 16;
        if (is3D) {
          zOffset += 8;
          bbox = extendBBox(bbox, point);
        }
        if (holdsM) mOffset += 8;
        index++;
      }
      coordinates.push(line);
    }

    if (!isPoly && numParts === 1) {
      return { type: 'LineString', is3D, coordinates: coordinates[0], bbox };
    } else {
      return { type: isPoly ? 'Polygon' : 'MultiLineString', is3D, coordinates, bbox };
    }
  }
}
