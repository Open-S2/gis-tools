import { parseCSVAsRecord } from '../../';

/**
 *
 * TransferType enumerates how a rider can transfer between routes/trips/stops:
 * 0 or empty = Recommended transfer
 * 1 = Timed transfer
 * 2 = Requires a minimum transfer time
 * 3 = Transfers not possible
 * 4 = In-seat transfer (stay onboard, same vehicle)
 * 5 = In-seat transfers not allowed
 */
export enum TransferType {
  Recommended = 0,
  Timed = 1,
  MinTimeRequired = 2,
  NotPossible = 3,
  InSeatTransfer = 4,
  InSeatNotAllowed = 5,
}

/**
 * # Transfers
 *
 * **Optional**
 * Defines additional rules/overrides for transfers between routes/trips/stops.
 * The level of specificity is determined by which fields are present:
 * - from_trip_id & to_trip_id (most specific)
 * - route vs. trip combos
 * - only from_stop_id & to_stop_id (least specific)
 *
 * **Primary Key**: (from_stop_id, to_stop_id, from_trip_id, to_trip_id, from_route_id, to_route_id)
 */
export class GTFSTransfer {
  /**
   * **Conditionally Required**
   * Identifies where a connection begins (`stops.stop_id`, location_type=0 or 1).
   * Required if transfer_type is 1, 2, or 3. Optional if transfer_type is 4 or 5.
   */
  fromStopId?: string;
  /**
   * **Conditionally Required**
   * Identifies where a connection ends (`stops.stop_id`, location_type=0 or 1).
   * Required if transfer_type is 1, 2, or 3. Optional if transfer_type is 4 or 5.
   */
  toStopId?: string;
  /**
   * **Optional**
   * Identifies a route on which the arriving trip is running.
   * If both `fromTripId` and `fromRouteId` are defined, the trip must belong to that route,
   * but `fromTripId` takes precedence.
   */
  fromRouteId?: string;
  /**
   * **Optional**
   * Identifies a route on which the departing trip is running.
   * If both `toTripId` and `toRouteId` are defined, the trip must belong to that route,
   * but `toTripId` takes precedence.
   */
  toRouteId?: string;
  /**
   * **Conditionally Required**
   * Identifies the arriving trip (`trips.trip_id`).
   * Required if transfer_type is 4 or 5; optional otherwise.
   */
  fromTripId?: string;
  /**
   * **Conditionally Required**
   * Identifies the departing trip (`trips.trip_id`).
   * Required if transfer_type is 4 or 5; optional otherwise.
   */
  toTripId?: string;
  /**
   * **Required**
   * Indicates the type of connection:
   * - 0 = Recommended
   * - 1 = Timed
   * - 2 = Requires min_transfer_time
   * - 3 = Not possible
   * - 4 = In-seat transfer
   * - 5 = In-seat transfer not allowed
   */
  transferType: TransferType;
  /**
   * **Optional**
   * Time in seconds required for a rider to complete the transfer. If `transfer_type=2`,
   * this is the minimum transfer time.
   */
  minTransferTime?: number;

  /** @param data - the parsed GTFS CSV data */
  constructor(data: Record<string, string>) {
    // Conditionals:
    this.fromStopId = data.from_stop_id;
    this.toStopId = data.to_stop_id;
    this.fromRouteId = data.from_route_id;
    this.toRouteId = data.to_route_id;
    this.fromTripId = data.from_trip_id;
    this.toTripId = data.to_trip_id;
    // Required transfer_type
    this.transferType =
      data.transfer_type !== undefined
        ? (parseInt(data.transfer_type, 10) as TransferType)
        : TransferType.Recommended; // Default if omitted

    // min_transfer_time is optional
    if (data.min_transfer_time !== undefined) {
      this.minTransferTime = parseInt(data.min_transfer_time, 10);
    }
  }
}

/**
 * @param input - the input string to parse from
 * @returns - an array of Transfers
 */
export function parseGTFSTransfers(input: string): GTFSTransfer[] {
  const data = parseCSVAsRecord(input);
  return data.map((d) => new GTFSTransfer(d));
}
