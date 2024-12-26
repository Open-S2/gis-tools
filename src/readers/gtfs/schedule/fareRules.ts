import { parseCSVAsRecord } from '../../';

/**
 * # Fare Rules
 *
 * **Optional**
 * Defines how fares in `fare_attributes.txt` apply to an itinerary.
 * For more complex fare structures, multiple combinations of fields
 * (route, origin, destination, zones) can be used.
 */
export class GTFSFareRule {
  /**
   * **Required**
   * Identifies a fare class (`fare_attributes.fare_id`).
   */
  fareId: string;
  /**
   * **Optional**
   * Route associated with this fare. If multiple routes share the same fare,
   * add multiple records in `fare_rules.txt`.
   */
  routeId?: string;
  /**
   * **Optional**
   * Origin zone (`stops.zone_id`). If a fare class applies to multiple origin zones,
   * each zone requires its own record.
   */
  originId?: string;
  /**
   * **Optional**
   * Destination zone (`stops.zone_id`). If a fare class applies to multiple destination zones,
   * each zone requires its own record.
   */
  destinationId?: string;
  /**
   * **Optional**
   * All zones traveled during the trip using this fare class.
   * If multiple zones must be passed, each is listed separately.
   */
  containsId?: string;

  /** @param data - the parsed GTFS CSV data */
  constructor(data: Record<string, string>) {
    this.fareId = data.fare_id;
    this.routeId = data.route_id;
    this.originId = data.origin_id;
    this.destinationId = data.destination_id;
    this.containsId = data.contains_id;
  }
}

/**
 * @param input - the input string to parse from
 * @returns - an array of GTFSFareRules
 */
export function parseGTFSFareRules(input: string): GTFSFareRule[] {
  const data = parseCSVAsRecord(input);
  return data.map((d) => new GTFSFareRule(d));
}
