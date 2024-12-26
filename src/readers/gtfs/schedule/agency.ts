import { parseCSVAsRecord } from '../../';

/**
 * # Agency Information
 *
 * ## Details
 * **Required** - Transit agencies with service represented in this dataset.
 */
export class GTFSAgency {
  /**
   * **Required**
   * Identifies a location: stop/platform, station, entrance/exit, generic node or boarding area (see location_type).
   * ID must be unique across all stops. `stop_id`, locations.geojson id, and location_groups.location_group_id values.
   * Multiple routes may use the same `stop_id`.
   */
  id: string;
  /**
   * **Required**
   * Full name of the transit agency.
   */
  name: string;
  /**
   * **Required**
   * URL of the transit agency.
   */
  url: string;
  /**
   * **Required**
   * Timezone where the transit agency is located.
   * If multiple agencies are specified in the dataset, each must have the same `agency_timezone`.
   */
  timezone: string;
  /**
   * **Optional**
   * Primary language used by this transit agency.
   * Should be provided to help GTFS consumers choose capitalization rules and other language-specific settings for the dataset.
   * See [ISO 639](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) for language codes.
   */
  lang?: string;
  /**
   * **Optional**
   * A voice telephone number for the specified agency.
   * This field is a string value that presents the telephone number as typical for the agency's service area.
   */
  phone?: string;
  /**
   * **Optional**
   * URL of a web page that allows a rider to purchase tickets or other fare instruments for that
   * agency online.
   */
  fareURL?: string;
  /**
   * **Optional**
   * Email address actively monitored by the agency’s customer service department. This email
   * address should be a direct contact point where transit riders can reach a customer service
   * representative at the agency.
   */
  email?: string;

  /** @param data - the parsed GTFS CSV data */
  constructor(data: Record<string, string>) {
    this.id = data.agency_id;
    this.name = data.agency_name;
    this.url = data.agency_url;
    this.timezone = data.agency_timezone;
    this.lang = data.agency_lang;
    this.phone = data.agency_phone;
    this.fareURL = data.agency_fare_url;
    this.email = data.agency_email;
  }
}

/**
 * @param input - the input string to parse from
 * @returns - an array of Agencies
 */
export function parseGTFSAgencies(input: string): Record<string, GTFSAgency> {
  const data = parseCSVAsRecord(input);
  const res: Record<string, GTFSAgency> = {};
  for (const d of data) {
    const agency = new GTFSAgency(d);
    res[agency.id] = agency;
  }

  return res;
}
