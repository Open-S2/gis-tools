import { GTFSRealtimePosition, GTFSRealtimeTripDescriptor } from './..';

import type { Pbf as Protobuf } from '../..';

/** Status of the vehicle relative to the stop */
export enum VehicleStopStatus {
  /**
   * The vehicle is just about to arrive at the stop (on a stop
   * display, the vehicle symbol typically flashes).
   */
  INCOMING_AT = 0,
  /** The vehicle is standing at the stop. */
  STOPPED_AT = 1,
  /** The vehicle has departed and is in transit to the next stop. */
  IN_TRANSIT_TO = 2,
}

/** Congestion level that is affecting this vehicle. */
export enum GTFSRealtimeCongestionLevel {
  UNKNOWN_CONGESTION_LEVEL = 0,
  RUNNING_SMOOTHLY = 1,
  STOP_AND_GO = 2,
  CONGESTION = 3,
  SEVERE_CONGESTION = 4, // People leaving their cars.
}

/**
 * The state of passenger occupancy for the vehicle or carriage.
 * Individual producers may not publish all OccupancyStatus values. Therefore, consumers
 * must not assume that the OccupancyStatus values follow a linear scale.
 * Consumers should represent OccupancyStatus values as the state indicated
 * and intended by the producer. Likewise, producers must use OccupancyStatus values that
 * correspond to actual vehicle occupancy states.
 * For describing passenger occupancy levels on a linear scale, see `occupancy_percentage`.
 * This field is still experimental, and subject to change. It may be formally adopted in the future.
 */
export enum GTFSRealtimeOccupancyStatus {
  /**
   * The vehicle or carriage is considered empty by most measures, and has few or no
   * passengers onboard, but is still accepting passengers.
   */
  EMPTY = 0,
  /**
   * The vehicle or carriage has a large number of seats available.
   * The amount of free seats out of the total seats available to be
   * considered large enough to fall into this category is determined at the
   * discretion of the producer.
   */
  MANY_SEATS_AVAILABLE = 1,
  /**
   * The vehicle or carriage has a relatively small number of seats available.
   * The amount of free seats out of the total seats available to be
   * considered small enough to fall into this category is determined at the
   * discretion of the feed producer.
   */
  FEW_SEATS_AVAILABLE = 2,
  /** The vehicle or carriage can currently accommodate only standing passengers. */
  STANDING_ROOM_ONLY = 3,
  /**
   * The vehicle or carriage can currently accommodate only standing passengers
   * and has limited space for them.
   */
  CRUSHED_STANDING_ROOM_ONLY = 4,
  /**
   * The vehicle or carriage is considered full by most measures, but may still be
   * allowing passengers to board.
   */
  FULL = 5,
  /** The vehicle or carriage is not accepting passengers, but usually accepts passengers for boarding. */
  NOT_ACCEPTING_PASSENGERS = 6,
  /** The vehicle or carriage doesn't have any occupancy data available at that time. */
  NO_DATA_AVAILABLE = 7,
  /**
   * The vehicle or carriage is not boardable and never accepts passengers.
   * Useful for special vehicles or carriages (engine, maintenance carriage, etc.).
   */
  NOT_BOARDABLE = 8,
}

