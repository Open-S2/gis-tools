import { GTFSRealtime } from '../../../src';
import { expect, test } from 'bun:test';

// TODO:
// - [ ] trip update
// - [ ] alert
// - [ ] shape
// - [ ] stop
// - [ ] trip modifications

test('vehicle position', async () => {
  const data = await Bun.file(`${__dirname}/fixtures/vehicle_position.pb`).arrayBuffer();
  const realtimeMessage = new GTFSRealtime(new Uint8Array(data));

  const { header, entities } = realtimeMessage;

  // header
  const { gtfsRealtimeVersion, incrementality, timestamp, feedVersion } = header;
  expect(gtfsRealtimeVersion).toEqual('1.0');
  expect(incrementality).toEqual(0);
  expect(timestamp).toBeUndefined();
  expect(feedVersion).toBeUndefined();

  expect(entities.length).toEqual(1);

  const firstEntity = entities[0];
  expect(firstEntity.messageType).toEqual('vehiclePosition');
  const { id, isDeleted, vehiclePosition } = firstEntity;
  expect(id).toEqual('1');
  expect(isDeleted).toEqual(false);
  expect(vehiclePosition).toBeDefined();
  if (vehiclePosition === undefined) throw Error('vehiclePosition is undefined');

  const {
    trip,
    position,
    currentStopSequence,
    currentStatus,
    timestamp: timestamp2,
    congestionLevel,
    stopId,
    vehicle,
    occupancyStatus,
    occupancyPercentage,
    multiCarriageDetails,
  } = vehiclePosition;

  expect(trip).toBeDefined();
  expect(position).toBeDefined();
  expect(currentStopSequence).toBeUndefined();
  expect(currentStatus).toEqual(2);
  expect(timestamp2).toBeUndefined();
  expect(congestionLevel).toBeUndefined();
  expect(stopId).toBeUndefined();
  expect(vehicle).toBeDefined();
  expect(occupancyStatus).toBeUndefined();
  expect(occupancyPercentage).toBeUndefined();
  expect(multiCarriageDetails).toEqual([]);

  if (trip === undefined || position === undefined || vehicle === undefined)
    throw Error('trip or position or vehicle is undefined');

  const { tripId, startTime, startDate, scheduleRelationship, routeId, directionId, modifiedTrip } =
    trip;
  expect(tripId).toEqual('t0');
  expect(startTime).toBeUndefined();
  expect(startDate).toBeUndefined();
  expect(scheduleRelationship).toBeUndefined();
  expect(routeId).toBeUndefined();
  expect(directionId).toBeUndefined();
  expect(modifiedTrip).toBeUndefined();

  const { latitude, longitude, bearing, odometer, speed } = position;
  expect(latitude).toEqual(47);
  expect(longitude).toEqual(-122);
  expect(bearing).toBeUndefined();
  expect(odometer).toBeUndefined();
  expect(speed).toBeUndefined();

  const { id: vehicleId, label, licensePlate, wheelchairAccessible } = vehicle;
  expect(vehicleId).toEqual('1');
  expect(label).toBeUndefined();
  expect(licensePlate).toBeUndefined();
  expect(wheelchairAccessible).toEqual(0);
});
