import type { Properties } from 's2-tools/geometry';

/**
 *
 */
export interface DBFHeader {
  lastUpdated: Date;
  records: number;
  headerLen: number;
  recLen: number;
}

/**
 *
 */
export interface DBFRow {
  name: string;
  dataType: string;
  len: number;
  decimal: number;
}

/**
 *
 */
export default class DataBaseFile {
  textDecoder: TextDecoder;
  header!: DBFHeader;
  properties: Properties[] = [];

  /**
   * @param buffer
   * @param encoding
   */
  constructor(
    public buffer: DataView,
    public encoding: string,
  ) {
    // @ts-expect-error - linting bug
    this.textDecoder = new TextDecoder(encoding.trim());
    this.#parseHeader();
    const rowHeaders = this.#parseRowHeader();
    this.#parseProperties(rowHeaders);
  }

  /**
   * @param index
   */
  getProperties(index: number): Properties | undefined {
    return this.properties.at(index);
  }

  /**
   *
   */
  #parseHeader() {
    const { buffer } = this;
    this.header = {
      lastUpdated: new Date(buffer.getUint8(1) + 1900, buffer.getUint8(2), buffer.getUint8(3)),
      records: buffer.getUint32(4, true),
      headerLen: buffer.getUint16(8, true),
      recLen: buffer.getUint16(10, true),
    };
  }

  /**
   *
   */
  #parseRowHeader(): DBFRow[] {
    const {
      buffer,
      header: { headerLen },
    } = this;
    const len = headerLen - 1;
    const res: DBFRow[] = [];

    let offset = 32;
    while (offset < len) {
      res.push({
        name: this.#decode(
          new Uint8Array(
            buffer.buffer.slice(buffer.byteOffset + offset, buffer.byteOffset + offset + 11),
          ),
        ),
        dataType: String.fromCharCode(buffer.getUint8(offset + 11)),
        len: buffer.getUint8(offset + 16),
        decimal: buffer.getUint8(offset + 17),
      });
      if (buffer.getUint8(offset + 32) === 13) {
        break;
      } else {
        offset += 32;
      }
    }

    return res;
  }

  /**
   * @param rowHeaders
   */
  #parseProperties(rowHeaders: DBFRow[]): void {
    const { header } = this;
    let offset = ((rowHeaders.length + 1) << 5) + 2;
    const { recLen } = header;
    let records = header.records;
    while (records) {
      this.properties.push(this.#parseRow(offset, rowHeaders));
      offset += recLen;
      records--;
    }
  }

  /**
   * @param offset
   * @param rowHeaders
   */
  #parseRow(offset: number, rowHeaders: DBFRow[]): Properties {
    const properties: Properties = {};
    let field;
    for (const header of rowHeaders) {
      field = this.#parseFunc(offset, header.len, header.dataType);
      offset += header.len;
      if (typeof field !== 'undefined') properties[header.name] = field;
    }

    return properties;
  }

  /**
   * @param offset
   * @param len
   * @param type
   */
  #parseFunc(offset: number, len: number, type: string) {
    const { buffer } = this;
    const data = new Uint8Array(
      buffer.buffer.slice(buffer.byteOffset + offset, buffer.byteOffset + offset + len),
    );

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
  #decode(data: Uint8Array): string {
    const { textDecoder } = this;
    const out =
      textDecoder.decode(data, {
        stream: true,
      }) + textDecoder.decode();
    return out.replace(/\0/g, '').trim();
  }
}