/** Realtime positioning information for a given vehicle. */
export class GTFSRealtimeVehiclePosition {
  /**
   * The Trip that this vehicle is serving.
   * Can be empty or partial if the vehicle can not be identified with a given
   * trip instance.
   */
  trip?: GTFSRealtimeTripDescriptor; // 1 [message]
  /** Current position of this vehicle. */
  position!: GTFSRealtimePosition; // 2 [message]
  /**
   * The stop sequence index of the current stop. The meaning of
   * current_stop_sequence (i.e., the stop that it refers to) is determined by
   * current_status.
   * If current_status is missing IN_TRANSIT_TO is assumed.
   */
  currentStopSequence?: number; // 3 [uint32]
  /**
   * The exact status of the vehicle with respect to the current stop.
   * Ignored if current_stop_sequence is missing.
   */
  currentStatus = VehicleStopStatus.IN_TRANSIT_TO; // 4 [enum]
  /**
   * Moment at which the vehicle's position was measured. In POSIX time
   * (i.e., number of seconds since January 1st 1970 00:00:00 UTC).
   */
  timestamp?: Date; // 5 [uint64]
  /** Congestion level that is affecting this vehicle. */
  congestionLevel?: GTFSRealtimeCongestionLevel; // 6 [enum]
  /**
   * Identifies the current stop. The value must be the same as in stops.txt in
   * the corresponding GTFS feed.
   */
  stopId?: string; // 7
  /** Additional information on the vehicle that is serving this trip. */
  vehicle?: GTFSRealtimeVehicleDescriptor; // 8 [message]
  /**
   * If multi_carriage_status is populated with per-carriage OccupancyStatus,
   * then this field should describe the entire vehicle with all carriages accepting passengers considered.
   */
  occupancyStatus?: GTFSRealtimeOccupancyStatus; // 9 [enum]
  /**
   * A percentage value indicating the degree of passenger occupancy in the vehicle.
   * The values are represented as an integer without decimals. 0 means 0% and 100 means 100%.
   * The value 100 should represent the total maximum occupancy the vehicle was designed for,
   * including both seated and standing capacity, and current operating regulations allow.
   * The value may exceed 100 if there are more passengers than the maximum designed capacity.
   * The precision of occupancy_percentage should be low enough that individual passengers cannot be tracked boarding or alighting the vehicle.
   * If multi_carriage_status is populated with per-carriage occupancy_percentage,
   * then this field should describe the entire vehicle with all carriages accepting passengers considered.
   * This field is still experimental, and subject to change. It may be formally adopted in the future.
   */
  occupancyPercentage?: number; // 10 [uint32]
  /**
   * Details of the multiple carriages of this given vehicle.
   * The first occurrence represents the first carriage of the vehicle,
   * given the current direction of travel.
   * The number of occurrences of the multi_carriage_details
   * field represents the number of carriages of the vehicle.
   * It also includes non boardable carriages,
   * like engines, maintenance carriages, etcâ€¦ as they provide valuable
   * information to passengers about where to stand on a platform.
   * This message/field is still experimental, and subject to change. It may be formally adopted in the future.
   */
  multiCarriageDetails: GTFSRealtimeMultiCarriageDetails[] = []; // 11 [message]

  /**
   * @param pbf - The Protobuf object to read from
   * @param end - The end position of the message in the buffer
   */
  constructor(pbf: Protobuf, end: number) {
    pbf.readFields(this.#readVehiclePosition, this, end);
  }

  /**
   * @param tag - The tag of the message
   * @param vehicle - The vehicle to mutate
   * @param pbf - The Protobuf object to read from
   */
  #readVehiclePosition(tag: number, vehicle: GTFSRealtimeVehiclePosition, pbf: Protobuf): void {
    if (tag === 1) vehicle.trip = new GTFSRealtimeTripDescriptor(pbf, pbf.readVarint() + pbf.pos);
    else if (tag === 2)
      vehicle.position = new GTFSRealtimePosition(pbf, pbf.readVarint() + pbf.pos);
    else if (tag === 3) vehicle.currentStopSequence = pbf.readVarint();
    else if (tag === 4) vehicle.currentStatus = pbf.readVarint() as VehicleStopStatus;
    else if (tag === 5) vehicle.timestamp = new Date(pbf.readVarint() * 1000);
    else if (tag === 6) vehicle.congestionLevel = pbf.readVarint() as GTFSRealtimeCongestionLevel;
    else if (tag === 7) vehicle.stopId = pbf.readString();
    else if (tag === 8)
      vehicle.vehicle = new GTFSRealtimeVehicleDescriptor(pbf, pbf.readVarint() + pbf.pos);
    else if (tag === 9) vehicle.occupancyStatus = pbf.readVarint();
    else if (tag === 10) vehicle.occupancyPercentage = pbf.readVarint();
    else if (tag === 11)
      vehicle.multiCarriageDetails.push(
        new GTFSRealtimeMultiCarriageDetails(pbf, pbf.readVarint() + pbf.pos),
      );
    else throw new Error('GTFSRealtimeVehiclePosition: unknown tag ' + tag);
  }
}

/** Wheelchair accessibility of the trip */
export enum GTFSRealtimeWheelchairAccessible {
  // The trip doesn't have information about wheelchair accessibility.
  // This is the **default** behavior. If the static GTFS contains a
  // _wheelchair_accessible_ value, it won't be overwritten.
  NO_VALUE = 0,
  // The trip has no accessibility value present.
  // This value will overwrite the value from the GTFS.
  UNKNOWN = 1,
  // The trip is wheelchair accessible.
  // This value will overwrite the value from the GTFS.
  WHEELCHAIR_ACCESSIBLE = 2,
  // The trip is **not** wheelchair accessible.
  // This value will overwrite the value from the GTFS.
  WHEELCHAIR_INACCESSIBLE = 3,
}

