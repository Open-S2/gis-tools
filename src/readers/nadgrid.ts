import { toReader } from '.';

import type { FeatureIterator, Reader, ReaderInputs } from '.';
import type {
  VectorFeature,
  VectorFeatureCollection,
  VectorMultiPoint,
  VectorPoint,
} from '../geometry';

/** Seconds to degrees (S / 3_600) */
const SEC2DEG = 0.00000484813681109536;

/** A Subgrid contained inside a NadGrid */
export interface NadSubGrid {
  cvs: VectorMultiPoint;
  ll: VectorPoint;
  del: VectorPoint;
  lim: VectorPoint;
  count: number;
}

/** A grid wrapper around a parsed .gsb file */
export interface NadGridDefinition {
  name: string;
  mandatory: boolean;
  grid?: NadGridReader;
  isNull: boolean;
}

/**
 * # NAD Grid V2 Reader
 *
 * ## Description
 * Store Grids from a NTv2 file (.gsb)
 *
 * ## Usage
 * ```ts
 * import { NadGridReader } from 'gis-tools-ts';
 * import { FileReader } from 'gis-tools-ts/file';
 * // Or use the MMapReader if using Bun:
 * // import { MMapReader } from 'gis-tools-ts/mmap';
 *
 * const store = new NadGridStore();
 *
 * // store a grid
 * store.addGridFromReader('BETA2007.gsb', new FileReader(`${__dirname}/fixtures/BETA2007.gsb`));
 *
 * // get a grid
 * const grid = store.getGrid('BETA2007.gsb');
 *
 * // get a grid given a list of names (comma separated):
 * const grids = store.getGridsFromString('BETA2007.gsb,TEST_A.gsb,TEST_B.gsb');
 * ```
 */
export class NadGridStore {
  grids = new Map<string, NadGridReader>();

  /**
   * Insert a new NadGrid into the store
   * @param grid - a nadgrid class to store
   */
  addGrid(grid: NadGridReader): void {
    this.grids.set(grid.key, grid);
  }

  /**
   * Get a grid from the store given a key or name
   * @param key - the key or name of the grid
   * @returns - the grid
   */
  getGrid(key: string): NadGridReader | undefined {
    return this.grids.get(key);
  }

  /**
   * Add a grid given a data input
   * @param key - the key or name of the grid
   * @param input - the input data to parse
   */
  addGridFromReader(key: string, input: ReaderInputs): void {
    const grid = new NadGridReader(key, input);
    this.addGrid(grid);
  }

  /**
   * Get grid definitions from a string name
   * @param keys - complex string of grid keys to test against
   * @returns - an array of grid definitions
   */
  getGridsFromString(keys?: string): NadGridDefinition[] {
    const res: NadGridDefinition[] = [];
    if (keys === undefined) return res;
    for (const grid of keys.split(',')) {
      const g = this.getGridFromString(grid);
      if (g !== undefined) res.push(g);
    }
    return res;
  }

