// BUILD GUIDE:
// GOTO: https://epsg.org/
// REGISTER
// GOTO: https://epsg.org/download-dataset.html
// DOWNLOAD: something like "EPSG v11.031 (WKT File) (5.40 MB)"
// EXTRACT: into ./tools/
// RUN: bun build:epsg

import { readdirSync } from 'fs';
import { WKTObject, parseWKTObject } from '../src';

const folder = 'EPSG-v12_007-WKT';

// get list of files in ./EPSG-v12_007-WKT
const files = readdirSync(`${__dirname}/${folder}`);

const keywordCounts = new Map<string, number>();
let CRS_ONLY_COUNT = 0;

for (const file of files) {
  if (file.includes('-CRS-')) CRS_ONLY_COUNT++;
  const file_str = await Bun.file(`${__dirname}/${folder}/${file}`).text();
  const wkt = parseWKTObject(file_str);
  findCapitalizedKeywords(wkt, keywordCounts);
}

console.info('CRS COUNT: ', CRS_ONLY_COUNT);

// Convert the map to an array of [keyword, count] pairs
const sortedKeywords = Array.from(keywordCounts.entries()).sort(
  ([, countA], [, countB]) => countB - countA,
);

console.info('Keywords sorted by frequency:');
sortedKeywords.forEach(([keyword, count]) => {
  console.info(`${keyword}: ${count}`);
});

/**
 * Finds all the keywords that are capitalized and counts their occurrences
 * @param wktArray - WKT array
 * @param keywordCounts - Map to store keyword counts
 */
function findCapitalizedKeywords(wktArray: WKTObject, keywordCounts: Map<string, number>): void {
  if (!Array.isArray(wktArray)) {
    return;
  }

  for (const item of wktArray) {
    if (typeof item === 'string' && item === item.toUpperCase() && item.length > 0) {
      if (item.includes(' ')) continue;
      if (/\d/.test(item)) continue;

      keywordCounts.set(item, (keywordCounts.get(item) ?? 0) + 1);
    } else if (Array.isArray(item)) {
      findCapitalizedKeywords(item, keywordCounts);
    }

    // if (item === 'PARAMETERFILE') console.info(wktArray);
  }
}

// ID: 214278
// EPSG: 214276
// PARAMETER: 40825
// LENGTHUNIT: 40304
// AXIS: 28074
// ANGLEUNIT: 23917
// MEMBER: 19846
// CS: 14093
// ELLIPSOID: 12351
// DATUM: 10182
// METHOD: 8986
// SCALEUNIT: 5952
// CONVERSION: 5857
// BASEGEOGCRS: 5834
// PROJCRS: 5830
// GEOGCRS: 5768
// SOURCECRS: 3145
// TARGETCRS: 3140
// OPERATIONACCURACY: 2953
// COORDINATEOPERATION: 2948
// VERSION: 2948
// ENSEMBLE: 2191
// ENSEMBLEACCURACY: 2191
// GEOIDMODEL: 1820
// VERTCRS: 1713
// VDATUM: 1691
// PARAMETERFILE: 1427
// ANCHOREPOCH: 1155
// DYNAMIC: 1024
// FRAMEEPOCH: 1024
// GEODCRS: 749
// COMPOUNDCRS: 703
// STEP: 396
// DEFININGTRANSFORMATION: 313
// NADCON: 288 (name of method)
// TIMEUNIT: 217
// CONCATENATEDOPERATION: 192
// DERIVINGCONVERSION: 176
// BASEVERTCRS: 172
// PRIMEM: 150
// MERIDIAN: 102
// ENGCRS: 29
// EDATUM: 29
// POINTMOTIONOPERATION: 5
// DERIVEDPROJECTED: 4
// BASEPROJCRS: 4
