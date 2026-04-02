import { GTFSRealtimeReader } from '../../../src/index.js';
import { expect, test } from 'bun:test';

test('vehicle position', async () => {
  const data = await Bun.file(`${__dirname}/fixtures/vehicle_position.pb`).arrayBuffer();
  const realtimeMessage = new GTFSRealtimeReader(new Uint8Array(data));

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

test('realtime_test_data_1', async () => {
  const data = await Bun.file(`${__dirname}/fixtures/realtime_test_data_1.pb`).arrayBuffer();
  const _realtimeMessage = new GTFSRealtimeReader(new Uint8Array(data));
  // const { header, entities } = realtimeMessage;
});

test('realtime_test_data_2', async () => {
  const data = await Bun.file(`${__dirname}/fixtures/realtime_test_data_2.pb`).arrayBuffer();
  const _realtimeMessage = new GTFSRealtimeReader(new Uint8Array(data));
  // const { header, entities } = realtimeMessage;
});

test('realtime_test_data_3', async () => {
  const data = await Bun.file(`${__dirname}/fixtures/realtime_test_data_3.pb`).arrayBuffer();
  const _realtimeMessage = new GTFSRealtimeReader(new Uint8Array(data));
  // const { header, entities } = realtimeMessage;
});

test('realtime_test_data_4', async () => {
  const data = await Bun.file(`${__dirname}/fixtures/realtime_test_data_4.pb`).arrayBuffer();
  const _realtimeMessage = new GTFSRealtimeReader(new Uint8Array(data));
  // const { header, entities } = realtimeMessage;
});

test('realtime_test_data_5', async () => {
  const data = await Bun.file(`${__dirname}/fixtures/realtime_test_data_5.pb`).arrayBuffer();
  const _realtimeMessage = new GTFSRealtimeReader(new Uint8Array(data));
  // const { header, entities } = realtimeMessage;
});

test('realtime_test_data_6', async () => {
  const data = await Bun.file(`${__dirname}/fixtures/realtime_test_data_6.pb`).arrayBuffer();
  const _realtimeMessage = new GTFSRealtimeReader(new Uint8Array(data));
  // const { header, entities } = realtimeMessage;
});

test('realtime_test_data_7', async () => {
  const data = await Bun.file(`${__dirname}/fixtures/realtime_test_data_7.pb`).arrayBuffer();
  const _realtimeMessage = new GTFSRealtimeReader(new Uint8Array(data));
  // const { header, entities } = realtimeMessage;
});

test('realtime_test_data_8', async () => {
  const data = await Bun.file(`${__dirname}/fixtures/realtime_test_data_8.pb`).arrayBuffer();
  const _realtimeMessage = new GTFSRealtimeReader(new Uint8Array(data));
  // const { header, entities } = realtimeMessage;
});

test('realtime_test_data_9', async () => {
  const data = await Bun.file(`${__dirname}/fixtures/realtime_test_data_9.pb`).arrayBuffer();
  const _realtimeMessage = new GTFSRealtimeReader(new Uint8Array(data));
  // const { header, entities } = realtimeMessage;
});

test('realtime_test_data_10', async () => {
  const data = await Bun.file(`${__dirname}/fixtures/realtime_test_data_10.pb`).arrayBuffer();
  const _realtimeMessage = new GTFSRealtimeReader(new Uint8Array(data));
  // const { header, entities } = realtimeMessage;
});

test('realtime_test_data_11', async () => {
  const data = await Bun.file(`${__dirname}/fixtures/realtime_test_data_11.pb`).arrayBuffer();
  const _realtimeMessage = new GTFSRealtimeReader(new Uint8Array(data));
  // const { header, entities } = realtimeMessage;
});
