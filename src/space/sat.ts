import { days2mdhms, jday } from './util/time';
import { deg2rad, minutesPerDay, pi } from './util/constants';
import { sgp4, sgp4init } from './propagation';

import type { SGP4ErrorOutput, SGP4Output } from './propagation';

/**
 * Classification of TLE
 * - U: unclassified
 * - C: classified
 * - S: secret
 */
export type Classification = 'U' | 'C' | 'S'; // (U: unclassified, C: classified, S: secret)
/**
 * Mode of operation AFSPC or Improved
 * - a: afspc
 * - i: improved
 */
export type OperationMode = 'a' | 'i'; // mode of operation afspc or improved 'a', 'i'
/**
 * Method of orbit determination
 * - d: deep space
 * - n: near earth
 */
export type Method = 'd' | 'n'; // flag for deep space 'd', 'n'

/** TLE Data Interface */
export interface TLEData {
  name: string;
  number: number;
  class: Classification;
  id: string;
  date: string | Date;
  fdmm: number;
  sdmm: number;
  drag: number;
  ephemeris: number;
  esn: number;
  inclination: number;
  ascension: number;
  eccentricity: number;
  perigee: number;
  anomaly: number;
  motion: number;
  revolution: number;
  rms?: number;
}

/** Celestrak TLE Data Interface */
export interface TLEDataCelestrak {
  OBJECT_NAME: string;
  OBJECT_ID: string;
  EPOCH: string;
  MEAN_MOTION: number;
  ECCENTRICITY: number;
  INCLINATION: number;
  RA_OF_ASC_NODE: number;
  ARG_OF_PERICENTER: number;
  MEAN_ANOMALY: number;
  EPHEMERIS_TYPE: number;
  CLASSIFICATION_TYPE: string;
  NORAD_CAT_ID: number;
  ELEMENT_SET_NO: number;
  REV_AT_EPOCH: number;
  BSTAR: number;
  MEAN_MOTION_DOT: number;
  MEAN_MOTION_DDOT: number;
  RMS: string;
  DATA_SOURCE: string;
}

/**
 * # Satellite Orbit Class
 *
 * Input TLE example
 * STARLINK-1007
 * 1 44713C 19074A   23048.53451389 -.00009219  00000+0 -61811-3 0   482
 * 2 44713  53.0512 157.2379 0001140  81.3827  74.7980 15.06382459    15
 */
