import { BufferReader } from '..';
import { DataBaseFile } from './dbf';
import { ShapeFileReader } from './shp';
import { Transformer } from '../../proj4';
import { iterItems } from '../../util';

import type { ProjectionTransformDefinition } from '../../proj4';

export * from './dbf';
export * from './shp';

/**
 * Assumes the input is pointing to shapefile data.
 * @param input - raw buffer of gzipped data (folder of shp, dbf, prj, and/or cpg)
 * @param defs - optional array of ProjectionTransformDefinitions to insert
 * @param epsgCodes - a record of EPSG codes to use for the transformer if needed
 * @returns - a Shapefile
 */
export async function shapefileFromGzip(
  input: ArrayBufferLike,
  defs?: ProjectionTransformDefinition[],
  epsgCodes?: Record<string, string>,
): Promise<ShapeFileReader> {
  let encoding = 'utf8';
  let transform: Transformer | undefined = undefined;
  let dbfReader: DataBaseFile | undefined = undefined;
  let shpData: Uint8Array | undefined = undefined;
  for (const item of iterItems(new Uint8Array(input))) {
    if (item.filename.endsWith('cpg')) {
      encoding = new TextDecoder('utf8').decode(await item.read());
    } else if (item.filename.endsWith('dbf')) {
      const data = await item.read();
      dbfReader = new DataBaseFile(new BufferReader(data.buffer), encoding);
    } else if (item.filename.endsWith('shp')) {
      shpData = await item.read();
    } else if (item.filename.endsWith('prj')) {
      const data = await item.read();
      transform = new Transformer(new TextDecoder('utf8').decode(data));
      if (defs !== undefined) {
        for (const def of defs) transform.insertDefinition(def);
      }
      if (epsgCodes !== undefined) {
        for (const [key, value] of Object.entries(epsgCodes)) transform.insertEPSGCode(key, value);
      }
    }
  }
  if (shpData === undefined) throw new Error('Shapefile not found');
  return new ShapeFileReader(new BufferReader(shpData.buffer), dbfReader, transform);
}

/**
 * Assumes the input is pointing to shapefile data or a gzipped folder with .shp, .dbf, .prj, and/or .cpg
 * @param url - the url to the shapefile
 * @returns - a Shapefile
 */
export async function shapefileFromURL(url: string): Promise<ShapeFileReader> {
  const data = await fetchShapefile(url);
  if (url.endsWith('.zip')) return shapefileFromGzip(data);
  return new ShapeFileReader(new BufferReader(data));
}

/**
 * Fetches a shapefile or gzipped folder
 * @param url - the url to the shapefile
 * @returns - raw data of a shapefile OR a gzipped folder that may include the dbf, prj, and/or cpg
 */
async function fetchShapefile(url: string): Promise<ArrayBufferLike> {
  return await fetch(url)
    .then(async (res) => {
      if (!res.ok) throw new Error(`Failed to fetch data from ${url}`);
      return await res.arrayBuffer();
    })
    .catch((err) => {
      throw new Error(`Failed to fetch data from ${url}: ${err}`);
    });
}
