// Implements https://www.esri.com/content/dam/esrisites/sitecore-archive/Files/Pdfs/library/whitepapers/pdfs/shapefile.pdf
import { extendBBox } from 's2-tools/geometry';

import type DataBaseFile from './dbf';
import type { Reader } from '..';
import type { Transformer } from 's2-tools/proj4';
import type {
  BBOX,
  BBox3D,
  FeatureCollection,
  VectorFeature,
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
} from 's2-tools/geometry';

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

/** The Shapefile Reader */
export default class Shapefile {
  #header!: SHPHeader;
  rows: number[] = [];
  /**
   * @param reader - the input data structure to parse
   * @param dbf - the dbf file
   * @param transform - transform mechanics if they exist
   */
  constructor(
    public reader: Reader,
    public dbf?: DataBaseFile,
    public transform?: Transformer,
  ) {
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
  getFeatureCollection(): FeatureCollection {
    const featureCollection: FeatureCollection = {
      type: 'FeatureCollection',
      features: [],
      bbox: this.#header.bbox,
    };

    for (const feature of this.iterate()) {
      featureCollection.features.push(feature);
    }

    return featureCollection;
  }

  /**
   * Iterate over all features in the shapefile
   * @yields {VectorFeature}
   */
  *iterate(): IterableIterator<VectorFeature> {
    for (let i = 0; i < this.rows.length; i++) {
      const feature = this.#parseRow(this.rows[i], i);
      if (feature !== undefined) yield feature;
    }
  }

  /** Internal parse for the header */
  #parseHeader() {
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
  #getRows() {
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
   * @param rowOffset - the row to get and parse
   * @param index - the index of the feature
   * @returns - the parsed feature
   */
  #parseRow(rowOffset: number, index: number): VectorFeature | undefined {
    const row = this.#getRow(rowOffset);
    if (row === undefined) return;
    const { id, type, data } = row;
    const geometry = this.#parseGeometry(type, data);
    if (geometry === undefined) return;

    return {
      id,
      type: 'VectorFeature',
      properties: this.dbf?.getProperties(index) ?? {},
      geometry,
    };
  }

  /**
   * @param type - the shape type
   * @param data - the shape data to parse
   * @returns - the parsed geometry if its valid
   */
  #parseGeometry(type: number, data: DataView): undefined | VectorGeometry {
    const is3D = type === 11 || type === 13 || type === 15 || type === 18;
    if (type === 1 || type === 11) {
      return {
        type: 'Point',
        is3D,
        coordinates: this.#parsePoint(data, 0, is3D ? 16 : undefined),
      };
    } else if (type === 8 || type === 18) {
      return this.#parseMultiPoint(data, is3D);
    } else if (type === 3 || type === 5 || type === 13 || type === 15) {
      const isPoly = type === 5 || type === 15;
      return this.#parseMultiLine(data, isPoly, is3D);
    } else throw new Error('invalid shape type');
  }

  /**
   * @param data - the raw data to decode
   * @param offset - the offset of the point to decode
   * @param offset3D - if provided, the offset of the Z value
   * @returns - the decoded point
   */
  #parsePoint(data: DataView, offset: number, offset3D?: number): VectorPoint {
    const point: VectorPoint = {
      x: data.getFloat64(offset, true),
      y: data.getFloat64(offset + 8, true),
      z: offset3D ? data.getFloat64(offset3D, true) : undefined,
    };
    return this.transform?.forward(point) ?? point;
  }

  /**
   * @param data - the raw data to decode
   * @param is3D - is the shape a 3D shape
   * @returns - the decoded point or multi-point
   */
  #parseMultiPoint(
    data: DataView,
    is3D = false,
  ): undefined | VectorPointGeometry | VectorMultiPointGeometry {
    const numPoints = data.getInt32(32, true);
    if (numPoints === 0) return;
    let offset = 0;
    let zOffset = 36 + 16 * numPoints;
    // grab the min-max
    const mins = this.#parsePoint(data, offset);
    const maxs = this.#parsePoint(data, offset + 16);
    offset += 36;
    let bbox: BBOX = [mins.x, mins.y, maxs.x, maxs.y, 0, 0];
    if (is3D) {
      bbox[4] = data.getFloat64(zOffset, true);
      bbox[5] = data.getFloat64(zOffset + 8, true);
      zOffset += 16;
    }

    const coordinates: VectorMultiPoint = [];
    let index = 0;
    while (index < numPoints) {
      const point = this.#parsePoint(data, offset, is3D ? zOffset : undefined);
      coordinates.push(point);
      offset += 16;
      if (is3D) {
        zOffset += 8;
        bbox = extendBBox(bbox, point);
      }
      index++;
    }

    if (numPoints === 1) {
      return { type: 'Point', is3D, coordinates: coordinates[0] };
    } else {
      return { type: 'MultiPoint', is3D, coordinates, bbox };
    }
  }

  /**
   * @param data - the raw data to decode
   * @param isPoly - is the shape a polygon or line(s)
   * @param is3D - is the shape a 3D shape
   * @returns - the decoded point or multi-point
   */
  #parseMultiLine(
    data: DataView,
    isPoly: boolean,
    is3D: boolean,
  ): undefined | VectorLineStringGeometry | VectorMultiLineStringGeometry | VectorPolygonGeometry {
    const numParts = data.getInt32(32, true); // The number of rings in the polygon.
    const numPoints = data.getInt32(36, true); // the total number of points in the polygon.
    if (numPoints === 0 || numParts === 0) return;
    let offset = 0;
    let zOffset = 40 + 4 * numParts + 16 * numPoints;
    // grab the min-max
    const mins = this.#parsePoint(data, offset);
    const maxs = this.#parsePoint(data, offset + 16);
    let bbox: BBOX = [mins.x, mins.y, maxs.x, maxs.y, 0, 0];
    offset += 40;
    if (is3D) {
      bbox[4] = data.getFloat64(zOffset, true);
      bbox[5] = data.getFloat64(zOffset + 8, true);
      zOffset += 16;
    }

    // build parts
    const parts: number[] = [];
    let done = 0;
    while (done < numParts) {
      parts.push(data.getInt32(offset, true));
      offset += 4;
      done++;
    }

    // build coordinates
    let index = 0;
    const coordinates: VectorMultiLineString = [];
    for (let i = 0; i < numParts; i++) {
      const partEnd = parts[i + 1] ?? numPoints;
      // build a line for part
      const line: VectorLineString = [];
      while (index < partEnd) {
        const point = this.#parsePoint(data, offset, is3D ? zOffset : undefined);
        line.push(point);
        offset += 16;
        if (is3D) {
          zOffset += 8;
          bbox = extendBBox(bbox, point);
        }
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
