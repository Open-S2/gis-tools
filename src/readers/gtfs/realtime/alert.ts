import { GTFSRealtimeEntitySelector, GTFSRealtimeTranslatedString } from '.';

import type { PbfReader } from '../..';

/** Severity of this alert. */
export const enum GTFSRealtimeSeverityLevel {
  UNKNOWN_SEVERITY = 1,
  INFO = 2,
  WARNING = 3,
  SEVERE = 4,
}

/**
 * What is the effect of this problem on the affected entity. If effect_detail is included, then
 * Effect must also be included.
 */
export const enum GTFSRealtimeEffect {
  NO_SERVICE = 1,
  REDUCED_SERVICE = 2,

  // We don't care about INsignificant delays: they are hard to detect, have
  // little impact on the user, and would clutter the results as they are too
  // frequent.
  SIGNIFICANT_DELAYS = 3,

  DETOUR = 4,
  ADDITIONAL_SERVICE = 5,
  MODIFIED_SERVICE = 6,
  OTHER_EFFECT = 7,
  UNKNOWN_EFFECT = 8,
  STOP_MOVED = 9,
  NO_EFFECT = 10,
  ACCESSIBILITY_ISSUE = 11,
}

/** Cause of this alert. If cause_detail is included, then Cause must also be included. */
export const enum GTFSRealtimeCause {
  UNKNOWN_CAUSE = 1,
  OTHER_CAUSE = 2, // Not machine-representable.
  TECHNICAL_PROBLEM = 3,
  STRIKE = 4, // Public transit agency employees stopped working.
  DEMONSTRATION = 5, // People are blocking the streets.
  ACCIDENT = 6,
  HOLIDAY = 7,
  WEATHER = 8,
  MAINTENANCE = 9,
  CONSTRUCTION = 10,
  POLICE_ACTIVITY = 11,
  MEDICAL_EMERGENCY = 12,
}

/** An alert, indicating some sort of incident in the public transit network. */
export class GTFSRealtimeAlert {
  /**
   * Time when the alert should be shown to the user. If missing, the
   * alert will be shown as long as it appears in the feed.
   * If multiple ranges are given, the alert will be shown during all of them.
   */
  activePeriods: GTFSRealtimeTimeRange[] = []; // 1 [repeated message]
  /** Entities whose users we should notify of this alert. */
  informedEntities: GTFSRealtimeEntitySelector[] = []; // 5 [repeated message]
  /** Cause of this alert. If cause_detail is included, then Cause must also be included. */
  cause = GTFSRealtimeCause.UNKNOWN_CAUSE; // 6 [enum]
  /**
   * What is the effect of this problem on the affected entity. If effect_detail is included, then
   * Effect must also be included.
   */
  effect = GTFSRealtimeEffect.UNKNOWN_EFFECT; // 7 [enum]
  /** The URL which provides additional information about the alert. */
  url?: GTFSRealtimeTranslatedString; // 8 [message]
  /** Alert header. Contains a short summary of the alert text as plain-text. */
  headerText?: GTFSRealtimeTranslatedString; // 10 [message]
  /**
   * Full description for the alert as plain-text. The information in the
   * description should add to the information of the header.
   */
  descriptionText?: GTFSRealtimeTranslatedString; // 11 [message]
  /**
   * Text for alert header to be used in text-to-speech implementations. This field is the
   * text-to-speech version of header_text.
   */
  ttsHeaderText?: GTFSRealtimeTranslatedString; // 12 [message]
  /**
   * Text for full description for the alert to be used in text-to-speech implementations.
   * This field is the text-to-speech version of description_text.
   */
  ttsDescriptionText?: GTFSRealtimeTranslatedString; // 13 [message]
  /** Severity of this alert. */
  severityLevel = GTFSRealtimeSeverityLevel.UNKNOWN_SEVERITY; // 14 [enum]
  /**
   * TranslatedImage to be displayed along the alert text. Used to explain visually the alert effect of a detour, station closure, etc. The image must enhance the understanding of the alert. Any essential information communicated within the image must also be contained in the alert text.
   * The following types of images are discouraged : image containing mainly text, marketing or branded images that add no additional information.
   * NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future.
   */
  image?: GTFSRealtimeTranslatedString; // 15 [message]
  /**
   * Text describing the appearance of the linked image in the `image` field (e.g., in case the image can't be displayed
   * or the user can't see the image for accessibility reasons). See the HTML spec for alt image text - https://html.spec.whatwg.org/#alt.
   * NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future
   */
  imageAlternativeText?: GTFSRealtimeTranslatedString; // 16 [message]
  /**
   * Description of the cause of the alert that allows for agency-specific language; more specific than the Cause. If cause_detail is included, then Cause must also be included.
   * NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future.
   */
  causeDetail?: GTFSRealtimeTranslatedString; // 17 [message]
  /**
   * Description of the effect of the alert that allows for agency-specific language; more specific than the Effect. If effect_detail is included, then Effect must also be included.
   * NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future.
   */
  effectDetail?: GTFSRealtimeTranslatedString; // 18 [message]

