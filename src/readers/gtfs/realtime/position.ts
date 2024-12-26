import type { Pbf as Protobuf } from '../..';

/** A Position is a point on the Earth's surface. */
export class GTFSRealtimePosition {
  /** Degrees North, in the WGS-84 coordinate system. */
  latitude!: number; // 1 [float]
  /** Degrees East, in the WGS-84 coordinate system. */
  longitude!: number; // 2 [float]
  /**
   * Bearing, in degrees, clockwise from North, i.e., 0 is North and 90 is East.
   * This can be the compass bearing, or the direction towards the next stop
   * or intermediate location.
   * This should not be direction deduced from the sequence of previous
   * positions, which can be computed from previous data.
   */
  bearing?: number; // 3 [float]
  /** Odometer value, in meters. */
  odometer?: number; // 4 [double]
  /** Momentary speed measured by the vehicle, in meters per second. */
  speed?: number; // 5 [float]

  /**
   * @param pbf - The Protobuf object to read from
   * @param end - The end position of the message in the buffer
   */
  constructor(pbf: Protobuf, end: number) {
    pbf.readFields(this.#readTripDescriptor, this, end);
  }

  /**
   * @param tag - The tag of the message
   * @param position - The position to mutate
   * @param pbf - The Protobuf object to read from
   */
  #readTripDescriptor(tag: number, position: GTFSRealtimePosition, pbf: Protobuf) {
    if (tag === 1) position.latitude = pbf.readFloat();
    else if (tag === 2) position.longitude = pbf.readFloat();
    else if (tag === 3) position.bearing = pbf.readFloat();
    else if (tag === 4) position.odometer = pbf.readDouble();
    else if (tag === 5) position.speed = pbf.readFloat();
    else throw new Error(`GTFSRealtimePosition: Unexpected tag: ${tag}`);
  }
}
