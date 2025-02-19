import { splitSectionChunks } from './sections';
import { BufferReader, toReader } from '..';

import type { FeatureIterator, Reader, ReaderInputs } from '..';
import type { Grib2ProductDefinition, Grib2Sections } from './sections';
import type { Properties, VectorFeature, VectorMultiPointGeometry } from '../../geometry';

export * from './sections';

export type { Grib2ProductDefinition, getGrib2Template4 } from './sections';

/** GFS sources available for download */
export type Grib2GFSSource = 'aws' | 'ftpprd' | 'nomads' | 'google' | 'azure' | string;

/** GFS domains available for download */
export type Grib2GFSDomain = 'atmos' | 'wave';

/**
 * GFS ATMOS products available for download
 * - `pgrb2.0p25` - common fields, 0.25 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/gfs/gfs.t00z.pgrb2.0p25.f000.shtml)
 * - `pgrb2.0p50` - common fields, 0.50 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/gfs/gfs.t00z.pgrb2.0p50.f000.shtml)
 * - `pgrb2.1p00` - common fields, 1.00 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/gfs/gfs.t00z.pgrb2.1p00.f000.shtml)
 * - `pgrb2b.0p25` - uncommon fields, 0.25 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/gfs/gfs.t00z.pgrb2b.0p25.f000.shtml)
 * - `pgrb2b.0p50` - uncommon fields, 0.50 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/gfs/gfs.t00z.pgrb2b.0p50.f000.shtml)
 * - `pgrb2b.1p00` - uncommon fields, 1.00 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/gfs/gfs.t00z.pgrb2b.1p00.f000.shtml)
 * - `pgrb2full.0p50` - combined grids of 0.50 resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/gfs/gfs.t12z.pgrb2full.0p50.f000.shtml)
 * - `sfluxgrb` - surface flux fields, T1534 Semi-Lagrangian grid [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/gfs/gfs.t00z.sfluxgrbf000.grib2.shtml)
 * - `goesimpgrb2.0p25` - 0.50 degree resolution for GOES-IMP [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/gfs/gfs.t00z.goessimpgrb2.0p25.f000.shtml)
 */
export type Grib2AtmosGFSProduct =
  | 'pgrb2.0p25'
  | 'pgrb2.0p50'
  | 'pgrb2.1p00'
  | 'pgrb2b.0p25'
  | 'pgrb2b.0p50'
  | 'pgrb2b.1p00'
  | 'pgrb2full.0p50'
  | 'sfluxgrb'
  | 'goesimpgrb2.0p25';

/**
 * GFS WAVE products available for download
 * - `arctic.9km` - Arctic, 9km resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/wave/gfswave.t12z.arctic.9km.f003.grib2.shtml)
 * - `atlocn.0p16` - Atlantic, 0.16 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/wave/gfswave.t12z.atlocn.0p16.f003.grib2.shtml)
 * - `epacif.0p16` - Eastern Pacific, 0.16 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/wave/gfswave.t12z.epacif.0p16.f003.grib2.shtml)
 * - `global.0p16` - Global, 0.16 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/wave/gfswave.t12z.global.0p16.f003.grib2.shtml)
 * - `global.0p25` - Global, 0.25 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/wave/gfswave.t12z.global.0p25.f003.grib2.shtml)
 * - `gsouth.0p25` - Gulf of South America, 0.25 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/wave/gfswave.t12z.gsouth.0p25.f003.grib2.shtml)
 * - `wcoast.0p16` - West Coast, 0.16 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/wave/gfswave.t12z.wcoast.0p16.f003.grib2.shtml)
 */
export type Grib2WaveGFSProduct =
  | 'arctic.9km'
  | 'atlocn.0p16'
  | 'epacif.0p16'
  | 'global.0p16'
  | 'global.0p25'
  | 'gsouth.0p25'
  | 'wcoast.0p16';

