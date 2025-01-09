import { toReader } from '..';

import type { FeatureIterator, Reader, ReaderInputs } from '..';
import type { MValue, Properties, VectorFeature, VectorPoint } from '../../geometry';

/** The kind of data that can be stored in a NetCDF file */
export type CDFValue = string | number | number[];

/** The kind of attributes that can be stored in a NetCDF file. Similar to a GeoJSON Properties object */
export type CDFAttributes = Record<string, CDFValue>;

/** Track the dimension and its max value (can be infinity) */
export interface CDFDimension {
  /** index of the dimension */
  index: number;
  /** name of the dimension */
  name: string;
  /** size of the dimension */
  size: number;
}

/** Track information about the dimensions, which is "unlimited" dimension, and variable sizes */
export interface CDFRecordDimension {
  /** Length of the record dimension sum of the varSize's of all the record variables */
  size: number;
  id?: number;
  name?: string;
  recordStep?: number;
}

/** A NetCDF variable */
export interface CDFVariable {
  /** name of the variable */
  name: string;
  /** Array with the dimension IDs of the variable */
  dimensions: CDFDimension[];
  /** Array with the attributes of the variable */
  attributes: CDFAttributes;
  /** type of the variable */
  type: CDFDataType;
  /** size of the variable */
  size: number;
  /** offset where of the variable begins */
  offset: number;
  /** True if is a record variable, false otherwise (unlimited size) */
  record: boolean;
}

// Grammar constants
const NC_UNLIMITED = 0;
const NC_DIMENSION = 10;
const NC_VARIABLE = 11;
const NC_ATTRIBUTE = 12;

/** Enum of the NetCDF data types available */
export const enum CDFDataType {
  /** Byte size (1 byte) */
  BYTE = 1,
  /** Char size (1 byte) */
  CHAR = 2,
  /** Short size (2 bytes) */
  SHORT = 3,
  /** Integer size (4 bytes) */
  INT = 4,
  /** Float size (4 bytes) */
  FLOAT = 5,
  /** Double size (8 bytes) */
  DOUBLE = 6,
}

/**
 * @param type - the NetCDF data type
 * @returns the number of bytes for the data type
 */
function typeToBytes(type: CDFDataType): number {
  switch (type) {
    case CDFDataType.BYTE:
    case CDFDataType.CHAR:
      return 1;
    case CDFDataType.SHORT:
      return 2;
    case CDFDataType.INT:
    case CDFDataType.FLOAT:
      return 4;
    case CDFDataType.DOUBLE:
      return 8;
    default:
      return -1;
  }
}

/** User defined options on how to parse the CSV file */
export interface NetCDFReaderOptions {
  /** If provided the lookup of the longitude [Default='lon'] */
  lonKey?: string;
  /** If provided the lookup of the latitude [Default='lat'] */
  latKey?: string;
  /** If provided the lookup for the height value [Default=undefined] */
  heightKey?: string;
  /** List of fields to include in the feature properties */
  propFields?: string[];
}

/**
 * # NetCDF v3.x Reader
 *
 * ## Description
 * Read the NetCDF v3.x file format
 * [See specification](https://www.unidata.ucar.edu/software/netcdf/docs/file_format_specifications.html)
 * Implements the {@link FeatureIterator} interface
 *
 * ## Usage
 * ```ts
 * import { NetCDFReader } from 'gis-tools';
 * import { FileReader } from 'gis-tools/file';
 *
 * const reader = new NetCDFReader(new FileReader('./data.nc'));
 * for (const feature of reader) {
 *   console.log(feature);
 * }
 * ```
 */
export class NetCDFReader<
  M = Record<string, unknown>,
  D extends MValue = MValue,
  P extends Properties = Properties,
