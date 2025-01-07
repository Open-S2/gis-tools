// References:
// - Wikipedia: https://en.wikipedia.org/wiki/Military_Grid_Reference_System
// - GEOTRANS: https://earth-info.nga.mil/#geotrans
import { degToRad, radToDeg } from '../geometry';

import type { BBox, VectorPoint } from '../geometry';

/** UTM zones are grouped, and assigned to one of a group of 6 sets. */
const NUM_100K_SETS = 6;
/** The column letters (for easting) of the lower left value, per set. */
const SET_ORIGIN_COLUMN_LETTERS = 'AJSAJS';
/** The row letters (for northing) of the lower left value, per set. */
const SET_ORIGIN_ROW_LETTERS = 'AFAFAF';

/** First eccentricity squared */
const ECC_SQUARED = 0.00669438;
/** Scale factor along the central meridian */
const SCALE_FACTOR = 0.9996;
/** Semimajor axis (half the width of the earth) in meters */
const SEMI_MAJOR_AXIS = 6378137;
/** The easting of the central meridian of each UTM zone */
const EASTING_OFFSET = 500000;
/** The northing of the equator for southern hemisphere locations (in UTM) */
const NORTHING_OFFFSET = 10000000;
/** UTM zone width in degrees */
const UTM_ZONE_WIDTH = 6;
/** Half the width of a UTM zone in degrees */
const HALF_UTM_ZONE_WIDTH = 3; // UTM_ZONE_WIDTH / 2;

const A = 65; // A
const I = 73; // I
const O = 79; // O
const V = 86; // V
const Z = 90; // Z

/**
 * Convert lat/lon to MGRS.
 * @param ll - Array with longitude and latitude on a
 *     WGS84 ellipsoid.
 * @param accuracy - Accuracy in digits (5 for 1 m, 4 for 10 m, 3 for
 *      100 m, 2 for 1 km, 1 for 10 km or 0 for 100 km). Optional, default is 5.
 * @returns the MGRS string for the given location and accuracy.
 */
export function mgrsForward(ll: VectorPoint, accuracy?: number): string {
  accuracy = accuracy !== undefined ? accuracy : 5; // default accuracy 1m

  const { x, y } = ll;
  if (x < -180 || x > 180) {
    throw new TypeError(`forward received an invalid longitude of ${x}`);
  }
  if (y < -90 || y > 90) {
    throw new TypeError(`forward received an invalid latitude of ${y}`);
  }
  if (y < -80 || y > 84) {
    throw new TypeError(
      `forward received a latitude of ${y}, but this library does not support conversions of points in polar regions below 80°S and above 84°N`,
    );
  }

  return encode(LLtoUTM(ll), accuracy);
}

/**
 * Convert MGRS to lat/lon bounding box.
 * @param mgrs - MGRS string.
 * @returns An array with left (longitude),
 *    bottom (latitude), right
 *    (longitude) and top (latitude) values in WGS84, representing the
 *    bounding box for the provided MGRS reference.
 */
export function mgrsInverse(mgrs: string): undefined | BBox {
  const bbox = UTMtoLL(decode(mgrs.toUpperCase()));
  if (bbox === undefined) return undefined;
  if (bbox.lat !== undefined && bbox.lon !== undefined) {
    return [bbox.lon, bbox.lat, bbox.lon, bbox.lat];
  }
  return [bbox.left, bbox.bottom, bbox.right, bbox.top];
}

/**
 * Convert MGRS to lat/lon.
 * @param mgrs - MGRS string.
 * @returns The center of the MGRS bounding box
 */
export function mgrsToPoint(mgrs: string): undefined | VectorPoint {
  const bbox = UTMtoLL(decode(mgrs.toUpperCase()));
  if (bbox === undefined) return undefined;
  if (bbox.lat !== undefined && bbox.lon !== undefined) return { x: bbox.lon, y: bbox.lat };
  return { x: (bbox.left + bbox.right) / 2, y: (bbox.top + bbox.bottom) / 2 };
}

/**
 * Converts a set of Longitude and Latitude co-ordinates to UTM
 * using the WGS84 ellipsoid.
 * @param ll Object literal with lat and lon properties
 *     representing the WGS84 coordinate to be converted.
 * @returns Object literal containing the UTM value with easting,
 *     northing, zoneNumber and zoneLetter properties, and an optional
 *     accuracy property in digits. Returns null if the conversion failed.
 */
