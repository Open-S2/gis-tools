import Ajv from 'ajv';
import addFormats from 'ajv-formats';
import { buildGBFSReader } from '../../../src';
import { buildServer } from '../../server';
import { expect, test } from 'bun:test';

import {
  gbfsFreeBikeStatusSchemaV11,
  gbfsSchemaV11,
  gbfsStationInformationSchemaV11,
  gbfsStationStatusSchemaV11,
  gbfsSystemAlertsSchemaV11,
  gbfsSystemCalendarSchemaV11,
  gbfsSystemHoursSchemaV11,
  gbfsSystemInformationSchemaV11,
  gbfsSystemPricingPlansSchemaV11,
  gbfsSystemRegionsSchemaV11,
  gbfsVersionsSchemaV11,
} from '../../../src';

import type { GBFSReaderV1 } from '../../../src';

const testFunc = process.env.FAST_TESTS_ONLY !== undefined ? test.skip : test;

const ajv = new Ajv({ strict: false });
addFormats(ajv);

testFunc('version 1.1', async () => {
  const server = buildServer();

  const gbfsReader = (await buildGBFSReader(
    `http://localhost:${server.port}/readers/gbfs/fixtures/v1.1/gbfs.json`,
  )) as GBFSReaderV1;
  const {
    freeBikeStatus,
    gbfs,
    versions,
    stationInformation,
    stationStatus,
    systemAlerts,
    systemCalendar,
    systemHours,
    systemInformation,
    systemPricingPlans,
    systemRegions,
  } = gbfsReader;

  // freeBikeStatus
  expect(freeBikeStatus).toBeDefined();
  const validFreeBikeStatus = ajv.compile(gbfsFreeBikeStatusSchemaV11);
  expect(validFreeBikeStatus(freeBikeStatus)).toBeTrue();
  // gbfs
  expect(gbfs).toBeDefined();
  const validGBFS = ajv.compile(gbfsSchemaV11);
  expect(validGBFS(gbfs)).toBeTrue();
  // versions
  if (versions === undefined) throw new Error('versions is undefined');
  expect(versions).toBeDefined();
  // @ts-expect-error - version 2.2-google is not supported
  versions.data.versions = versions.data.versions.filter((v) => v.version !== '2.2-google');
  const validGBFSVersions = ajv.compile(gbfsVersionsSchemaV11);
  expect(validGBFSVersions(versions)).toBeTrue();
  // systemInformation
  expect(systemInformation).toBeDefined();
  const validSystemInformation = ajv.compile(gbfsSystemInformationSchemaV11);
  expect(validSystemInformation(systemInformation)).toBeTrue();
  // stationInformation
  expect(stationInformation).toBeDefined();
  const validStationInformation = ajv.compile(gbfsStationInformationSchemaV11);
  expect(validStationInformation(stationInformation)).toBeTrue();
  // stationStatus
  expect(stationStatus).toBeDefined();
  const validStationStatus = ajv.compile(gbfsStationStatusSchemaV11);
  expect(validStationStatus(stationStatus)).toBeTrue();
  // systemAlerts
  expect(systemAlerts).toBeDefined();
  const validSystemAlerts = ajv.compile(gbfsSystemAlertsSchemaV11);
  expect(validSystemAlerts(systemAlerts)).toBeTrue();
  // systemCalendar
  expect(systemCalendar).toBeDefined();
  const validSystemCalendar = ajv.compile(gbfsSystemCalendarSchemaV11);
  expect(validSystemCalendar(systemCalendar)).toBeTrue();
  // systemHours
  expect(systemHours).toBeDefined();
  const validSystemHours = ajv.compile(gbfsSystemHoursSchemaV11);
  expect(validSystemHours(systemHours)).toBeTrue();
  // systemPricingPlans
  expect(systemPricingPlans).toBeDefined();
  const validSystemPricingPlans = ajv.compile(gbfsSystemPricingPlansSchemaV11);
  expect(validSystemPricingPlans(systemPricingPlans)).toBeTrue();
  // systemRegions
  expect(systemRegions).toBeDefined();
  const validSystemRegions = ajv.compile(gbfsSystemRegionsSchemaV11);
  expect(validSystemRegions(systemRegions)).toBeTrue();

  await server.stop();

  // const features = await Array.fromAsync(gbfsReader);
  // expect(features.length).toBe(104);
});
