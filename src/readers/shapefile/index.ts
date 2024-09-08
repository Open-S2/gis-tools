import { BufferReader } from '..';
import DataBaseFile from './dbf';
import Shapefile from './shp';

// import proj4 from 'proj4';

export * from './dbf';
export * from './shp';

/**
 * Assumes the input is pointing to shapefile data.
 * @param input - raw buffer of gzipped data (folder of shp, dbf, prj, and/or cpg)
 * @returns - a Shapefile
 */
export function fromGzip(input: ArrayBufferLike): undefined | Shapefile {
  // TODO: UNGZIP HERE
  return undefined;
}

/**
 * Assumes the input is pointing to shapefile data or a gzipped folder with .shp, .dbf, .prj, and/or .cpg
 * @param url - the url to the shapefile
 * @returns - a Shapefile
 */
export async function fromURL(url: string): Promise<undefined | Shapefile> {
  const { isGziped, data } = await fetchShapefile(url);
  if (isGziped) return fromGzip(data);
  return new Shapefile(new BufferReader(data));
}

/** Returned data from fetching a shapefile */
interface FetchResponse {
  data: ArrayBufferLike;
  isGziped: boolean;
}

/**
 * Fetches a shapefile or gzipped folder
 * @param url - the url to the shapefile
 * @returns - raw data of a shapefile OR a gzipped folder that may include the dbf, prj, and/or cpg
 */
async function fetchShapefile(url: string): Promise<FetchResponse> {
  return fetch(url)
    .then(async (res) => {
      const encoding = res.headers.get('encoding');
      if (res.status === 200)
        return { data: await res.arrayBuffer(), isGziped: encoding === 'gzip' };
      else throw new Error(`failed to fetch data from ${url}`);
    })
    .catch((err) => {
      throw new Error(`failed to fetch data from ${url}: ${err}`);
    });
}
