import Ajv from 'ajv';
import addFormats from 'ajv-formats';
import { buildGBFSReader } from '../../../src';
import { buildServer } from '../../server';
import { expect, test } from 'bun:test';

import {
  GBFSReaderV3,
  gbfsGeofencingZonesSchemaV30,
  gbfsManifestSchemaV30,
  gbfsSchemaV30,
  gbfsStationInformationSchemaV30,
  gbfsStationStatusSchemaV30,
  gbfsSystemAlertsSchemaV30,
  gbfsSystemInformationSchemaV30,
  gbfsSystemPricingPlansSchemaV30,
  gbfsSystemRegionsSchemaV30,
  gbfsVehicleStatusSchemaV30,
  gbfsVersionsSchemaV30,
} from '../../../src';

const ajv = new Ajv({ strict: false });
addFormats(ajv);

test('version 3.0', async () => {
  const server = buildServer();

  const gbfsReader = (await buildGBFSReader(
    `http://localhost:${server.port}/readers/gbfs/fixtures/v3.0/gbfs.json`,
  )) as GBFSReaderV3;
  const {
    gbfs,
    version,
    gbfsVersions,
    systemInformation,
    stationInformation,
    stationStatus,
    vehicleStatus,
    systemAlerts,
    systemRegions,
    systemPricingPlans,
    geofencingZones,
    manifest,
  } = gbfsReader;

  expect(version).toEqual(3);

  // gbfs
  expect(gbfs).toBeDefined();
  const validGBFS = ajv.compile(gbfsSchemaV30);
  expect(validGBFS(gbfs)).toBeTrue();
  // versions
  expect(gbfsVersions).toBeDefined();
  const validGBFSVersions = ajv.compile(gbfsVersionsSchemaV30);
  expect(validGBFSVersions(gbfsVersions)).toBeTrue();
  // systemInformation
  expect(systemInformation).toBeDefined();
  const validSystemInformation = ajv.compile(gbfsSystemInformationSchemaV30);
  expect(validSystemInformation(systemInformation)).toBeTrue();
  // stationInformation
  expect(stationInformation).toBeDefined();
  const validStationInformation = ajv.compile(gbfsStationInformationSchemaV30);
  expect(validStationInformation(stationInformation)).toBeTrue();
  // stationStatus
  expect(stationStatus).toBeDefined();
  const validStationStatus = ajv.compile(gbfsStationStatusSchemaV30);
  expect(validStationStatus(stationStatus)).toBeTrue();
  // vehicleStatus
  expect(vehicleStatus).toBeDefined();
  const validVehicleStatus = ajv.compile(gbfsVehicleStatusSchemaV30);
  expect(validVehicleStatus(vehicleStatus)).toBeTrue();
  // systemAlerts
  expect(systemAlerts).toBeDefined();
  const validSystemAlerts = ajv.compile(gbfsSystemAlertsSchemaV30);
  expect(validSystemAlerts(systemAlerts)).toBeTrue();
  // systemRegions
  expect(systemRegions).toBeDefined();
  const validSystemRegions = ajv.compile(gbfsSystemRegionsSchemaV30);
  expect(validSystemRegions(systemRegions)).toBeTrue();
  // systemPricingPlans
  expect(systemPricingPlans).toBeDefined();
  const validSystemPricingPlans = ajv.compile(gbfsSystemPricingPlansSchemaV30);
  expect(validSystemPricingPlans(systemPricingPlans)).toBeTrue();
  // geofencingZones
  expect(geofencingZones).toBeDefined();
  const validGeofencingZones = ajv.compile(gbfsGeofencingZonesSchemaV30);
  expect(validGeofencingZones(geofencingZones)).toBeTrue();
  // manifest
  expect(manifest).toBeDefined();
  const validManifest = ajv.compile(gbfsManifestSchemaV30);
  expect(validManifest(manifest)).toBeTrue();

  await server.stop();

  const features = await Array.fromAsync(gbfsReader);
  expect(features.length).toBe(105);
});
