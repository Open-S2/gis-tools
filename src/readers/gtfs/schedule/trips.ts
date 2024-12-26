import { parseCSVAsRecord } from '../../';

/**
 * Indicates the direction of travel for a trip. This field should not be used in routing; it provides a way to separate trips by direction when publishing time tables. Valid options are:
 * - 0 - Travel in one direction (e.g. outbound travel).
 * - 1 - Travel in the opposite direction (e.g. inbound travel).
 * Example: The trip_headsign and direction_id fields may be used together to assign a name to travel in each direction for a set of trips. A trips.txt file could contain these records for use in time tables:
 * ```csv
 * trip_id,...,trip_headsign,direction_id
 * 1234,...,Airport,0
 * 1505,...,Downtown,1
 * ```
 */
export enum GTFSDirectionId {
  Outbound = 0, // e.g., "Airport"
  Inbound = 1, // e.g., "Downtown"
}

/**
 * Indicates wheelchair accessibility. Valid options are:
 * - 0 or empty - No accessibility information for the trip.
 * - 1 - Vehicle being used on this particular trip can accommodate at least one rider in a wheelchair.
 * - 2 - No riders in wheelchairs can be accommodated on this trip.
 */
export enum GTFSWheelchairAccessibility {
  NoInfo = 0, // or empty
  Accessible = 1, // at least one wheelchair space
  NotAccessible = 2, // no wheelchair accommodation
}

/**
 * Indicates whether bikes are allowed. Valid options are:
 * - 0 or empty - No bike information for the trip.
 * - 1 - Vehicle being used on this particular trip can accommodate at least one bicycle.
 * - 2 - No bicycles are allowed on this trip.
 */
export enum GTFSBikesAllowed {
  NoInfo = 0, // or empty
  Allowed = 1, // at least one bicycle can be accommodated
  NotAllowed = 2, // no bicycles allowed
}

/**
 * # Trip Information
 *
 * ## Details
 * **Required** - Trips for each route. A trip is a sequence of two or more stops that occur during
 * a specific time period.
 */
export class GTFSTrip {
  /**
   * **Required**
   * Identifies which route this trip belongs to (`routes.route_id`).
   */
  routeId: string;
  /**
   * **Required**
   * Identifies a set of dates when service is available (`calendar.service_id` or `calendar_dates.service_id`).
   */
  serviceId: string;
  /**
   * **Required**
   * Unique identifier for a trip (`trip_id`).
   */
  id: string;
  /**
   * **Optional**
   * Destination sign text that identifies the trip’s destination to riders.
   */
  headsign?: string;
  /**
   * **Optional**
   * Public-facing text used to identify the trip (e.g., train numbers).
   */
  shortName?: string;
  /**
   * Updated to use an enum for direction.
   * 0 = Outbound, 1 = Inbound.
   */
  directionId?: GTFSDirectionId;
  /**
   * **Optional**
   * Identifies the block this trip belongs to. Sequential trips with the same block_id typically use the same vehicle.
   */
  blockId?: string;
  /**
   * **Conditionally Required**
   * References a geospatial shape describing the vehicle's travel path (`shapes.shape_id`).
   * Required if the trip uses continuous pickup or drop-off rules; otherwise optional.
   */
  shapeId?: string;
  /**
   * Updated to use an enum for wheelchair accessibility.
   * 0 = NoInfo, 1 = Accessible, 2 = NotAccessible.
   */
  wheelchairAccessible?: GTFSWheelchairAccessibility;
  /**
   * Updated to use an enum for bikes allowed.
   * 0 = NoInfo, 1 = Allowed, 2 = NotAllowed.
   */
  bikesAllowed?: GTFSBikesAllowed;

  /** @param data - the parsed GTFS CSV data */
  constructor(data: Record<string, string>) {
    this.routeId = data.route_id;
    this.serviceId = data.service_id;
    this.id = data.trip_id;
    this.headsign = data.trip_headsign;
    this.shortName = data.trip_short_name;
    this.directionId =
      data.direction_id !== undefined
        ? (parseInt(data.direction_id, 10) as GTFSDirectionId)
        : undefined;
    this.blockId = data.block_id;
    this.shapeId = data.shape_id;
    this.wheelchairAccessible =
      data.wheelchair_accessible !== undefined
        ? (parseInt(data.wheelchair_accessible, 10) as GTFSWheelchairAccessibility)
        : undefined;
    this.bikesAllowed =
      data.bikes_allowed !== undefined
        ? (parseInt(data.bikes_allowed, 10) as GTFSBikesAllowed)
        : undefined;
  }
}

/**
 * @param input - the input string to parse from
 * @returns - an array of Trips
 */
export function parseGTFSTrips(input: string): GTFSTrip[] {
  const data = parseCSVAsRecord(input);
  return data.map((d) => new GTFSTrip(d));
}
