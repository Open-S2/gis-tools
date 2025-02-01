// https://docs.ogc.org/is/19-008r4/19-008r4.html#_requirements_class_tiff
import { ARRAY_FIELDS, FIELD_TAG_NAMES, FIELD_TYPES, GEO_KEY_NAMES } from './constants';

import type { Reader } from '..';
import type { GeoKeyDirectory, TagNames } from './constants';

/**
 * Container to store the parsed file directory, geo key directory and
 * offset to the next IFD
 */
export interface ImageFileDirectory extends TagNames {
  GeoKeyDirectory?: GeoKeyDirectory;
  ModelPixelScale?: number[];
  ModelTiepoint?: number[];
  ModelTransformation?: number[];
  GeoDoubleParams?: number[];
  GeoAsciiParams?: string;
  pixelScale?: [x: number, y: number, z: number];
  tiepoint?: number[];
}

/** A key value pair */
interface KeyValue {
  key: keyof ImageFileDirectory;
  value: undefined | number | number[] | string;
}

/**
 * GeoTIFF Header Reader
 */
export class GeoTIFFHeaderReader {
  #littleEndian = true;
  #bigTiff = false;
  imageDirectories: ImageFileDirectory[] = [];
  /** @param reader - the geotiff reader to parse data from */
  constructor(public reader: Reader) {
    this.#readheader();
  }

  /** @returns - the number of images in the GeoTIFF */
  get length(): number {
    return this.imageDirectories.length;
  }

  /** @returns - the littleEndian flag */
  get littleEndian(): boolean {
    return this.#littleEndian;
  }

  /** @returns - the bigTIFF flag */
  get bigTiff(): boolean {
    return this.#bigTiff;
  }

  /** parses the header data to begin parsing the GeoTIFF */
  #readheader() {
    const { reader } = this;
    // pull the endianess from the header
    const BOM = reader.getUint16(0, false);
    if (BOM === 0x4949) {
      this.#littleEndian = true;
    } else if (BOM === 0x4d4d) {
      this.#littleEndian = false;
    } else {
      throw new TypeError('Invalid byte order value.');
    }

    const magicNumber = reader.getUint16(2, this.littleEndian);
    if (magicNumber === 42) {
      this.#bigTiff = false;
    } else if (magicNumber === 43) {
      this.#bigTiff = true;
      const offsetByteSize = reader.getUint16(4, this.littleEndian);
      if (offsetByteSize !== 8) {
        throw new Error('Unsupported offset byte-size.');
      }
    } else {
      throw new TypeError('Invalid magic number.');
    }

    const firstIFDOffset = this.bigTiff
      ? Number(reader.getBigUint64(8, this.littleEndian))
      : reader.getUint32(4, this.littleEndian);