  /**
   * Get a grid definition from a string
   * @param name - a single grid name to test against
   * @returns - a grid definition
   */
  getGridFromString(name: string): undefined | NadGridDefinition {
    if (name.length === 0) return undefined;
    const optional = name[0] === '@';
    if (optional) name = name.slice(1);
    if (name === 'null') {
      return { name: 'null', mandatory: !optional, grid: undefined, isNull: true };
    }
    return {
      name: name,
      mandatory: !optional,
      grid: this.grids.get(name),
      isNull: false,
    };
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

/**
 * # NAD Grid Reader
 *
 * ## Description
 * Loads/reads a binary NTv2 file (.gsb) implementing the {@link FeatureIterator} interface
 *
 * It should be noted that a proj4 Transformer usually uses this class internally. But if you want
 * to manually parse a .gsb file, you can use this class directly.
 *
 * ## Usage
 *
 * ```ts
 * import { NadGridReader } from 'gis-tools-ts'
 * import { FileReader } from 'gis-tools-ts/file';
 * // or use the MMapReader if using Bun:
 * // import { MMapReader } from 'gis-tools-ts/mmap';
 *
 * const reader = new NadGridReader('BETA2007.gsb', new FileReader('./BETA2007.gsb'));
 *
 * // access all the vector features
 * const data = await Array.fromAsync(reader);
 * ```
 *
 * ## Links
 * - https://web.archive.org/web/20140127204822if_/http://www.mgs.gov.on.ca:80/stdprodconsume/groups/content/@mgs/@iandit/documents/resourcelist/stel02_047447.pdf
 * - http://mimaka.com/help/gs/html/004_NTV2%20Data%20Format.htm
 */
export class NadGridReader implements FeatureIterator<NadGridMetadata> {
  reader: Reader;
  #isLittleEndian = false;
  #header!: NadGridHeader;
  subgrids: NadSubGrid[] = [];

  /**
   * @param key - the key or name of the grid
   * @param input - the input data to parse
   */
  constructor(
    public key: string,
    input: ReaderInputs,
  ) {
    this.reader = toReader(input);
    this.#isLittleEndian = this.#detectLittleEndian();
    this.#readHeader();
    this.#readSubGrids();
  }

  /** @returns - The header describing how to decode the feature */
  get header(): NadGridHeader {
    return { ...this.#header };
  }

  /**
   * Return all the features in the shapefile
   * @returns - a collection of VectorFeatures
   */
  getFeatureCollection(): VectorFeatureCollection<NadGridMetadata> {
    const features = this.subgrids.map(this.#subGrideToVectorFeature);
    return { type: 'FeatureCollection', features };
  }

  /**
   * Iterate over all features in the shapefile
   * @yields {VectorFeature<NadGridMetadata>}
   */
  async *[Symbol.asyncIterator](): AsyncGenerator<VectorFeature<NadGridMetadata>> {
    for (const subgrid of this.subgrids) yield this.#subGrideToVectorFeature(subgrid);
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
  #readHeader(): void {
    const { reader } = this;
    const le = this.#isLittleEndian;
    this.#header = {
      nFields: reader.getInt32(8, le),
      nSubgridFields: reader.getInt32(24, le),
      nSubgrids: reader.getInt32(40, le),
      shiftType: reader.parseString(56, 8),
      fromSemiMajorAxis: reader.getFloat64(120, le),
      fromSemiMinorAxis: reader.getFloat64(136, le),
      toSemiMajorAxis: reader.getFloat64(152, le),
      toSemiMinorAxis: reader.getFloat64(168, le),
    };
  }

  /** Build all grid data */
  #readSubGrids(): void {
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

      const subGrid: NadSubGrid = {
        cvs: nodes.map(({ longitudeShift, latitudeShift }) => {
          return { x: longitudeShift * SEC2DEG, y: latitudeShift * SEC2DEG };
        }),
        ll: {
          x: subHeader.lowerLongitude * SEC2DEG,
          y: subHeader.lowerLatitude * SEC2DEG,
        },
        del: { x: subHeader.longitudeInterval * SEC2DEG, y: subHeader.latitudeInterval * SEC2DEG },
        lim: { x: lonColumnCount, y: latColumnCount },
        count: subHeader.gridNodeCount,
      };

      this.subgrids.push(subGrid);
      gridOffset += 176 + subHeader.gridNodeCount * 16;
    }
  }

  /**
   * @param subGrid - the subgrid to convert to a vector feature
   * @returns - the vector feature
   */
  #subGrideToVectorFeature(subGrid: NadSubGrid): VectorFeature<NadGridMetadata> {
    const { cvs, ll, del, lim, count } = subGrid;
    return {
      type: 'VectorFeature',
      properties: {},
      geometry: {
        type: 'MultiPoint',
        is3D: false,
        // CVS => lonLat coords
        coordinates: cvs,
      },
      metadata: {
        // ll => lowerLonLat
        lowerLonLat: ll,
        // del => lonLatInterval
        lonLatInterval: del,
        // lim => lonLatColumnCount
        lonLatColumnCount: lim,
        // count => count
        count,
      },
    };
  }

  /**
   * @param offset - offset to read in the subgrid header
   * @returns - the subgrid header
   */
  #readSubGridHeader(offset: number): NadSubGridHeader {
    const { reader } = this;
    const le = this.#isLittleEndian;
    return {
      name: reader.parseString(offset + 8, 8),
      parent: reader.parseString(offset + 24, 8),
      lowerLatitude: reader.getFloat64(offset + 72, le),
      upperLatitude: reader.getFloat64(offset + 88, le),
      lowerLongitude: reader.getFloat64(offset + 104, le),
      upperLongitude: reader.getFloat64(offset + 120, le),
      latitudeInterval: reader.getFloat64(offset + 136, le),
      longitudeInterval: reader.getFloat64(offset + 152, le),
      gridNodeCount: reader.getInt32(offset + 168, le),
    };
  }

  /**
   * @param offset - offset of the grid
   * @param gridHeader - header of the grid
   * @returns - an array of grid nodes
   */
  #readGridNodes(offset: number, gridHeader: NadSubGridHeader): GridNode[] {
    const { reader } = this;
    const le = this.#isLittleEndian;
    const nodesOffset = offset + 176;
    const gridRecordLength = 16;
    const gridShiftRecords = [];
    for (let i = 0; i < gridHeader.gridNodeCount; i++) {
      const record = {
        latitudeShift: reader.getFloat32(nodesOffset + i * gridRecordLength, le),
        longitudeShift: reader.getFloat32(nodesOffset + i * gridRecordLength + 4, le),
        latitudeAccuracy: reader.getFloat32(nodesOffset + i * gridRecordLength + 8, le),
        longitudeAccuracy: reader.getFloat32(nodesOffset + i * gridRecordLength + 12, le),
      };
      gridShiftRecords.push(record);
    }
    return gridShiftRecords;
  }
}
