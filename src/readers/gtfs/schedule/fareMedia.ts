import { parseCSVAsRecord } from '../../';

/**
 *
 * Describes the type of fare media used.
 * 0 - None
 * 1 - Physical paper ticket
 * 2 - Physical transit card
 * 3 - cEMV (contactless)
 * 4 - Mobile app
 */
export const enum GTFSFareMediaType {
  None = 0,
  PhysicalPaperTicket = 1,
  PhysicalTransitCard = 2,
  CEMV = 3,
  MobileApp = 4,
}

/**
 * # Fare Media
 *
 * **Optional**
 * Describes physical or virtual holders used for the representation and validation of a fare product.
 */
export class GTFSFareMedia {
  /**
   * **Required**
   * Identifies a fare media (`fare_media_id`).
   */
  id: string;
  /**
   * **Optional**
   * Rider-facing name for this fare media.
   */
  name?: string;
  /**
   * **Required**
   * Type of fare media. One of:
   * - 0 = None
   * - 1 = Physical paper ticket
   * - 2 = Physical transit card
   * - 3 = cEMV (contactless)
   * - 4 = Mobile app
   */
  type: GTFSFareMediaType;

  /** @param data - the parsed GTFS CSV data */
  constructor(data: Record<string, string>) {
    this.id = data.fare_media_id;
    this.name = data.fare_media_name;
    this.type =
      data.fare_media_type !== undefined
        ? (parseInt(data.fare_media_type, 10) as GTFSFareMediaType)
        : GTFSFareMediaType.None;
  }
}

/**
 * @param input - the input string to parse from
 * @returns - an array of GTFSFareMedias
 */
export function parseGTFSFareMedias(input: string): Record<string, GTFSFareMedia> {
  const data = parseCSVAsRecord(input);
  const res: Record<string, GTFSFareMedia> = {};
  for (const d of data) {
    const fm = new GTFSFareMedia(d);
    res[fm.id] = fm;
  }

  return res;
}
