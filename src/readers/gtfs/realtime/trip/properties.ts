import { parseGtfsDate } from '../../utils';

import type { Pbf as Protobuf } from '../../..';

/**
 * Defines updated properties of the trip, such as a new shape_id when there is a detour. Or defines the
 * trip_id, start_date, and start_time of a DUPLICATED trip.
 * NOTE: This message is still experimental, and subject to change. It may be formally adopted in the future.
 */
export class GTFSRealtimeTripProperties {
  /**
   * Defines the identifier of a new trip that is a duplicate of an existing trip defined in (CSV) GTFS trips.txt
   * but will start at a different service date and/or time (defined using the TripProperties.start_date and
   * TripProperties.start_time fields). See definition of trips.trip_id in (CSV) GTFS. Its value must be different
   * than the ones used in the (CSV) GTFS. Required if schedule_relationship=DUPLICATED, otherwise this field must not
   * be populated and will be ignored by consumers.
   * NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future.
   */
  tripId?: string; // 1 [string]
  /**
   * Service date on which the DUPLICATED trip will be run, in YYYYMMDD format. Required if
   * schedule_relationship=DUPLICATED, otherwise this field must not be populated and will be ignored by consumers.
   * NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future.
   */
  startDate?: Date; // 2 [string]
  /**
   * Defines the departure start time of the trip when itâ€™s duplicated. See definition of stop_times.departure_time
   * in (CSV) GTFS. Scheduled arrival and departure times for the duplicated trip are calculated based on the offset
   * between the original trip departure_time and this field. For example, if a GTFS trip has stop A with a
   * departure_time of 10:00:00 and stop B with departure_time of 10:01:00, and this field is populated with the value
   * of 10:30:00, stop B on the duplicated trip will have a scheduled departure_time of 10:31:00. Real-time prediction
   * delay values are applied to this calculated schedule time to determine the predicted time. For example, if a
   * departure delay of 30 is provided for stop B, then the predicted departure time is 10:31:30. Real-time
   * prediction time values do not have any offset applied to them and indicate the predicted time as provided.
   * For example, if a departure time representing 10:31:30 is provided for stop B, then the predicted departure time
   * is 10:31:30. This field is required if schedule_relationship is DUPLICATED, otherwise this field must not be
   * populated and will be ignored by consumers.
   * NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future.
   */
  startTime?: string; // 3 [string]
  /**
   * Specifies the shape of the vehicle travel path when the trip shape differs from the shape specified in
   * (CSV) GTFS or to specify it in real-time when it's not provided by (CSV) GTFS, such as a vehicle that takes differing
   * paths based on rider demand. See definition of trips.shape_id in (CSV) GTFS. If a shape is neither defined in (CSV) GTFS
   * nor in real-time, the shape is considered unknown. This field can refer to a shape defined in the (CSV) GTFS in shapes.txt
   * or a Shape in the (protobuf) real-time feed. The order of stops (stop sequences) for this trip must remain the same as
   * (CSV) GTFS. Stops that are a part of the original trip but will no longer be made, such as when a detour occurs, should
   * be marked as schedule_relationship=SKIPPED.
   * NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future.
   */
  shapeId?: string; // 4 [string]

  /**
   * @param pbf - The Protobuf object to read from
   * @param end - The end position of the message in the buffer
   */
  constructor(pbf: Protobuf, end: number) {
    pbf.readFields(this.#readTripUpdate, this, end);
  }

  /**
   * @param tag - The tag of the message
   * @param props - The props to mutate
   * @param pbf - The Protobuf object to read from
   */
  #readTripUpdate(tag: number, props: GTFSRealtimeTripProperties, pbf: Protobuf) {
    if (tag === 1) props.tripId = pbf.readString();
    else if (tag === 2) props.startDate = parseGtfsDate(pbf.readString());
    else if (tag === 3) props.startTime = pbf.readString();
    else if (tag === 4) props.shapeId = pbf.readString();
    else throw new Error('unknown tag ' + tag);
  }
}
