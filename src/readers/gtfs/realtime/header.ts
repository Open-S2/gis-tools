import type { Pbf as Protobuf } from '../..';

/**
 * Determines whether the current fetch is incremental. Currently,
 * DIFFERENTIAL mode is unsupported and behavior is unspecified for feeds
 * that use this mode.  There are discussions on the GTFS Realtime mailing
 * list around fully specifying the behavior of DIFFERENTIAL mode and the
 * documentation will be updated when those discussions are finalized.
 */
export enum Incrementality {
  FULL_DATASET = 0,
  DIFFERENTIAL = 1,
}

/** Metadata about a feed, included in feed messages. */
export class GTFSRealtimeHeader {
  /**
   * Version of the feed specification.
   * The current version is 2.0.  Valid versions are "2.0", "1.0".
   */
  gtfsRealtimeVersion = '2'; // 1 [string]
  /**
   * Determines whether the current fetch is incremental. Currently,
   * DIFFERENTIAL mode is unsupported and behavior is unspecified for feeds
   * that use this mode.  There are discussions on the GTFS Realtime mailing
   * list around fully specifying the behavior of DIFFERENTIAL mode and the
   * documentation will be updated when those discussions are finalized.
   */
  incrementality = Incrementality.FULL_DATASET; // 2 [enum]
  /**
   * This timestamp identifies the moment when the content of this feed has been
   * created (in server time). In POSIX time (i.e., number of seconds since
   * January 1st 1970 00:00:00 UTC).
   */
  timestamp?: Date; // 3 [uint64]
  /**
   * String that matches the feed_info.feed_version from the GTFS feed that the real
   * time data is based on. Consumers can use this to identify which GTFS feed is
   * currently active or when a new one is available to download.
   */
  feedVersion?: string; // 4 [string]
  /**
   * @param pbf - The Protobuf object to read from
   * @param end - The end position of the message in the buffer
   */
  constructor(pbf: Protobuf, end: number) {
    pbf.readFields(this.#readHeader, this, end);
  }

  /**
   * @param tag - The tag of the message
   * @param layer - The layer to mutate
   * @param pbf - The Protobuf object to read from
   */
  #readHeader(tag: number, layer: GTFSRealtimeHeader, pbf: Protobuf): void {
    if (tag === 1) layer.gtfsRealtimeVersion = pbf.readString();
    else if (tag === 2) layer.incrementality = pbf.readVarint() as Incrementality;
    else if (tag === 3) layer.timestamp = new Date(pbf.readVarint() * 1000);
    else if (tag === 4) layer.feedVersion = pbf.readString();
    else throw new Error('GTFSRealtimeHeader: unknown tag ' + tag);
  }
}