function LLtoUTM(ll: VectorPoint): UTM {
  const { x: lon, y: lat } = ll;
  const a = SEMI_MAJOR_AXIS;
  const latRad = degToRad(lat);
  const longRad = degToRad(lon);
  let ZoneNumber;
  // (int)
  ZoneNumber = Math.floor((lon + 180) / 6) + 1;

  //Make sure the longitude 180 is in Zone 60
  if (lon === 180) ZoneNumber = 60;

  // Special zone for Norway
  if (lat >= 56 && lat < 64 && lon >= 3 && lon < 12) ZoneNumber = 32;

  // Special zones for Svalbard
  if (lat >= 72 && lat < 84) {
    if (lon >= 0 && lon < 9) {
      ZoneNumber = 31;
    } else if (lon >= 9 && lon < 21) {
      ZoneNumber = 33;
    } else if (lon >= 21 && lon < 33) {
      ZoneNumber = 35;
    } else if (lon >= 33 && lon < 42) {
      ZoneNumber = 37;
    }
  }

  // +HALF_UTM_ZONE_WIDTH puts origin in middle of zone
  const LongOrigin = (ZoneNumber - 1) * UTM_ZONE_WIDTH - 180 + HALF_UTM_ZONE_WIDTH;

  const LongOriginRad = degToRad(LongOrigin);

  const eccPrimeSquared = ECC_SQUARED / (1 - ECC_SQUARED);

  const N = a / Math.sqrt(1 - ECC_SQUARED * Math.sin(latRad) * Math.sin(latRad));
  const T = Math.tan(latRad) * Math.tan(latRad);
  const C = eccPrimeSquared * Math.cos(latRad) * Math.cos(latRad);
  const A = Math.cos(latRad) * (longRad - LongOriginRad);

  const M =
    a *
    ((1 -
      ECC_SQUARED / 4 -
      (3 * ECC_SQUARED * ECC_SQUARED) / 64 -
      (5 * ECC_SQUARED * ECC_SQUARED * ECC_SQUARED) / 256) *
      latRad -
      ((3 * ECC_SQUARED) / 8 +
        (3 * ECC_SQUARED * ECC_SQUARED) / 32 +
        (45 * ECC_SQUARED * ECC_SQUARED * ECC_SQUARED) / 1024) *
        Math.sin(2 * latRad) +
      ((15 * ECC_SQUARED * ECC_SQUARED) / 256 +
        (45 * ECC_SQUARED * ECC_SQUARED * ECC_SQUARED) / 1024) *
        Math.sin(4 * latRad) -
      ((35 * ECC_SQUARED * ECC_SQUARED * ECC_SQUARED) / 3072) * Math.sin(6 * latRad));

  const UTMEasting =
    SCALE_FACTOR *
      N *
      (A +
        ((1 - T + C) * A * A * A) / 6 +
        ((5 - 18 * T + T * T + 72 * C - 58 * eccPrimeSquared) * A * A * A * A * A) / 120) +
    EASTING_OFFSET;

  let UTMNorthing =
    SCALE_FACTOR *
    (M +
      N *
        Math.tan(latRad) *
        ((A * A) / 2 +
          ((5 - T + 9 * C + 4 * C * C) * A * A * A * A) / 24 +
          ((61 - 58 * T + T * T + 600 * C - 330 * eccPrimeSquared) * A * A * A * A * A * A) / 720));
  if (lat < 0) UTMNorthing += NORTHING_OFFFSET;

  return {
    northing: Math.trunc(UTMNorthing),
    easting: Math.trunc(UTMEasting),
    zoneNumber: ZoneNumber,
    zoneLetter: mgrsGetLetterDesignator(lat),
  };
}

/** UTM parameters */
interface UTM {
  northing: number;
  easting: number;
  zoneNumber: number;
  zoneLetter: string;
  accuracy?: number;
}

/** Bounds in an object format */
interface PointBounds {
  top: number;
  right: number;
  bottom: number;
  left: number;
  lon?: number;
  lat?: number;
}

