// Not perfect, just experimenting to see roughly what kind of keywords are most common to start

import { readdirSync } from 'fs';
import { WKTObject, parseWKTObject } from '../src';

const folder = 'EPSG-v12_007-WKT';

// get list of files in ./EPSG-v12_007-WKT
const files = readdirSync(`${__dirname}/${folder}`);

const firstWords = new Set<string>();
const secondWords = new Set<string>();
// let CRS_ONLY_COUNT = 0;

for (const file of files) {
  // if (file.includes('-CRS-')) CRS_ONLY_COUNT++;
  const file_str = await Bun.file(`${__dirname}/${folder}/${file}`).text();
  const wkt = parseWKTObject(file_str);
  findFirstAndSecondWords(wkt);
}

/**
 * Find keywords that are common at the top level
 * @param wkt - the object
 */
function findFirstAndSecondWords(wkt: WKTObject) {
  const firstWord = wkt[0];
  const secondWord = wkt[1]?.[1] ?? '';
  firstWords.add(firstWord as string);
  secondWords.add(secondWord as string);
}

console.info('firstWords', [...firstWords]);
console.info('secondWords', [...secondWords]);

// SECOND KEYWORD
// "BASEGEOGCRS",
// "VERSION",
// "VDATUM",
// "DATUM",
// "PROJCRS",
// "DYNAMIC",
// "SOURCECRS",
// "BASEPROJCRS",
// "GEOGCRS",
// "EDATUM",
// "ENSEMBLE",
// "BASEVERTCRS"
// "ENGCRS"
