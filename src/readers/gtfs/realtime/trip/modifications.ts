import type { PbfReader } from '../../..';

/** NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future. */
export class GTFSRealtimeTripModifications {
  /** A list of selected trips affected by this TripModifications. */
  selectedTrips: GTFSRealtimeSelectedTrips[] = []; // 1 [repeated message]
  /**
   * A list of start times in the real-time trip descriptor for the trip_id defined in trip_ids.
   * Useful to target multiple departures of a trip_id in a frequency-based trip.
   */
  startTimes: string[] = []; // 2 [repeated string]
  /**
   * Dates on which the modifications occurs, in the YYYYMMDD format. Producers SHOULD only
   * transmit detours occurring within the next week.
   * The dates provided should not be used as user-facing information, if a user-facing start and
   * end date needs to be provided, they can be provided in the linked service alert with `service_alert_id`
   */
  serviceDates: string[] = []; // 3 [repeated string]
  /** A list of modifications to apply to the affected trips. */
  modifications: GTFSRealtimeModification[] = []; // 4 [repeated message]

  /**
   * @param pbf - The Protobuf object to read from
   * @param end - The end position of the message in the buffer
   */
  constructor(pbf: PbfReader, end: number) {
    pbf.readFields(this.#readTripMods, this, end);
  }

  /**
   * @param tag - The tag of the message
   * @param tripMods - The tripMods to mutate
   * @param pbf - The Protobuf object to read from
   */
  #readTripMods(tag: number, tripMods: GTFSRealtimeTripModifications, pbf: PbfReader) {
    if (tag === 1)
      tripMods.selectedTrips.push(new GTFSRealtimeSelectedTrips(pbf, pbf.readVarint() + pbf.pos));
    else if (tag === 2) tripMods.startTimes.push(pbf.readString());
    else if (tag === 3) tripMods.serviceDates.push(pbf.readString());
    else if (tag === 4)
      tripMods.modifications.push(new GTFSRealtimeModification(pbf, pbf.readVarint() + pbf.pos));
    else throw new Error(`GTFSRealtimeTripModifications: Unexpected tag: ${tag}`);
  }
}

/**
 * A `Modification` message replaces a span of n stop times from each affected trip starting at `start_stop_selector`.
 */
export class GTFSRealtimeModification {
  /**
   * The stop selector of the first stop_time of the original trip that is to be affected by this modification.
   * Used in conjuction with `end_stop_selector`.
   * `start_stop_selector` is required and is used to define the reference stop used with `travel_time_to_stop`.
   */
  startStopSelector?: GTFSRealtimeStopSelector; // 1 [message]
  /**
   * The stop selector of the last stop of the original trip that is to be affected by this modification.
   * The selection is inclusive, so if only one stop_time is replaced by that modification, `start_stop_selector`
   * and `end_stop_selector` must be equivalent.
   * If no stop_time is replaced, `end_stop_selector` must not be provided. It's otherwise required.
   */
  endStopSelector?: GTFSRealtimeStopSelector; // 2 [message]
  /**
   * The number of seconds of delay to add to all departure and arrival times following the end of this modification.
   * If multiple modifications apply to the same trip, the delays accumulate as the trip advances.
   */
  propagatedModificationDelay = 0; // 3 [int32]
  /**
   * A list of replacement stops, replacing those of the original trip.
   * The length of the new stop times may be less, the same, or greater than the number of replaced stop times.
   */
  replacementStops: GTFSRealtimeReplacementStop[] = []; // 4 [repeated message]
  /**
   * An `id` value from the `FeedEntity` message that contains the `Alert` describing this Modification
   * for user-facing communication.
   */
  serviceAlertId?: string; // 5 [string]
  /**
   * This timestamp identifies the moment when the modification has last been changed.
   * In POSIX time (i.e., number of seconds since January 1st 1970 00:00:00 UTC).
   */
  lastModifiedTime?: Date; // 6 [uint64]

  /**
   * @param pbf - The Protobuf object to read from
   * @param end - The end position of the message in the buffer
   */
  constructor(pbf: PbfReader, end: number) {
    pbf.readFields(this.#readMod, this, end);
  }

  /**
   * @param tag - The tag of the message
   * @param mod - The mod to mutate
   * @param pbf - The Protobuf object to read from
   */
  #readMod(tag: number, mod: GTFSRealtimeModification, pbf: PbfReader) {
    if (tag === 1)
      mod.startStopSelector = new GTFSRealtimeStopSelector(pbf, pbf.readVarint() + pbf.pos);
    else if (tag === 2)
      mod.endStopSelector = new GTFSRealtimeStopSelector(pbf, pbf.readVarint() + pbf.pos);
    else if (tag === 3) mod.propagatedModificationDelay = pbf.readSVarint();
    else if (tag === 4)
      mod.replacementStops.push(new GTFSRealtimeReplacementStop(pbf, pbf.readVarint() + pbf.pos));
    else if (tag === 5) mod.serviceAlertId = pbf.readString();
    else if (tag === 6) mod.lastModifiedTime = new Date(pbf.readVarint() * 1000);
    else throw new Error(`GTFSRealtimeModification: Unexpected tag: ${tag}`);
  }
}

