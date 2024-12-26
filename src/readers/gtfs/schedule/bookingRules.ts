import { parseCSVAsRecord } from '../../';

/**
 *
 * Describes how far in advance rider can book:
 * 0 - Real-time
 * 1 - Same-day (with advance notice)
 * 2 - Prior day(s)
 */
export enum GTFSBookingType {
  RealTime = 0,
  SameDay = 1,
  PriorDays = 2,
}

/**
 * # Booking Rules
 *
 * **Optional**
 * Defines rules for booking rider-requested services. Useful when a trip or stop_time requires
 * advanced scheduling (e.g., dial-a-ride, on-demand pickup).
 *
 * **Primary Key**: (booking_rule_id)
 */
export class GTFSBookingRule {
  /**
   * **Required**
   * Identifies a booking rule (`booking_rule_id`).
   */
  id: string;
  /**
   * **Required**
   * Indicates how far in advance booking can be made.
   * 0 = Real-time, 1 = Same-day, 2 = Prior-day(s)
   */
  bookingType: GTFSBookingType;
  /**
   * **Conditionally Required**
   * Minimum number of minutes before travel to make the request.
   * Required for booking_type=1; forbidden otherwise.
   */
  priorNoticeDurationMin?: number;
  /**
   * **Conditionally Forbidden**
   * Maximum number of minutes before travel to make the same-day request.
   * - Forbidden for booking_type=0 or booking_type=2
   * - Optional for booking_type=1
   */
  priorNoticeDurationMax?: number;
  /**
   * **Conditionally Required**
   * Last day before travel to make booking request. E.g., 1 = 1 day in advance.
   * Required for booking_type=2; forbidden otherwise.
   */
  priorNoticeLastDay?: number;
  /**
   * **Conditionally Required**
   * Last time on the last day before travel to make booking request, e.g. "17:00:00".
   * Required if prior_notice_last_day is defined; forbidden otherwise.
   */
  priorNoticeLastTime?: string;
  /**
   * **Conditionally Forbidden**
   * Earliest day before travel to make booking request.
   * - Forbidden for booking_type=0.
   * - Forbidden for booking_type=1 if prior_notice_duration_max is defined.
   * - Optional otherwise (mainly for booking_type=2).
   */
  priorNoticeStartDay?: number;
  /**
   * **Conditionally Required**
   * Earliest time on the earliest day before travel, e.g. "00:00:00".
   * Required if prior_notice_start_day is defined; forbidden otherwise.
   */
  priorNoticeStartTime?: string;
  /**
   * **Conditionally Forbidden**
   * Service days on which last_day / start_day are counted (`calendar.service_id`).
   * - Optional if booking_type=2.
   * - Forbidden otherwise.
   */
  priorNoticeServiceId?: string;
  /**
   * **Optional**
   * Generic message to riders for on-demand booking instructions.
   */
  message?: string;
  /**
   * **Optional**
   * Message for on-demand pickup instructions.
   */
  pickupMessage?: string;
  /**
   * **Optional**
   * Message for on-demand drop-off instructions.
   */
  dropOffMessage?: string;
  /**
   * **Optional**
   * Phone number riders call to make the booking request.
   */
  phoneNumber?: string;
  /**
   * **Optional**
   * URL providing additional booking info.
   */
  infoUrl?: string;
  /**
   * **Optional**
   * URL to an online interface or app to make a booking request.
   */
  bookingUrl?: string;

  /** @param data - the parsed GTFS CSV data */
  constructor(data: Record<string, string>) {
    // Required
    this.id = data.booking_rule_id;
    this.bookingType =
      data.booking_type !== undefined && data.booking_type !== ''
        ? (parseInt(data.booking_type, 10) as GTFSBookingType)
        : GTFSBookingType.RealTime; // Default or handle differently
    // Conditionally required/forbidden fields
    if (data.prior_notice_duration_min !== undefined && data.prior_notice_duration_min !== '') {
      this.priorNoticeDurationMin = parseInt(data.prior_notice_duration_min, 10);
    }
    if (data.prior_notice_duration_max !== undefined && data.prior_notice_duration_max !== '') {
      this.priorNoticeDurationMax = parseInt(data.prior_notice_duration_max, 10);
    }
    if (data.prior_notice_last_day !== undefined && data.prior_notice_last_day !== '') {
      this.priorNoticeLastDay = parseInt(data.prior_notice_last_day, 10);
    }
    this.priorNoticeLastTime = data.prior_notice_last_time;
    // Start day/time logic
    if (data.prior_notice_start_day !== undefined && data.prior_notice_start_day !== '') {
      this.priorNoticeStartDay = parseInt(data.prior_notice_start_day, 10);
    }
    this.priorNoticeStartTime = data.prior_notice_start_time;
    // Service ID for advanced notice calculation
    this.priorNoticeServiceId = data.prior_notice_service_id;
    // Optional messages/phone/URLs
    this.message = data.message;
    this.pickupMessage = data.pickup_message;
    this.dropOffMessage = data.drop_off_message;
    this.phoneNumber = data.phone_number;
    this.infoUrl = data.info_url;
    this.bookingUrl = data.booking_url;
  }
}

/**
 * @param input - the input string to parse from
 * @returns - an array of BookingRules
 */
export function parseGTFSBookingRules(input: string): Record<string, GTFSBookingRule> {
  const data = parseCSVAsRecord(input);
  const res: Record<string, GTFSBookingRule> = {};
  for (const d of data) {
    const br = new GTFSBookingRule(d);
    res[br.id] = br;
  }

  return res;
}
