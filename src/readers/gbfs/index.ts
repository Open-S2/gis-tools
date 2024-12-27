import { parseCSVAsRecord } from '..';
import { GBFSReaderV3, GBFSV3, buildGBFSReaderV3 } from './schemaV3';

export * from './schemaV3';

/** The versions of GBFS reader classes this data could be */
export type GBFSReaders = GBFSReaderV3;

/** The versions of GBFS schemas this data could be */
export type GBFSTypess = GBFSV3;

/**
 * Given a link to a GBFS feed, build the appropriate reader for the feed.
 * Examples:
 * - v3: https://backend.citiz.fr/public/provider/9/gbfs/v3.0/gbfs.json
 * - v2: https://gbfs.helbiz.com/v2.2/durham/gbfs.json
 * - v1: https://gbfs.urbansharing.com/gbfs/gbfs.json
 * @param url - The link to the GBFS feed
 * @returns - a GBFSReader of the appropriate version
 */
export async function buildGBFSReader(url: string): Promise<GBFSReaders> {
  const data = await fetch(url).then(async (res) => (await res.json()) as GBFSTypess);
  const versionMajor = data.version[0];
  if (versionMajor === '3') {
    return await buildGBFSReaderV3(data);
  } else throw Error('Unsupported GBFS version');
}

// TODO:
// - [ ] create schema for v1.1 and v2.3
// - [ ] convert each schema to typescript interfaces/enums/types
// - [ ] given a link, fetch all associated data and put in GBFSReaderV1/GBFSReaderV2/GBFSReaderV3
// - [ ] build VectorFeatures from data

/** System Definition that is returned from the github CSV file. */
export interface System {
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
 * Fetches the systems from the github CSV file
 * @param url - The URL of the CSV file. The default is the one used by GBFS. This variable exists for testing
 * @returns - an array of systems
 */
export async function fetchGTFSSystems(
  url = 'https://raw.githubusercontent.com/MobilityData/gbfs/refs/heads/master/systems.csv',
): Promise<System[]> {
  const res: System[] = [];
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
