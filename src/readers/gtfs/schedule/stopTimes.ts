import { parseCSVAsRecord } from '../../';

/**
 * Pickup method.
 * 0 or empty = Regularly scheduled pickup
 * 1 = No pickup available
 * 2 = Must phone agency to arrange pickup
 * 3 = Must coordinate with driver to arrange pickup
 */
export const enum GTFSPickupType {
  /** Regularly scheduled pickup */
  Regular = 0,
  /** No pickup available */
  None = 1,
  /** Must phone agency to arrange pickup */
  PhoneAgency = 2,
  /** Must coordinate with driver to arrange pickup */
  CoordinateDriver = 3,
}

/**
 * Drop-off method.
 * 0 or empty = Regularly scheduled drop off
 * 1 = No drop off available
 * 2 = Must phone agency to arrange drop off
 * 3 = Must coordinate with driver to arrange drop off
 */
export const enum GTFSDropOffType {
  /** Regularly scheduled drop off */
  Regular = 0,
  /** No drop off available */
  None = 1,
  /** Must phone agency to arrange drop off */
  PhoneAgency = 2,
  /** Must coordinate with driver to arrange drop off */
  CoordinateDriver = 3,
}

/**
 * Continuous pickup behavior from this stop_time to the next.
 * 0 = Continuous stopping pickup
 * 1 or empty = No continuous stopping pickup
 * 2 = Must phone agency
 * 3 = Must coordinate with driver
 */
export const enum ContinuousPickup {
  /** Continuous stopping pickup */
  Continuous = 0,
  /** No continuous stopping pickup */
  None = 1,
  /** Must phone agency */
  PhoneAgency = 2,
  /** Must coordinate with driver */
  CoordinateDriver = 3,
}

/**
 * Continuous drop-off behavior from this stop_time to the next.
 * 0 = Continuous stopping drop off
 * 1 or empty = No continuous stopping drop off
 * 2 = Must phone agency
 * 3 = Must coordinate with driver
 */
export const enum ContinuousDropOff {
  /** Continuous stopping drop off */
  Continuous = 0,
  /** No continuous stopping drop off */
  None = 1,
  /** Must phone agency */
  PhoneAgency = 2,
  /** Must coordinate with driver */
  CoordinateDriver = 3,
}

/**
 * Indicates if arrival/departure times are exact or approximate.
 * 0 = Approximate times
 * 1 = Exact times
 */
export const enum Timepoint {
  /** Approximate times */
  Approximate = 0,
  /** Exact times */
  Exact = 1,
}

/**
 * # Stop Time Information
 *
 * **Required** - Times that a vehicle arrives at and departs from stops for each trip.
 */
