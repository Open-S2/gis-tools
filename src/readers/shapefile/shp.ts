// Implements https://www.esri.com/content/dam/esrisites/sitecore-archive/Files/Pdfs/library/whitepapers/pdfs/shapefile.pdf

import type DataBaseFile from './dbf';
import type { ProjectionTransform } from 's2-tools/proj4/projections';
import type {
  BBox,
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

/**
 *
 */
export interface SHPHeader {
  length: number;
  version: number;
  shpCode: number;
  bbox: BBox;
}

/**
 *
 */
export interface SHPRow {
  id: number;
  index: number;
  len: number;
  type: number;
  data: DataView;
}

/**
 *
 */
export default class Shapefile {
  header!: SHPHeader;
  rows!: SHPRow[];
  /**
   * @param buffer
   * @param trans
   * @param transform
   * @param dbf
   */
  constructor(
    public buffer: DataView,
    public transform?: ProjectionTransform,
    public dbf?: DataBaseFile,
  ) {
    this.buffer = buffer;
    this.#parseHeader();
    this.#getRows();
  }

  /**
   * Return all the features in the shapefile
   * @returns - a collection of VectorFeatures
   */
  getFeatureCollection(): FeatureCollection {
    const featureCollection: FeatureCollection = {
      type: 'FeatureCollection',
      features: [],
      bbox: this.header.bbox,
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
    for (const row of this.rows) {
      const feature = this.#parseRow(row);
      if (feature !== undefined) yield feature;
    }
  }

  /**
   *
   */
  #parseHeader() {
    const view = this.buffer;
    this.header = {
      length: view.getInt32(6 << 2) << 1,
      version: view.getInt32(7 << 2, true),
      shpCode: view.getInt32(8 << 2, true),
      bbox: [
        view.getFloat64(9 << 2, true),
        view.getFloat64(11 << 2, true),
        view.getFloat64(13 << 2, true),
        view.getFloat64(15 << 2, true),
      ],
    };
    if (this.header.shpCode > 20) {
      this.header.shpCode -= 20;
    }
  }

  /**
   *
   */
  #getRows() {
    let index = 0;
    let offset = 100;
    const len = this.buffer.byteLength - 8;
    while (offset <= len) {
      const current = this.#getRow(offset, index);
      index++;
      if (current === undefined) {
        break;
      }
      offset += 8;
      offset += current.len;
      if (current.type !== 0) {
        this.rows.push(current);
      }
    }
  }

  /**
   * @param offset - offset of the row
   * @param index - index of the row
   * @returns - the row if it exists
   */
  #getRow(offset: number, index: number): undefined | SHPRow {
    const id = this.buffer.getInt32(offset);
    const len = this.buffer.getInt32(offset + 4) << 1;
    if (len === 0) {
      return { id, index, len: 0, data: new DataView(new ArrayBuffer(0)), type: 0 };
    }
    if (offset + len + 8 > this.buffer.byteLength) {
      return;
    }
    return {
      id,
      index,
      len,
      data: new DataView(this.buffer.buffer, this.buffer.byteOffset + offset + 12, len - 4),
      type: this.buffer.getInt32(offset + 8, true),
    };
  }

  /**
   * @param row
   */
  #parseRow(row: SHPRow): VectorFeature | undefined {
    const { id, index, type, data } = row;
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
   * @param type
   * @param offset
   * @param data
   * @param length
   */
  #parseGeometry(type: number, data: DataView): undefined | VectorGeometry {
    const is3D = type === 11 || type === 13 || type === 15 || type === 18;
    if (type === 1 || type === 11) {
      return {
        type: 'Point',
        is3D,
        coordinates: this.#parsePoint(data, 0, is3D),
      };
    } else if (type === 8 || type === 18) {
      return this.#parseMultiPoint(data, is3D);
    } else if (type === 3 || type === 5 || type === 13 || type === 15) {
      const isPoly = type === 5 || type === 15;
      return this.#parseMultiLine(data, isPoly, is3D);
    } else throw new Error('invalid shape type');
  }

  /**
   * @param data
   * @param offset
   * @param is3D
   */
  #parsePoint(data: DataView, offset: number, is3D = false): VectorPoint {
    const point: VectorPoint = {
      x: data.getFloat64(offset, true),
      y: data.getFloat64(offset + 8, true),
      z: is3D ? data.getFloat64(offset + 16, true) : undefined,
    };
    return this.transform?.inverse(point) ?? point;
  }

  /**
   * @param offset
   * @param length
   * @param data
   * @param is3D
   */
  #parseMultiPoint(
    data: DataView,
    is3D = false,
  ): undefined | VectorPointGeometry | VectorMultiPointGeometry {
    const size = data.getInt32(32, true);
    if (size === 0) return;
    let offset = 0;
    // grab the min-max
    const mins = this.#parsePoint(data, offset);
    const maxs = this.#parsePoint(data, offset + 16);
    offset += 36;

    const coordinates: VectorMultiPoint = [];
    let done = 0;
    while (done < size) {
      coordinates.push(this.#parsePoint(data, offset, is3D));
      offset += 16;
      if (is3D) offset += 8;
      done++;
    }

    if (size === 1) {
      return { type: 'Point', is3D, coordinates: coordinates[0] };
    } else {
      return { type: 'MultiPoint', is3D, coordinates, bbox: [mins.x, mins.y, maxs.x, maxs.y] };
    }
  }

  /**
   * @param offset
   * @param data
   * @param isPoly
   * @param is3D
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
    // grab the min-max
    const mins = this.#parsePoint(data, offset);
    const maxs = this.#parsePoint(data, offset + 16);
    const bbox: BBox = [mins.x, mins.y, maxs.x, maxs.y];
    offset += 40;

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
    for (const part of parts) {
      // build a line for part
      const line: VectorLineString = [];
      while (index < part) {
        line.push(this.#parsePoint(data, offset, is3D));
        offset += 16;
        if (is3D) offset += 8;
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