/**
 * Fetch ATMOS GFS data.
 *
 * You can find some data to reference what's available [here](https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/).
 *
 * An example of what variable data means can be found [here](https://www.nco.ncep.noaa.gov/pmb/products/gfs/gfs.t00z.pgrb2.0p50.f000.shtml).
 * @param source - The source of the data, `aws` | `ftpprd` | `nomads` | `google` | `azure` | or a user provided url
 * @param product - which product to fetch
 * @param year - The year to fetch given a 4 digit year
 * @param month - The month to fetch given a 2 digit month 01 is January and 12 is December
 * @param day - The day to fetch given a 2 digit day, e.g. '01' or '31'
 * @param hour - The forecast hour with 2 digits often in increments of 6 up to 18, e.g. '00' or '12'
 * @param forecast - The forecast hour with 3 digits often in increments of 3 up to 384, e.g. '000' or '003'
 * @param filters - The filters to apply by filtering lines in the .idx file
 * @returns - A GRIB2Reader of the specific sections
 */
export async function fetchGFSAtmos(
  source: Grib2GFSSource,
  product: Grib2AtmosGFSProduct,
  year: string,
  month: string,
  day: string,
  hour: '00' | '06' | '12' | '18',
  forecast = '000',
  filters?: string[],
): Promise<GRIB2Reader> {
  // If year is not 4 chars, month not 2, day not 2, or forecast is not 3 chars, return error
  if (year.length !== 4 || month.length !== 2 || day.length !== 2 || forecast.length !== 3) {
    throw new Error(
      'Year, month, day, and forecast must be 4, 2, 2, and 3 characters, respectively.',
    );
  }
  const link = getGFSLink(source, product, 'atmos', year, month, day, hour, forecast);
  // pull .idx file FIRST
  const idxs = await parsedIDXFromURL(`${link}.idx`, filters ?? []);

  return await GRIB2Reader.fromIDX(link, idxs);
}

/**
 * Fetch WAVE GFS data.
 *
 * You can find some data to reference what's available [here](https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/).
 *
 * An example of what variable data means can be found [here](https://www.nco.ncep.noaa.gov/pmb/products/wave/gfswave.t12z.arctic.9km.f003.grib2.shtml).
 * @param source - The source of the data, `aws` | `ftpprd` | `nomads` | `google` | `azure` | or a user provided url
 * @param product - which product to fetch
 * @param year - The year to fetch given a 4 digit year
 * @param month - The month to fetch given a 2 digit month 01 is January and 12 is December
 * @param day - The day to fetch given a 2 digit day, e.g. '01' or '31'
 * @param hour - The forecast hour with 2 digits often in increments of 6 up to 18, e.g. '00' or '12'
 * @param forecast - The forecast hour with 3 digits often in increments of 3 up to 384, e.g. '000' or '003'
 * @param filters - The filters to apply by filtering lines in the .idx file
 * @returns - A GRIB2Reader of the specific sections
 */
export async function fetchGFSWave(
  source: Grib2GFSSource,
  product: Grib2WaveGFSProduct,
  year: string,
  month: string,
  day: string,
  hour: '00' | '06' | '12' | '18',
  forecast = '000',
  filters?: string[],
): Promise<GRIB2Reader> {
  // If year is not 4 chars, month not 2, day not 2, or forecast is not 3 chars, return error
  if (year.length !== 4 || month.length !== 2 || day.length !== 2 || forecast.length !== 3) {
    throw new Error(
      'Year, month, day, and forecast must be 4, 2, 2, and 3 characters, respectively.',
    );
  }
  const link = getGFSLink(source, product, 'wave', year, month, day, hour, forecast);
  // pull .idx file FIRST
  const idxs = await parsedIDXFromURL(`${link}.idx`, filters ?? []);

  return await GRIB2Reader.fromIDX(link, idxs);
}

/**
 * Get the link to download GFS Atmos data
 * @param source - The source of the data, `aws` | `ftpprd` | `nomads` | `google` | `azure` | or a user provided url
 * @param product - which product to fetch
 * @param domain - The domain of the data, either 'atmos' for atmospheric data or 'wave' for ocean wave data
 * @param year - The year to fetch given a 4 digit year
 * @param month - The month to fetch given a 2 digit month 01 is January and 12 is December
 * @param day - The day to fetch given a 2 digit day, e.g. '01' or '31'
 * @param hour - The forecast hour with 2 digits often in increments of 6 up to 18, e.g. '00' or '12'
 * @param forecast - The forecast hour with 3 digits often in increments of 3 up to 384, e.g. '000' or '003'
 * @returns - A GRIB2Reader of the specific sections
 */