/**
 * Converts UTM coords to lat/long, using the WGS84 ellipsoid. This is a convenience
 * class where the Zone can be specified as a single string eg."60N" which
 * is then broken down into the ZoneNumber and ZoneLetter.
 * @param utm An object literal with northing, easting, zoneNumber
 *     and zoneLetter properties. If an optional accuracy property is
 *     provided (in meters), a bounding box will be returned instead of
 *     latitude and longitude.
 * @returns An object literal containing either lat and lon values
 *     (if no accuracy was provided), or top, right, bottom and left values
 *     for the bounding box calculated according to the provided accuracy.
 *     Returns null if the conversion failed.
 */
function UTMtoLL(utm: UTM): PointBounds | undefined {
  const UTMNorthing = utm.northing;
  const UTMEasting = utm.easting;
  const { zoneLetter, zoneNumber } = utm;
  // check the ZoneNummber is valid
  if (zoneNumber < 0 || zoneNumber > 60) return;

  const a = SEMI_MAJOR_AXIS;
  const e1 = (1 - Math.sqrt(1 - ECC_SQUARED)) / (1 + Math.sqrt(1 - ECC_SQUARED));

  // remove 500,000 meter offset for longitude
  const x = UTMEasting - EASTING_OFFSET;
  let y = UTMNorthing;

  // We must know somehow if we are in the Northern or Southern
  // hemisphere, this is the only time we use the letter So even
  // if the Zone letter isn't exactly correct it should indicate
  // the hemisphere correctly
  if (zoneLetter < 'N') {
    y -= NORTHING_OFFFSET; // remove offset used for southern hemisphere
  }

  // +HALF_UTM_ZONE_WIDTH puts origin in middle of zone
  const LongOrigin = (zoneNumber - 1) * UTM_ZONE_WIDTH - 180 + HALF_UTM_ZONE_WIDTH;

  const eccPrimeSquared = ECC_SQUARED / (1 - ECC_SQUARED);

  const M = y / SCALE_FACTOR;
  const mu =
    M /
    (a *
      (1 -
        ECC_SQUARED / 4 -
        (3 * ECC_SQUARED * ECC_SQUARED) / 64 -
        (5 * ECC_SQUARED * ECC_SQUARED * ECC_SQUARED) / 256));

  const phi1Rad =
    mu +
    ((3 * e1) / 2 - (27 * e1 * e1 * e1) / 32) * Math.sin(2 * mu) +
    ((21 * e1 * e1) / 16 - (55 * e1 * e1 * e1 * e1) / 32) * Math.sin(4 * mu) +
    ((151 * e1 * e1 * e1) / 96) * Math.sin(6 * mu);
  // double phi1 = ProjMath.radToDeg(phi1Rad);

  const N1 = a / Math.sqrt(1 - ECC_SQUARED * Math.sin(phi1Rad) * Math.sin(phi1Rad));
  const T1 = Math.tan(phi1Rad) * Math.tan(phi1Rad);
  const C1 = eccPrimeSquared * Math.cos(phi1Rad) * Math.cos(phi1Rad);
  const R1 =
    (a * (1 - ECC_SQUARED)) /
    Math.pow(1 - ECC_SQUARED * Math.sin(phi1Rad) * Math.sin(phi1Rad), 1.5);
  const D = x / (N1 * SCALE_FACTOR);

  let lat =
    phi1Rad -
    ((N1 * Math.tan(phi1Rad)) / R1) *
      ((D * D) / 2 -
        ((5 + 3 * T1 + 10 * C1 - 4 * C1 * C1 - 9 * eccPrimeSquared) * D * D * D * D) / 24 +
        ((61 + 90 * T1 + 298 * C1 + 45 * T1 * T1 - 252 * eccPrimeSquared - 3 * C1 * C1) *
          D *
          D *
          D *
          D *
          D *
          D) /
          720);
  lat = radToDeg(lat);

  let lon =
    (D -
      ((1 + 2 * T1 + C1) * D * D * D) / 6 +
      ((5 - 2 * C1 + 28 * T1 - 3 * C1 * C1 + 8 * eccPrimeSquared + 24 * T1 * T1) *
        D *
        D *
        D *
        D *
        D) /
        120) /
    Math.cos(phi1Rad);
  lon = LongOrigin + radToDeg(lon);

  let result;
  if (typeof utm.accuracy === 'number') {
    const topRight = UTMtoLL({
      northing: utm.northing + utm.accuracy,
      easting: utm.easting + utm.accuracy,
      zoneLetter: utm.zoneLetter,
      zoneNumber: utm.zoneNumber,
    });
    result = {
      top: topRight?.lat ?? lat,
      right: topRight?.lon ?? lon,
      bottom: lat,
      left: lon,
    };
  } else {
    result = {
      top: lat,
      right: lon,
      bottom: lat,
      left: lon,
      lat,
      lon,
    };
  }
  return result;
}