  /**
   * @param pbf - The Protobuf object to read from
   * @param end - The end position of the message in the buffer
   */
  constructor(pbf: PbfReader, end: number) {
    pbf.readFields(this.#readAlert, this, end);
  }

  /**
   * @param tag - The tag of the message
   * @param alert - The alert to mutate
   * @param pbf - The Protobuf object to read from
   */
  #readAlert(tag: number, alert: GTFSRealtimeAlert, pbf: PbfReader) {
    if (tag === 1)
      alert.activePeriods.push(new GTFSRealtimeTimeRange(pbf, pbf.readVarint() + pbf.pos));
    else if (tag === 5)
      alert.informedEntities.push(new GTFSRealtimeEntitySelector(pbf, pbf.readVarint() + pbf.pos));
    else if (tag === 6) alert.cause = pbf.readVarint() as GTFSRealtimeCause;
    else if (tag === 7) alert.effect = pbf.readVarint() as GTFSRealtimeEffect;
    else if (tag === 8)
      alert.url = new GTFSRealtimeTranslatedString(pbf, pbf.readVarint() + pbf.pos);
    else if (tag === 10)
      alert.headerText = new GTFSRealtimeTranslatedString(pbf, pbf.readVarint() + pbf.pos);
    else if (tag === 11)
      alert.descriptionText = new GTFSRealtimeTranslatedString(pbf, pbf.readVarint() + pbf.pos);
    else if (tag === 12)
      alert.ttsHeaderText = new GTFSRealtimeTranslatedString(pbf, pbf.readVarint() + pbf.pos);
    else if (tag === 13)
      alert.ttsDescriptionText = new GTFSRealtimeTranslatedString(pbf, pbf.readVarint() + pbf.pos);
    else if (tag === 14) alert.severityLevel = pbf.readVarint() as GTFSRealtimeSeverityLevel;
    else if (tag === 15)
      alert.image = new GTFSRealtimeTranslatedString(pbf, pbf.readVarint() + pbf.pos);
    else if (tag === 16)
      alert.imageAlternativeText = new GTFSRealtimeTranslatedString(
        pbf,
        pbf.readVarint() + pbf.pos,
      );
    else if (tag === 17)
      alert.causeDetail = new GTFSRealtimeTranslatedString(pbf, pbf.readVarint() + pbf.pos);
    else if (tag === 18)
      alert.effectDetail = new GTFSRealtimeTranslatedString(pbf, pbf.readVarint() + pbf.pos);
    else throw new Error('GTFSRealtimeAlert: unknown tag: ' + tag);
  }
}

/**
 * A time interval. The interval is considered active at time 't' if 't' is
 * greater than or equal to the start time and less than the end time.
 */
export class GTFSRealtimeTimeRange {
  // Start time, in POSIX time (i.e., number of seconds since January 1st 1970
  // 00:00:00 UTC).
  // If missing, the interval starts at minus infinity.
  start?: Date; // 1 [uint64]

  // End time, in POSIX time (i.e., number of seconds since January 1st 1970
  // 00:00:00 UTC).
  // If missing, the interval ends at plus infinity.
  end?: Date; // 2 [uint64]

  /**
   * @param pbf - The Protobuf object to read from
   * @param end - The end position of the message in the buffer
   */
  constructor(pbf: PbfReader, end: number) {
    pbf.readFields(this.#readTimeRange, this, end);
  }

  /**
   * @param tag - The tag of the message
   * @param timeRange - The timeRange to mutate
   * @param pbf - The Protobuf object to read from
   */
  #readTimeRange(tag: number, timeRange: GTFSRealtimeTimeRange, pbf: PbfReader) {
    if (tag === 1) timeRange.start = new Date(pbf.readVarint() * 1000);
    else if (tag === 2) timeRange.end = new Date(pbf.readVarint() * 1000);
    else throw new Error('GTFSRealtimeTimeRange: unknown tag: ' + tag);
  }
}
