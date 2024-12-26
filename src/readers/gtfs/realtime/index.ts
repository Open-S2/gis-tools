import { GTFSRealtimeEntity } from './entity';
import { GTFSRealtimeHeader } from './header';
import { Pbf as Protobuf } from '../..';

export * from './stop';
export * from './trip';
export * from './alert';
export * from './entity';
export * from './header';
export * from './position';
export * from './shape';
export * from './util';
export * from './vehiclePosition';

/**
 * A GTFS Realtime message.
 * The input is a Uint8Array that has encoded protobuffer messages.
 * @see {@link Protobuf}.
 *
 * The contents of a feed message.
 * A feed is a continuous stream of feed messages. Each message in the stream is
 * obtained as a response to an appropriate HTTP GET request.
 * A realtime feed is always defined with relation to an existing GTFS feed.
 * All the entity ids are resolved with respect to the GTFS feed.
 * Note that "required" and "optional" as stated in this file refer to Protocol
 * Buffer cardinality, not semantic cardinality.  See reference.md at
 * https://github.com/google/transit/tree/master/gtfs-realtime for field
 * semantic cardinality.
 *
 * Example:
 *
 * ```ts
 * import { GTFSRealtime } from 's2-tools';
 *
 * const vectorTile = new GTFSRealtime(data);
 * const { landuse } = vectorTile.layers;
 * const firstFeature = landuse.features(0);
 * ```
 */
export class GTFSRealtime {
  header!: GTFSRealtimeHeader;
  entities: GTFSRealtimeEntity[] = [];
  /**
   * @param data - the input data to parse
   * @param end - the size of the data, leave blank to parse the entire data
   */
  constructor(data: Uint8Array<ArrayBuffer>, end = 0) {
    const pbf = new Protobuf(data);
    pbf.readFields(this.#readMessage, this, end);
  }

  /**
   * Read in the message
   * @param tag - the tag to read
   * @param message - the GTFSRealtime object to modify
   * @param pbf - the Protobuf to pull the appropriate data from
   */
  #readMessage(tag: number, message: GTFSRealtime, pbf: Protobuf): void {
    if (tag === 1) {
      message.header = new GTFSRealtimeHeader(pbf, pbf.readVarint() + pbf.pos);
    } else if (tag === 2) {
      message.entities.push(new GTFSRealtimeEntity(pbf, pbf.readVarint() + pbf.pos));
    } else throw new Error('GTFSRealtime: unknown tag ' + tag);
  }
}
