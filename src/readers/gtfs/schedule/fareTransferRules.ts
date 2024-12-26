import { parseCSVAsRecord } from '../../';

/**
 *
 * Duration limit type for how transfer durations are measured.
 * Required if `duration_limit` is defined, forbidden otherwise.
 *
 * 0 - Between departure of current leg & arrival of next leg
 * 1 - Between departure of current leg & departure of next leg
 * 2 - Between arrival of current leg & departure of next leg
 * 3 - Between arrival of current leg & arrival of next leg
 */
export enum GTFSDurationLimitType {
  DepCurrentArrNext = 0,
  DepCurrentDepNext = 1,
  ArrCurrentDepNext = 2,
  ArrCurrentArrNext = 3,
}

/**
 *
 * Fare transfer type describing how costs are processed between consecutive legs:
 *
 * 0 = (A) + (AB)
 * 1 = (A) + (AB) + (B)
 * 2 = (AB)
 */
export enum GTFSFareTransferType {
  FromLegPlusTransfer = 0, // A + AB
  FromLegTransferToLeg = 1, // A + AB + B
  TransferOnly = 2, // AB
}

/**
 * # Fare Transfer Rules
 *
 * **Optional**
 * Defines the cost of transferring between fare legs specified in `fare_leg_rules.txt`.
 * Matching uses:
 * - from_leg_group_id
 * - to_leg_group_id
 * - transfer_count
 * - duration_limit
 * - duration_limit_type
 * - fare_transfer_type
 * - fare_product_id
 *
 * **Primary Key**: (from_leg_group_id, to_leg_group_id, fare_product_id, transfer_count, duration_limit)
 */
export class GTFSFareTransferRule {
  /**
   * **Optional**
   * The pre-transfer fare leg group (`fare_leg_rules.leg_group_id`).
   * - If no exact match is found, empty corresponds to all leg groups not listed under `from_leg_group_id`.
   */
  fromLegGroupId?: string;
  /**
   * **Optional**
   * The post-transfer fare leg group (`fare_leg_rules.leg_group_id`).
   * - If no exact match is found, empty corresponds to all leg groups not listed under `to_leg_group_id`.
   */
  toLegGroupId?: string;
  /**
   * **Conditionally Forbidden / Required**
   * Defines how many consecutive transfers this rule may be applied to.
   * - `-1` means no limit.
   * - `1` or more = the transfer count this rule applies to.
   *
   * Forbidden if `from_leg_group_id !== to_leg_group_id`.
   * Required if `from_leg_group_id === to_leg_group_id`.
   */
  transferCount?: number;
  /**
   * **Optional**
   * Duration limit (in seconds) for the transfer. Empty means no limit.
   */
  durationLimit?: number;
  /**
   * **Conditionally Required**
   * Defines how to measure the `durationLimit`.
   * - Required if `durationLimit` is defined.
   * - Forbidden if `durationLimit` is empty.
   */
  durationLimitType?: GTFSDurationLimitType;
  /**
   * **Required**
   * Indicates how to combine transfer costs:
   * - 0 = from-leg cost + transfer cost
   * - 1 = from-leg + transfer + to-leg cost
   * - 2 = transfer cost only
   */
  fareTransferType: GTFSFareTransferType;
  /**
   * **Optional**
   * Fare product ID for the transfer. If empty, cost is 0 (no transfer cost).
   */
  fareProductId?: string;

  /** @param data - the parsed GTFS CSV data */
  constructor(data: Record<string, string>) {
    // Optional fields
    this.fromLegGroupId = data.from_leg_group_id;
    this.toLegGroupId = data.to_leg_group_id;
    // Parse transfer_count if provided
    // GTFS calls it "Non-zero integer," plus -1 meaning "no limit"
    // If the rule is forbidden for mismatch, that must be validated externally.
    if (data.transfer_count !== undefined && data.transfer_count !== '') {
      this.transferCount = parseInt(data.transfer_count, 10);
    }
    // duration_limit is optional
    if (data.duration_limit !== undefined && data.duration_limit !== '') {
      this.durationLimit = parseInt(data.duration_limit, 10);
    }
    // If durationLimit is defined, we must parse durationLimitType
    // If it's empty or undefined, then durationLimitType must be undefined
    if (this.durationLimit !== undefined) {
      this.durationLimitType =
        data.duration_limit_type !== undefined
          ? (parseInt(data.duration_limit_type, 10) as GTFSDurationLimitType)
          : GTFSDurationLimitType.DepCurrentArrNext; // default or handle differently
    }
    // Required field fare_transfer_type
    // Default to 0 if none provided, or handle error
    if (data.fare_transfer_type !== undefined) {
      this.fareTransferType = parseInt(data.fare_transfer_type, 10) as GTFSFareTransferType;
    } else {
      this.fareTransferType = GTFSFareTransferType.FromLegPlusTransfer;
    }
    // Optional fare_product_id - cost is 0 if empty
    this.fareProductId = data.fare_product_id;
  }
}

/**
 * @param input - the input string to parse from
 * @returns - an array of GTFSFareTransferRules
 */
export function parseGTFSFareTransferRules(input: string): GTFSFareTransferRule[] {
  const data = parseCSVAsRecord(input);
  return data.map((d) => new GTFSFareTransferRule(d));
}
