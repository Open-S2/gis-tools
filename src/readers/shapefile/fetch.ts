// TODO: Fetch from URL, if gziped we need to decode all of it
// TODO: otherwise we only have a shapefile. Return 3 cases: nothing (error), shapefile, and gzipped shapefile

/**
 * @param _url
 * @param type
 * @param url
 */
export default async function binaryAjax(url: string) {
  const isOptionalTxt = type === 'prj' || type === 'cpg';
  try {
    const resp = await fetch(url);
    if (resp.status > 399) {
      throw new Error(resp.statusText);
    }
    if (isOptionalTxt) {
      return resp.text();
    }
    const parsed = await resp.arrayBuffer();
    return new DataView(parsed);
  } catch (e) {
    if (isOptionalTxt || type === 'dbf') {
      return false;
    }
    throw e;
  }
}
