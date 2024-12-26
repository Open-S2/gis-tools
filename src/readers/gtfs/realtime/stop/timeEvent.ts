import type { Pbf as Protobuf } from '../../..';

/**
 * Timing information for a single predicted event (either arrival or
 * departure).
 * Timing consists of delay and/or estimated time, and uncertainty.
 * - delay should be used when the prediction is given relative to some
 *   existing schedule in GTFS.
 * - time should be given whether there is a predicted schedule or not. If
 *   both time and delay are specified, time will take precedence
 *   (although normally, time, if given for a scheduled trip, should be
 *   equal to scheduled time in GTFS + delay).
 *
 * Uncertainty applies equally to both time and delay.
 * The uncertainty roughly specifies the expected error in true delay (but
 * note, we don't yet define its precise statistical meaning). It's possible
 * for the uncertainty to be 0, for example for trains that are driven under
 * computer timing control.
 */
export class GTFSRealtimeStopTimeEvent {
  /**
   *  Delay (in seconds) can be positive (meaning that the vehicle is late) or
   *  negative (meaning that the vehicle is ahead of schedule). Delay of 0
   *  means that the vehicle is exactly on time.
   */
  delay?: number; // 1 [int32]
  /**
   *  Event as absolute time.
   *  In Unix time (i.e., number of seconds since January 1st 1970 00:00:00
   *  UTC).
   */
  time?: number; // 2 [int64]
  /**
   *  If uncertainty is omitted, it is interpreted as unknown.
   *  If the prediction is unknown or too uncertain, the delay (or time) field
   *  should be empty. In such case, the uncertainty field is ignored.
   *  To specify a completely certain prediction, set its uncertainty to 0.
   */
  uncertainty?: number; // 3 [int32]

  /**
   * @param pbf - The Protobuf object to read from
   * @param end - The end position of the message in the buffer
   */
  constructor(pbf: Protobuf, end: number) {
    pbf.readFields(this.#readstopTimeUpdate, this, end);
  }

  /**
   * @param tag - The tag of the message
   * @param stopTimeEvent - The stopTimeEvent to mutate
   * @param pbf - The Protobuf object to read from
   */
  #readstopTimeUpdate(tag: number, stopTimeEvent: GTFSRealtimeStopTimeEvent, pbf: Protobuf) {
    if (tag === 1) stopTimeEvent.delay = pbf.readSVarint();
    else if (tag === 2) stopTimeEvent.time = pbf.readSVarint();
    else if (tag === 3) stopTimeEvent.uncertainty = pbf.readSVarint();
    else throw new Error('unknown tag ' + tag);
  }
}
