import { GTFSRealtimeTranslatedString } from '..';

import type { Pbf as Protobuf } from '../../..';

export * from './timeEvent';
export * from './update';

/** The type of wheelchair boarding accessibility at a stop. */
export const enum GTFSRealtimeWheelchairBoarding {
  UNKNOWN = 0,
  AVAILABLE = 1,
  NOT_AVAILABLE = 2,
}

/**
 * Describes a stop which is served by trips. All fields are as described in the GTFS-Static specification.
 * NOTE: This message is still experimental, and subject to change. It may be formally adopted in the future.
 */
export class GTFSRealtimeStop {
  stopId?: string; // 1 [string]
  stopCode?: GTFSRealtimeTranslatedString; // 2 [message]
  stopName?: GTFSRealtimeTranslatedString; // 3 [message]
  ttsStopName?: GTFSRealtimeTranslatedString; // 4 [message]
  stopDesc?: GTFSRealtimeTranslatedString; // 5 [message]
  stopLat?: number; // 6 [float]
  stopLon?: number; // 7 [float]
  zoneId?: string; // 8 [string]
  stopUrl?: GTFSRealtimeTranslatedString; // 9 [string]
  parentStation?: string; // 11 [string]
  stopTimezone?: string; // 12 [string]
  wheelchairBoarding = GTFSRealtimeWheelchairBoarding.UNKNOWN; // 13 [enum]
  levelId?: string; // 14 [string]
  platformCode?: GTFSRealtimeTranslatedString; // 15 [string]

  /**
   * @param pbf - The Protobuf object to read from
   * @param end - The end position of the message in the buffer
   */
  constructor(pbf: Protobuf, end: number) {
    pbf.readFields(this.#readStop, this, end);
  }

  /**
   * @param tag - The tag of the message
   * @param stop - The stop to mutate
   * @param pbf - The Protobuf object to read from
   */
  #readStop(tag: number, stop: GTFSRealtimeStop, pbf: Protobuf): void {
    if (tag === 1) stop.stopId = pbf.readString();
    else if (tag === 2)
      stop.stopCode = new GTFSRealtimeTranslatedString(pbf, pbf.readVarint() + pbf.pos);
    else if (tag === 3)
      stop.stopName = new GTFSRealtimeTranslatedString(pbf, pbf.readVarint() + pbf.pos);
    else if (tag === 4)
      stop.ttsStopName = new GTFSRealtimeTranslatedString(pbf, pbf.readVarint() + pbf.pos);
    else if (tag === 5)
      stop.stopDesc = new GTFSRealtimeTranslatedString(pbf, pbf.readVarint() + pbf.pos);
    else if (tag === 6) stop.stopLat = pbf.readFloat();
    else if (tag === 7) stop.stopLon = pbf.readFloat();
    else if (tag === 8) stop.zoneId = pbf.readString();
    else if (tag === 9)
      stop.stopUrl = new GTFSRealtimeTranslatedString(pbf, pbf.readVarint() + pbf.pos);
    else if (tag === 11) stop.parentStation = pbf.readString();
    else if (tag === 12) stop.stopTimezone = pbf.readString();
    else if (tag === 13)
      stop.wheelchairBoarding = pbf.readVarint() as GTFSRealtimeWheelchairBoarding;
    else if (tag === 14) stop.levelId = pbf.readString();
    else if (tag === 15)
      stop.platformCode = new GTFSRealtimeTranslatedString(pbf, pbf.readVarint() + pbf.pos);
  }
}
