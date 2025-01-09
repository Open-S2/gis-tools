import { parseCSVAsRecord } from '../../';

/**
 * Indicates an organization’s role in producing the dataset:
 * 0 or empty = No role
 * 1 = Has the role
 *
 * At least one of is_producer, is_operator, or is_authority should be 1 in each record.
 */
export const enum GTFSAttributionRole {
  None = 0,
  Yes = 1,
}

/**
 * # Attributions
 *
 * **Optional**
 * Defines the attributions applied to the dataset or parts of it.
 * If `agency_id`, `route_id`, or `trip_id` is specified, the attribution
 * applies only to that entity. If none are specified, the attribution
 * applies to the entire dataset.
 *
 * **Primary Key**: (attribution_id) - optional
 */
export class GTFSAttribution {
  /**
   * **Optional**
   * Unique ID that identifies this attribution record.
   * Useful if multiple attributions exist or for referencing translations.
   */
  id?: string;
  /**
   * **Optional**
   * Agency to which this attribution applies (`agency.agency_id`).
   * Must be empty if route_id or trip_id are specified.
   */
  agencyId?: string;
  /**
   * **Optional**
   * Route to which this attribution applies (`routes.route_id`).
   * Must be empty if agency_id or trip_id are specified.
   */
  routeId?: string;
  /**
   * **Optional**
   * Trip to which this attribution applies (`trips.trip_id`).
   * Must be empty if agency_id or route_id are specified.
   */
  tripId?: string;
  /**
   * **Required**
   * Organization name to which the dataset is attributed.
   */
  organizationName: string;
  /**
   * **Optional**
   * 0 or empty = Not a producer, 1 = Is a producer
   */
  isProducer?: GTFSAttributionRole;
  /**
   * **Optional**
   * 0 or empty = Not an operator, 1 = Is an operator
   */
  isOperator?: GTFSAttributionRole;
  /**
   * **Optional**
   * 0 or empty = Not an authority, 1 = Is an authority
   */
  isAuthority?: GTFSAttributionRole;
  /**
   * **Optional**
   * URL of the organization.
   */
  attributionUrl?: string;
  /**
   * **Optional**
   * Email of the organization.
   */
  attributionEmail?: string;
  /**
   * **Optional**
   * Phone number of the organization.
   */
  attributionPhone?: string;

  /** @param data - the parsed GTFS CSV data */
  constructor(data: Record<string, string>) {
    this.id = data.attribution_id;
    this.agencyId = data.agency_id;
    this.routeId = data.route_id;
    this.tripId = data.trip_id;
    this.organizationName = data.organization_name;
    this.attributionUrl = data.attribution_url;
    this.attributionEmail = data.attribution_email;
    this.attributionPhone = data.attribution_phone;

    // Roles: if empty, default to 'None'
    this.isProducer =
      data.is_producer !== undefined
        ? (parseInt(data.is_producer, 10) as GTFSAttributionRole)
        : GTFSAttributionRole.None;
    this.isOperator =
      data.is_operator !== undefined
        ? (parseInt(data.is_operator, 10) as GTFSAttributionRole)
        : GTFSAttributionRole.None;
    this.isAuthority =
      data.is_authority !== undefined
        ? (parseInt(data.is_authority, 10) as GTFSAttributionRole)
        : GTFSAttributionRole.None;
  }
}

/**
 * @param input - the input string to parse from
 * @returns - an array of Attributions
 */
export function parseGTFSAttributions(input: string): Record<string, GTFSAttribution> {
  const data = parseCSVAsRecord(input);
  const res: Record<string, GTFSAttribution> = {};
  for (const d of data) {
    const attribution = new GTFSAttribution(d);
    res[attribution.organizationName] = attribution;
  }

  return res;
}
