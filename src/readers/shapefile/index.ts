// import fetcher from './fetch';

import DataBaseFile from './dbf';
import Shapefile from './shp';
import { exists, readFile } from 'fs/promises';

// import proj4 from 'proj4';
// import unzip from './unzip';

export * from './dbf';
export * from './shp';

/**
 *
 */
export interface Definition {
  /** The path to the .shp file */
  shp: string;
  /** The path to the .dbf file. dbf is optional, but needed if you want attributes */
  dbf?: string;
  /**
   * The path to the .prj file. prj is optional, but needed if your file is in some
   * projection you don't want it in
   */
  prj?: string;
  /**
   * The path to the .cpg file. cpg is optional, but needed if your dbf is in some
   * weird (non utf8) encoding.
   */
  cpg?: string;
}

// /**
//  * @param input
//  */
// export function fromGzip(input: string) {}

// /**
//  * @param input
//  */
// export function fromURL(input: string) {}

/**
 * Assumes the input is pointing to a shapefile or name without the extension.
 * The algorithm will find the rest of the paths if they exist.
 * @param input - the path to the .shp file or name without the extension
 * @returns - a Shapefile
 */
export async function fromPath(input: string) {
  const path = input.replace('.shp', '');
  const shp = `${path}.shp`;
  const dbf = `${path}.dbf`;
  const prj = `${path}.prj`;
  const cpg = `${path}.cpg`;
  if (!(await exists(shp))) throw new Error('Shapefile does not exist');
  const definition: Definition = {
    shp,
    dbf: (await exists(dbf)) ? dbf : undefined,
    prj: (await exists(prj)) ? prj : undefined,
    cpg: (await exists(cpg)) ? cpg : undefined,
  };
  return fromDefinition(definition);
}

/**
 * @param def
 */
export async function fromDefinition(def: Definition): Promise<Shapefile> {
  const { shp, dbf, cpg } = def; // TODO: prj
  const encoding = cpg ? await readFile(cpg, { encoding: 'utf8' }) : 'utf8';
  const dbfBuffer = dbf !== undefined ? new DataView((await readFile(dbf)).buffer) : undefined;
  const databaseFile = dbfBuffer !== undefined ? new DataBaseFile(dbfBuffer, encoding) : undefined;
  // TODO: Projection
  const shpBuffer = new DataView((await readFile(shp)).buffer);
  return new Shapefile(shpBuffer, undefined, databaseFile);
}