/**
 * Calculates the MGRS letter designator for the given latitude.
 * (Not intended for public API, only exported for testing.)
 * @param latitude - The latitude in WGS84 to get the letter designator
 *     for.
 * @returns The letter designator.
 */
export function mgrsGetLetterDesignator(latitude: number): string {
  if (latitude <= 84 && latitude >= 72) {
    // the X band is 12 degrees high
    return 'X';
  } else if (latitude < 72 && latitude >= -80) {
    // Latitude bands are lettered C through X, excluding I and O
    const bandLetters = 'CDEFGHJKLMNPQRSTUVWX';
    const bandHeight = 8;
    const minLatitude = -80;
    const index = Math.floor((latitude - minLatitude) / bandHeight);
    return bandLetters[index];
  }
  //This is here as an error flag to show that the Latitude is
  //outside MGRS limits
  return 'Z';
}

/**
 * Encodes a UTM location as MGRS string.
 * @param utm - An object literal with easting, northing,
 *     zoneLetter, zoneNumber
 * @param accuracy - Accuracy in digits (0-5).
 * @returns MGRS string for the given UTM location.
 */
function encode(utm: UTM, accuracy: number): string {
  // prepend with leading zeroes
  const seasting = '00000' + utm.easting;
  const snorthing = '00000' + utm.northing;
  const seLen = seasting.length - 5;
  const snLen = snorthing.length - 5;

  return (
    utm.zoneNumber +
    utm.zoneLetter +
    get100kID(utm.easting, utm.northing, utm.zoneNumber) +
    seasting.slice(seLen, seLen + accuracy) +
    snorthing.slice(snLen, snLen + accuracy)
  );
}

/**
 * Get the two letter 100k designator for a given UTM easting,
 * northing and zone number value.
 * @param easting - UTM easting
 * @param northing - UTM northing
 * @param zoneNumber - UTM zone number
 * @returns the two letter 100k designator for the given UTM location.
 */
function get100kID(easting: number, northing: number, zoneNumber: number): string {
  const setParm = get100kSetForZone(zoneNumber);
  const setColumn = Math.floor(easting / 100000);
  const setRow = Math.floor(northing / 100000) % 20;
  return getLetter100kID(setColumn, setRow, setParm);
}

/**
 * Given a UTM zone number, figure out the MGRS 100K set it is in.
 * @param i - An UTM zone number.
 * @returns the 100k set the UTM zone is in.
 */
function get100kSetForZone(i: number): number {
  let setParm = i % NUM_100K_SETS;
  if (setParm === 0) {
    setParm = NUM_100K_SETS;
  }

  return setParm;
}

/**
 * Get the two-letter MGRS 100k designator given information
 * translated from the UTM northing, easting and zone number.
 * @param column - the column index as it relates to the MGRS
 *        100k set spreadsheet, created from the UTM easting.
 *        Values are 1-8.
 * @param row - the row index as it relates to the MGRS 100k set
 *        spreadsheet, created from the UTM northing value. Values
 *        are from 0-19.
 * @param parm - the set block, as it relates to the MGRS 100k set
 *        spreadsheet, created from the UTM zone. Values are from
 *        1-60.
 * @returns two letter MGRS 100k code.
 */
