// function createDataView(): DataView {
//   // Calculate total byte size based on the sizes of each data type
//   const bufferSize = 1  // Uint8 = 1 byte
//                    + 2  // Uint16 = 2 bytes
//                    + 4  // Uint32 = 4 bytes
//                    + 1  // Int8 = 1 byte
//                    + 2  // Int16 = 2 bytes
//                    + 4  // Int32 = 4 bytes
//                    + 4  // Float32 = 4 bytes
//                    + 8  // Float64 = 8 bytes
//                    + 8  // BigUint64 = 8 bytes
//                    + 8; // BigInt64 = 8 bytes

//   // Create the buffer and wrap it in a DataView
//   const buffer = new ArrayBuffer(bufferSize);
//   const dv = new DataView(buffer);

//   let offset = 0;

//   // Set data in the DataView using the specified methods

//   // Set Uint8 (1 byte)
//   dv.setUint8(offset, 255);
//   offset += 1;

//   // Set Uint16 (2 bytes)
//   dv.setUint16(offset, 65535, true); // littleEndian
//   offset += 2;

//   // Set Uint32 (4 bytes)
//   dv.setUint32(offset, 4294967295, true); // littleEndian
//   offset += 4;

//   // Set Int8 (1 byte)
//   dv.setInt8(offset, -128);
//   offset += 1;

//   // Set Int16 (2 bytes)
//   dv.setInt16(offset, -32768, true); // littleEndian
//   offset += 2;

//   // Set Int32 (4 bytes)
//   dv.setInt32(offset, -2147483648, true); // littleEndian
//   offset += 4;

//   // Set Float32 (4 bytes)
//   dv.setFloat32(offset, 3.14, true); // littleEndian
//   offset += 4;

//   // Set Float64 (8 bytes)
//   dv.setFloat64(offset, 3.14159265359, true); // littleEndian
//   offset += 8;

//   // Set BigUint64 (8 bytes)
//   dv.setBigUint64(offset, 12345678901234567890n, true); // littleEndian
//   offset += 8;

//   // Set BigInt64 (8 bytes)
//   dv.setBigInt64(offset, -1234567890123456789n, true); // littleEndian
//   offset += 8;

//   return dv;
// }

// // Usage
// const dataView = createDataView();
// console.log(dataView);
// Bun.write('./tests/readers/fixtures/dv.bin', dataView.buffer);




const wktStr = 'PROJCS["NZGD49 / New Zealand Map Grid",GEOGCS["NZGD49",DATUM["New_Zealand_Geodetic_Datum_1949",SPHEROID["International 1924",6378388,297,AUTHORITY["EPSG","7022"]],TOWGS84[59.47,-5.04,187.44,0.47,-0.1,1.024,-4.5993],AUTHORITY["EPSG","6272"]],PRIMEM["Greenwich",0,AUTHORITY["EPSG","8901"]],UNIT["degree",0.01745329251994328,AUTHORITY["EPSG","9122"]],AUTHORITY["EPSG","4272"]],UNIT["metre",1,AUTHORITY["EPSG","9001"]],PROJECTION["New_Zealand_Map_Grid"],PARAMETER["latitude_of_origin",-41],PARAMETER["central_meridian",173],PARAMETER["false_easting",2510000],PARAMETER["false_northing",6023150],AUTHORITY["EPSG","27200"],AXIS["Easting",EAST],AXIS["Northing",NORTH]]'

// PROJCS[
//   "NZGD49 / New Zealand Map Grid",
//   GEOGCS[
//     "NZGD49",
//     DATUM[
//       "New_Zealand_Geodetic_Datum_1949",
//       SPHEROID["International 1924",6378388,297,AUTHORITY["EPSG","7022"]],
//       TOWGS84[59.47,-5.04,187.44,0.47,-0.1,1.024,-4.5993],
//       AUTHORITY["EPSG","6272"]
//     ],
//     PRIMEM["Greenwich",0,AUTHORITY["EPSG","8901"]],
//     UNIT["degree",0.01745329251994328,AUTHORITY["EPSG","9122"]],
//     AUTHORITY["EPSG","4272"]
//   ],
//   UNIT["metre",1,AUTHORITY["EPSG","9001"]],
//   PROJECTION["New_Zealand_Map_Grid"],
//   PARAMETER["latitude_of_origin",-41],
//   PARAMETER["central_meridian",173],
//   PARAMETER["false_easting",2510000],
//   PARAMETER["false_northing",6023150],
//   AUTHORITY["EPSG","27200"],
//   AXIS["Easting",EAST],
//   AXIS["Northing",NORTH]
// ]

const wktStr1 = 'PROJCS["NZGD49 / New Zealand Map Grid","stringCase"]'
const wktStr2 = 'PROJCS["NZGD49 / New Zealand Map Grid",TOWGS84[59.47,-5.04,187.44,0.47,-0.1,1.024,-4.5993],PARAMETER["central_meridian",173]]'
// const wktStr2 = 'PROJCS["NZGD49 / New Zealand Map Grid",TOWGS84[59.47,-5.04,187.44,0.47,-0.1,1.024,-4.5993]]'