export class Satellite {
  init = false;
  // Line 0
  name = 'default';
  // Line 1
  number = 0; // (satnum) Satellite catalog number or NORAD (North American Aerospace Defense) Catalog Number
  class: Classification = 'U'; // Classification (U: unclassified, C: classified, S: secret)
  id = 'null'; // International Designator
  date: Date = new Date(); // (epochyr + epochdays)
  epochyr = 0;
  epochdays = 0;
  jdsatepoch;
  fdmm = 0; // (ndot) First derivative of mean motion; the ballistic coefficient
  sdmm = 0; // (nddot) Second derivative of mean motion (decimal point assumed)
  drag = 0; // (bstar) B*, the drag term, or radiation pressure coefficient (decimal point assumed)
  ephemeris = 0; // Ephemeris type (always zero; only used in undistributed TLE data)
  esn = 0; // Element set number. Incremented when a new TLE is generated for this object.
  // Line 2
  inclination = 0; // Inclination (degrees)
  ascension = 0; // Right ascension of the ascending node (degrees)
  eccentricity = 0; // Eccentricity (decimal point assumed)
  perigee = 0; // Argument of perigee (degrees)
  anomaly = 0; // Mean anomaly (degrees)
  motion = 0; // Mean motion (revolutions per day)
  revolution = 0; // Revolution number at epoch (revolutions)
  // extra
  opsmode: OperationMode = 'i';
  rms?: number;
  // ----------- all near earth variables ------------
  isimp = 0;
  method: Method = 'n';
  aycof = 0;
  con41 = 0;
  cc1 = 0;
  cc4 = 0;
  cc5 = 0;
  d2 = 0;
  d3 = 0;
  d4 = 0;
  delmo = 0;
  eta = 0;
  argpdot = 0;
  omgcof = 0;
  sinmao = 0;
  t2cof = 0;
  t3cof = 0;
  t4cof = 0;
  t5cof = 0;
  x1mth2 = 0;
  x7thm1 = 0;
  mdot = 0;
  nodedot = 0;
  xlcof = 0;
  xmcof = 0;
  nodecf = 0;
  // ----------- all deep space variables ------------
  irez = 0;
  d2201 = 0;
  d2211 = 0;
  d3210 = 0;
  d3222 = 0;
  d4410 = 0;
  d4422 = 0;
  d5220 = 0;
  d5232 = 0;
  d5421 = 0;
  d5433 = 0;
  dedt = 0;
  del1 = 0;
  del2 = 0;
  del3 = 0;
  didt = 0;
  dmdt = 0;
  dnodt = 0;
  domdt = 0;
  e3 = 0;
  ee2 = 0;
  peo = 0;
  pgho = 0;
  pho = 0;
  pinco = 0;
  plo = 0;
  se2 = 0;
  se3 = 0;
  sgh2 = 0;
  sgh3 = 0;
  sgh4 = 0;
  sh2 = 0;
  sh3 = 0;
  si2 = 0;
  si3 = 0;
  sl2 = 0;
  sl3 = 0;
  sl4 = 0;
  gsto = 0;
  xfact = 0;
  xgh2 = 0;
  xgh3 = 0;
  xgh4 = 0;
  xh2 = 0;
  xh3 = 0;
  xi2 = 0;
  xi3 = 0;
  xl2 = 0;
  xl3 = 0;
  xl4 = 0;
  xlamo = 0;
  zmol = 0;
  zmos = 0;
  atime = 0;
  xli = 0;
  xni = 0;
  /**
   * Constructor
   * @param data - TLE data or TLE string
   * @param initialize - initialize the object on creation
   */
  constructor(data: TLEData | string, initialize = true) {
    if (typeof data === 'string') this.#parseTLE(data);
    else this.#parseJSON(data);
    // convert degrees to rad
    this.inclination *= deg2rad;
    this.ascension *= deg2rad;
    this.perigee *= deg2rad;
    this.anomaly *= deg2rad;
    // convert revolution from deg/day to rad/minute
    this.motion /= 1440 / (2 * pi); // rad/min (229.1831180523293)
    // find sgp4epoch time of element set
    // remember that sgp4 uses units of days from 0 jan 1950 (sgp4epoch)
    // and minutes from the epoch (time)
    const year = this.epochyr < 57 ? this.epochyr + 2000 : this.epochyr + 1900;
    const mdhmsResult = days2mdhms(year, this.epochdays);
    const { mon, day, hr, minute, sec } = mdhmsResult;
    this.jdsatepoch = jday(year, mon, day, hr, minute, sec, 0);

    if (initialize) sgp4init(this);
  }

  /** API */

  /**
   * propagate the satellite's position and velocity given a Date input
   * @param time - Date object
   * @returns - SGP4ErrorOutput or SGP4Output
   */
  propagate(time: Date): SGP4ErrorOutput | SGP4Output {
    const j = jday(time);
    return sgp4(this, (j - this.jdsatepoch) * minutesPerDay);
  }

  /**
   * time in minutes since epoch
   * @param time - time in minutes
   * @returns - satellite state at that time
   */
  sgp4(time: number): SGP4ErrorOutput | SGP4Output {
    return sgp4(this, time);
  }

