import { GTFSRealtimeOccupancyStatus, GTFSRealtimeStopTimeEvent } from '..';

import type { PbfReader } from '../../..';

/** The relation between the StopTimeEvents and the static schedule. */
export const enum GTFSRealtimeScheduleRelationshipUpdate {
  /**
   * The vehicle is proceeding in accordance with its static schedule of
   * stops, although not necessarily according to the times of the schedule.
   * At least one of arrival and departure must be provided. If the schedule
   * for this stop contains both arrival and departure times then so must
   * this update. Frequency-based trips (GTFS frequencies.txt with exact_times = 0)
   * should not have a SCHEDULED value and should use UNSCHEDULED instead.
   */
  SCHEDULED = 0,
  /**
   * The stop is skipped, i.e., the vehicle will not stop at this stop.
   * Arrival and departure are optional.
   */
  SKIPPED = 1,
  /**
   * No StopTimeEvents are given for this stop.
   * The main intention for this value is to give time predictions only for
   * part of a trip, i.e., if the last update for a trip has a NO_DATA
   * specifier, then StopTimeEvents for the rest of the stops in the trip
   * are considered to be unspecified as well.
   * Neither arrival nor departure should be supplied.
   */
  NO_DATA = 2,
  /**
   * The vehicle is operating a trip defined in GTFS frequencies.txt with exact_times = 0.
   * This value should not be used for trips that are not defined in GTFS frequencies.txt,
   * or trips in GTFS frequencies.txt with exact_times = 1. Trips containing StopTimeUpdates
   * with ScheduleRelationship=UNSCHEDULED must also set TripDescriptor.ScheduleRelationship=UNSCHEDULED.
   * NOTE: This field is still experimental, and subject to change. It may be
   * formally adopted in the future.
   */
  UNSCHEDULED = 3,
}

/**
 * Realtime update for arrival and/or departure events for a given stop on a
 * trip. Updates can be supplied for both past and future events.
 * The producer is allowed, although not required, to drop past events.
 *
 * The update is linked to a specific stop either through stop_sequence or
 * stop_id, so one of the fields below must necessarily be set.
 * See the documentation in TripDescriptor for more information.
 */
export class GTFSRealtimeStopTimeUpdate {
  /** Must be the same as in stop_times.txt in the corresponding GTFS feed. */
  stopSequence?: number; // 1 [uint32]
  arrival?: GTFSRealtimeStopTimeEvent; // 2 [message]
  departure?: GTFSRealtimeStopTimeEvent; // 3 [message]
  /** Must be the same as in stops.txt in the corresponding GTFS feed. */
  stopId?: string; // 4 [string]
  /** The relation between the StopTimeEvents and the static schedule. */
  scheduleRelationship = GTFSRealtimeScheduleRelationshipUpdate.SCHEDULED; // 5 [enum]
  /**
   * Realtime updates for certain properties defined within GTFS stop_times.txt
   * NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future.
   */
  stopTimeProperties?: GTFSRealtimeStopTimeProperties; // 6 [message]
  /**
   * Expected occupancy after departure from the given stop.
   * Should be provided only for future stops.
   * In order to provide departure_occupancy_status without either arrival or
   * departure StopTimeEvents, ScheduleRelationship should be set to NO_DATA.
   */
  departureOccupancyStatus?: GTFSRealtimeOccupancyStatus; // 7 [enum]

  /**
   * @param pbf - The Protobuf object to read from
   * @param end - The end position of the message in the buffer
   */
  constructor(pbf: PbfReader, end: number) {
    pbf.readFields(this.#readstopTimeUpdate, this, end);
  }

  /**
   * @param tag - The tag of the message
   * @param stopTimeUpdate - The stopTimeUpdate to mutate
   * @param pbf - The Protobuf object to read from
   */
  #readstopTimeUpdate(tag: number, stopTimeUpdate: GTFSRealtimeStopTimeUpdate, pbf: PbfReader) {
    if (tag === 1) stopTimeUpdate.stopSequence = pbf.readVarint();
    else if (tag === 2)
      stopTimeUpdate.arrival = new GTFSRealtimeStopTimeEvent(pbf, pbf.readVarint() + pbf.pos);
    else if (tag === 3)
      stopTimeUpdate.departure = new GTFSRealtimeStopTimeEvent(pbf, pbf.readVarint() + pbf.pos);
    else if (tag === 4) stopTimeUpdate.stopId = pbf.readString();
    else if (tag === 5)
      stopTimeUpdate.scheduleRelationship =
        pbf.readVarint() as GTFSRealtimeScheduleRelationshipUpdate;
    else if (tag === 6)
      stopTimeUpdate.stopTimeProperties = new GTFSRealtimeStopTimeProperties(
        pbf,
        pbf.readVarint() + pbf.pos,
      );
    else if (tag === 7)
      stopTimeUpdate.departureOccupancyStatus = pbf.readVarint() as GTFSRealtimeOccupancyStatus;
    else throw new Error('GTFSRealtimeStopTimeUpdate: unknown tag: ' + tag);
  }
}

/**
 * Provides the updated values for the stop time.
 * NOTE: This message is still experimental, and subject to change. It may be formally adopted in the future.
 */
export class GTFSRealtimeStopTimeProperties {
  /**
   * Supports real-time stop assignments. Refers to a stop_id defined in the GTFS stops.txt.
   * The new assigned_stop_id should not result in a significantly different trip experience for the end user than
   * the stop_id defined in GTFS stop_times.txt. In other words, the end user should not view this new stop_id as an
   * "unusual change" if the new stop was presented within an app without any additional context.
   * For example, this field is intended to be used for platform assignments by using a stop_id that belongs to the
   * same station as the stop originally defined in GTFS stop_times.txt.
   * To assign a stop without providing any real-time arrival or departure predictions, populate this field and set
   * StopTimeUpdate.schedule_relationship = NO_DATA.
   * If this field is populated, it is preferred to omit `StopTimeUpdate.stop_id` and use only `StopTimeUpdate.stop_sequence`. If
   * `StopTimeProperties.assigned_stop_id` and `StopTimeUpdate.stop_id` are populated, `StopTimeUpdate.stop_id` must match `assigned_stop_id`.
   * Platform assignments should be reflected in other GTFS-realtime fields as well
   * (e.g., `VehiclePosition.stop_id`).
   * NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future.
   */
  assignedStopId?: string; // 1 [string]

  /**
   * @param pbf - The Protobuf object to read from
   * @param end - The end position of the message in the buffer
   */
  constructor(pbf: PbfReader, end: number) {
    pbf.readFields(this.#readStopTimeProps, this, end);
  }

  /**
   * @param tag - The tag of the message
   * @param stopTimeProp - The stopTimeProp to mutate
   * @param pbf - The Protobuf object to read from
   */
  #readStopTimeProps(tag: number, stopTimeProp: GTFSRealtimeStopTimeProperties, pbf: PbfReader) {
    if (tag === 1) stopTimeProp.assignedStopId = pbf.readString();
    else throw new Error('GTFSRealtimeStopTimeProperties: unknown tag: ' + tag);
  }
}
