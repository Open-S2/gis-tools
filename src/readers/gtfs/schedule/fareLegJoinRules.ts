import { parseCSVAsRecord } from '../../';

/**
 * # Fare Leg Join Rules
 *
 * **Optional**
 * Defines when two consecutive legs with a transfer should be considered as
 * a single “effective fare leg” for the purpose of matching rules in `fare_leg_rules.txt`.
 *
 * **Primary Key**: (`from_network_id`, `to_network_id`, `from_stop_id`, `to_stop_id`)
 *
 * Matching Logic:
 * - If both `from_network_id` and `to_network_id` match consecutive legs’ networks,
 *   and `from_stop_id`/`to_stop_id` match station or stop IDs for the transfer,
 *   those two legs merge into one effective leg.
 * - If a field is empty, that field is ignored for matching.
 * - Consecutive transfers that each match a join rule merge the entire sub-journey
 *   into a single effective fare leg.
 */
export class GTFSFareLegJoinRule {
  /**
   * **Required**
   * Matches the pre-transfer leg’s route network (`routes.network_id` or `networks.network_id`).
   * Must be specified alongside `toNetworkId`.
   */
  fromNetworkId: string;
  /**
   * **Required**
   * Matches the post-transfer leg’s route network (`routes.network_id` or `networks.network_id`).
   * Must be specified alongside `fromNetworkId`.
   */
  toNetworkId: string;
  /**
   * **Conditionally Required**
   * Matches the pre-transfer leg’s ending stop/station (`stops.stop_id`).
   * Required if `toStopId` is defined; optional otherwise.
   */
  fromStopId?: string;
  /**
   * **Conditionally Required**
   * Matches the post-transfer leg’s starting stop/station (`stops.stop_id`).
   * Required if `fromStopId` is defined; optional otherwise.
   */
  toStopId?: string;

  /** @param data - the parsed GTFS CSV data */
  constructor(data: Record<string, string>) {
    this.fromNetworkId = data.from_network_id;
    this.toNetworkId = data.to_network_id;
    // If from_stop_id is provided, to_stop_id should be defined as well, and vice versa.
    if (data.from_stop_id !== undefined) this.fromStopId = data.from_stop_id;
    if (data.to_stop_id !== undefined) this.toStopId = data.to_stop_id;
  }
}

/**
 * @param input - the input string to parse from
 * @returns - an array of GTFSFareLegJoinRules
 */
export function parseGTFSFareLegJoinRules(input: string): GTFSFareLegJoinRule[] {
  const data = parseCSVAsRecord(input);
  return data.map((d) => new GTFSFareLegJoinRule(d));
}