  /**
   * Converts satellite state to an array that is readable by the GPU
   * @returns - satellite state in an array
   */
  gpu(): number[] {
    return [
      this.anomaly,
      this.motion,
      this.eccentricity,
      this.inclination,
      this.method === 'd' ? 0 : 1, // 0 -> 'd', 1 -> 'n'
      this.opsmode === 'a' ? 0 : 1, // 0 -> 'a'; 1 -> 'i'
      this.drag,
      this.mdot,
      this.perigee,
      this.argpdot,
      this.ascension,
      this.nodedot,
      this.nodecf,
      this.cc1,
      this.cc4,
      this.cc5,
      this.t2cof,
      this.isimp,
      this.omgcof,
      this.eta,
      this.xmcof,
      this.delmo,
      this.d2,
      this.d3,
      this.d4,
      this.sinmao,
      this.t3cof,
      this.t4cof,
      this.t5cof,
      this.irez,
      this.d2201,
      this.d2211,
      this.d3210,
      this.d3222,
      this.d4410,
      this.d4422,
      this.d5220,
      this.d5232,
      this.d5421,
      this.d5433,
      this.dedt,
      this.del1,
      this.del2,
      this.del3,
      this.didt,
      this.dmdt,
      this.dnodt,
      this.domdt,
      this.gsto,
      this.xfact,
      this.xlamo,
      this.atime,
      this.xli,
      this.xni,
      this.aycof,
      this.xlcof,
      this.con41,
      this.x1mth2,
      this.x7thm1,
      this.zmos,
      this.zmol,
      this.se2,
      this.se3,
      this.si2,
      this.si3,
      this.sl2,
      this.sl3,
      this.sl4,
      this.sgh2,
      this.sgh3,
      this.sgh4,
      this.sh2,
      this.sh3,
      this.ee2,
      this.e3,
      this.xi2,
      this.xi3,
      this.xl2,
      this.xl3,
      this.xl4,
      this.xgh2,
      this.xgh3,
      this.xgh4,
      this.xh2,
      this.xh3,
      this.peo,
      this.pinco,
      this.plo,
      this.pgho,
      this.pho,
    ];
  }

  /** INTERNAL */

  /** @param data - TLE data object */
  #parseJSON(data: TLEData): void {
    if (typeof data.date === 'string') data.date = new Date(data.date);
    for (const [key, value] of Object.entries(data)) {
      if (key in this) this[key as keyof this] = value;
    }

    this.epochyr = this.date.getFullYear() % 100;
    const start = new Date(Date.UTC(this.date.getFullYear(), 0, 0));
    this.epochdays = jday(this.date) - jday(start);
  }

  /**
   * Parse TLE string
   * https://en.wikipedia.org/wiki/Two-line_element_set
   * @param value - TLE string
   */
  #parseTLE(value: string): void {
    const lines = trim(String(value)).split(/\r?\n/g);
    let line: string;
    let checksum: number;

    // Line 0
    if (lines.length === 3) {
      this.name = trim(lines.shift() as string);
      if (this.name.substring(0, 2) === '0 ') this.name = this.name.substring(2);
    }

    // Line 1
    line = lines.shift() as string;
    checksum = check(line);

    if (checksum !== Number(line.substring(68, 69))) {
      throw new Error(
        `Line 1 checksum mismatch: ${checksum} != ${line.substring(68, 69)}: ${line}`,
      );
    }

    this.number = this.#parseFloat(alpha5Converter(line.substring(2, 7)));
    this.class = trim(line.substring(7, 9)) as Classification;
    this.id = trim(line.substring(9, 18));
    this.date = this.#parseDate(line.substring(18, 33));
    this.fdmm = this.#parseFloat(line.substring(33, 44));
    this.sdmm = this.#parseFloat(line.substring(44, 53));
    this.drag = this.#parseDrag(line.substring(53, 62));
    this.ephemeris = this.#parseFloat(line.substring(62, 64));
    this.esn = this.#parseFloat(line.substring(64, 68));

    // Line 2
    line = lines.shift() as string;
    checksum = check(line);

    if (checksum !== Number(line.substring(68, 69))) {
      throw new Error(
        `Line 2 checksum mismatch: ${checksum} != ${line.substring(68, 69)}: ${line}`,
      );
    }

    this.inclination = this.#parseFloat(line.substring(8, 17));
    this.ascension = this.#parseFloat(line.substring(17, 26));
    this.eccentricity = this.#parseFloat('0.' + line.substring(26, 34));
    this.perigee = this.#parseFloat(line.substring(34, 43));
    this.anomaly = this.#parseFloat(line.substring(43, 52));
    this.motion = this.#parseFloat(line.substring(52, 63));
    this.revolution = this.#parseFloat(line.substring(63, 68));
  }

  /**
   * Parse a float number from a string
   * @param value - string
   * @returns - the parsed float number
   */
  #parseFloat(value: string): number {
    const pattern = /([-])?([.\d]+)([+-]\d+)?/;
    const match = pattern.exec(value);

    if (match !== null) {
      const sign = match[1] === '-' ? -1 : 1;
      const power = match[3] !== undefined ? 'e' + match[3] : 'e0';
      const value = match[2];
      return sign * parseFloat(value + power);
    }

    return NaN;
  }

  /**
   * Parse a drag coefficient
   * @param value - string
   * @returns - the parsed drag coefficient
   */
  #parseDrag(value: string): number {
    const pattern = /([-])?([.\d]+)([+-]\d+)?/;
    const match = pattern.exec(value);

    if (match !== null) {
      const sign = match[1] === '-' ? -1 : 1;
      const power = match[3] !== undefined ? 'e' + match[3] : 'e0';
      const value = !match[2].includes('.') ? '0.' + match[2] : match[2];
      return sign * parseFloat(value + power);
    }

    return NaN;
  }

  /**
   * Parse a date
   * @param value - string of the date
   * @returns - the parsed date
   */
  #parseDate(value: string): Date {
    value = String(value).replace(/^\s+|\s+$/, '');

    const epoch = (this.epochyr = parseInt(value.substring(0, 2), 10));
    const days = (this.epochdays = parseFloat(value.substring(2)));

    let year = new Date().getFullYear();
    const currentEpoch = year % 100;
    const century = year - currentEpoch;

    year = epoch > currentEpoch + 1 ? century - 100 + epoch : century + epoch;

    const day = Math.floor(days);
    const hours = 24 * (days - day);
    const hour = Math.floor(hours);
    const minutes = 60 * (hours - hour);
    const minute = Math.floor(minutes);
    const seconds = 60 * (minutes - minute);
    const second = Math.floor(seconds);
    const millisecond = 1000 * (seconds - second);

    const utc = Date.UTC(year, 0, day, hour, minute, second, millisecond);

    return new Date(utc);
  }
}

