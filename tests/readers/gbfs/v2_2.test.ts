import Ajv from 'ajv';
import addFormats from 'ajv-formats';
import { buildGBFSReader } from '../../../src';
import { buildServer } from '../../server';
import { expect, test } from 'bun:test';

import {
  gbfsFreeBikeStatusSchemaV22,
  gbfsGeofencingZonesSchemaV22,
  gbfsSchemaV22,
  gbfsStationInformationSchemaV22,
  gbfsStationStatusSchemaV22,
  gbfsSystemAlertsSchemaV22,
  gbfsSystemCalendarSchemaV22,
  gbfsSystemHoursSchemaV22,
  gbfsSystemInformationSchemaV22,
  gbfsSystemPricingPlansSchemaV22,
  gbfsSystemRegionsSchemaV22,
  gbfsVehicleTypesSchemaV22,
  gbfsVersionsSchemaV22,
} from '../../../src';

import type { GBFSReaderV2 } from '../../../src';

const ajv = new Ajv({ strict: false });
addFormats(ajv);

test('version 2.2', async () => {
  const server = buildServer();

  const gbfsReader = (await buildGBFSReader(
    `http://localhost:${server.port}/readers/gbfs/fixtures/v2.2/gbfs.json`,
  )) as GBFSReaderV2;
  const {
    freeBikeStatus,
    gbfs,
    version,
    gbfsVersions,
    geofencingZones,
    stationInformation,
    stationStatus,
    systemAlerts,
    systemCalendar,
    systemHours,
    systemInformation,
    systemPricingPlans,
    systemRegions,
    vehicleTypes,
  } = gbfsReader;

  expect(version).toEqual(2);

  // freeBikeStatus
  expect(freeBikeStatus).toBeDefined();
  const validFreeBikeStatus = ajv.compile(gbfsFreeBikeStatusSchemaV22);
  expect(validFreeBikeStatus(freeBikeStatus)).toBeTrue();
  // gbfs
  expect(gbfs).toBeDefined();
  const validGBFS = ajv.compile(gbfsSchemaV22);
  expect(validGBFS(gbfs)).toBeTrue();
  // versions
  if (gbfsVersions === undefined) throw new Error('versions is undefined');
  expect(gbfsVersions).toBeDefined();
  // @ts-expect-error - version 2.2-google is not supported
  gbfsVersions.data.versions = gbfsVersions.data.versions.filter((v) => v.version !== '2.2-google');
  const validGBFSVersions = ajv.compile(gbfsVersionsSchemaV22);
  expect(validGBFSVersions(gbfsVersions)).toBeTrue();
  // geofencingZones
  expect(geofencingZones).toBeDefined();
  const validGeofencingZones = ajv.compile(gbfsGeofencingZonesSchemaV22);
  expect(validGeofencingZones(geofencingZones)).toBeTrue();
  // systemInformation
  expect(systemInformation).toBeDefined();
  const validSystemInformation = ajv.compile(gbfsSystemInformationSchemaV22);
  // fix invalid timezone
  systemInformation.data.timezone = 'America/New_York';
  expect(validSystemInformation(systemInformation)).toBeTrue();
  // stationInformation
  expect(stationInformation).toBeDefined();
  const validStationInformation = ajv.compile(gbfsStationInformationSchemaV22);
  expect(validStationInformation(stationInformation)).toBeTrue();
  // stationStatus
  expect(stationStatus).toBeDefined();
  const validStationStatus = ajv.compile(gbfsStationStatusSchemaV22);
  expect(validStationStatus(stationStatus)).toBeTrue();
  // systemAlerts
  expect(systemAlerts).toBeDefined();
  const validSystemAlerts = ajv.compile(gbfsSystemAlertsSchemaV22);
  expect(validSystemAlerts(systemAlerts)).toBeTrue();
  // systemCalendar
  expect(systemCalendar).toBeDefined();
  const validSystemCalendar = ajv.compile(gbfsSystemCalendarSchemaV22);
  expect(validSystemCalendar(systemCalendar)).toBeTrue();
  // systemHours
  expect(systemHours).toBeDefined();
  const validSystemHours = ajv.compile(gbfsSystemHoursSchemaV22);
  expect(validSystemHours(systemHours)).toBeTrue();
  // systemPricingPlans
  expect(systemPricingPlans).toBeDefined();
  const validSystemPricingPlans = ajv.compile(gbfsSystemPricingPlansSchemaV22);
  expect(validSystemPricingPlans(systemPricingPlans)).toBeTrue();
  // systemRegions
  expect(systemRegions).toBeDefined();
  const validSystemRegions = ajv.compile(gbfsSystemRegionsSchemaV22);
  expect(validSystemRegions(systemRegions)).toBeTrue();
  // vehicleTypes
  expect(vehicleTypes).toBeDefined();
  const validVehicleTypes = ajv.compile(gbfsVehicleTypesSchemaV22);
  expect(validVehicleTypes(vehicleTypes)).toBeTrue();

  await server.stop();

  const features = await Array.fromAsync(gbfsReader);
  expect(features.length).toBe(92);
});