export function getGFSLink(
  source: Grib2GFSSource,
  product: Grib2AtmosGFSProduct | Grib2WaveGFSProduct,
  domain: Grib2GFSDomain,
  year: string,
  month: string,
  day: string,
  hour: '00' | '06' | '12' | '18',
  forecast: string,
): string {
  let link = '';
  if (source === 'aws') {
    link = `https://noaa-gfs-bdp-pds.s3.amazonaws.com/`;
  } else if (source === 'ftpprd') {
    link = 'https://ftpprd.ncep.noaa.gov/data/nccf/com/gfs/prod/';
  } else if (source === 'nomads') {
    link = 'https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/';
  } else if (source === 'google') {
    link = 'https://storage.googleapis.com/global-forecast-system/';
  } else if (source === 'azure') {
    link = 'https://noaagfs.blob.core.windows.net/gfs/';
  } else {
    link = source;
  }
  const domainStr = domain === 'atmos' ? 'atmos' : 'wave/gridded';
  const startName = domain === 'atmos' ? 'gfs' : 'gfswave';
  const endName = domain === 'atmos' ? '' : '.grib2';
  link += `gfs.${year}${month}${day}/${hour}/${domainStr}/${startName}.t${hour}z.${product}.f${forecast}${endName}`;

  return link;
}

/** Description of a section in the GRIB2 file */
export interface SectionLocations {
  /** Start/offset of section */
  start: number;
  /** If missing, assume the end is the end of the file */
  end?: number;
  /** The entire line detailing the section */
  line: string;
  /** The name of the filter */
  name: string;
}

/**
 * Parse the .idx file for GRIB2 section details using a URL
 * @param url - The URL of the .idx file
 * @param filters - The filters to apply
 * @param offsetPosition - The position of the offset in the ":" sequence
 * @returns - An array of SectionLocations
 */
export async function parsedIDXFromURL(
  url: string,
  filters: string[],
  offsetPosition = 1,
): Promise<SectionLocations[]> {
  const data = await fetch(url).then(async (res) => await res.text());
  return parseIDX(data, filters, offsetPosition);
}

/**
 * Parse the .idx file for GRIB2 section details
 * @param data - The contents of the .idx file
 * @param filters - The filters to apply
 * @param offsetPosition - The position of the offset in the ":" sequence
 * @returns - An array of SectionLocations
 */
export function parseIDX(data: string, filters: string[], offsetPosition = 1): SectionLocations[] {
  let res: SectionLocations[] = [];
  // split lines, parse information, and add to array
  const lines = data.split('\n');
  for (const line of lines) {
    if (line.length === 0) continue;
    const pieces = line.split(':');
    const offset = parseInt(pieces[offsetPosition], 10);
    res.push({ start: offset, line, name: line });
  }
  // now add the "end"s
  for (let i = 0; i < res.length - 1; i++) res[i].end = res[i + 1].start;
  // lastly add the filters
  if (filters.length > 0) {
    res = res.filter((sL) => filters.some((f) => sL.line.includes(f)));
  }
  // set names to filter names
  for (let i = 0; i < res.length; i++) res[i].name = filters[i];

  return res;
}

/**
 * # GRIB2 Reader
 *
 * ## Description
 *
 * This class reads a GRIB2 file and returns a list of GRIB2 products.
 * Implements the {@link FeatureIterator} interface.
 *
 * ## Usage
 *
 * ### The recommended way to parse grib files is to filter out what you want:
 * ```ts
 * // pull .idx file FIRST and filter the ones you want
 * const filters = [':DZDT:0.01 mb:', ':TMP:0.4 mb:', ':ABSV:0.4 mb:anl:'];
 * const idxs = await parsedIDXFromURL(`${link}.idx`, filters);
 * // now bulid the reader
 * const gribReader =  await GRIB2Reader.fromIDX(link, idxs);
 *
 * for await (const feature of gribReader) {
 *   console.log(feature);
 * }
 * ```
 *
 * ### Parsing the entire grib file:
 * ```ts
 * const gribReader = new GRIB2Reader(link);
 * ```
 *
 * ## Links
 * - https://en.wikipedia.org/wiki/GRIB
 * - https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/
 */