// /**
//  * @param root0
//  * @param root0."0"
//  * @param root0."1"
//  */
// export const combine = function ([shp, dbf]) {
//   const out = {};
//   out.type = 'FeatureCollection';
//   out.features = [];
//   let i = 0;
//   const len = shp.length;
//   if (!dbf) {
//     dbf = [];
//   }
//   while (i < len) {
//     out.features.push({
//       type: 'Feature',
//       geometry: shp[i],
//       properties: dbf[i] || {},
//     });
//     i++;
//   }
//   return out;
// };
// /**
//  * @param buffer
//  * @param whiteList
//  */
// export const parseZip = async function (buffer, whiteList) {
//   let key;
//   buffer = toUitn8Arr(buffer);
//   const zip = await unzip(buffer);
//   const names = [];
//   whiteList = whiteList || [];
//   for (key in zip) {
//     if (key.indexOf('__MACOSX') !== -1) {
//       continue;
//     }
//     if (key.slice(-4).toLowerCase() === '.shp') {
//       names.push(key.slice(0, -4));
//       zip[key.slice(0, -3) + key.slice(-3).toLowerCase()] = zip[key];
//     } else if (key.slice(-4).toLowerCase() === '.prj') {
//       zip[key.slice(0, -3) + key.slice(-3).toLowerCase()] = proj4(zip[key]);
//     } else if (
//       key.slice(-5).toLowerCase() === '.json' ||
//       whiteList.indexOf(key.split('.').pop()) > -1
//     ) {
//       names.push(key.slice(0, -3) + key.slice(-3).toLowerCase());
//     } else if (key.slice(-4).toLowerCase() === '.dbf' || key.slice(-4).toLowerCase() === '.cpg') {
//       zip[key.slice(0, -3) + key.slice(-3).toLowerCase()] = zip[key];
//     }
//   }
//   if (!names.length) {
//     throw new Error('no layers founds');
//   }
//   const geojson = names.map(function (name) {
//     let parsed, dbf;
//     const lastDotIdx = name.lastIndexOf('.');
//     if (lastDotIdx > -1 && name.slice(lastDotIdx).indexOf('json') > -1) {
//       parsed = JSON.parse(zip[name]);
//       parsed.fileName = name.slice(0, lastDotIdx);
//     } else if (whiteList.indexOf(name.slice(lastDotIdx + 1)) > -1) {
//       parsed = zip[name];
//       parsed.fileName = name;
//     } else {
//       if (zip[name + '.dbf']) {
//         dbf = parseDbf(zip[name + '.dbf'], zip[name + '.cpg']);
//       }
//       parsed = combine([parseShp(zip[name + '.shp'], zip[name + '.prj']), dbf]);
//       parsed.fileName = name;
//     }
//     return parsed;
//   });
//   if (geojson.length === 1) {
//     return geojson[0];
//   } else {
//     return geojson;
//   }
// };
// /**
//  * @param base
//  * @param whiteList
//  */
// async function getZip(base, whiteList) {
//   const a = await binaryAjax(base);
//   return parseZip(a, whiteList);
// }
// /**
//  * @param base
//  */
// const handleShp = async (base) => {
//   const args = await Promise.all([binaryAjax(base, 'shp'), binaryAjax(base, 'prj')]);
//   let prj = false;
//   try {
//     if (args[1]) {
//       prj = proj4(args[1]);
//     }
//   } catch (e) {
//     prj = false;
//   }
//   return parseShp(args[0], prj);
// };
// /**
//  * @param base
//  */
// const handleDbf = async (base) => {
//   const [dbf, cpg] = await Promise.all([binaryAjax(base, 'dbf'), binaryAjax(base, 'cpg')]);
//   if (!dbf) {
//     return;
//   }
//   return parseDbf(dbf, cpg);
// };
// /**
//  * @param base
//  * @param suffix
//  */
// const checkSuffix = (base, suffix) => {
//   const url = new URL(base, globalThis?.document?.location);
//   return url.pathname.slice(-4).toLowerCase() === suffix;
// };
// /**
//  * @param root0
//  * @param root0.shp
//  * @param root0.dbf
//  * @param root0.cpg
//  * @param root0.prj
//  */
// const fromObject = ({ shp, dbf, cpg, prj }) => {
//   const things = [_parseShp(shp, prj)];
//   if (dbf) {
//     things.push(_parseDbf(dbf, cpg));
//   }
//   return combine(things);
// };
// /**
//  * @param base
//  * @param whiteList
//  */
// export const getShapefile = async function (base, whiteList) {
//   if (typeof base !== 'string') {
//     if (isArrayBuffer(base) || ArrayBuffer.isView(base) || isDataView(base)) {
//       return parseZip(base);
//     }
//     if (base.shp) {
//       return fromObject(base);
//     }
//     throw new TypeError(
//       'must be a string, some sort of Buffer, or an object with at least a .shp property',
//     );
//   }
//   if (checkSuffix(base, '.zip')) {
//     return getZip(base, whiteList);
//   }
//   if (checkSuffix(base, '.shp')) {
//     base = base.slice(0, -4);
//   }
//   const results = await Promise.all([handleShp(base), handleDbf(base)]);
//   return combine(results);
// };
// /**
//  * @param shp
//  * @param prj
//  */
// const _parseShp = function (shp, prj) {
//   shp = toDataView(shp);
//   prj = toString(prj);
//   if (typeof prj === 'string') {
//     try {
//       prj = proj4(prj);
//     } catch (e) {
//       prj = false;
//     }
//   }
//   return parseShp(shp, prj);
// };
// /**
//  * @param dbf
//  * @param cpg
//  */
// const _parseDbf = function (dbf, cpg) {
//   dbf = toDataView(dbf);
//   cpg = toString(cpg);
//   return parseDbf(dbf, cpg);
// };
// export default getShapefile;
// export { _parseDbf as parseDbf, _parseShp as parseShp };
