import { parseCSVAsRecord } from '../../';

/**
 *
 * Indicates the type of service for a trip with frequencies:
 * 0 or empty = Frequency-based trips
 * 1 = Schedule-based trips (with identical headway)
 */
export enum GTFSExactTimes {
  FrequencyBased = 0,
  ScheduleBased = 1,
}

/**
 * # Frequency
 *
 * **Optional**
 * Defines headway-based (or compressed schedule-based) service for specific trips.
 * Each record references a single trip and indicates:
 * - A start/end time window
 * - A headway (seconds between departures)
 * - Whether it’s frequency-based (exact_times=0) or schedule-based (exact_times=1).
 *
 * **Primary Key**: (`trip_id`, `start_time`)
 */
export class GTFSFrequency {
  /**
   * **Required**
   * Identifies the trip (`trips.trip_id`) to which the specified headway of service applies.
   */
  tripId: string;
  /**
   * **Required**
   * Time at which the first vehicle departs from the trip’s first stop
   * with the specified headway (HH:MM:SS, can exceed 24:00:00 if overnight).
   */
  startTime: string;
  /**
   * **Required**
   * Time at which service changes or ends (HH:MM:SS, can exceed 24:00:00 if overnight).
   */
  endTime: string;
  /**
   * **Required**
   * Headway in seconds between departures from the same stop for this trip,
   * during [start_time, end_time).
   */
  headwaySecs: number;
  /**
   * **Optional**
   * Whether this is frequency-based or schedule-based service.
   * - 0 or empty = Frequency-based
   * - 1 = Schedule-based
   */
  exactTimes?: GTFSExactTimes;

  /** @param data - the parsed GTFS CSV data */
  constructor(data: Record<string, string>) {
    this.tripId = data.trip_id;
    this.startTime = data.start_time;
    this.endTime = data.end_time;
    this.headwaySecs = data.headway_secs !== undefined ? parseInt(data.headway_secs, 10) : 0;

    // Default to 0 if none provided
    if (data.exact_times !== undefined && data.exact_times !== '') {
      this.exactTimes = parseInt(data.exact_times, 10) as GTFSExactTimes;
    }
  }
}

/**
 * @param input - the input string to parse from
 * @returns - an array of Frequency
 */
export function parseGTFSFrequencies(input: string): GTFSFrequency[] {
  const data = parseCSVAsRecord(input);
  return data.map((d) => new GTFSFrequency(d));
}
