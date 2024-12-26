import { parseCSVAsRecord } from '../../';

/**
 * # Timeframes
 *
 * **Optional**
 * Describes fare variations based on time of day, day of week, or specific dates.
 * Timeframes can be associated with fare products in `fare_leg_rules.txt`.
 * There must be no overlapping [start_time, end_time) intervals for the same
 * `timeframe_group_id` and `service_id`.
 */
export class GTFSTimeframe {
  /**
   * **Required**
   * Identifies a timeframe (or set of timeframes).
   */
  timeframeGroupId: string;
  /**
   * **Conditionally Required**
   * Beginning of a timeframe in HH:MM:SS format (<= 24:00:00).
   * The interval **includes** this time.
   * - If `end_time` is defined, `start_time` is required.
   * - If `end_time` is absent, `start_time` must be absent.
   * - If `start_time` is empty in the CSV, it is considered `00:00:00`.
   */
  startTime?: string;
  /**
   * **Conditionally Required**
   * End of a timeframe in HH:MM:SS format (<= 24:00:00).
   * The interval **excludes** this time.
   * - If `start_time` is defined, `end_time` is required.
   * - If `start_time` is absent, `end_time` must be absent.
   * - If `end_time` is empty in the CSV, it is considered `24:00:00`.
   */
  endTime?: string;
  /**
   * **Required**
   * Identifies a set of dates (`calendar.service_id` or `calendar_dates.service_id`)
   * when this timeframe is in effect.
   */
  serviceId: string;

  /** @param data - the parsed GTFS CSV data */
  constructor(data: Record<string, string>) {
    this.timeframeGroupId = data.timeframe_group_id;
    this.serviceId = data.service_id;
    /**
     * Since `start_time` and `end_time` are conditionally required,
     * we set them only if present, else leave them `undefined`.
     *
     * - If `data.start_time` is empty, treat it as "00:00:00".
     * - If `data.end_time` is empty, treat it as "24:00:00".
     * - Values > 24:00:00 are forbidden by GTFS specification.
     */
    if (data.start_time !== undefined) {
      this.startTime = data.start_time === undefined ? '00:00:00' : data.start_time;
    }
    if (data.end_time !== undefined) {
      this.endTime = data.end_time === undefined ? '24:00:00' : data.end_time;
    }
  }
}

/**
 * @param input - the input string to parse from
 * @returns - an array of Timeframes
 */
export function parseGTFSTimeframes(input: string): Record<string, GTFSTimeframe> {
  const data = parseCSVAsRecord(input);
  const res: Record<string, GTFSTimeframe> = {};
  for (const d of data) {
    const tf = new GTFSTimeframe(d);
    res[tf.timeframeGroupId] = tf;
  }

  return res;
}
