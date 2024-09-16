import {
  D2R,
  FT_TO_M,
  PRIME_MERIDIAN,
  US_FT_TO_M,
  deriveEccentricity,
  deriveSphere,
} from './constants';

import type { ProjectionParams } from './projections';

/**
 * @param code
 * @param proj
 */
export function parseProjStr(code: string): ProjectionParams {
  const params = code[0] === '+' ? parseProj4Str(code) : parseWktStr(code);

  // adjust params as needed
  deriveSphere(params);
  deriveEccentricity(params);
  params.lat1 = params.lat1 ?? params.lat0; // Lambert_Conformal_Conic_1SP, for example, needs this
  params.k0 = params.k ?? params.k0;

  return params;
}

/**
 * @param code
 * @param proj
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
 * @param res
 * @param key
 * @param value
 */
function parseProj4StrKeyValue(res: ProjectionParams, key: string, value: string): void {
  // adjust key
  key = key.replace(/_([a-zA-Z0-9])/g, (_, letter) => letter.toUpperCase());
  if (key === 'proj') res.name = value;
  else if (['rf', 'gamma', 'x0', 'y0', 'k0', 'k', 'a', 'b'].includes(key))
    res[key as 'rf' | 'gamma' | 'x0' | 'y0' | 'k0' | 'k' | 'a' | 'b'] = parseFloat(value);
  else if (['alpha', 'lat0', 'lat1', 'lat2', 'latTs'].includes(key))
    res[key as 'alpha' | 'lat0' | 'lat1' | 'lat2' | 'latTs'] = parseFloat(value) * D2R;
  else if (['lon0', 'lon1', 'lon2', 'lonc'].includes(key)) {
    const modKey = key.replace('n', 'ng') as keyof ProjectionParams;
    res[modKey as 'long0' | 'long1' | 'long2' | 'longc'] = parseFloat(value) * D2R;
  } else if (['ellps', 'datum', 'units'].includes(key))
    res[key as 'ellps' | 'datum' | 'units'] = value;
  else if (key === 'noDefs') res.noDefs = true;
}

/**
 * @param code
 * @param proj
 */
function parseWktStr(code: string): ProjectionParams {
  // TODO:
  return {} as ProjectionParams;
}

// /**
//  * @param defData
//  */
// export function parseProjString(defData) {
//   const self = {};
//   const paramObj = defData
//     .split('+')
//     .map(function (v) {
//       return v.trim();
//     })
//     .filter(function (a) {
//       return a;
//     })
//     .reduce(function (p, a) {
//       const split = a.split('=');
//       split.push(true);
//       p[split[0].toLowerCase()] = split[1];
//       return p;
//     }, {});
//   let paramName, paramVal, paramOutname;
//   const params = {
//     proj: 'projName',
//     datum: 'datumCode',

//     /**
//      *
//      */
//     r_a: function () {
//       self.R_A = true;
//     },
//     /**
//      * @param v
//      */
//     zone: function (v) {
//       self.zone = parseInt(v, 10);
//     },
//     /**
//      *
//      */
//     south: function () {
//       self.utmSouth = true;
//     },
//     /**
//      * @param v
//      */
//     towgs84: function (v) {
//       self.datum_params = v.split(',').map(function (a) {
//         return parseFloat(a);
//       });
//     },
//     /**
//      * @param v
//      */
//     to_meter: function (v) {
//       self.to_meter = parseFloat(v);
//     },
//     /**
//      * @param v
//      */
//     units: function (v) {
//       self.units = v;
//       // units:
//       //     ft: {to_meter: 0.3048},
//       // 'us-ft': {to_meter: 1200 / 3937}
//       const unit = match(units, v);
//       if (unit) {
//         self.to_meter = unit.to_meter;
//       }
//     },
//     /**
//      * @param v
//      */
//     from_greenwich: function (v) {
//       self.from_greenwich = v * D2R;
//     },
//     /**
//      * @param v
//      */
//     pm: function (v) {
//       const pm = match(PRIME_MERIDIAN, v);
//       self.from_greenwich = (pm ? pm : parseFloat(v)) * D2R;
//     },
//     /**
//      * @param v
//      */
//     nadgrids: function (v) {
//       if (v === '@null') {
//         self.datumCode = 'none';
//       } else {
//         self.nadgrids = v;
//       }
//     },
//     /**
//      * @param v
//      */
//     axis: function (v) {
//       const legalAxis = 'ewnsud';
//       if (
//         v.length === 3 &&
//         legalAxis.indexOf(v.substr(0, 1)) !== -1 &&
//         legalAxis.indexOf(v.substr(1, 1)) !== -1 &&
//         legalAxis.indexOf(v.substr(2, 1)) !== -1
//       ) {
//         self.axis = v;
//       }
//     },
//     /**
//      *
//      */
//     approx: function () {
//       self.approx = true;
//     },
//   };
//   for (paramName in paramObj) {
//     paramVal = paramObj[paramName];
//     if (paramName in params) {
//       paramOutname = params[paramName];
//       if (typeof paramOutname === 'function') {
//         paramOutname(paramVal);
//       } else {
//         self[paramOutname] = paramVal;
//       }
//     } else {
//       self[paramName] = paramVal;
//     }
//   }
//   if (typeof self.datumCode === 'string' && self.datumCode !== 'WGS84') {
//     self.datumCode = self.datumCode.toLowerCase();
//   }
//   return self;
// }