/** Identification information for the vehicle performing the trip. */
export class GTFSRealtimeVehicleDescriptor {
  /**
   * Internal system identification of the vehicle. Should be unique per
   * vehicle, and can be used for tracking the vehicle as it proceeds through
   * the system.
   */
  id?: string; // 1 [string]
  /**
   * User visible label, i.e., something that must be shown to the passenger to
   * help identify the correct vehicle.
   */
  label?: string; // 2 [string]
  /** The license plate of the vehicle. */
  licensePlate?: string; // 3 [string]
  /** Wheelchair accessibility of the trip */
  wheelchairAccessible = GTFSRealtimeWheelchairAccessible.NO_VALUE; // 4 [enum]

  /**
   * @param pbf - The Protobuf object to read from
   * @param end - The end position of the message in the buffer
   */
  constructor(pbf: Protobuf, end: number) {
    pbf.readFields(this.#readDescriptor, this, end);
  }

  /**
   * @param tag - The tag of the message
   * @param descriptor - The multi carriage details to mutate
   * @param pbf - The Protobuf object to read from
   */
  #readDescriptor(tag: number, descriptor: GTFSRealtimeVehicleDescriptor, pbf: Protobuf): void {
    if (tag === 1) descriptor.id = pbf.readString();
    else if (tag === 2) descriptor.label = pbf.readString();
    else if (tag === 3) descriptor.licensePlate = pbf.readString();
    else if (tag === 4)
      descriptor.wheelchairAccessible = pbf.readVarint() as GTFSRealtimeWheelchairAccessible;
    else throw new Error('GTFSRealtimeVehicleDescriptor: unknown tag ' + tag);
  }
}

/**
 * Carriage specific details, used for vehicles composed of several carriages
 * This message/field is still experimental, and subject to change. It may be formally adopted in the future.
 */
export class GTFSRealtimeMultiCarriageDetails {
  /** Identification of the carriage. Should be unique per vehicle. */
  id?: string; // 1 [string]
  /**
   * User visible label that may be shown to the passenger to help identify
   * the carriage. Example: "7712", "Car ABC-32", etc...
   * This message/field is still experimental, and subject to change. It may be formally adopted in the future.
   */
  label?: string; // 2 [string]
  /**
   * Occupancy status for this given carriage, in this vehicle
   * This message/field is still experimental, and subject to change. It may be formally adopted in the future.
   */
  occupancyStatus: GTFSRealtimeOccupancyStatus = GTFSRealtimeOccupancyStatus.NO_DATA_AVAILABLE; // 3 [enum]
  /**
   * Occupancy percentage for this given carriage, in this vehicle.
   * Follows the same rules as "VehiclePosition.occupancy_percentage"
   * -1 in case data is not available for this given carriage (as protobuf defaults to 0 otherwise)
   * This message/field is still experimental, and subject to change. It may be formally adopted in the future.
   */
  occupancyPercentage = -1; // 4 [int32]
  /**
   * Identifies the order of this carriage with respect to the other
   * carriages in the vehicle's list of CarriageDetails.
   * The first carriage in the direction of travel must have a value of 1.
   * The second value corresponds to the second carriage in the direction
   * of travel and must have a value of 2, and so forth.
   * For example, the first carriage in the direction of travel has a value of 1.
   * If the second carriage in the direction of travel has a value of 3,
   * consumers will discard data for all carriages (i.e., the multi_carriage_details field).
   * Carriages without data must be represented with a valid carriage_sequence number and the fields
   * without data should be omitted (alternately, those fields could also be included and set to the "no data" values).
   * This message/field is still experimental, and subject to change. It may be formally adopted in the future.
   */
  carriageSequence?: number; // 5 [uint32]

  /**
   * @param pbf - The Protobuf object to read from
   * @param end - The end position of the message in the buffer
   */
  constructor(pbf: Protobuf, end: number) {
    pbf.readFields(this.#readMultiCarriageDetails, this, end);
  }

  /**
   * @param tag - The tag of the message
   * @param multiCarriageDetails - The multi carriage details to mutate
   * @param pbf - The Protobuf object to read from
   */
  #readMultiCarriageDetails(
    tag: number,
    multiCarriageDetails: GTFSRealtimeMultiCarriageDetails,
    pbf: Protobuf,
  ): void {
    if (tag === 1) multiCarriageDetails.id = pbf.readString();
    else if (tag === 2) multiCarriageDetails.label = pbf.readString();
    else if (tag === 3)
      multiCarriageDetails.occupancyStatus = pbf.readVarint() as GTFSRealtimeOccupancyStatus;
    else if (tag === 4) multiCarriageDetails.occupancyPercentage = pbf.readSVarint();
    else if (tag === 5) multiCarriageDetails.carriageSequence = pbf.readVarint();
    else throw new Error('GTFSRealtimeMultiCarriageDetails: unknown tag ' + tag);
  }
}