> implements FeatureIterator<M, D, P>
{
  private reader: Reader;
  readonly recordDimension: CDFRecordDimension = { size: 0 };
  /** List of dimensions */
  readonly dimensions: CDFDimension[] = [];
  /** List of global attributes */
  globalAttributes: CDFAttributes = {};
  /** List of variables */
  readonly variables: CDFVariable[] = [];
  /** Describes if offsets are 32 or 64 bits */
  readonly is64: boolean;
  /** Track the cursor for parsing the header */
  #cursor = 4;
  #lonKey = 'lon';
  #latKey = 'lat';
  #heightKey?: string;
  #propFields: string[];

  /**
   * @param input - The data as either a buffer or file reader
   * @param options - User defined options to apply when reading the NetCDF file
   */
  constructor(input: ReaderInputs, options?: NetCDFReaderOptions) {
    this.reader = toReader(input);
    // Validate that it's a NetCDF file
    const magic = this.reader.parseString(0, 3);
    if (magic !== 'CDF') throw new TypeError('Not a valid NetCDF file: should start with CDF');
    // Check the NetCDF format
    this.is64 = this.reader.getUint8(3) === 1 ? false : true;
    // Read the header
    this.#parseHeader();

    this.#lonKey = options?.lonKey ?? 'lon';
    this.#latKey = options?.latKey ?? 'lat';
    this.#heightKey = options?.heightKey;
    this.#propFields = options?.propFields ?? [];
  }

  /**
   * Retrieves the data for a given variable
   * @param variableName - Name of the variable to search or variable object
   * @returns The variable values
   */
  getDataVariable(variableName: string): CDFValue[] | undefined {
    const variable = this.variables.find((val) => {
      return val.name === variableName;
    });
    // return nothing if not found
    if (variable === undefined) return undefined;
    // go to the offset position
    this.#cursor = variable.offset;
    // return the data
    if (variable.record) {
      return this.#getRecord(variable);
    } else {
      return this.#getNonRecord(variable);
    }
  }

  /**
   * Generator to iterate over each (Geo|S2)JSON object in the file
   * @yields {VectorFeature}
   */
  async *[Symbol.asyncIterator](): AsyncGenerator<VectorFeature<M, D, P>> {
    const lat = this.getDataVariable(this.#latKey)?.flat() as number[] | undefined;
    const lon = this.getDataVariable(this.#lonKey)?.flat() as number[] | undefined;
    const height =
      this.#heightKey !== undefined
        ? (this.getDataVariable(this.#heightKey)?.flat() as number[] | undefined)
        : undefined;
    const fieldProps: { [field: string]: CDFValue[] } = {};
    for (const field of this.#propFields)
      fieldProps[field] = this.getDataVariable(field)?.flat() ?? [];
    if (lat === undefined || lon === undefined) return;
    for (let index = 0; index < lat.length; index++) {
      const point: VectorPoint<D> = { x: lon[index], y: lat[index], z: height?.[index] };
      const properties: Properties = {};
      for (const field of this.#propFields) properties[field] = fieldProps[field][index];
      yield {
        type: 'VectorFeature',
        geometry: {
          type: 'Point',
          is3D: point.z !== undefined,
          coordinates: point,
        },
        properties: properties as P,
      };
    }
  }

  /**
   * Internal method to Parse the header
   */
  #parseHeader(): void {
    // build dimension list
    this.recordDimension.size = this.#getU32();
    this.#buildDimensionList();
    // build global attributes
    this.globalAttributes = this.#buildAttributes();
    // build the variable list
    this.#buildVariablesList();
  }

  /**
   * Internal method to build the dimension list
   */
  #buildDimensionList() {
    const dimListTag = this.#getU32();

    if (dimListTag === 0) {
      const ensureEmpty = this.#getU32();
      if (ensureEmpty !== 0) throw new TypeError('wrong empty tag for list of dimensions');
    } else {
      if (dimListTag !== NC_DIMENSION) throw new TypeError('wrong tag for list of dimensions');

      // Length of dimensions
      const dimensionSize = this.#getU32();
      //populate `name` and `size` for each dimension
      for (let index = 0; index < dimensionSize; index++) {
        // Read name
        const name = this.#getName();
        // Read dimension size
        const size = this.#getU32();
        if (size === NC_UNLIMITED) {
          // in netcdf 3 one field can be of size unlimited
          this.recordDimension.id = index;
          this.recordDimension.name = name;
        }
        // store the dimension
        this.dimensions.push({
          index,
          name,
          size,
        });
      }
    }
  }

  /**
   * Internal method to build attributes including global attributes
   * @returns - attributes from a block of data at a given offset
   */
  #buildAttributes(): CDFAttributes {
    const atrributes: CDFAttributes = {};
    const gAttTag = this.#getU32();
    if (gAttTag === 0) {
      const ensureEmpty = this.#getU32();
      if (ensureEmpty !== 0) throw new TypeError('wrong empty tag for list of attributes');
    } else {
      if (gAttTag !== NC_ATTRIBUTE) throw new TypeError('wrong tag for list of attributes');
      // Length of attributes
      const attributeSize = this.#getU32();
      // Populate `name`, `type` and `value` for each attribute
      for (let gaIdx = 0; gaIdx < attributeSize; gaIdx++) {
        // Read name, type, and size of data block
        const name = this.#getName();
        const type = this.#getU32() as CDFDataType;
        const size = this.#getU32();
        // store the attribute key-value
        atrributes[name] = this.#getType(type, size);
      }
    }

    return atrributes;
  }

  /**
   * Internal method to build a variable list from a block of data at a given offset
   */
  #buildVariablesList(): void {
    const varTag = this.#getU32();
    let recordStep = 0;
    if (varTag === 0) {
      const ensureEmpty = this.#getU32();
      if (ensureEmpty !== 0) throw new TypeError('wrong empty tag for list of variables');
    } else {
      if (varTag !== NC_VARIABLE) throw new TypeError('wrong tag for list of variables');
      // Length of variables
      const varSize = this.#getU32();
      for (let vIdx = 0; vIdx < varSize; vIdx++) {
        // Read name, dimensionality, and index into the list of dimensions
        const name = this.#getName();
        const dimensionality = this.#getU32();
        const dimensionsIds: number[] = [];
        for (let dim = 0; dim < dimensionality; dim++) dimensionsIds.push(this.#getU32());
        // Read variables size
        const attributes = this.#buildAttributes();
        // Read type
        const type = this.#getU32() as CDFDataType;
        // Read variable size
        // The 32-bit varSize field is not large enough to contain the size of variables that require
        // more than 2^32 - 4 bytes, so 2^32 - 1 is used in the varSize field for such variables.
        const varSize = this.#getU32();
        // Read offset
        const offset = this.#getOffset();
        let record = false;
        // Count amount of record variables
        if (dimensionsIds.length > 0 && dimensionsIds[0] === this.recordDimension.id) {
          recordStep += varSize;
          record = true;
        }
        this.variables.push({
          name,
          dimensions: dimensionsIds.map((id) => this.dimensions[id]),
          attributes,
          type,
          size: varSize,
          offset,
          record,
        });
      }
    }
    this.recordDimension.recordStep = recordStep;
  }

  /**
   * Internal method to get the current offset
   * @returns - the current offset
   */
  #getOffset(): number {
    if (this.is64) return Number(this.#getU64());
    return this.#getU32();
  }

  /**
   * Internal method to get a 32 but value under the cursor
   * @returns - a 32 bit value
   */
  #getU32() {
    const data = this.reader.getUint32(this.#cursor);
    this.#cursor += 4;
    return data;
  }

  /**
   * Internal method to get a 64 but value under the cursor
   * @returns - a 64 bit value
   */
  #getU64() {
    const data = this.reader.getBigUint64(this.#cursor);
    this.#cursor += 8;
    return data;
  }

  /**
   * Internal method to read a string under the cursor
   * @returns - a string
   */
  #getName() {
    const nameLength = this.#getU32();
    const name = this.reader.parseString(this.#cursor, nameLength);
    this.#cursor += nameLength;
    this.#padding();
    return name;
  }

  /**
   * @param type - the data type
   * @param size - the data size
   * @returns - the data
   */
  #getType(type: CDFDataType, size: number): string | number | number[] {
    let res: string | number | number[];
    if (type === CDFDataType.BYTE) {
      res = [];
      for (let i = 0; i < size; i++) {
        res.push(this.reader.getUint8(this.#cursor));
        this.#cursor++;
      }
    } else if (type === CDFDataType.CHAR) {
      res = this.reader.parseString(this.#cursor, size);
      this.#cursor += size;
    } else if (
      type === CDFDataType.SHORT ||
      type === CDFDataType.INT ||
      type === CDFDataType.FLOAT ||
      type === CDFDataType.DOUBLE
    ) {
      const step = type === CDFDataType.DOUBLE ? 8 : type === CDFDataType.SHORT ? 2 : 4;
      const readNumber: (byteOffset: number) => number =
        type === CDFDataType.SHORT
          ? this.reader.getInt16.bind(this.reader)
          : type === CDFDataType.INT
            ? this.reader.getInt32.bind(this.reader)
            : type === CDFDataType.FLOAT
              ? this.reader.getFloat32.bind(this.reader)
              : this.reader.getFloat64.bind(this.reader);
      res = [];
      for (let i = 0; i < size; i++) {
        res.push(readNumber(this.#cursor));
        this.#cursor += step;
      }
      if (res.length === 1) res = res[0];
    } else {
      throw new Error(`non valid type ${type}`);
    }

    this.#padding();
    return res;
  }

  /**
   * Read data for the given non-record variable
   * @param variable - Variable metadata
   * @returns - Data of the element
   */
  #getNonRecord(variable: CDFVariable): CDFValue[] {
    // variable type
    const { size, type } = variable;
    // size of the data
    const totalSize = size / typeToBytes(type);
    // iterates over the data
    const data: CDFValue[] = [];
    for (let i = 0; i < totalSize; i++) data.push(this.#getType(type, 1));

    return data;
  }

  /**
   * Read data for the given record variable
   * @param variable - Variable metadata
   * @returns - Data of the element
   */
  #getRecord(variable: CDFVariable): CDFValue[] {
    // prep variables
    const { recordStep, size: totalSize } = this.recordDimension;
    const { size, type } = variable;
    const width = size !== 0 ? size / typeToBytes(type) : 1;
    // TODO streaming data

    if (recordStep === undefined) throw new Error('recordDimension.recordStep is undefined');

    // iterates over the data
    const data: CDFValue[] = [];
    for (let i = 0; i < totalSize; i++) {
      const currentOffset = this.#cursor;
      data.push(this.#getType(type, width));
      this.#cursor = currentOffset + recordStep;
    }

    return data;
  }

  /** Apply padding as data is mapped to 4-byte alignment */
  #padding(): void {
    if (this.#cursor % 4 !== 0) this.#cursor += 4 - (this.#cursor % 4);
  }
}
