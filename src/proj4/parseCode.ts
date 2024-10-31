import { buildDatum } from './datum';
import {
  D2R,
  FT_TO_M,
  PRIME_MERIDIAN,
  US_FT_TO_M,
  deriveEccentricity,
  deriveSphere,
} from './constants';
import { isWKTProjection, parseWKTProjection } from '../readers/wkt';

import type { DatumParams } from '../readers/wkt';
import type { ProjectionParams } from './projections';
import type { Transformer } from './transformer';

/**
 * Parse a proj4 string or object into a projection object
 * TODO: Support json objects that use the https://proj.org/schemas/v0.7/projjson.schema.json
 * @param code - a proj4 projection definition string or object
 * @param transformer - the transformer to build the projection state around
 * @returns - a projection object
 */
export function parseProj(
  code: string | ProjectionParams,
  transformer: Transformer,
): ProjectionParams {
  let params: ProjectionParams = {};
  if (typeof code === 'object') params = code;
  else if (code[0] === '+') params = parseProj4Str(code);
  else if (isWKTProjection(code)) params = parseWKTProjection(code);
  else throw new Error(`Invalid projection string: ${code}`);
  // adjust params as needed
  deriveSphere(params);
  deriveEccentricity(params);
  params.lat1 = params.lat1 ?? params.lat0; // Lambert_Conformal_Conic_1SP, for example, needs this
  params.k0 = params.k ?? params.k0;
  buildDatum(params, transformer);
  // filter undefined values
  Object.keys(params).forEach((key) => {
    // @ts-expect-error - key is a string
    if (params[key] === undefined) delete params[key];
  });

  return params;
}

/**
 * Parse a proj4 string into a projection object
 * @param code - a proj4 projection definition string
 * @returns - a projection description object
 */
function parseProj4Str(code: string): ProjectionParams {
  return code
    .split('+')
    .map((v) => v.trim())
    .filter((a) => a.length > 0)
    .reduce((res, a) => {
      const [key, value] = a.split('=');
      parseProj4StrKeyValue(res, key, value);
      return res;
    }, {} as ProjectionParams);
}

/**
 * Modify a key/value pair in a proj4 string to match the CRS projection standard
 * @param res - a projection description object
 * @param key - a proj4 string key
 * @param value - a proj4 string value
 */
function parseProj4StrKeyValue(res: ProjectionParams, key: string, value: string): void {
  // adjust key
  key = key.replace(/_([a-zA-Z0-9])/g, (_, letter) => letter.toUpperCase());
  if (key === 'proj') res.name = value;
  else if (
    [
      'alpha',
      'lat0',
      'lat1',
      'lat2',
      'latTs',
      'fromGreenwich',
      'long0',
      'long1',
      'long2',
      'longc',
    ].includes(key)
  )
    res[key as 'alpha' | 'lat0' | 'lat1' | 'lat2' | 'latTs' | 'fromGreenwich'] =
      parseFloat(value) * D2R;
  else if (['lon0', 'lon1', 'lon2', 'lonc'].includes(key)) {
    const modKey = key.replace('n', 'ng') as keyof ProjectionParams;
    res[modKey as 'long0' | 'long1' | 'long2' | 'longc'] = parseFloat(value) * D2R;
  } else if (['ellps', 'datumCode'].includes(key)) res[key as 'ellps' | 'datumCode'] = value;
  else if (key === 'datum') res.datumCode = value;
  else if (['noDefs', 'noOff', 'noRot', 'noUoff', 'rA', 'utmSouth', 'approx'].includes(key))
    res[key as 'noDefs' | 'rA' | 'utmSouth' | 'approx'] = true;
  else if (key === 'zone') res[key as 'zone'] = parseInt(value, 10);
  else if (key === 'units') {
    res.units = value;
    if (value === 'us-ft') res.toMeter = US_FT_TO_M;
    else if (value === 'ft') res.toMeter = FT_TO_M;
  } else if (key === 'nadgrids') {
    res.nadgrids = value;
    if (value === '@null') res.datumCode = 'none';
  } else if (key === 'pm') {
    const pm = PRIME_MERIDIAN[value as keyof typeof PRIME_MERIDIAN] ?? 0;
    res.fromGreenwich = pm * D2R;
  } else if (key === 'towgs84' || key === 'datumParams') {
    res.datumParams = value.split(',').map((a) => parseFloat(a)) as DatumParams;
  } else if (key === 'axis') {
    const legalAxis = 'ewnsud';
    if (
      value.length === 3 &&
      legalAxis.indexOf(value.slice(0, 1)) !== -1 &&
      legalAxis.indexOf(value.slice(1, 2)) !== -1 &&
      legalAxis.indexOf(value.slice(2, 3)) !== -1
    ) {
      res.axis = value;
    }
  } else if (key === 'r' || key === 'R') {
    res.a = res.b = parseFloat(value);
  } else if (key === 'gamma') {
    res.rectifiedGridAngle = parseFloat(value);
  } else if (key === 'sweep') {
    res.sweep = value === 'x' ? 'x' : 'y';
  } else if (!isNaN(value as unknown as number))
    // @ts-expect-error - key is a string in ProjectionParams
    res[key as keyof ProjectionParams] = parseFloat(value);
  // @ts-expect-error - key is a string in ProjectionParams
  else res[key as keyof ProjectionParams] = value;
}
