import { parseGtfsDate } from '../../utils';

import type { Pbf as Protobuf } from '../../..';

/**
 * The relation between this trip and the static schedule. If a trip is done
 * in accordance with temporary schedule, not reflected in GTFS, then it
 * shouldn't be marked as SCHEDULED, but likely as ADDED.
 */
export const enum GTFSRealtimeScheduleRelationship {
  /**
   * Trip that is running in accordance with its GTFS schedule, or is close
   * enough to the scheduled trip to be associated with it.
   */
  SCHEDULED = 0,
  /**
   * An extra trip that was added in addition to a running schedule, for
   * example, to replace a broken vehicle or to respond to sudden passenger
   * load.
   * NOTE: Currently, behavior is unspecified for feeds that use this mode. There are discussions on the GTFS GitHub
   * [(1)](https://github.com/google/transit/issues/106) [(2)](https://github.com/google/transit/pull/221)
   * [(3)](https://github.com/google/transit/pull/219) around fully specifying or deprecating ADDED trips and the
   * documentation will be updated when those discussions are finalized.
   */
  ADDED = 1,
  /**
   * A trip that is running with no schedule associated to it (GTFS frequencies.txt exact_times=0).
   * Trips with ScheduleRelationship=UNSCHEDULED must also set all StopTimeUpdates.ScheduleRelationship=UNSCHEDULED.
   */
  UNSCHEDULED = 2,
  /**  A trip that existed in the schedule but was removed. */
  CANCELED = 3,
  /** Should not be used - for backwards-compatibility only. */
  REPLACEMENT = 5, // deprecated
  /**
   * An extra trip that was added in addition to a running schedule, for example, to replace a broken vehicle or to
   * respond to sudden passenger load. Used with TripUpdate.TripProperties.trip_id, TripUpdate.TripProperties.start_date,
   * and TripUpdate.TripProperties.start_time to copy an existing trip from static GTFS but start at a different service
   * date and/or time. Duplicating a trip is allowed if the service related to the original trip in (CSV) GTFS
   * (in calendar.txt or calendar_dates.txt) is operating within the next 30 days. The trip to be duplicated is
   * identified via TripUpdate.TripDescriptor.trip_id. This enumeration does not modify the existing trip referenced by
   * TripUpdate.TripDescriptor.trip_id - if a producer wants to cancel the original trip, it must publish a separate
   * TripUpdate with the value of CANCELED or DELETED. Trips defined in GTFS frequencies.txt with exact_times that is
   * empty or equal to 0 cannot be duplicated. The VehiclePosition.TripDescriptor.trip_id for the new trip must contain
   * the matching value from TripUpdate.TripProperties.trip_id and VehiclePosition.TripDescriptor.ScheduleRelationship
   * must also be set to DUPLICATED.
   * Existing producers and consumers that were using the ADDED enumeration to represent duplicated trips must follow
   * the migration guide (https://github.com/google/transit/tree/master/gtfs-realtime/spec/en/examples/migration-duplicated.md)
   * to transition to the DUPLICATED enumeration.
   * NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future.
   */
  DUPLICATED = 6,
  /**
   * A trip that existed in the schedule but was removed and must not be shown to users.
   * DELETED should be used instead of CANCELED to indicate that a transit provider would like to entirely remove
   * information about the corresponding trip from consuming applications, so the trip is not shown as cancelled to
   * riders, e.g. a trip that is entirely being replaced by another trip.
   * This designation becomes particularly important if several trips are cancelled and replaced with substitute service.
   * If consumers were to show explicit information about the cancellations it would distract from the more important
   * real-time predictions.
   * NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future.
   */
  DELETED = 7,
}

/**
 * A descriptor that identifies an instance of a GTFS trip, or all instances of
 * a trip along a route.
 * - To specify a single trip instance, the trip_id (and if necessary,
 *   start_time) is set. If route_id is also set, then it should be same as one
 *   that the given trip corresponds to.
 * - To specify all the trips along a given route, only the route_id should be
 *   set. Note that if the trip_id is not known, then stop sequence ids in
 *   TripUpdate are not sufficient, and stop_ids must be provided as well. In
 *   addition, absolute arrival/departure times must be provided.
 */