    this.#getImageMetadata(firstIFDOffset);
  }

  /**
   * Reads the value of the tag at the given offset (16 bits if not bigTIFF)
   * @param offset - the offset to read the tag from
   * @returns - the value of the tag
   */
  #readTag(offset: number): number {
    const { reader, bigTiff, littleEndian } = this;
    return bigTiff
      ? Number(reader.getBigUint64(offset, littleEndian))
      : reader.getUint16(offset, littleEndian);
  }

  /**
   * Reads the value of the tag at the given offset (32 bits if not bigTIFF)
   * @param offset - the offset to read the tag from
   * @returns - the value of the tag
   */
  #readOffset(offset: number): number {
    const { reader, bigTiff, littleEndian } = this;
    return bigTiff
      ? Number(reader.getBigUint64(offset, littleEndian))
      : reader.getUint32(offset, littleEndian);
  }

  /**
   * Instructs to parse an image file directory at the given file offset.
   * As there is no way to ensure that a location is indeed the start of an IFD,
   * this function must be called with caution (e.g only using the IFD offsets from
   * the headers or other IFDs).
   * @param firstOffset - the offset to begin parsing the IFDs (Image File Directory) at.
   */
  #getImageMetadata(firstOffset: number): void {
    const { reader, bigTiff, littleEndian } = this;
    const entrySize = bigTiff ? 20 : 12;
    const offsetSize = bigTiff ? 8 : 2;
    let offset = firstOffset;

    let ifdOffset = firstOffset;
    while (ifdOffset !== 0) {
      const ifd: ImageFileDirectory = {};
      const numDirEntries = this.#readTag(offset);

      let i = offset + offsetSize;
      let geokeyDirOffset: number | undefined = undefined;
      let prevTag = 0;
      for (let entryCount = 0; entryCount < numDirEntries; i += entrySize, entryCount++) {
        const fieldTag = reader.getUint16(i, littleEndian);
        if (fieldTag < prevTag) throw new Error(`Invalid IFD, ${fieldTag} < ${prevTag}`);
        prevTag = fieldTag;
        if (fieldTag === 33550) {
          // PixelScaleTag
          ifd.pixelScale = this.#getPixelScale(i);
        } else if (fieldTag === 33922) {
          // TiepointTag
          ifd.tiepoint = this.#getTiepoint(i);
        } else if (fieldTag === 34735) {
          // GeoKeyDirectory - map to use after all keys are cached.
          geokeyDirOffset = i;
        } else {
          const { key, value } = this.#getKeyValue(fieldTag, i);
          // @ts-expect-error - its ok to set the key-value pair.
          ifd[key] = value;
        }
        // NOTE: Technically geotiffs support column encoding of double and ascii values. Seems like it's not common enough to use though
        // else if (fieldTag === 34736) {
        //   // location of DoubleValues
        // } else if (fieldTag === 34737) {
        //   // location of ASCIIValues
        // }
      }
      // Validate it has a TransformationTag or a TiepointTag before storing
      if (geokeyDirOffset === undefined)
        console.info('No GeoKeyDirectory found. May contain errors');
      else ifd.GeoKeyDirectory = this.#getGeoKeyDirectory(geokeyDirOffset, ifd);
      if (ifd.tiepoint === undefined && ifd.ModelTransformation === undefined)
        console.info('No ModelTiepoint or ModelTransformation found. May contain errors');
      if (Object.keys(ifd).length > 0) this.imageDirectories.push(ifd);
      else break;
      // increment offset and check for the next IFD
      // 814
      offset += offsetSize + entrySize * numDirEntries;
      ifdOffset = this.#readTag(offset);
      offset += offsetSize;
    }
  }

  /**
   * Get the pixel scale from the GeoKeyDirectory
   * @param offset - the offset to begin parsing the IFDs (GeoKeyDirectory) at.
   * @returns the parsed GeoKeyDirectory
   */
  #getPixelScale(offset: number): [x: number, y: number, z: number] {
    const { reader, littleEndian, bigTiff } = this;
    const fieldType = reader.getUint16(offset + 2, littleEndian);
    if (fieldType !== 12)
      throw new Error(`Invalid GeoKeyDirectory type ${reader.getUint16(offset + 2)}`);
    const numKeys = this.#readOffset(offset + 4);
    if (numKeys !== 3) throw new Error(`Invalid GeoKeyDirectory numKeys ${numKeys}`);
    const valueOffset = this.#readOffset(offset + (bigTiff ? 12 : 8));

    const xscale = reader.getFloat64(valueOffset, littleEndian);
    const yscale = reader.getFloat64(valueOffset + 8, littleEndian);
    const zscale = reader.getFloat64(valueOffset + 16, littleEndian);

    return [xscale, yscale, zscale];
  }

  /**
   * https://docs.ogc.org/is/19-008r4/19-008r4.html#_geokey_directory_test
   * @param offset - the offset to begin parsing the IFDs (GeoKeyDirectory) at.
   * @param fileDir - the parsed ImageFileDirectory thus far
   * @returns the parsed GeoKeyDirectory
   */
  #getGeoKeyDirectory(offset: number, fileDir: ImageFileDirectory): GeoKeyDirectory {
    const { reader, bigTiff } = this;

    const numKeys = this.#readOffset(offset + 4);
    const valueOffset = this.#readOffset(offset + (bigTiff ? 12 : 8));
    const rawGeoKeys = new Uint16Array(reader.slice(valueOffset, valueOffset + numKeys * 2).buffer);
    const GeoKeyDirectory = parseGeotiffRawGeoKeys(rawGeoKeys, fileDir);
    // Validate that there is a GTModelType GeoKey in the GeoKey Directory
    if (GeoKeyDirectory.GTModelTypeGeoKey === undefined) {
      throw new Error(`Missing "GTModelTypeGeoKey" in GeoKeyDirectory`);
    }

    return GeoKeyDirectory;
  }

  /**
   * @param offset - the offset to begin parsing the IFDs (TiepointTag) at.
   * @returns the parsed Tiepoint
   */
  #getTiepoint(offset: number): number[] {
    const { reader, bigTiff, littleEndian } = this;
    // Validate that Bytes 2-3 = 12 (Double)
    const fieldType = reader.getUint16(offset + 2, littleEndian);
    if (fieldType !== 12) throw new Error(`Invalid TiepointTag type ${fieldType}`);
    // get size to the value in Bytes 4-7
    const count = this.#readOffset(offset + 4);
    // Set TagValue to the value in Bytes 8-11
    const valueOffset = this.#readOffset(offset + (bigTiff ? 12 : 8));
    const tiepoint: number[] = [];
    for (let i = 0; i < count; i++) {
      tiepoint.push(reader.getFloat64(valueOffset + i * 8, littleEndian));
    }
    return tiepoint;
  }

  /**
   * @param fieldTag - the tag to read
   * @param offset - the current offset in the IFD header data
   * @returns the parsed key value
   */
  #getKeyValue(fieldTag: number, offset: number): KeyValue {
    const { reader, littleEndian } = this;
    const fieldType = reader.getUint16(offset + 2, littleEndian);
    const typeCount = this.#readOffset(offset + 4);

    const fieldTypeLength = getFieldTypeLength(fieldType as keyof typeof FIELD_TAG_NAMES);
    const valueOffset = offset + (this.bigTiff ? 12 : 8);
    const actualOffset =
      fieldTypeLength * typeCount <= (this.bigTiff ? 8 : 4)
        ? valueOffset
        : this.#readOffset(valueOffset);
    const value = this.#getValue(fieldTag, fieldType, typeCount, actualOffset);

    // write the tags value to the file directly
    return {
      key: FIELD_TAG_NAMES[fieldTag as keyof typeof FIELD_TAG_NAMES] as keyof ImageFileDirectory,
      value,
    };
  }

  /**
   * @param fieldTag - the tag to read
   * @param fieldType - the field type
   * @param typeCount - the number of values
   * @param valueOffset - the value offset
   * @returns - the parsed value
   */
  #getValue(
    fieldTag: number,
    fieldType: number,
    typeCount: number,
    valueOffset: number,
  ): undefined | number | number[] | string {
    const { reader, littleEndian } = this;
    const res: number[] = [];
    // console.log('GET VALUE', fieldTag, fieldType, typeCount, valueOffset);
    if (fieldType === FIELD_TYPES.ASCII) {
      return reader.parseString(valueOffset, typeCount);
    } else if (fieldType === FIELD_TYPES.BYTE || fieldType === FIELD_TYPES.UNDEFINED) {
      for (let i = 0; i < typeCount; i++) res.push(reader.getUint8(valueOffset + i));
    } else if (fieldType === FIELD_TYPES.SBYTE) {
      for (let i = 0; i < typeCount; i++) res.push(reader.getInt8(valueOffset + i));
    } else if (fieldType === FIELD_TYPES.SHORT) {
      for (let i = 0; i < typeCount; i++)
        res.push(reader.getUint16(valueOffset + i * 2, littleEndian));
    } else if (fieldType === FIELD_TYPES.SSHORT) {
      for (let i = 0; i < typeCount; i++)
        res.push(reader.getInt16(valueOffset + i * 2, littleEndian));
    } else if (fieldType === FIELD_TYPES.LONG) {
      for (let i = 0; i < typeCount; i++)
        res.push(reader.getUint32(valueOffset + i * 4, littleEndian));
    } else if (fieldType === FIELD_TYPES.SLONG) {
      for (let i = 0; i < typeCount; i++)
        res.push(reader.getInt32(valueOffset + i * 4, littleEndian));
    } else if (fieldType === FIELD_TYPES.FLOAT) {
      for (let i = 0; i < typeCount; i++)
        res.push(reader.getFloat32(valueOffset + i * 4, littleEndian));
    } else if (fieldType === FIELD_TYPES.RATIONAL) {
      typeCount *= 2;
      for (let i = 0; i < typeCount; i += 2) {
        res.push(reader.getUint32(valueOffset + i * 4, littleEndian));
        res.push(reader.getUint32(valueOffset + i * 4 + 4, littleEndian));
      }
    } else if (fieldType === FIELD_TYPES.SRATIONAL) {
      typeCount *= 2;
      for (let i = 0; i < typeCount; i += 2) {
        res.push(reader.getInt32(valueOffset + i * 4, littleEndian));
        res.push(reader.getInt32(valueOffset + i * 4 + 4, littleEndian));
      }
    } else if (fieldType === FIELD_TYPES.DOUBLE) {
      for (let i = 0; i < typeCount; i++)
        res.push(reader.getFloat64(valueOffset + i * 8, littleEndian));
    } else if (fieldType === FIELD_TYPES.LONG8) {
      for (let i = 0; i < typeCount; i++)
        res.push(Number(reader.getBigUint64(valueOffset + i * 8, littleEndian)));
    } else if (fieldType === FIELD_TYPES.SLONG8) {
      for (let i = 0; i < typeCount; i++)
        res.push(Number(reader.getBigInt64(valueOffset + i * 8, littleEndian)));
    }

    // unpack single values from the array
    if (
      typeCount === 1 &&
      ARRAY_FIELDS.indexOf(fieldTag) === -1 &&
      !(fieldType === FIELD_TYPES.RATIONAL || fieldType === FIELD_TYPES.SRATIONAL)
    ) {
      return res[0];
    } else {
      return res;
    }
  }
}

