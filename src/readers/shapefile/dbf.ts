import type { Properties } from 's2-tools/geometry';
import type { Reader } from '..';

/** The Header data explaining the contents of the DBF file */
export interface DBFHeader {
  /** The last updated date */
  lastUpdated: Date;
  /** The number of records */
  records: number;
  /** The length of the header data */
  headerLen: number;
  /** The length of each row */
  recLen: number;
}

/** Each row is a key definition to build the properties for each column */
export interface DBFRow {
  /** The name of the row */
  name: string;
  /** The data type of the row */
  dataType: string;
  /** The length of the row */
  len: number;
  /** The decimal places of the row */
  decimal: number;
}

/** A DBF file class to parse the data from a DBF */
export default class DataBaseFile {
  #header!: DBFHeader;
  #rows: DBFRow[];
  textDecoder: TextDecoder;

  /**
   * @param reader - the input data structure to parse
   * @param encoding - the encoding of the raw data. defaults to 'utf-8'
   */
  constructor(
    public reader: Reader,
    encoding = 'utf-8',
  ) {
    // @ts-expect-error - linting bug
    this.textDecoder = new TextDecoder(encoding);
    this.#parseHeader();
    this.#rows = this.#parseRowHeader();
  }

  /**
   * Create a copy of the header data
   * @returns - a copy of the header
   */
  getHeader(): DBFHeader {
    return { ...this.#header };
  }

  /**
   * @param index - the index of the properties data we want
   * @returns - the properties for the given index
   */
  getProperties(index: number): Properties | undefined {
    const { records, recLen } = this.#header;
    if (index > records) return undefined;
    const offset = ((this.#rows.length + 1) << 5) + 2 + index * recLen;
    return this.#parseProperties(offset);
  }

  /**
   * Get all the properties in the DBF
   * @returns - an array of Properties
   */
  getAllProperties(): Properties[] {
    const { records } = this.#header;
    const res: Properties[] = [];
    for (let i = 0; i < records; i++) {
      const properties = this.getProperties(i);
      if (properties) res.push(properties);
    }
    return res;
  }

  /**
   * Parse the header and store it in the class
   */
  #parseHeader() {
    const { reader } = this;
    this.#header = {
      lastUpdated: new Date(reader.getUint8(1) + 1900, reader.getUint8(2), reader.getUint8(3)),
      records: reader.getUint32(4, true),
      headerLen: reader.getUint16(8, true),
      recLen: reader.getUint16(10, true),
    };
  }

  /**
   * Parses the row header and builds an array of keys that each property may have
   * @returns - an array of Rows that describe keys in each property
   */
  #parseRowHeader(): DBFRow[] {
    const { reader } = this;
    const { headerLen } = this.#header;
    const len = headerLen - 1;
    const res: DBFRow[] = [];

    let offset = 32;
    while (offset < len) {
      res.push({
        name: this.#decode(reader.slice(offset, offset + 11)),
        dataType: String.fromCharCode(reader.getUint8(offset + 11)),
        len: reader.getUint8(offset + 16),
        decimal: reader.getUint8(offset + 17),
      });
      if (reader.getUint8(offset + 32) === 13) {
        break;
      } else {
        offset += 32;
      }
    }

    return res;
  }

  /**
   * Parse the properties starting from the given offset
   * @param offset - offset of the row
   * @returns - a Properties object
   */
  #parseProperties(offset: number): Properties {
    const properties: Properties = {};
    for (const header of this.#rows) {
      const value = this.#parseValue(offset, header.len, header.dataType);
      offset += header.len;
      if (typeof value !== 'undefined') properties[header.name] = value;
    }

    return properties;
  }

  /**
   * @param offset - offset of the value
   * @param len - length of the value
   * @param type - the type of the value
   * @returns - the value as a string, number or boolean
   */
  #parseValue(offset: number, len: number, type: string) {
    const { reader } = this;
    const data = reader.slice(offset, offset + len);

    const textData = this.#decode(data);
    switch (type) {
      case 'N':
      case 'F':
      case 'O':
        return parseFloat(textData);
      case 'D':
        return new Date(
          parseFloat(textData.slice(0, 4)),
          parseInt(textData.slice(4, 6), 10) - 1,
          parseFloat(textData.slice(6, 8)),
        ).getUTCDate();
      case 'L':
        return textData.toLowerCase() === 'y' || textData.toLowerCase() === 't';
      default:
        return textData;
    }
  }

  /**
   * @param data - a Uint8Array input
   * @returns - a string
   */
  #decode(data: DataView): string {
    const { textDecoder } = this;
    const out =
      textDecoder.decode(data, {
        stream: true,
      }) + textDecoder.decode();
    return out.replace(/\0/g, '').trim();
  }
}