/**
 * NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future.
 * Select a stop by stop sequence or by stop_id. At least one of the two values must be provided.
 */
export class GTFSRealtimeStopSelector {
  /** Must be the same as in stop_times.txt in the corresponding GTFS feed. */
  stopSequence?: number; // 1 [uint32]
  /** Must be the same as in stops.txt in the corresponding GTFS feed. */
  stopId?: string; // 2 [string]

  /**
   * @param pbf - The Protobuf object to read from
   * @param end - The end position of the message in the buffer
   */
  constructor(pbf: PbfReader, end: number) {
    pbf.readFields(this.#readStopSelector, this, end);
  }

  /**
   * @param tag - The tag of the message
   * @param stopSelector - The stopSelector to mutate
   * @param pbf - The Protobuf object to read from
   */
  #readStopSelector(tag: number, stopSelector: GTFSRealtimeStopSelector, pbf: PbfReader) {
    if (tag === 1) stopSelector.stopSequence = pbf.readVarint();
    else if (tag === 2) stopSelector.stopId = pbf.readString();
    else throw new Error(`GTFSRealtimeStopSelector: Unexpected tag: ${tag}`);
  }
}

/** Selected trips affected by TripModifications. */
export class GTFSRealtimeSelectedTrips {
  /** A list of trips affected with this replacement that all have the same new `shape_id`. */
  tripIds: string[] = []; // 1 [repeated string]
  /**
   * The ID of the new shape for the modified trips in this SelectedTrips.
   * May refer to a new shape added using a GTFS-RT Shape message, or to an existing shape defined in
   * the GTFS-Static feed's shapes.txt.
   */
  shapeId?: string; // 2 [string]

  /**
   * @param pbf - The Protobuf object to read from
   * @param end - The end position of the message in the buffer
   */
  constructor(pbf: PbfReader, end: number) {
    pbf.readFields(this.#readSelectedTrips, this, end);
  }

  /**
   * @param tag - The tag of the message
   * @param selectedTrips - The selectedTrips to mutate
   * @param pbf - The Protobuf object to read from
   */
  #readSelectedTrips(tag: number, selectedTrips: GTFSRealtimeSelectedTrips, pbf: PbfReader) {
    if (tag === 1) selectedTrips.tripIds.push(pbf.readString());
    else if (tag === 2) selectedTrips.shapeId = pbf.readString();
    else throw new Error('GTFSRealtimeSelectedTrips: unknown tag: ' + tag);
  }
}

/** NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future. */
export class GTFSRealtimeReplacementStop {
  /**
   * The difference in seconds between the arrival time at this stop and the arrival time at the reference
   * stop. The reference stop is the stop prior to start_stop_selector. If the modification begins
   * at the first stop of the trip, then the first stop of the trip is the reference stop.
   * This value MUST be monotonically increasing and may only be a negative number if the first
   * stop of the original trip is the reference stop.
   */
  travelTimeToStop?: number; // 1 [int32]

  /**
   * The replacement stop ID which will now be visited by the trip. May refer to a new stop added
   * using a GTFS-RT Stop message, or to an existing stop defined in the GTFS-Static feed's stops.txt.
   * The stop MUST have location_type=0 (routable stops).
   */
  stopId?: string; // 2 [string]

  /**
   * @param pbf - The Protobuf object to read from
   * @param end - The end position of the message in the buffer
   */
  constructor(pbf: PbfReader, end: number) {
    pbf.readFields(this.#readReplacementStop, this, end);
  }

  /**
   * @param tag - The tag of the message
   * @param replaceStop - The replaceStop to mutate
   * @param pbf - The Protobuf object to read from
   */
  #readReplacementStop(tag: number, replaceStop: GTFSRealtimeReplacementStop, pbf: PbfReader) {
    if (tag === 1) replaceStop.travelTimeToStop = pbf.readSVarint();
    else if (tag === 2) replaceStop.stopId = pbf.readString();
    else throw new Error('GTFSRealtimeReplacementStop: unknown tag: ' + tag);
  }
}
