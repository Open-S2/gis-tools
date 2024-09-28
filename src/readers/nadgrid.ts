import type { Reader } from '.';

import type { FeatureCollection, VectorFeature, VectorPoint } from 's2-tools/geometry';

/**
 * Resources for details of NTv2 file formats:
 * - https://web.archive.org/web/20140127204822if_/http://www.mgs.gov.on.ca:80/stdprodconsume/groups/content/@mgs/@iandit/documents/resourcelist/stel02_047447.pdf
 * - http://mimaka.com/help/gs/html/004_NTV2%20Data%20Format.htm
 */

/** Store Grids from a NTv2 file (.gsb) */
export class NadGridStore {
  grids = new Map<string, NadGrid>();

  /** @param grid - a nadgrid class to store */
  addGrid(grid: NadGrid) {
    this.grids.set(grid.key, grid);
  }

  /**
   * @param key - the key or name of the grid
   * @returns - the grid
   */
  getGrid(key: string): NadGrid | undefined {
    return this.grids.get(key);
  }

  /**
   * @param key - the key or name of the grid
   * @param reader - the input data to parse
   */
  addGridFromReader(key: string, reader: Reader) {
    const grid = new NadGrid(key, reader);
    this.addGrid(grid);
  }
}

/** The header of a NTv2 file */
export interface NadGridHeader {
  nFields: number;
  nSubgridFields: number;
  nSubgrids: number;
  shiftType: string;
  fromSemiMajorAxis: number;
  fromSemiMinorAxis: number;
  toSemiMajorAxis: number;
  toSemiMinorAxis: number;
}

/** Each subgrid has it's own data on how to decode the points inside the subgrid */
export interface NadSubGridHeader {
  name: string;
  parent: string;
  lowerLatitude: number;
  upperLatitude: number;
  lowerLongitude: number;
  upperLongitude: number;
  latitudeInterval: number;
  longitudeInterval: number;
  gridNodeCount: number;
}

/** A single Node describing how to decode the point */
export interface GridNode {
  latitudeShift: number;
  longitudeShift: number;
  latitudeAccuracy: number;
  longitudeAccuracy: number;
}

/** The metadata inside each vector feature */
export interface NadGridMetadata {
  lowerLonLat: VectorPoint;
  lonLatInterval: VectorPoint;
  lonLatColumnCount: VectorPoint;
  count: number;
}

/** Load a binary NTv2 file (.gsb) */
export class NadGrid {
  isLittleEndian = false;
  #header!: NadGridHeader;
  features: VectorFeature<NadGridMetadata>[] = [];

  /**
   * @param key - the key or name of the grid
   * @param reader - the input data to parse
   */
  constructor(
    public key: string,
    public reader: Reader,
  ) {
    this.#detectLittleEndian();
    this.#readHeader();
    this.#readSubGrids();
  }

  /** @returns - The header describing how to decode the feature */
  getHeader(): NadGridHeader {
    return { ...this.#header };
  }

  /**
   * Return all the features in the shapefile
   * @returns - a collection of VectorFeatures
   */
  getFeatureCollection(): FeatureCollection<NadGridMetadata> {
    const { features } = this;
    return { type: 'FeatureCollection', features };
  }

  /**
   * Iterate over all features in the shapefile
   * @yields {VectorFeature<NadGridMetadata>}
   */
  *[Symbol.iterator](): Generator<VectorFeature<NadGridMetadata>> {
    for (const feature of this.features) yield feature;
  }

  /**
   * @returns - true if the file is little-endian
   */
  #detectLittleEndian(): boolean {
    const { reader } = this;
    let nFields = reader.getInt32(8, false);
    if (nFields === 11) return false;
    nFields = reader.getInt32(8, true);
    if (nFields !== 11) {
      console.warn('Failed to detect nadgrid endian-ness, defaulting to little-endian');
    }
    return true;
  }

