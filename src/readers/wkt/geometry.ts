import { cleanString } from '.';

import type {
  FeatureIterator,
  MValue,
  Properties,
  VectorFeature,
  VectorGeometry,
  VectorLineString,
  VectorMultiLineString,
  VectorMultiPolygon,
  VectorPoint,
  VectorPointGeometry,
} from '../..';

/** WKT Value can be a point or an array of points */
export type WKTAValue = VectorPoint | WKTAValue[];
/** WKT Array can be an array of points or even nested arrays of points */
export type WKTArray = WKTAValue[];

/**
 * # WKT Geometry Reader
 *
 * ## Description
 * Parse a collection of WKT geometries from a string
 * implements the {@link FeatureIterator} interface
 *
 * ## Usage
 * ```ts
 * import { WKTGeometryReader } from 'gis-tools-ts';
 *
 * const reader = new WKTGeometryReader('POINT(4 6) GEOMETRYCOLLECTION(POINT(1 2), LINESTRING(3 4,5 6))');
 *
 * // read the features
 * for await (const feature of reader) {
 *   console.log(feature);
 * }
 * ```
 *
 * ## Links
 * - https://en.wikipedia.org/wiki/Well-known_text_representation_of_geometry
 */
export class WKTGeometryReader<
  M = Record<string, unknown>,
  D extends MValue = MValue,
  P extends Properties = Properties,
> implements FeatureIterator<M, D, P>
{
  data: VectorFeature<M, D, P>[] = [];

  /** @param data - the WKT geometry string to parase */
  constructor(data: string) {
    const wktStrings = splitWKTGeometry(data);
    for (const wktString of wktStrings) {
      const geometry = parseWKTGeometry<D>(wktString);
      if (geometry !== undefined)
        this.data.push({
          type: 'VectorFeature',
          geometry,
          properties: {} as P,
        });
    }
  }

  /**
   * Generator to iterate over each (Geo|S2)JSON object in the file
   * @yields {VectorFeature}
   */
  async *[Symbol.asyncIterator](): AsyncGenerator<VectorFeature<M, D, P>> {
    for (const feature of this.data) yield feature;
  }
}

/**
 * # WKT Geometry Parser
 *
 * ## Description
 * Parse individual geometries from a WKT string into a VectorGeometry
 *
 * ## Usage
 * ```ts
 * import { parseWKTGeometry } from 'gis-tools-ts';
 *
 * const geometry = parseWKTGeometry('POINT (1 2)');
 * ```
 *
 * ## Links
 * - https://en.wikipedia.org/wiki/Well-known_text_representation_of_geometry
 * @param wktStr - WKT string
 * @returns - VectorGeometry
 */
export function parseWKTGeometry<D extends MValue = MValue>(
  wktStr: string,
): VectorGeometry<D> | undefined {
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
}

/**
 * Split a WKT string into individual geometries
 * @param input - WKT string that is a collection of geometries
 * @returns - Array of individual WKT geometries
 */
export function splitWKTGeometry(input: string): string[] {
  // First remove all instances of EMPTY geometry.
  // So if EMPTY found, delete the word and the word prior:
  const words = input.split(' ');
  for (let i = 0; i < words.length; i++) {
    const word = words[i];
    if (word.includes('EMPTY') && i > 0) {
      words.splice(i - 1, 2);
      i--;
    }
  }
  input = words.join(' ');

  const geometries: string[] = [];
  let start = 0;
  let found = false;
  let end = 0;
  let depth = 0;

  for (let i = start; i < input.length; i++) {
    const char = input[i];

    if (char === '(') {
      depth++;
      found = true;
    }
    if (char === ')') depth--;
    if (found && depth === 0) {
      end = i + 1;
      geometries.push(input.slice(start, end).trim());
      start = end;
      found = false;
    }
  }

  for (let i = 0; i < geometries.length; i++) {
    const geometry = geometries[i];
    if (geometry.startsWith('GEOMETRYCOLLECTION')) {
      geometries.splice(i, 1);
      i--;
      // remove "GEOMETRYCOLLECTION(" from beginning of string and ")" from end
      const clean = geometry.slice(geometry.indexOf('(') + 1, geometry.length - 1);
      geometries.push(...splitWKTGeometry(clean));
    } else if (geometry.startsWith(',')) {
      geometries[i] = geometry.slice(1).trim();
    }
  }

  return geometries.filter((g) => g.length > 0);
}

/**
 * Parse a WKT point string to a VectorPoint
 * @param wktStr - WKT string
 * @param is3D - true if the point is 3D
 * @returns - VectorPoint
 */
function parseWKTPoint<D extends MValue = MValue>(
  wktStr: string,
  is3D: boolean,
): VectorPointGeometry<D> {
  const geo = parseWKTArray(wktStr);
  return {
    type: 'Point',
    is3D,
    coordinates: geo[0] as VectorPoint<D>,
  };
}

/**
 * Parse a WKT array to a LineString or MultiPoint geometry
 * @param wktStr - WKT string
 * @param type - 'MultiPoint' or 'LineString'
 * @param is3D - true if the point is 3D
 * @returns - VectorGeometry (LineString or MultiPoint)
 */
function parseWKTLine<D extends MValue = MValue>(
  wktStr: string,
  type: 'MultiPoint' | 'LineString',
  is3D: boolean,
): VectorGeometry<D> {
  let geo = parseWKTArray(wktStr);
  geo =
    geo.length > 0 && Array.isArray(geo[0])
      ? geo.map((e) => (e as [VectorPoint])[0] as VectorPoint)
      : geo;
  return {
    type,
    is3D,
    coordinates: geo as VectorLineString<D>,
  };
}

/**
 * Parse a WKT array to a MultiLineString or Polygon
 * @param wktStr - WKT string
 * @param type - 'MultiLineString' or 'Polygon'
 * @param is3D - true if the point is 3D
 * @returns - VectorGeometry
 */
function parseWKTMultiLine<D extends MValue = MValue>(
  wktStr: string,
  type: 'MultiLineString' | 'Polygon',
  is3D: boolean,
): VectorGeometry<D> {
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
    coordinates: geo as VectorMultiLineString<D>,
  };
}

/**
 * Parse a WKT array to a MultiPolygon
 * @param wktStr - WKT string
 * @param is3D - true if each point is 3D
 * @returns - VectorGeometry
 */
function parseWKTMultiPolygon<D extends MValue = MValue>(
  wktStr: string,
  is3D: boolean,
): VectorGeometry<D> {
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
    coordinates: geo as VectorMultiPolygon<D>,
  };
}

/**
 * Parse a WKT array
 * @param wktStr - WKT string
 * @returns - collection of points
 */
function parseWKTArray(wktStr: string): WKTArray {
  const res: WKTArray = [];
  _parseWKTArray(wktStr, res);
  return res.length > 0 ? (res[0] as WKTArray) : res;
}

/**
 * Parse a WKT array.
 * always return the endBracketIndex if we hit it
 * @param wktStr - WKT string
 * @param res - collection to store the values
 * @returns - a sliced WKT string with the parsed values
 */
function _parseWKTArray(wktStr: string, res: WKTArray): string {
  // first get the array name and build the residual
  while (wktStr.length > 0) {
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
 * Build a point from a WKT string
 * @param str - WKT string
 * @returns - VectorPoint
 */
function buildPoint(str: string): VectorPoint {
  const [x, y, z] = cleanString(str).split(' ');
  return { x: Number(x), y: Number(y), z: z !== undefined ? Number(z) : undefined };
}
