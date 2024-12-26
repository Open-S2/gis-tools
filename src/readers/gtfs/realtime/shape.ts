import type { Pbf as Protobuf } from '../..';

/**
 * Describes the physical path that a vehicle takes when it's not part of the (CSV) GTFS,
 * such as for a detour. Shapes belong to Trips, and consist of a sequence of shape points.
 * Tracing the points in order provides the path of the vehicle.  Shapes do not need to intercept
 * the location of Stops exactly, but all Stops on a trip should lie within a small distance of
 * the shape for that trip, i.e. close to straight line segments connecting the shape points
 * NOTE: This message is still experimental, and subject to change. It may be formally adopted in the future.
 */
export class GTFSRealtimeShape {
  /**
   * Identifier of the shape. Must be different than any shape_id defined in the (CSV) GTFS.
   * This field is required as per reference.md, but needs to be specified here optional because "Required is Forever"
   * See https://developers.google.com/protocol-buffers/docs/proto#specifying_field_rules
   * NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future.
   */
  shapeId?: string; // 1 [string]
  /**
   * Encoded polyline representation of the shape. This polyline must contain at least two points.
   * For more information about encoded polylines, see https://developers.google.com/maps/documentation/utilities/polylinealgorithm
   * This field is required as per reference.md, but needs to be specified here optional because "Required is Forever"
   * See https://developers.google.com/protocol-buffers/docs/proto#specifying_field_rules
   * NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future.
   */
  encodedPolyline?: string; // 2 [string]

  /**
   * @param pbf - The Protobuf object to read from
   * @param end - The end position of the message in the buffer
   */
  constructor(pbf: Protobuf, end: number) {
    pbf.readFields(this.#readShape, this, end);
  }

  /**
   * @param tag - The tag of the message
   * @param shape - The shape to mutate
   * @param pbf - The Protobuf object to read from
   */
  #readShape(tag: number, shape: GTFSRealtimeShape, pbf: Protobuf): void {
    if (tag === 1) shape.shapeId = pbf.readString();
    else if (tag === 2) shape.encodedPolyline = pbf.readString();
    else throw new Error(`GTFSRealtimeShape: Unexpected tag: ${tag}`);
  }
}
