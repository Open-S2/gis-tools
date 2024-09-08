import DataBaseFile from './dbf';
import Shapefile from './shp';

// import proj4 from 'proj4';

export * from './dbf';
export * from './shp';

/**
 * @param input
 */
export function fromGzip(input: ArrayBufferLike): Shapefile {}

/**
 * @param url - the url to the shapefile
 * @returns - a Shapefile
 */
export async function fromURL(url: string): Promise<Shapefile> {
  const { isGziped, data } = await fetchShapefile(url);
  if (isGziped) return fromGzip(data);
  return new Shapefile(new DataView(data));
}

/** Returned data from fetching a shapefile */
interface FetchResponse {
  data: ArrayBufferLike;
  isGziped: boolean;
}

/**
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