export class GTFSRealtimeTripDescriptor {
  /**
   * The trip_id from the GTFS feed that this selector refers to.
   * For non frequency-based trips, this field is enough to uniquely identify
   * the trip. For frequency-based trip, start_time and start_date might also be
   * necessary. When schedule_relationship is DUPLICATED within a TripUpdate, the trip_id identifies the trip from
   * static GTFS to be duplicated. When schedule_relationship is DUPLICATED within a VehiclePosition, the trip_id
   * identifies the new duplicate trip and must contain the value for the corresponding TripUpdate.TripProperties.trip_id.
   */
  tripId?: string; // 1 [string]
  /**
   * The initially scheduled start time of this trip instance.
   * When the trip_id corresponds to a non-frequency-based trip, this field
   * should either be omitted or be equal to the value in the GTFS feed. When
   * the trip_id correponds to a frequency-based trip, the start_time must be
   * specified for trip updates and vehicle positions. If the trip corresponds
   * to exact_times=1 GTFS record, then start_time must be some multiple
   * (including zero) of headway_secs later than frequencies.txt start_time for
   * the corresponding time period. If the trip corresponds to exact_times=0,
   * then its start_time may be arbitrary, and is initially expected to be the
   * first departure of the trip. Once established, the start_time of this
   * frequency-based trip should be considered immutable, even if the first
   * departure time changes -- that time change may instead be reflected in a
   * StopTimeUpdate.
   * Format and semantics of the field is same as that of
   * GTFS/frequencies.txt/start_time, e.g., 11:15:35 or 25:15:35.
   */
  startTime?: string; // 2 [string]
  /**
   * The scheduled start date of this trip instance.
   * Must be provided to disambiguate trips that are so late as to collide with
   * a scheduled trip on a next day. For example, for a train that departs 8:00
   * and 20:00 every day, and is 12 hours late, there would be two distinct
   * trips on the same time.
   * This field can be provided but is not mandatory for schedules in which such
   * collisions are impossible - for example, a service running on hourly
   * schedule where a vehicle that is one hour late is not considered to be
   * related to schedule anymore.
   * In YYYYMMDD format.
   */
  startDate?: Date; // 3 [string]
  /**
   * The relation between this trip and the static schedule. If a trip is done
   * in accordance with temporary schedule, not reflected in GTFS, then it
   * shouldn't be marked as SCHEDULED, but likely as ADDED.
   */
  scheduleRelationship?: GTFSRealtimeScheduleRelationship; // 4 [enum]
  /** The route_id from the GTFS that this selector refers to. */
  routeId?: string; // 5 [string]
  /**
   * The direction_id from the GTFS feed trips.txt file, indicating the
   * direction of travel for trips this selector refers to.
   */
  directionId?: number; // 6 [uint32]
  /**
   * Linkage to any modifications done to this trip (shape changes, removal or addition of stops).
   * If this field is provided, the `trip_id`, `route_id`, `direction_id`, `start_time`, `start_date` fields of the `TripDescriptor` MUST be left empty, to avoid confusion by consumers that aren't looking for the `ModifiedTripSelector` value.
   */
  modifiedTrip?: GTFSRealtimeModifiedTripSelector; // 7 [message]

  /**
   * @param pbf - The Protobuf object to read from
   * @param end - The end position of the message in the buffer
   */
  constructor(pbf: Protobuf, end: number) {
    pbf.readFields(this.#readTripDescriptor, this, end);
  }

  /**
   * @param tag - The tag of the message
   * @param tripDescriptor - The tripDescriptor to mutate
   * @param pbf - The Protobuf object to read from
   */
  #readTripDescriptor(tag: number, tripDescriptor: GTFSRealtimeTripDescriptor, pbf: Protobuf) {
    if (tag === 1) tripDescriptor.tripId = pbf.readString();
    else if (tag === 2) tripDescriptor.startTime = pbf.readString();
    else if (tag === 3) tripDescriptor.startDate = parseGtfsDate(pbf.readString());
    else if (tag === 4)
      tripDescriptor.scheduleRelationship = pbf.readVarint() as GTFSRealtimeScheduleRelationship;
    else if (tag === 5) tripDescriptor.routeId = pbf.readString();
    else if (tag === 6) tripDescriptor.directionId = pbf.readVarint();
    else if (tag === 7)
      tripDescriptor.modifiedTrip = new GTFSRealtimeModifiedTripSelector(
        pbf,
        pbf.readVarint() + pbf.pos,
      );
    else throw new Error('GTFSRealtimeTripDescriptor: unknown tag ' + tag);
  }
}

/**
 * A descriptor that identifies an instance of a GTFS trip, or all instances of
 * a trip along a route.
 * - To specify a single trip instance, the trip_id (and if necessary,
 *   start_time) is set. If route_id is also set, then it should be same as one
 *   that the given trip corresponds to.
 * - To specify all the trips along a given route, only the route_id should be
 *   set. Note that if the trip_id is not known, then stop sequence ids in
 *   TripUpdate are not sufficient, and stop_ids must be provided as well. In
 *   addition, absolute arrival/departure times must be provided.
 */
export class GTFSRealtimeModifiedTripSelector {
  /** The 'id' from the FeedEntity in which the contained TripModifications object affects this trip. */
  modificationsId?: string; // 1 [string]
  /** The trip_id from the GTFS feed that is modified by the modifications_id */
  affectedTripId?: string; // 2 [string]
  /** The initially scheduled start time of this trip instance, applied to the frequency based modified trip. Same definition as start_time in TripDescriptor. */
  startTime?: string; // 3 [string]
  /** The start date of this trip instance in YYYYMMDD format, applied to the modified trip. Same definition as start_date in TripDescriptor. */
  startDate?: Date; // 4 [string]

  /**
   * @param pbf - The Protobuf object to read from
   * @param end - The end position of the message in the buffer
   */
  constructor(pbf: Protobuf, end: number) {
    pbf.readFields(this.#readMTS, this, end);
  }

  /**
   * @param tag - The tag of the message
   * @param tripDescriptor - The tripDescriptor to mutate
   * @param pbf - The Protobuf object to read from
   */
  #readMTS(tag: number, tripDescriptor: GTFSRealtimeModifiedTripSelector, pbf: Protobuf) {
    if (tag === 1) tripDescriptor.modificationsId = pbf.readString();
    else if (tag === 2) tripDescriptor.affectedTripId = pbf.readString();
    else if (tag === 3) tripDescriptor.startTime = pbf.readString();
    else if (tag === 4) tripDescriptor.startDate = parseGtfsDate(pbf.readString());
    else throw new Error('GTFSRealtimeModifiedTripSelector: unknown tag ' + tag);
  }
}