function getLetter100kID(column: number, row: number, parm: number): string {
  // colOrigin and rowOrigin are the letters at the origin of the set
  const index = parm - 1;
  const colOrigin = SET_ORIGIN_COLUMN_LETTERS.charCodeAt(index);
  const rowOrigin = SET_ORIGIN_ROW_LETTERS.charCodeAt(index);

  // colInt and rowInt are the letters to build to return
  let colInt = colOrigin + column - 1;
  let rowInt = rowOrigin + row;
  let rollover = false;

  if (colInt > Z) {
    colInt = colInt - Z + A - 1;
    rollover = true;
  }

  if (
    colInt === I ||
    (colOrigin < I && colInt > I) ||
    ((colInt > I || colOrigin < I) && rollover)
  ) {
    colInt++;
  }

  if (
    colInt === O ||
    (colOrigin < O && colInt > O) ||
    ((colInt > O || colOrigin < O) && rollover)
  ) {
    colInt++;

    if (colInt === I) {
      colInt++;
    }
  }

  if (colInt > Z) {
    colInt = colInt - Z + A - 1;
  }

  if (rowInt > V) {
    rowInt = rowInt - V + A - 1;
    rollover = true;
  } else {
    rollover = false;
  }

  if (
    rowInt === I ||
    (rowOrigin < I && rowInt > I) ||
    ((rowInt > I || rowOrigin < I) && rollover)
  ) {
    rowInt++;
  }

  if (
    rowInt === O ||
    (rowOrigin < O && rowInt > O) ||
    ((rowInt > O || rowOrigin < O) && rollover)
  ) {
    rowInt++;

    if (rowInt === I) {
      rowInt++;
    }
  }

  if (rowInt > V) {
    rowInt = rowInt - V + A - 1;
  }

  const twoLetter = String.fromCharCode(colInt) + String.fromCharCode(rowInt);
  return twoLetter;
}

/**
 * Decode the UTM parameters from a MGRS string.
 * @param mgrsString - an UPPERCASE coordinate string is expected.
 * @returns An object literal with easting, northing, zoneLetter,
 *     zoneNumber and accuracy (in meters) properties.
 */
function decode(mgrsString: string): UTM {
  if (mgrsString.length === 0) {
    throw new TypeError('MGRSPoint coverting from nothing');
  }

  //remove any spaces in MGRS String
  mgrsString = mgrsString.replace(/ /g, '');

  const { length } = mgrsString;

  let hunK = null;
  let sb = '';
  let testChar;
  let i = 0;

  // get Zone number
  while (!/[A-Z]/.test((testChar = mgrsString.charAt(i)))) {
    if (i >= 2) {
      throw new Error(`MGRSPoint bad conversion from: ${mgrsString}`);
    }
    sb += testChar;
    i++;
  }

  const zoneNumber = parseInt(sb, 10);

  if (i === 0 || i + 3 > length) {
    // A good MGRS string has to be 4-5 digits long,
    // ##AAA/#AAA at least.
    throw new Error(`MGRSPoint bad conversion from ${mgrsString}`);
  }

  const zoneLetter = mgrsString.charAt(i++);

  // Should we check the zone letter here? Why not.
  if (
    zoneLetter <= 'A' ||
    zoneLetter === 'B' ||
    zoneLetter === 'Y' ||
    zoneLetter >= 'Z' ||
    zoneLetter === 'I' ||
    zoneLetter === 'O'
  ) {
    throw new Error(`MGRSPoint zone letter ${zoneLetter} not handled: ${mgrsString}`);
  }

  hunK = mgrsString.substring(i, (i += 2));

  const set = get100kSetForZone(zoneNumber);

  const east100k = getEastingFromChar(hunK.charAt(0), set);
  let north100k = getNorthingFromChar(hunK.charAt(1), set);

  // We have a bug where the northing may be 2000000 too low.
  // How
  // do we know when to roll over?

  while (north100k < getMinNorthing(zoneLetter)) {
    north100k += 2000000;
  }

  // calculate the char index for easting/northing separator
  const remainder = length - i;

  if (remainder % 2 !== 0) {
    throw new Error(`MGRSPoint has to have an even number
of digits after the zone letter and two 100km letters - front
half for easting meters, second half for
northing meters ${mgrsString}`);
  }

  const sep = remainder / 2;

  let sepEasting = 0;
  let sepNorthing = 0;
  let accuracy = 0;
  let sepEastingString: string, sepNorthingString: string;
  if (sep > 0) {
    accuracy = 100000 / Math.pow(10, sep);
    sepEastingString = mgrsString.substring(i, i + sep);
    sepEasting = parseFloat(sepEastingString) * accuracy;
    sepNorthingString = mgrsString.substring(i + sep);
    sepNorthing = parseFloat(sepNorthingString) * accuracy;
  }

  const easting = sepEasting + east100k;
  const northing = sepNorthing + north100k;

  return {
    easting,
    northing,
    zoneLetter,
    zoneNumber,
    accuracy,
  };
}

