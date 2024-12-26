import { parseCSVAsRecord } from '../../';

/**
 * # Fare Products
 *
 * **Optional**
 * Describes different fare products riders can purchase.
 * Used by GTFS-Fares V2 to model fare product costs, media, and potential discounts for multi-leg journeys.
 *
 * Multiple rows with the same `fare_product_id` can exist, each paired with a different `fare_media_id`.
 */
export class GTFSFareProduct {
  /**
   * **Required**
   * Identifies a fare product or set of fare products.
   */
  id: string;
  /**
   * **Optional**
   * The name of the fare product as displayed to riders.
   */
  fareProductName?: string;
  /**
   * **Optional**
   * Identifies a fare media (`fare_media.fare_media_id`) that can be employed to use this fare product.
   * When empty, the fare media is unknown.
   */
  fareMediaId?: string;
  /**
   * **Required**
   * The cost of the fare product. May be:
   * - Negative: Transfer discount
   * - Zero: Free fare
   * - Positive: Standard fare cost
   */
  amount: number;
  /**
   * **Required**
   * Currency code (e.g., "USD", "EUR") for the cost of this product.
   */
  currency: string;

  /** @param data - the parsed GTFS CSV data */
  constructor(data: Record<string, string>) {
    this.id = data.fare_product_id;
    this.fareProductName = data.fare_product_name;
    this.fareMediaId = data.fare_media_id;
    this.amount = data.amount !== undefined ? parseFloat(data.amount) : 0;
    this.currency = data.currency;
  }
}

/**
 * @param input - the input string to parse from
 * @returns - an array of GTFSFareProducts
 */
export function parseGTFSFareProducts(input: string): Record<string, GTFSFareProduct> {
  const data = parseCSVAsRecord(input);
  const res: Record<string, GTFSFareProduct> = {};
  for (const d of data) {
    const fp = new GTFSFareProduct(d);
    res[fp.id] = fp;
  }

  return res;
}
