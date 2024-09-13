import { cleanString } from '.';

import type {
  VectorGeometry,
  VectorLineString,
  VectorMultiLineString,
  VectorMultiPolygon,
  VectorPoint,
  VectorPointGeometry,
} from 's2-tools/geometry';

/**
 *
 */
export type WKTAValue = VectorPoint | WKTAValue[];
/**
 *
 */
export type WKTArray = WKTAValue[];

/**
 * @param wktStr
 */
export function parseWKTGeometry(wktStr: string): VectorGeometry {
  if (wktStr.startsWith('POINT')) return parseWKTPoint(wktStr, wktStr.startsWith('POINT Z'));
  else if (wktStr.startsWith('MULTIPOINT'))
    return parseWKTLine(wktStr, 'MultiPoint', wktStr.startsWith('MULTIPOINT Z'));
  else if (wktStr.startsWith('LINESTRING'))
    return parseWKTLine(wktStr, 'LineString', wktStr.startsWith('LINESTRING Z'));
  else if (wktStr.startsWith('MULTILINESTRING'))
    return parseWKTMultiLine(wktStr, 'MultiLineString', wktStr.startsWith('MULTILINESTRING Z'));
  else if (wktStr.startsWith('POLYGON'))
    return parseWKTMultiLine(wktStr, 'Polygon', wktStr.startsWith('POLYGON Z'));
  else if (wktStr.startsWith('MULTIPOLYGON'))
    return parseWKTMultiPolygon(wktStr, wktStr.startsWith('MULTIPOLYGON Z'));
  throw new Error('Unimplemented WKT geometry: ' + wktStr);
}

/**
 * @param wktStr
 * @param is3D
 */
function parseWKTPoint(wktStr: string, is3D: boolean): VectorPointGeometry {
  const geo = parseWKTArray(wktStr);
  return {
    type: 'Point',
    is3D,
    coordinates: geo[0] as VectorPoint,
  };
}

/**
 * @param wktStr
 * @param type
 * @param is3D
 */
function parseWKTLine(
  wktStr: string,
  type: 'MultiPoint' | 'LineString',
  is3D: boolean,
): VectorGeometry {
  let geo = parseWKTArray(wktStr);
  geo =
    geo.length > 0 && Array.isArray(geo[0])
      ? geo.map((e) => (e as [VectorPoint])[0] as VectorPoint)
      : geo;
  return {
    type,
    is3D,
    coordinates: geo as VectorLineString,
  };
}

/**
 * @param wktStr
 * @param type
 * @param is3D
 */
function parseWKTMultiLine(
  wktStr: string,
  type: 'MultiLineString' | 'Polygon',
  is3D: boolean,
): VectorGeometry {
  let geo = parseWKTArray(wktStr) as VectorMultiLineString;
  geo =
    geo.length > 0 && geo[0].length > 0 && Array.isArray(geo[0][0])
      ? geo.map((e) => {
          return e.map((e2) => (e2 as unknown as [VectorPoint])[0] as VectorPoint);
        })
      : geo;
  return {
    type,
    is3D,
    coordinates: geo as VectorMultiLineString,
  };
}

/**
 * @param wktStr
 * @param is3D
 */
function parseWKTMultiPolygon(wktStr: string, is3D: boolean): VectorGeometry {
  let geo = parseWKTArray(wktStr) as VectorMultiPolygon;
  geo =
    geo.length > 0 && geo[0].length > 0 && geo[0][0].length > 0 && Array.isArray(geo[0][0][0])
      ? geo.map((e) => {
          return e.map((e2) => e2.map((e3) => (e3 as unknown as [VectorPoint])[0] as VectorPoint));
        })
      : geo;
  return {
    type: 'MultiPolygon',
    is3D,
    coordinates: geo as VectorMultiPolygon,
  };
}

/**
 * @param wktStr
 */
function parseWKTArray(wktStr: string): WKTArray {
  const res: WKTArray = [];
  _parseWKTArray(wktStr, res);
  return res.length ? (res[0] as WKTArray) : res;
}

// always return the endBracketIndex if we hit it
/**
 * @param wktStr
 * @param res
 * @param startChar
 * @param endChar
 */
function _parseWKTArray(wktStr: string, res: WKTArray): string {
  // first get the array name and build the residual
  while (wktStr.length) {
    let commaIndex = wktStr.indexOf(',');
    let startBracketIndex = wktStr.indexOf('(');
    const endBracketIndex = wktStr.indexOf(')');
    if (commaIndex === -1) commaIndex = Infinity;
    if (startBracketIndex === -1) startBracketIndex = Infinity;
    if (commaIndex < Math.min(startBracketIndex, endBracketIndex)) {
      // store the value
      const key = wktStr.slice(0, commaIndex);
      if (key.length > 0) res.push(buildPoint(key));
      wktStr = wktStr.slice(commaIndex + 1);
    } else if (startBracketIndex < endBracketIndex) {
      // store the array
      const array: WKTArray = [];
      wktStr = _parseWKTArray(wktStr.slice(startBracketIndex + 1), array);
      res.push(array);
    } else {
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

/**
 * @param str
 */
function buildPoint(str: string): VectorPoint {
  const [x, y, z] = cleanString(str).split(' ');
  return { x: +x, y: +y, z: z ? +z : undefined };
}