export class GTFSStopTime {
  /**
   * **Required**
   * Identifies a trip (`trips.trip_id`).
   */
  tripId: string;
  /**
   * **Conditionally Required**
   * Arrival time at the stop in HH:MM:SS (local) or possibly > 24:00:00 after midnight.
   * Required for the first/last stop of the trip or if `timepoint=1`.
   * Forbidden if `start_pickup_drop_off_window` or `end_pickup_drop_off_window` are defined.
   */
  arrivalTime?: string;
  /**
   * **Conditionally Required**
   * Departure time at the stop in HH:MM:SS (local) or possibly > 24:00:00 after midnight.
   * Required if `timepoint=1`.
   * Forbidden if `start_pickup_drop_off_window` or `end_pickup_drop_off_window` are defined.
   */
  departureTime?: string;
  /**
   * **Conditionally Required**
   * References a stop (`stops.stop_id`). Must be a location_type of 0 or empty.
   * Required if neither `location_group_id` nor `location_id` is used.
   * Forbidden if `location_group_id` or `location_id` is defined.
   */
  stopId?: string;
  /**
   * **Conditionally Forbidden**
   * References a location group (`location_groups.location_group_id`).
   * Forbidden if `stop_id` or `location_id` is defined.
   */
  locationGroupId?: string;
  /**
   * **Conditionally Forbidden**
   * References a GeoJSON location ID (`locations.geojson`).
   * Forbidden if `stop_id` or `location_group_id` is defined.
   */
  locationId?: string;
  /**
   * **Required**
   * Order of stops (or location groups, or GeoJSON locations) for this trip.
   * Must increase along the trip, but need not be consecutive.
   */
  stopSequence: number;
  /**
   * **Optional**
   * Overrides the trip’s headsign at this specific stop.
   */
  stopHeadsign?: string;
  /**
   * **Conditionally Required**
   * Time on-demand service becomes available at this location/stop/location group.
   * Required if `end_pickup_drop_off_window` is defined, or if `location_group_id` or `location_id` is used.
   * Forbidden if `arrival_time` or `departure_time` is defined.
   */
  startPickupDropOffWindow?: string;
  /**
   * **Conditionally Required**
   * Time on-demand service ends at this location/stop/location group.
   * Required if `start_pickup_drop_off_window` is defined, or if `location_group_id` or `location_id` is used.
   * Forbidden if `arrival_time` or `departure_time` is defined.
   */
  endPickupDropOffWindow?: string;
  /**
   * **Conditionally Forbidden**
   * Pickup method:
   * 0 or empty = Regular, 1 = None, 2 = Phone Agency, 3 = Coordinate with Driver
   * Forbidden if `start_pickup_drop_off_window` or `end_pickup_drop_off_window` are defined (for 0 or 3).
   */
  pickupType?: GTFSPickupType;
  /**
   * **Conditionally Forbidden**
   * Drop-off method:
   * 0 or empty = Regular, 1 = None, 2 = Phone Agency, 3 = Coordinate with Driver
   * Forbidden if `start_pickup_drop_off_window` or `end_pickup_drop_off_window` are defined (for 0).
   */
  dropOffType?: GTFSDropOffType;
  /**
   * **Conditionally Forbidden**
   * Continuous pickup from this stop_time to the next.
   * 0 = Continuous, 1 or empty = None, 2 = Phone Agency, 3 = Coordinate with Driver
   * Forbidden if `start_pickup_drop_off_window` or `end_pickup_drop_off_window` are defined.
   */
  continuousPickup?: ContinuousPickup;
  /**
   * **Conditionally Forbidden**
   * Continuous drop-off from this stop_time to the next.
   * 0 = Continuous, 1 or empty = None, 2 = Phone Agency, 3 = Coordinate with Driver
   * Forbidden if `start_pickup_drop_off_window` or `end_pickup_drop_off_window` are defined.
   */
  continuousDropOff?: ContinuousDropOff;
  /**
   * **Optional**
   * Distance traveled along the associated shape from the first stop to this record’s stop.
   * Must be in the same units used in shapes.txt.
   */
  shapeDistTraveled?: number;
  /**
   * **Optional**
   * 0 = Times are approximate, 1 = Times are exact.
   */
  timepoint?: Timepoint;
  /**
   * **Optional**
   * Boarding booking rule reference (`booking_rules.booking_rule_id`).
   * Recommended if `pickup_type=2`.
   */
  pickupBookingRuleId?: string;
  /**
   * **Optional**
   * Alighting booking rule reference (`booking_rules.booking_rule_id`).
   * Recommended if `drop_off_type=2`.
   */
  dropOffBookingRuleId?: string;

  /** @param data - the parsed GTFS CSV data */
  constructor(data: Record<string, string>) {
    this.tripId = data.trip_id;
    this.arrivalTime = data.arrival_time;
    this.departureTime = data.departure_time;
    this.stopId = data.stop_id;
    this.locationGroupId = data.location_group_id;
    this.locationId = data.location_id;
    this.stopSequence = data.stop_sequence !== undefined ? parseInt(data.stop_sequence, 10) : 0;
    this.stopHeadsign = data.stop_headsign;
    this.startPickupDropOffWindow = data.start_pickup_drop_off_window;
    this.endPickupDropOffWindow = data.end_pickup_drop_off_window;
    this.pickupType =
      data.pickup_type !== undefined
        ? (parseInt(data.pickup_type, 10) as GTFSPickupType)
        : undefined;
    this.dropOffType =
      data.drop_off_type !== undefined
        ? (parseInt(data.drop_off_type, 10) as GTFSDropOffType)
        : undefined;

    this.continuousPickup =
      data.continuous_pickup !== undefined
        ? (parseInt(data.continuous_pickup, 10) as ContinuousPickup)
        : undefined;
    this.continuousDropOff =
      data.continuous_drop_off !== undefined
        ? (parseInt(data.continuous_drop_off, 10) as ContinuousDropOff)
        : undefined;
    this.shapeDistTraveled =
      data.shape_dist_traveled !== undefined ? parseFloat(data.shape_dist_traveled) : undefined;
    this.timepoint =
      data.timepoint !== undefined ? (parseInt(data.timepoint, 10) as Timepoint) : undefined;
    this.pickupBookingRuleId = data.pickup_booking_rule_id;
    this.dropOffBookingRuleId = data.drop_off_booking_rule_id;
  }
}

/**
 * @param input - the input string to parse from
 * @returns - an array of StopTimes
 */
export function parseGTFSStopTimes(input: string): GTFSStopTime[] {
  const data = parseCSVAsRecord(input);
  return data.map((d) => new GTFSStopTime(d));
}