export class GRIB2Reader implements FeatureIterator<Grib2ProductDefinition[]> {
  packets: Grib2Sections[] = [];
  /**
   * @param readers - Reader(s) for entire GRIB file. If array, its grib chunks, otherwise it will be the entire file
   * @param idxs - The list of section locations we will be parsing
   */
  constructor(
    readers: ReaderInputs | Reader[],
    private idxs?: SectionLocations[],
  ) {
    const gribChunks = Array.isArray(readers) ? readers : splitGribChunks(toReader(readers));
    for (const gribChunk of gribChunks) this.packets.push(splitSectionChunks(gribChunk));
  }

  /**
   * Create a GRIB2Reader from a .idx file
   * @param source - Either the http path to the .idx file or the entire GRIB file
   * @param idxs - The parsed .idx file with the locations of each section
   * @returns A GRIB2Reader of the specific sections
   */
  static async fromIDX(source: string | Reader, idxs: SectionLocations[]): Promise<GRIB2Reader> {
    const readers: Reader[] = [];
    if (typeof source === 'string') {
      for (const { start, end } of idxs) {
        const chunk = await fetch(source, {
          headers: { Range: `bytes=${start}-${end === undefined ? '' : end}` },
        }).then(async (res) => await res.arrayBuffer());
        readers.push(new BufferReader(chunk));
      }
    } else {
      for (const idx of idxs) {
        readers.push(new BufferReader(source.slice(idx.start, idx.end).buffer));
      }
    }
    return new GRIB2Reader(readers, idxs);
  }

  /**
   * Iterate through each packet and add their products to the geometry
   * @yields {VectorFeature}
   */
  async *[Symbol.asyncIterator](): AsyncGenerator<
    VectorFeature<
      Grib2ProductDefinition[],
      Record<string, number>,
      Properties,
      VectorMultiPointGeometry<Record<string, number>>
    >
  > {
    // setup metadata
    const productMetadata: Grib2ProductDefinition[] = this.packets
      .map((packet) => packet.productDefinition?.values)
      .filter((p) => p !== undefined);
    // setup geometry
    const geometry = this.packets[0].gridDefinition?.values.buildGrid();
    if (geometry === undefined) return;
    // add M-Values from each packet
    // for (const packet of this.packets) {
    for (let i = 0; i < this.packets.length; i++) {
      const packet = this.packets[i];
      const name = this.idxs?.[i]?.name ?? String(i);
      const data = packet.data?.getData();
      if (data === undefined) continue;
      for (let i = 0; i < data.length; i++) {
        const mValue = data[i];
        if (mValue === undefined) continue;
        const geo = geometry[i];
        if (geo.m === undefined) geo.m = {};
        geo.m[name] = mValue;
      }
    }

    yield {
      type: 'VectorFeature',
      geometry: {
        type: 'MultiPoint',
        coordinates: geometry,
        is3D: false,
      },
      properties: {},
      metadata: productMetadata,
    };
  }
}

/**
 * Split the bytes of the GRIB file into individual GRIB chunks that represent sections
 * @param reader - Reader for entire GRIB file
 * @returns Array of GRIB Chunk Buffers containing individual GRIB definitions in file
 */
function splitGribChunks(reader: Reader): Reader[] {
  if (reader.byteLength === 0) return [];
  const length = Number(reader.slice(8, 16).getBigUint64(0));
  const gribData = new BufferReader(reader.slice(0, length).buffer);

  const chunks: Reader[] = [gribData];
  if (length === reader.byteLength) return chunks;
  const rest = new BufferReader(reader.slice(length).buffer);
  chunks.push(...splitGribChunks(rest));

  return chunks;
}
