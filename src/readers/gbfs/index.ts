import { parseCSVAsRecord } from '..';
import { GBFSReaderV1, GBFSV1, buildGBFSReaderV1 } from './schemaV1';
import { GBFSReaderV2, GBFSV2, buildGBFSReaderV2 } from './schemaV2';
import { GBFSReaderV3, GBFSV3, buildGBFSReaderV3 } from './schemaV3';

export * from './schemaV1';
export * from './schemaV2';
export * from './schemaV3';

/**
 * # General Bikeshare Feed Specification (GBFS) Reader
 *
 * ## Description
 * The versions of GBFS reader classes this data could be (1, 2, or 3)
 * Implements the {@link FeatureIterator} interface.
 *
 * ## Usage
 *
 * ```ts
 * import { buildGBFSReader } from 'gis-tools-ts';
 *
 * const reader = await buildGBFSReader('https://gbfs.urbansharing.com/gbfs/gbfs.json');
 * // read the features
 * for await (const feature of reader) {
 *   // do something with the feature
 * }
 * ```
 *
 * ## Links
 * - https://github.com/MobilityData/gbfs
 * - https://github.com/MobilityData/gbfs-json-schema/tree/master/v3.0
 */
export type GBFSReader = GBFSReaderV1 | GBFSReaderV2 | GBFSReaderV3;

/** The versions of GBFS schemas this data could be */
export type GBFSTypes = GBFSV1 | GBFSV2 | GBFSV3;

/**
 * # General Bikeshare Feed Specification (GBFS) Reader
 *
 * ## Description
 * Given a link to a GBFS feed, build the appropriate reader for the feed.
 * The versions of GBFS reader classes this data could be (1, 2, or 3).
 * Implements the {@link FeatureIterator} interface.
 *
 * ## Usage
 *
 * ```ts
 * import { buildGBFSReader } from 'gis-tools-ts';
 *
 * const reader = await buildGBFSReader('https://gbfs.urbansharing.com/gbfs/gbfs.json');
 * // read the features
 * for await (const feature of reader) {
 *   // do something with the feature
 * }
 * ```
 *
 * ## Links
 * - https://github.com/MobilityData/gbfs
 * - https://github.com/MobilityData/gbfs-json-schema/tree/master/v3.0
 * - v3 example data: https://backend.citiz.fr/public/provider/9/gbfs/v3.0/gbfs.json
 * - v2 example data: https://gbfs.helbiz.com/v2.2/durham/gbfs.json
 * - v1 example data: https://gbfs.urbansharing.com/gbfs/gbfs.json
 * @param url - The link to the GBFS feed
 * @param locale - The locale to use if provided, otherwise default to "en" (e.g., "en", "en-US").
 * @returns - a GBFSReader of the appropriate version
 */
export async function buildGBFSReader(url: string, locale = 'en'): Promise<GBFSReader> {
  const data = await fetch(url).then(async (res) => (await res.json()) as GBFSTypes);
  const path = url.includes('localhost') ? url.split('/').slice(0, -1).join('/') : undefined;
  const versionMajor = 'version' in data ? data.version[0] : '1';
  if (versionMajor === '1') {
    return await buildGBFSReaderV1(data as GBFSV1, locale, path);
  } else if (versionMajor === '2') {
    return await buildGBFSReaderV2(data as GBFSV2, locale, path);
  } else if (versionMajor === '3') {
    return await buildGBFSReaderV3(data as GBFSV3, path);
  } else throw Error('Unsupported GBFS version');
}

/** System Definition that is returned from the github CSV file. */
export interface GBFSSystem {
  /** [**Required**] ISO 3166-1 alpha-2 code designating the country where the system is located. */
  countryCode: string;
  /** [**Required**] Name of the mobility system. This MUST match the name field in system_information.json */
  name: string;
  /**
   * [**Required**] Primary city in which the system is located, followed by the 2-letter state code
   * for US systems. The location name SHOULD be in English if the location has an English name
   * (eg: Brussels).
   */
  location: string;
  /** [**Required**] ID for the system. This MUST match the system_id field in system_information.json. */
  systemId: string;
  /**
   * [**Required**] URL for the system from the url field in system_information.json.
   * If the url field is not included in system_information.json this SHOULD be the primary URL
   * for the system operator.
   */
  url: string;
  /** [**Required**] URL for the system's gbfs.json auto-discovery file. */
  autoDiscoveryUrl: string;
  /**
   * [**Required**] List of GBFS version(s) under which the feed is published. Multiple values are
   * separated by a semi-colon surrounded with 1 space on each side for readability (" ; ").
   */
  supportedVersions: string[];
  /**
   * [**Conditionally Required**] If authentication is required, this MUST contain a URL to a
   * human-readable page describing how the authentication should be performed and how credentials
   * can be created, or directly contain the public key-value pair to append to the feed URLs.
   */
  authInfo?: string;
}

/**
 * # General Bikeshare Feed Specification (GBFS) Reader
 *
 * ## Description
 * Fetches the list of GBFS systems from the github CSV file
 *
 * ## Usage
 *
 * ```ts
 * import { fetchGTFSSystems } from 'gis-tools-ts';
 *
 * const systems = await fetchGTFSSystems();
 * console.log(systems);
 * ```
 *
 * ## Links
 * - https://github.com/MobilityData/gbfs/blob/master/systems.csv
 * @param url - The URL of the CSV file. The default is the one used by GBFS. This variable exists for testing
 * @returns - an array of systems
 */
export async function fetchGTFSSystems(
  url = 'https://raw.githubusercontent.com/MobilityData/gbfs/refs/heads/master/systems.csv',
): Promise<GBFSSystem[]> {
  const res: GBFSSystem[] = [];
  const data = await fetch(url).then(async (res) => await res.text());
  const parsed = parseCSVAsRecord(data);

  for (const system of parsed) {
    const { Name: name, Location: location, URL: url } = system;
    const countryCode = system['Country Code'];
    const systemId = system['System ID'];
    const autoDiscoveryUrl = system['Auto-Discovery URL'];
    const supportedVersions = (system['Supported Versions'] ?? '').split(' ; ');
    const authInfo = system['Authentication Info'];
    res.push({
      name,
      location,
      url,
      countryCode,
      systemId,
      autoDiscoveryUrl,
      supportedVersions,
      authInfo,
    });
  }

  return res;
}