/**
 * Get the field type length
 * @param fieldType - the field type
 * @returns - the field type length
 */
function getFieldTypeLength(fieldType: keyof typeof FIELD_TAG_NAMES): 1 | 2 | 4 | 8 {
  switch (fieldType) {
    case FIELD_TYPES.BYTE:
    case FIELD_TYPES.ASCII:
    case FIELD_TYPES.SBYTE:
    case FIELD_TYPES.UNDEFINED:
      return 1;
    case FIELD_TYPES.SHORT:
    case FIELD_TYPES.SSHORT:
      return 2;
    case FIELD_TYPES.LONG:
    case FIELD_TYPES.SLONG:
    case FIELD_TYPES.FLOAT:
    case FIELD_TYPES.IFD:
      return 4;
    case FIELD_TYPES.RATIONAL:
    case FIELD_TYPES.SRATIONAL:
    case FIELD_TYPES.DOUBLE:
    case FIELD_TYPES.LONG8:
    case FIELD_TYPES.SLONG8:
    case FIELD_TYPES.IFD8:
      return 8;
    default:
      throw new RangeError(`Invalid field type: ${fieldType}`);
  }
}

/**
 * Parse the raw geo keys
 * @param rawGeoKeys - the raw geo keys
 * @param fileDir - the image file directory
 * @returns - the parsed geo keys
 */
