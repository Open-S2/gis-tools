import { nonRecord, record } from './data';
import { Header, header } from './header';
import { notNetcdf } from './utils';

import type { Reader } from '..';

/**
 * Reads a NetCDF v3.x file
 * [See specification](https://www.unidata.ucar.edu/software/netcdf/docs/file_format_specifications.html)
 * @param data - ArrayBuffer or any Typed Array (including Node.js' Buffer from v4) with the data
 */
export class NetCDFReader {
  #header: Header;

  /**
   * @param data
   * @param reader
   */
  constructor(public reader: Reader) {
    if (reader.parseString(0, 3) !== 'CDF') throw new Error('Not a valid NetCDF v3.x file');

    // Check the NetCDF format
    const version = buffer.readByte();
    notNetcdf(version > 2, 'unknown version');

    // Read the header
    this.header = header(buffer, version);
    this.buffer = buffer;
  }

  /**
   * @returns - Version for the NetCDF format
   */
  get version() {
    if (this.header.version === 1) {
      return 'classic format';
    } else {
      return '64-bit offset format';
    }
  }

  /**
   * @returns - Metadata for the record dimension
   *  `length`: Number of elements in the record dimension
   *  `id`: Id number in the list of dimensions for the record dimension
   *  `name`: String with the name of the record dimension
   *  `recordStep`: Number with the record variables step size
   */
  get recordDimension() {
    return this.header.recordDimension;
  }

  /**
   * @returns - Array - List of dimensions with:
   *  `name`: String with the name of the dimension
   *  `size`: Number with the size of the dimension
   */
  get dimensions() {
    return this.header.dimensions;
  }

  /**
   * @returns - Array - List of global attributes with:
   *  `name`: String with the name of the attribute
   *  `type`: String with the type of the attribute
   *  `value`: A number or string with the value of the attribute
   */
  get globalAttributes(): Header['globalAttributes'] {
    return this.header.globalAttributes;
  }

  /**
   * Returns the value of an attribute
   * @param - AttributeName
   * @param attributeName
   * @returns - Value of the attributeName or null
   */
  getAttribute(attributeName: string) {
    const attribute = this.globalAttributes.find((val) => val.name === attributeName);
    if (attribute) return attribute.value;
    return null;
  }

  /**
   * Returns the value of a variable as a string
   * @param - variableName
   * @param variableName
   * @returns - Value of the variable as a string or null
   */
  getDataVariableAsString(variableName: string) {
    const variable = this.getDataVariable(variableName);
    if (variable) return variable.join('');
    return null;
  }

  /**
   *
   */
  get variables() {
    return this.header.variables;
  }

  toString = toString;

  /**
   * Retrieves the data for a given variable
   * @param variableName - Name of the variable to search or variable object
   * @returns The variable values
   */
  getDataVariable(variableName: string | Header['variables'][number]) {
    let variable;
    if (typeof variableName === 'string') {
      // search the variable
      variable = this.header.variables.find((val) => {
        return val.name === variableName;
      });
    } else {
      variable = variableName;
    }

    // throws if variable not found
    if (variable === undefined) {
      throw new Error('Not a valid NetCDF v3.x file: variable not found');
    }

    // go to the offset position
    this.buffer.seek(variable.offset);

    if (variable.record) {
      // record variable case
      return record(this.buffer, variable, this.header.recordDimension);
    } else {
      // non-record variable case
      return nonRecord(this.buffer, variable);
    }
  }

  /**
   * Check if a dataVariable exists
   * @param variableName - Name of the variable to find
   * @returns boolean
   */
  dataVariableExists(variableName: string) {
    const variable = this.header.variables.find((val) => {
      return val.name === variableName;
    });
    return variable !== undefined;
  }

  /**
   * Check if an attribute exists
   * @param attributeName - Name of the attribute to find
   * @returns boolean
   */
  attributeExists(attributeName: string) {
    const attribute = this.globalAttributes.find((val) => val.name === attributeName);
    return attribute !== undefined;
  }

  /**
   *
   */
  toString(): string {
    const result: string[] = [];
    result.push('DIMENSIONS');
    for (const dimension of this.dimensions) {
      result.push(`  ${dimension.name.padEnd(30)} = size: ${dimension.size}`);
    }

    result.push('');
    result.push('GLOBAL ATTRIBUTES');
    for (const attribute of this.globalAttributes) {
      result.push(`  ${attribute.name.padEnd(30)} = ${attribute.value}`);
    }

    const variables = JSON.parse(JSON.stringify(this.variables));
    result.push('');
    result.push('VARIABLES:');
    for (const variable of variables) {
      variable.value = this.getDataVariable(variable);
      let stringify = JSON.stringify(variable.value);
      if (stringify.length > 50) stringify = stringify.substring(0, 50);
      if (!isNaN(variable.value.length)) {
        stringify += ` (length: ${variable.value.length})`;
      }
      result.push(`  ${variable.name.padEnd(30)} = ${stringify}`);
    }
    return result.join('\n');
  }
}

// UTIL:

// /**
//  * Moves 1, 2, or 3 bytes to next 4-byte boundary
//  * @param buffer - Buffer for the file data
//  */
// export function padding(buffer: IOBuffer) {
//   if (buffer.offset % 4 !== 0) {
//     buffer.skip(4 - (buffer.offset % 4));
//   }
// }

// /**
//  * Reads the name
//  * @param buffer - Buffer for the file data
//  * @return Name
//  */
// export function readName(buffer: IOBuffer) {
//   // Read name
//   const nameLength = buffer.readUint32();
//   const name = buffer.readChars(nameLength);

//   // validate name
//   // TODO
//   // Apply padding
//   padding(buffer);
//   return name;
// }
