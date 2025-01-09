import { parseCSVAsRecord } from '../../';
import { parseGtfsDate } from '../utils';

/**
 * Describes whether service is added or removed on a specific date.
 * 1 - Service added for this date.
 * 2 - Service removed for this date.
 */
export const enum GTFSExceptionType {
  Added = 1,
  Removed = 2,
}

/**
 * # Calendar Dates
 *
 * **Conditionally Required**
 * Explicitly activates or disables service on particular dates.
 * - If used with `calendar.txt`, it modifies the default service patterns.
 * - If `calendar.txt` is omitted, all service dates must be listed here.
 */
export class GTFSCalendarDate {
  /**
   * **Required**
   * Identifies a set of dates where service exception occurs.
   * References `calendar.service_id` if used with `calendar.txt`;
   * or acts as a standalone ID if `calendar.txt` is omitted.
   */
  serviceId: string;
  /**
   * **Required**
   * Date of the service exception, parsed as a JavaScript Date.
   * Originally in GTFS as a YYYYMMDD string (no time component).
   */
  date: Date;
  /**
   * **Required**
   * Indicates whether service is added (1) or removed (2) on this date.
   */
  exceptionType: GTFSExceptionType;

  /** @param data - the parsed GTFS CSV data */
  constructor(data: Record<string, string>) {
    this.serviceId = data.service_id;
    // If the date is present, parse it to a native Date.
    // Otherwise, handle missing or invalid dates as appropriate (e.g., throw an error).
    this.date = data.date !== undefined ? parseGtfsDate(data.date) : new Date(NaN);
    this.exceptionType =
      data.exception_type !== undefined
        ? (parseInt(data.exception_type, 10) as GTFSExceptionType)
        : GTFSExceptionType.Added; // default or handle as needed
  }
}

/**
 * @param input - the input string to parse from
 * @returns - an array of GTFSCalendarDates
 */
export function parseGTFSCalendarDates(input: string): GTFSCalendarDate[] {
  const data = parseCSVAsRecord(input);
  return data.map((d) => new GTFSCalendarDate(d));
}