/**
 * Given the first letter from a two-letter MGRS 100k zone, and given the
 * MGRS table set for the zone number, figure out the easting value that
 * should be added to the other, secondary easting value.
 * @param e - The first letter from a two-letter MGRS 100´k zone.
 * @param set - The MGRS table set for the zone number.
 * @returns The easting value for the given letter and set.
 */
function getEastingFromChar(e: string, set: number): number {
  // colOrigin is the letter at the origin of the set for the
  // column
  let curCol = SET_ORIGIN_COLUMN_LETTERS.charCodeAt(set - 1);
  let eastingValue = 100000;
  let rewindMarker = false;

  while (curCol !== e.charCodeAt(0)) {
    curCol++;
    if (curCol === I) curCol++;
    if (curCol === O) curCol++;
    if (curCol > Z) {
      if (rewindMarker) {
        throw new Error(`Bad character: ${e}`);
      }
      curCol = A;
      rewindMarker = true;
    }
    eastingValue += 100000;
  }

  return eastingValue;
}

/**
 * Given the second letter from a two-letter MGRS 100k zone, and given the
 * MGRS table set for the zone number, figure out the northing value that
 * should be added to the other, secondary northing value. You have to
 * remember that Northings are determined from the equator, and the vertical
 * cycle of letters mean a 2000000 additional northing meters. This happens
 * approx. every 18 degrees of latitude. This method does *NOT* count any
 * additional northings. You have to figure out how many 2000000 meters need
 * to be added for the zone letter of the MGRS coordinate.
 * @param n - Second letter of the MGRS 100k zone
 * @param set - The MGRS table set number, which is dependent on the
 *     UTM zone number.
 * @returns The northing value for the given letter and set.
 */
function getNorthingFromChar(n: string, set: number): number {
  if (n > 'V') throw new TypeError(`MGRSPoint given invalid Northing ${n}`);

  // rowOrigin is the letter at the origin of the set for the
  // column
  let curRow = SET_ORIGIN_ROW_LETTERS.charCodeAt(set - 1);
  let northingValue = 0;
  let rewindMarker = false;

  while (curRow !== n.charCodeAt(0)) {
    curRow++;
    if (curRow === I) {
      curRow++;
    }
    if (curRow === O) {
      curRow++;
    }
    // fixing a bug making whole application hang in this loop
    // when 'n' is a wrong character
    if (curRow > V) {
      if (rewindMarker) {
        // making sure that this loop ends
        throw new Error(`Bad character: ${n}`);
      }
      curRow = A;
      rewindMarker = true;
    }
    northingValue += 100000;
  }

  return northingValue;
}

/**
 * The function getMinNorthing returns the minimum northing value of a MGRS
 * zone.
 *
 * Ported from Geotrans' c Lattitude_Band_Value structure table.
 * @param zoneLetter The MGRS zone to get the min northing for.
 * @returns - The minimum northing value of the MGRS zone.
 */
function getMinNorthing(zoneLetter: string): number {
  let northing;
  switch (zoneLetter) {
    case 'C':
      northing = 1100000;
      break;
    case 'D':
      northing = 2000000;
      break;
    case 'E':
      northing = 2800000;
      break;
    case 'F':
      northing = 3700000;
      break;
    case 'G':
      northing = 4600000;
      break;
    case 'H':
      northing = 5500000;
      break;
    case 'J':
      northing = 6400000;
      break;
    case 'K':
      northing = 7300000;
      break;
    case 'L':
      northing = 8200000;
      break;
    case 'M':
      northing = 9100000;
      break;
    case 'N':
      northing = 0;
      break;
    case 'P':
      northing = 800000;
      break;
    case 'Q':
      northing = 1700000;
      break;
    case 'R':
      northing = 2600000;
      break;
    case 'S':
      northing = 3500000;
      break;
    case 'T':
      northing = 4400000;
      break;
    case 'U':
      northing = 5300000;
      break;
    case 'V':
      northing = 6200000;
      break;
    case 'W':
      northing = 7000000;
      break;
    case 'X':
      northing = 7900000;
      break;
    default:
      northing = -1;
  }
  if (northing >= 0) {
    return northing;
  } else {
    throw new TypeError(`Invalid zone letter: ${zoneLetter}`);
  }
}