/**
 * Covnert Celestrak TLE data to a standard TLE data object
 * [JSON example](https://celestrak.org/NORAD/elements/supplemental/index.php?FORMAT=json)
 * @param data - Celestrak TLE data
 * @returns - TLE data
 */
export function convertCelestrak(data: TLEDataCelestrak): TLEData {
  // convert date to UTC to avoid javascripts localization issues
  const date = new Date(data.EPOCH);
  const utc = Date.UTC(
    date.getFullYear(),
    date.getMonth(),
    date.getDate(),
    date.getHours(),
    date.getMinutes(),
    date.getSeconds(),
    date.getMilliseconds(),
  );
  return {
    name: data.OBJECT_NAME,
    number: data.NORAD_CAT_ID,
    class: data.CLASSIFICATION_TYPE as Classification,
    id: data.OBJECT_ID,
    date: new Date(utc),
    fdmm: data.MEAN_MOTION_DOT,
    sdmm: data.MEAN_MOTION_DDOT,
    drag: data.BSTAR,
    ephemeris: data.EPHEMERIS_TYPE,
    esn: data.ELEMENT_SET_NO,
    inclination: data.INCLINATION,
    ascension: data.RA_OF_ASC_NODE,
    eccentricity: data.ECCENTRICITY,
    perigee: data.ARG_OF_PERICENTER,
    anomaly: data.MEAN_ANOMALY,
    motion: data.MEAN_MOTION,
    revolution: data.REV_AT_EPOCH,
    rms: parseFloat(data.RMS),
  };
}

/**
 * Check TLE checksum
 * @param line - TLE line
 * @returns - TLE checksum
 */
function check(line: string): number {
  let sum = 0;

  for (const char of line.substring(0, 68)) {
    // deno-lint-ignore no-explicit-any
    if (!isNaN(char as unknown as number)) sum += Number(char);
    else if (char === '-') sum++;
  }

  return sum % 10;
}

/**
 * Converts alpha5 to alpha2
 * NOTE: Alpha5 skips I and O
 * @param str - alpha5 input string
 * @returns - alpha2 string
 */
function alpha5Converter(str: string): string {
  const firstChar = str.charAt(0);
  // deno-lint-ignore no-explicit-any
  if (!isNaN(firstChar as unknown as number)) return str;
  const alpha5Index = 'ABCDEFGHJKLMNPQRSTUVWXYZ'.indexOf(firstChar);
  return String(alpha5Index + 10) + str.slice(1);
}

/**
 * Trim leading and trailing whitespace
 * @param str - string to trim
 * @returns - trimmed string
 */
function trim(str: string): string {
  return str.replace(/^[\s\uFEFF\xA0]+|[\s\uFEFF\xA0]+$/g, '');
}
