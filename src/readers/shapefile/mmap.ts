import { DataBaseFile } from './dbf';
import { MMapReader } from '../mmap';
import { ShapeFileReader } from './shp';
import { Transformer } from '../../proj4';
import { shapefileFromGzip } from '.';
import { exists, readFile } from 'fs/promises';

export * from './dbf';
export * from './shp';

/** A description of what relevant files exist and where */
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

/**
 * # Build a Shapefile from an input path
 *
 * ## Description
 * Given a path to where all the shapefile relevant files exist, build a Shapefile
 *
 * Assumes the input is pointing to a shapefile or name without the extension.
 * The algorithm will find the rest of the paths if they exist. May also be a gzipped folder.
 *
 * ## Usage
 * ```ts
 * import { LambertConformalConic, EPSG_9974 } from 'gis-tools-ts';
 * import { shapefileFromPath } from 'gis-tools-ts/mmap';
 *
 * const reader = await shapefileFromPath('path/to/files', [LambertConformalConic], { EPSG_9974 });
 *
 * for await (const feature of reader) {
 *   console.log(feature);
 * }
 * ```
 * @param input - the path to the .shp file or name without the extension
 * @returns - a Shapefile
 */
export async function shapefileFromPath(input: string) {
  if (input.endsWith('.zip')) {
    const gzipData = await readFile(input);
    return shapefileFromGzip(gzipData.buffer);
  }
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
  return shapefileFromDefinition(definition);
}

/**
 * # Build a Shapefile from a Definition
 *
 * ## Description
 * Given a collection of files, build a Shapefile
 *
 * ## Usage
 * ```ts
 * import { LambertConformalConic, EPSG_9974 } from 'gis-tools-ts';
 * import { shapefileFromDefinition } from 'gis-tools-ts/mmap';
 * import type { Definition } from 'gis-tools-ts';
 *
 * const def: Definition = {
 *   shp: 'path/to/file.shp',
 *   dbf: 'path/to/file.dbf',
 *   prj: 'path/to/file.prj',
 *   cpg: 'path/to/file.cpg'
 * };
 *
 * const reader = await shapefileFromDefinition(def, [LambertConformalConic], { EPSG_9974 });
 *
 * for await (const feature of reader) {
 *   console.log(feature);
 * }
 * ```
 * @param def - a description of the data to parse
 * @returns - a Shapefile
 */
export async function shapefileFromDefinition(def: Definition): Promise<ShapeFileReader> {
  const { shp, dbf, prj, cpg } = def;
  const encoding = cpg !== undefined ? await readFile(cpg, { encoding: 'utf8' }) : 'utf8';
  const transform =
    prj !== undefined ? new Transformer(await readFile(prj, { encoding: 'utf8' })) : undefined;
  const dbfReader = dbf !== undefined ? new MMapReader(dbf) : undefined;
  const databaseFile = dbfReader !== undefined ? new DataBaseFile(dbfReader, encoding) : undefined;

  return new ShapeFileReader(new MMapReader(shp), databaseFile, transform);
}