  /** Parse the main header data. Describes the subgrids to decode lon-lat */
  #readHeader() {
    const { reader, isLittleEndian } = this;
    this.#header = {
      nFields: reader.getInt32(8, isLittleEndian),
      nSubgridFields: reader.getInt32(24, isLittleEndian),
      nSubgrids: reader.getInt32(40, isLittleEndian),
      shiftType: reader.parseString(56, 8),
      fromSemiMajorAxis: reader.getFloat64(120, isLittleEndian),
      fromSemiMinorAxis: reader.getFloat64(136, isLittleEndian),
      toSemiMajorAxis: reader.getFloat64(152, isLittleEndian),
      toSemiMinorAxis: reader.getFloat64(168, isLittleEndian),
    };
  }

  /** Build all grid data */
  #readSubGrids() {
    let gridOffset = 176;
    for (let i = 0; i < this.#header.nSubgrids; i++) {
      const subHeader = this.#readSubGridHeader(gridOffset);
      const nodes = this.#readGridNodes(gridOffset, subHeader);
      const lonColumnCount = Math.round(
        1 + (subHeader.upperLongitude - subHeader.lowerLongitude) / subHeader.longitudeInterval,
      );
      const latColumnCount = Math.round(
        1 + (subHeader.upperLatitude - subHeader.lowerLatitude) / subHeader.latitudeInterval,
      );

      const feature: VectorFeature<NadGridMetadata> = {
        type: 'VectorFeature',
        properties: {},
        geometry: {
          type: 'MultiPoint',
          is3D: false,
          // CVS => lonLat coords
          coordinates: nodes.map(({ longitudeShift, latitudeShift }) => {
            return { x: secondsToDegrees(longitudeShift), y: secondsToDegrees(latitudeShift) };
          }),
        },
        metadata: {
          // ll => lowerLonLat
          lowerLonLat: {
            x: secondsToDegrees(subHeader.lowerLongitude),
            y: secondsToDegrees(subHeader.lowerLatitude),
          },
          // del => lonLatInterval
          lonLatInterval: { x: subHeader.longitudeInterval, y: subHeader.latitudeInterval },
          // lim => lonLatColumnCount
          lonLatColumnCount: { x: lonColumnCount, y: latColumnCount },
          // count => count
          count: subHeader.gridNodeCount,
        },
      };
      this.features.push(feature);
      gridOffset += 176 + subHeader.gridNodeCount * 16;
    }
  }

  /**
   * @param offset - offset to read in the subgrid header
   * @returns - the subgrid header
   */
  #readSubGridHeader(offset: number): NadSubGridHeader {
    const { reader, isLittleEndian } = this;
    return {
      name: reader.parseString(offset + 8, 8),
      parent: reader.parseString(offset + 24, 8),
      lowerLatitude: reader.getFloat64(offset + 72, isLittleEndian),
      upperLatitude: reader.getFloat64(offset + 88, isLittleEndian),
      lowerLongitude: reader.getFloat64(offset + 104, isLittleEndian),
      upperLongitude: reader.getFloat64(offset + 120, isLittleEndian),
      latitudeInterval: reader.getFloat64(offset + 136, isLittleEndian),
      longitudeInterval: reader.getFloat64(offset + 152, isLittleEndian),
      gridNodeCount: reader.getInt32(offset + 168, isLittleEndian),
    };
  }

  /**
   * @param offset - offset of the grid
   * @param gridHeader - header of the grid
   * @returns - an array of grid nodes
   */
  #readGridNodes(offset: number, gridHeader: NadSubGridHeader): GridNode[] {
    const { reader, isLittleEndian } = this;
    const nodesOffset = offset + 176;
    const gridRecordLength = 16;
    const gridShiftRecords = [];
    for (let i = 0; i < gridHeader.gridNodeCount; i++) {
      const record = {
        latitudeShift: reader.getFloat32(nodesOffset + i * gridRecordLength, isLittleEndian),
        longitudeShift: reader.getFloat32(nodesOffset + i * gridRecordLength + 4, isLittleEndian),
        latitudeAccuracy: reader.getFloat32(nodesOffset + i * gridRecordLength + 8, isLittleEndian),
        longitudeAccuracy: reader.getFloat32(
          nodesOffset + i * gridRecordLength + 12,
          isLittleEndian,
        ),
      };
      gridShiftRecords.push(record);
    }
    return gridShiftRecords;
  }
}

/**
 * @param seconds - number of seconds
 * @returns radians
 */
function secondsToDegrees(seconds: number): number {
  return seconds / 3600;
}
