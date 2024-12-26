import { parseCSVAsRecord } from '../../';

/**
 * # Fare Leg Rules
 *
 * **Optional**
 * Defines more granular fare rules for individual legs of travel (GTFS-Fares V2).
 *
 * Use these rules by filtering on:
 * - `network_id`
 * - `from_area_id`
 * - `to_area_id`
 * - `from_timeframe_group_id`
 * - `to_timeframe_group_id`
 *
 * Multiple matching strategies exist depending on the presence or absence of `rule_priority`:
 * - If `rule_priority` does **not** exist, empty fields represent an **inverse** match against
 *   all possible values **except** those otherwise specified.
 * - If `rule_priority` **does** exist, empty fields mean the field does not affect matching.
 *
 * For matching an “effective fare leg” that spans multiple legs, see the specification for
 * rules on using the first vs. last leg’s departure/arrival areas and timeframes.
 */
export class GTFSFareLegRule {
  /**
   * **Optional**
   * Identifies a group of entries in fare_leg_rules.txt that can be referenced
   * in `fare_transfer_rules.from_leg_group_id` or `fare_transfer_rules.to_leg_group_id`.
   */
  legGroupId?: string;
  /**
   * **Optional**
   * Identifies a route network (`routes.network_id` or `networks.network_id`) this rule applies to.
   * - If `rule_priority` is omitted and no matching `network_id`, empty matches “all but listed”.
   * - If `rule_priority` exists, empty means network does not affect matching.
   */
  networkId?: string;
  /**
   * **Optional**
   * Identifies a departure area (`areas.area_id`) for this fare leg rule.
   * - If `rule_priority` is omitted and no matching `from_area_id`, empty matches “all but listed”.
   * - If `rule_priority` exists, empty means departure area does not affect matching.
   */
  fromAreaId?: string;
  /**
   * **Optional**
   * Identifies an arrival area (`areas.area_id`) for this fare leg rule.
   * - If `rule_priority` is omitted and no matching `to_area_id`, empty matches “all but listed”.
   * - If `rule_priority` exists, empty means arrival area does not affect matching.
   */
  toAreaId?: string;
  /**
   * **Optional**
   * References a `timeframes.timeframe_group_id` for the start of the fare leg.
   * An empty value means the start time does not affect matching.
   */
  fromTimeframeGroupId?: string;
  /**
   * **Optional**
   * References a `timeframes.timeframe_group_id` for the end of the fare leg.
   * An empty value means the end time does not affect matching.
   */
  toTimeframeGroupId?: string;
  /**
   * **Required**
   * References a `fare_products.fare_product_id`.
   * The rider must possess/purchase this fare product for the described leg.
   */
  fareProductId: string;
  /**
   * **Optional**
   * Defines the order of priority in which matching rules are applied.
   * - Higher priority overrides lower priority when multiple rules match.
   * - Empty is treated as zero.
   */
  rulePriority?: number;

  /** @param data - the parsed GTFS CSV data */
  constructor(data: Record<string, string>) {
    this.legGroupId = data.leg_group_id;
    this.networkId = data.network_id;
    this.fromAreaId = data.from_area_id;
    this.toAreaId = data.to_area_id;
    this.fromTimeframeGroupId = data.from_timeframe_group_id;
    this.toTimeframeGroupId = data.to_timeframe_group_id;
    // Required field
    this.fareProductId = data.fare_product_id;
    // If empty, treat as zero; otherwise parse as integer
    if (data.rule_priority !== undefined) {
      this.rulePriority = data.rule_priority === undefined ? 0 : parseInt(data.rule_priority, 10);
    }
  }
}

/**
 * @param input - the input string to parse from
 * @returns - an array of GTFSFareLegRules
 */
export function parseGTFSFareLegRules(input: string): GTFSFareLegRule[] {
  const data = parseCSVAsRecord(input);
  return data.map((d) => new GTFSFareLegRule(d));
}
