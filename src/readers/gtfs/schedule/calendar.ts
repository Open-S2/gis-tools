import { parseCSVAsRecord } from '../../';
import { parseGtfsDate } from '../utils';

/**
 * Enumeration to represent day availability in the calendar.
 * 0 = Not available, 1 = Available
 */
export enum GTFSDayAvailability {
  /** 0 - Service not available on this day. */
  NotAvailable = 0,
  /** 1 - Service available on this day. */
  Available = 1,
}

/**
 * # Calendar Information
 *
 * **Conditionally Required**
 * Defines a set of dates when service is available for one or more routes.
 * Required unless all dates of service are defined in `calendar_dates.txt`.
 */
export class GTFSCalendar {
  /**
   * **Required**
   * Identifies a set of dates when service is available.
   */
  serviceId: string;
  /**
   * **Required**
   * Service availability on Mondays: 0 or 1.
   */
  monday: GTFSDayAvailability;
  /**
   * **Required**
   * Service availability on Tuesdays: 0 or 1.
   */
  tuesday: GTFSDayAvailability;
  /**
   * **Required**
   * Service availability on Wednesdays: 0 or 1.
   */
  wednesday: GTFSDayAvailability;
  /**
   * **Required**
   * Service availability on Thursdays: 0 or 1.
   */
  thursday: GTFSDayAvailability;
  /**
   * **Required**
   * Service availability on Fridays: 0 or 1.
   */
  friday: GTFSDayAvailability;
  /**
   * **Required**
   * Service availability on Saturdays: 0 or 1.
   */
  saturday: GTFSDayAvailability;
  /**
   * **Required**
   * Service availability on Sundays: 0 or 1.
   */
  sunday: GTFSDayAvailability;
  /**
   * **Required**
   * Start service day (inclusive) for the interval. Format: YYYYMMDD
   */
  startDate: Date;
  /**
   * **Required**
   * End service day (inclusive) for the interval. Format: YYYYMMDD
   */
  endDate: Date;

  /** @param data - the parsed GTFS CSV data */
  constructor(data: Record<string, string>) {
    this.serviceId = data.service_id;
    this.monday =
      data.monday !== undefined
        ? (parseInt(data.monday, 10) as GTFSDayAvailability)
        : GTFSDayAvailability.NotAvailable;
    this.tuesday =
      data.tuesday !== undefined
        ? (parseInt(data.tuesday, 10) as GTFSDayAvailability)
        : GTFSDayAvailability.NotAvailable;
    this.wednesday =
      data.wednesday !== undefined
        ? (parseInt(data.wednesday, 10) as GTFSDayAvailability)
        : GTFSDayAvailability.NotAvailable;
    this.thursday =
      data.thursday !== undefined
        ? (parseInt(data.thursday, 10) as GTFSDayAvailability)
        : GTFSDayAvailability.NotAvailable;
    this.friday =
      data.friday !== undefined
        ? (parseInt(data.friday, 10) as GTFSDayAvailability)
        : GTFSDayAvailability.NotAvailable;
    this.saturday =
      data.saturday !== undefined
        ? (parseInt(data.saturday, 10) as GTFSDayAvailability)
        : GTFSDayAvailability.NotAvailable;
    this.sunday =
      data.sunday !== undefined
        ? (parseInt(data.sunday, 10) as GTFSDayAvailability)
        : GTFSDayAvailability.NotAvailable;
    this.startDate = parseGtfsDate(data.start_date);
    this.endDate = parseGtfsDate(data.end_date);
  }
}

/**
 * @param input - the input string to parse from
 * @returns - an array of Calendars
 */
export function parseGTFSCalendars(input: string): GTFSCalendar[] {
  const data = parseCSVAsRecord(input);
  return data.map((d) => new GTFSCalendar(d));
}