const wktStr3 = 'PROJCS["A",B["C", D, 1[2, 3]], E["F", G, H]]'

type WKTValue = undefined | string | { [key: string]: WKTValue }
type WKTObject = Record<string, WKTValue>

function parseWKT(wktStr: string): WKTObject {
  const res: WKTObject = {};
  _parseWKTObject(wktStr, res);
  return res;
}

// always return the endBracketIndex if we hit it
function _parseWKTObject(wktStr: string, res: WKTObject): string {
  // first get the object name and build the residual
  while (wktStr.length) {
    let commaIndex = wktStr.indexOf(',');
    let startBracketIndex = wktStr.indexOf('[');
    const endBracketIndex = wktStr.indexOf(']');
    if (commaIndex === -1) commaIndex = Infinity;
    if (startBracketIndex === -1) startBracketIndex = Infinity;
    if (commaIndex < Math.min(startBracketIndex, endBracketIndex)) {
      // store the value
      const key = wktStr.slice(0, commaIndex);
      if (key.length > 0) res[cleanString(key)] = undefined;
      wktStr = wktStr.slice(commaIndex + 1);
    } else if (startBracketIndex < endBracketIndex) {
      // store the object
      const key = wktStr.slice(0, startBracketIndex);
      const obj = res[cleanString(key)] = {};
      wktStr = _parseWKTObject(wktStr.slice(startBracketIndex + 1), obj);
    } else {
      // store the LAST value if it exists, be sure to increment past the bracket for this recursive call
      if (endBracketIndex > 0) {
        res[cleanString(wktStr.slice(0, endBracketIndex))] = undefined;
        wktStr = wktStr.slice(endBracketIndex + 1);
      } else {
        wktStr = wktStr.slice(1);
      }
      return wktStr;
    }
  }
  // hit the end
  return wktStr;
}

function cleanString(str: string): string {
  return str
    .trim() // Remove whitespace at the start and end
    .replace(/^['"]|['"]$/g, '') // Remove single or double quotes from start and end
    .replace(/\s+/g, ' '); // Replace multiple spaces with a single space
}

// TRACKERS: offset, commaIndex, startBracketIndex, endBracketIndex

// CASE A: hit the end (59.47])
// CASE B: hit a comma (59.47,"-5.04",...etc)
// CASE C: hit a bracket/object (WORD[-5.04,..],...etc)

// Value is either a string, an array of strings, or an object.
// const wktObj = parseWKT(wktStr1)
// console.log('RES 1', wktObj)

// const wktObj2 = parseWKT(wktStr2)
// console.log('RES 2', wktObj2)

// const wktObj3 = parseWKT(wktStr3)
// console.log('RES 3', wktObj3)

// const wktObjFull = parseWKT(wktStr);
// console.log('RES FULL', wktObjFull)

interface VectorPoint {
  x: number;
  y: number;
  z?: number;
}

type WKTAValue = VectorPoint | WKTAValue[]
type WKTArray = WKTAValue[]

function parseWKTArray(wktStr: string): WKTArray {
  const res: WKTArray = [];
  _parseWKTArray(wktStr, res);
  return res.length ? (res[0] as WKTArray) : res;
}

// always return the endBracketIndex if we hit it
function _parseWKTArray(wktStr: string, res: WKTArray): string {
  console.log('PARSE', wktStr);
  // first get the array name and build the residual
  while (wktStr.length) {
    let commaIndex = wktStr.indexOf(',');
    let startBracketIndex = wktStr.indexOf('(');
    const endBracketIndex = wktStr.indexOf(')');
    if (commaIndex === -1) commaIndex = Infinity;
    if (startBracketIndex === -1) startBracketIndex = Infinity;
    if (commaIndex < Math.min(startBracketIndex, endBracketIndex)) {
      console.log('CASE A');
      // store the value
      const key = wktStr.slice(0, commaIndex);
      if (key.length > 0) res.push(buildPoint(key));
      wktStr = wktStr.slice(commaIndex + 1);
    } else if (startBracketIndex < endBracketIndex) {
      console.log('CASE B')
      // store the array
      const array: WKTArray = [];
      wktStr = _parseWKTArray(wktStr.slice(startBracketIndex + 1), array);
      res.push(array);
    } else {
      console.log('CASE C', wktStr, endBracketIndex)
      // store the LAST value if it exists, be sure to increment past the bracket for this recursive call
      if (endBracketIndex > 0) {
        res.push(buildPoint(wktStr.slice(0, endBracketIndex)));
        wktStr = wktStr.slice(endBracketIndex + 1);
      } else {
        wktStr = wktStr.slice(1);
      }
      return wktStr;
    }
  }
  // hit the end
  return wktStr;
}

function buildPoint(str: string): VectorPoint {
  const [x, y, z] = cleanString(str).split(' ');
  return { x: +x, y: +y, z: z ? +z : undefined };
}

// const str = '(0 1)';
const str = '(30 10, 10 30, 40 40)';
// const str = '(((40 40, 20 45, 45 30, 40 40)),((20 35, 10 30, 10 10, 30 5, 45 20, 20 35),(30 20, 20 15, 20 25, 30 20)))';
console.log(parseWKTArray(str));