export function parseGeotiffRawGeoKeys(
  rawGeoKeys: Uint16Array,
  fileDir: ImageFileDirectory,
): GeoKeyDirectory {
  const GeoKeyDirectory: GeoKeyDirectory = {};
  for (let i = 4; i <= rawGeoKeys[3] * 4; i += 4) {
    const geoKey = rawGeoKeys[i];
    const key = GEO_KEY_NAMES[geoKey as keyof typeof GEO_KEY_NAMES];
    const location =
      rawGeoKeys[i + 1] !== 0
        ? FIELD_TAG_NAMES[rawGeoKeys[i + 1] as keyof typeof FIELD_TAG_NAMES]
        : null;
    const count = rawGeoKeys[i + 2];
    const offset = rawGeoKeys[i + 3];

    let value: null | string | number | number[] = null;
    if (location === null) {
      value = offset;
    } else {
      value = fileDir[location as keyof ImageFileDirectory] as string | number | number[];
      if (typeof value === 'undefined' || value === null) {
        throw new Error(`Could not get value of geoKey '${key}' at location '${location}'.`);
      } else if (typeof value === 'string') {
        value = value.substring(offset, offset + count - 1);
      } else if (Array.isArray(value)) {
        value = value.slice(offset, offset + count);
        if (count === 1) value = value[0];
      }
    }
    // @ts-expect-error - value assignment is ok here
    GeoKeyDirectory[key] = value;
  }
  return GeoKeyDirectory;
}
