import {
  GTFSRealtimeAlert,
  GTFSRealtimeShape,
  GTFSRealtimeStop,
  GTFSRealtimeTripDescriptor,
  GTFSRealtimeTripModifications,
  GTFSRealtimeTripUpdate,
  GTFSRealtimeVehiclePosition,
} from '.';

import type { PbfReader } from '../..';

/** The type of the message. */
export type GTFSRealtimeMessageType =
  | 'deleted'
  | 'tripUpdate'
  | 'vehiclePosition'
  | 'alert'
  | 'shape'
  | 'stop'
  | 'tripModifications';

/**
 * A definition (or update) of an entity in the transit feed.
 * May be a TripUpdate, VehiclePosition, Alert, Shape, Stop, and/or TripModifications.
 * At least one of the above must be present (unless the entity is being deleted).
 */
export class GTFSRealtimeEntity {
  /**
   * The ids are used only to provide incrementality support. The id should be
   * unique within a FeedMessage. Consequent FeedMessages may contain
   * FeedEntities with the same id. In case of a DIFFERENTIAL update the new
   * FeedEntity with some id will replace the old FeedEntity with the same id
   * (or delete it - see is_deleted below).
   * The actual GTFS entities (e.g. stations, routes, trips) referenced by the
   * feed must be specified by explicit selectors (see EntitySelector below for
   * more info).
   */
  id!: string; // 1 [string]
  /** Whether this entity is to be deleted. Relevant only for incremental fetches */
  isDeleted = false; // 2 [bool]

  tripUpdate?: GTFSRealtimeTripUpdate; // 3
  /** Realtime positioning information for a given vehicle. */
  vehiclePosition?: GTFSRealtimeVehiclePosition; // 4
  alert?: GTFSRealtimeAlert; // 5
  /**
   * Describes the physical path that a vehicle takes when it's not part of the (CSV) GTFS,
   * such as for a detour. Shapes belong to Trips, and consist of a sequence of shape points.
   * Tracing the points in order provides the path of the vehicle.  Shapes do not need to intercept
   * the location of Stops exactly, but all Stops on a trip should lie within a small distance of
   * the shape for that trip, i.e. close to straight line segments connecting the shape points
   * NOTE: This message is still experimental, and subject to change. It may be formally adopted in the future.
   */
  shape?: GTFSRealtimeShape; // 6
  stop?: GTFSRealtimeStop; // 7
  tripModifications?: GTFSRealtimeTripModifications; // 8

  /**
   * @param pbf - The Protobuf object to read from
   * @param end - The end position of the message in the buffer
   */
  constructor(pbf: PbfReader, end: number) {
    pbf.readFields(this.#readEntity, this, end);
  }

  /**
   * The type of the message
   * @returns The type of the message
   */
  get messageType(): GTFSRealtimeMessageType {
    if (this.tripUpdate !== undefined) return 'tripUpdate';
    else if (this.vehiclePosition !== undefined) return 'vehiclePosition';
    else if (this.alert !== undefined) return 'alert';
    else if (this.shape !== undefined) return 'shape';
    else if (this.stop !== undefined) return 'stop';
    else if (this.tripModifications !== undefined) return 'tripModifications';
    else return 'deleted';
  }

  /**
   * @param tag - The tag of the message
   * @param entity - The entity to mutate
   * @param pbf - The Protobuf object to read from
   */
  #readEntity(tag: number, entity: GTFSRealtimeEntity, pbf: PbfReader): void {
    if (tag === 1) entity.id = pbf.readString();
    else if (tag === 2) entity.isDeleted = pbf.readBoolean();
    else if (tag === 3)
      entity.tripUpdate = new GTFSRealtimeTripUpdate(pbf, pbf.readVarint() + pbf.pos);
    else if (tag === 4)
      entity.vehiclePosition = new GTFSRealtimeVehiclePosition(pbf, pbf.readVarint() + pbf.pos);
    else if (tag === 5) entity.alert = new GTFSRealtimeAlert(pbf, pbf.readVarint() + pbf.pos);
    else if (tag === 6) entity.shape = new GTFSRealtimeShape(pbf, pbf.readVarint() + pbf.pos);
    else if (tag === 7) entity.stop = new GTFSRealtimeStop(pbf, pbf.readVarint() + pbf.pos);
    else if (tag === 8)
      entity.tripModifications = new GTFSRealtimeTripModifications(pbf, pbf.readVarint() + pbf.pos);
    else throw new Error('GTFSRealtimeEntity: unknown tag ' + tag);
  }
}

/**
 * A selector for an entity in a GTFS feed.
 * The values of the fields should correspond to the appropriate fields in the
 * GTFS feed.
 * At least one specifier must be given. If several are given, then the
 * matching has to apply to all the given specifiers.
 */
export class GTFSRealtimeEntitySelector {
  agencyId?: string; // 1 [string]
  routeId?: string; // 2 [string]

  // corresponds to route_type in GTFS.

  routeType?: number; // 3 [int32]
  trip?: GTFSRealtimeTripDescriptor; // 4 [message]
  stopId?: string; // 5 [string]

  // Corresponds to trip direction_id in GTFS trips.txt. If provided the
  // route_id must also be provided.

  directionId?: number; // 6 [uint32]

  /**
   * @param pbf - The Protobuf object to read from
   * @param end - The end position of the message in the buffer
   */
  constructor(pbf: PbfReader, end: number) {
    pbf.readFields(this.#readEntity, this, end);
  }

  /**
   * @param tag - The tag of the message
   * @param entitySel - The entitySel to mutate
   * @param pbf - The Protobuf object to read from
   */
  #readEntity(tag: number, entitySel: GTFSRealtimeEntitySelector, pbf: PbfReader): void {
    if (tag === 1) entitySel.agencyId = pbf.readString();
    else if (tag === 2) entitySel.routeId = pbf.readString();
    else if (tag === 3) entitySel.routeType = pbf.readSVarint();
    else if (tag === 4)
      entitySel.trip = new GTFSRealtimeTripDescriptor(pbf, pbf.readVarint() + pbf.pos);
    else if (tag === 5) entitySel.stopId = pbf.readString();
    else if (tag === 6) entitySel.directionId = pbf.readVarint();
    else throw new Error('GTFSRealtimeEntitySelector: unknown tag ' + tag);
  }
}
