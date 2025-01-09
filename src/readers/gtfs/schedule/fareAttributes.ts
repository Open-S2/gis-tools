import { parseCSVAsRecord } from '../../';

/**
 * NOTE:
 * The files associated with GTFS-Fares V1 are:
 * - fare_attributes.txt
 * - fare_rules.txt
 *
 * The files associated with GTFS-Fares V2 are:
 * - fare_media.txt
 * - fare_products.txt
 * - fare_leg_rules.txt
 * - fare_transfer_rules.txt
 */

/**
 *
 * Indicates when the fare must be paid:
 * - 0 = On board
 * - 1 = Before boarding
 */
export const enum GTFSPaymentMethod {
  OnBoard = 0,
  PreBoard = 1,
}

/**
 * Transfers can be:
 * - 0 = No transfers permitted
 * - 1 = One transfer
 * - 2 = Two transfers
 * - '' (empty string) = Unlimited transfers
 */
export type GTFSTransfersType = 0 | 1 | 2 | '';

/**
 * # Fare Attributes (GTFS-Fares V1)
 *
 * **Optional** - But required if using GTFS-Fares V1 approach.
 * Defines basic fare information such as price, currency, and transfer limits.
 */
export class GTFSFareAttribute {
  /**
   * **Required**
   * Identifies a fare class.
   */
  id: string;
  /**
   * **Required**
   * Fare price in the currency specified by `currencyType`.
   */
  price: number;
  /**
   * **Required**
   * Currency code (e.g., "USD", "EUR").
   */
  currencyType: string;
  /**
   * **Required**
   * When the fare must be paid.
   * - 0 = Paid on board
   * - 1 = Must be paid before boarding
   */
  paymentMethod: GTFSPaymentMethod;
  /**
   * **Required**
   * Number of transfers permitted on this fare.
   * - 0 = No transfers
   * - 1 = One transfer
   * - 2 = Two transfers
   * - '' (empty) = Unlimited transfers
   */
  transfers: GTFSTransfersType;
  /**
   * **Conditionally Required**
   * Agency for the specified fare.
   * Required if multiple agencies exist in `agency.txt`.
   */
  agencyId?: string;
  /**
   * **Optional**
   * Length of time in seconds before a transfer (or this fare) expires.
   * When transfers=0, may indicate ticket validity duration or be empty.
   */
  transferDuration?: number;

  /** @param data - the parsed GTFS CSV data */
  constructor(data: Record<string, string>) {
    this.id = data.fare_id;
    this.price = data.price !== undefined ? parseFloat(data.price) : 0;
    this.currencyType = data.currency_type;
    this.paymentMethod =
      data.payment_method !== undefined
        ? (parseInt(data.payment_method, 10) as GTFSPaymentMethod)
        : GTFSPaymentMethod.OnBoard;
    // If transfers is an empty string, interpret as unlimited.
    // Otherwise, parse the number or set it to empty string.
    this.transfers =
      data.transfers !== undefined ? (parseInt(data.transfers, 10) as GTFSTransfersType) : '';
    this.agencyId = data.agency_id;
    this.transferDuration =
      data.transfer_duration !== undefined && data.transfer_duration !== ''
        ? parseInt(data.transfer_duration, 10)
        : undefined;
  }
}

/**
 * @param input - the input string to parse from
 * @returns - an array of GTFSFareAttributes
 */
export function parseGTFSFareAttributes(input: string): Record<string, GTFSFareAttribute> {
  const data = parseCSVAsRecord(input);
  const res: Record<string, GTFSFareAttribute> = {};
  for (const d of data) {
    const fa = new GTFSFareAttribute(d);
    res[fa.id] = fa;
  }

  return res;
}
