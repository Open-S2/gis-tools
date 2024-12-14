/** GFS sources available for download */
export type Grib2GFSSource = 'aws' | 'ftpprd' | 'nomads' | 'google' | 'azure' | string;

/**
 * GFS products available for download
 * `pgrb2.0p25` - common fields, 0.25 degree resolution
 * `pgrb2.0p50` - common fields, 0.50 degree resolution
 * `pgrb2.1p00` - common fields, 1.00 degree resolution
 * `pgrb2b.0p25` - uncommon fields, 0.25 degree resolution
 * `pgrb2b.0p50` - uncommon fields, 0.50 degree resolution
 * `pgrb2b.1p00` - uncommon fields, 1.00 degree resolution
 * `pgrb2full.0p50` - combined grids of 0.50 resolution
 * `sfluxgrb` - surface flux fields, T1534 Semi-Lagrangian grid
 * `goesimpgrb2.0p25` - 0.50 degree resolution
 */
export type Grib2GFSProduct =
  | 'pgrb2.0p25'
  | 'pgrb2.0p50'
  | 'pgrb2.1p00'
  | 'pgrb2b.0p25'
  | 'pgrb2b.0p50'
  | 'pgrb2b.1p00'
  | 'pgrb2full.0p50'
  | 'sfluxgrb'
  | 'goesimpgrb2.0p25';

// TODO: Support GFS Wave
//  model: 'atmos' | 'wave',

/**
 * @param source
 * @param product
 * @param year
 * @param month
 * @param day
 * @param hour
 * @param forecast
 * @param filter
 */
export function fetchGFSAtmos(
  source: Grib2GFSSource,
  product: Grib2GFSProduct,
  year: string,
  month: string,
  day: string,
  hour: '00' | '06' | '12' | '18',
  forecast = '000',
  filter?: string,
): void {
  // TODO: Filters
  // TODO: If year is not 4 chars, month not 2, day not 2, or forecast is not 3 chars, return error
  const link = getGFSLink(source, product, year, month, day, hour, forecast);
  // TODO: pull .idx file FIRST

  // TODO: if filter provided we only fetch the values specified, otherwise fetch entire grib file
}

/**
 * @param source
 * @param product
 * @param model
 * @param year
 * @param month
 * @param day
 * @param hour
 * @param forecast
 */
function getGFSLink(
  source: Grib2GFSSource,
  product: Grib2GFSProduct,
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
  link += `gfs.${year}${month}${day}/${hour}/atmos/gfs.t${hour}z.${product}.f${forecast}`;

  return link;
}

// /**
//  * @param data Buffer containing entire GRIB file contents
//  * @returns Parsed GRIB file object
//  */
// export function parse(data: ArrayBuffer): Array<GRIBPacket> {
//   const buffer = Buffer.from(data);
//   const gribChunks = splitGribChunks(buffer);

//   const packets = gribChunks.map((gribChunk: Buffer) => {
//     const sectionChunks = splitSectionChunks(gribChunk);
//     const sectionValues = parseSections(sectionChunks);
//     const sections = lookupSections(sectionValues);

//     return createPacket(sections);
//   });

//   return packets;
// }
