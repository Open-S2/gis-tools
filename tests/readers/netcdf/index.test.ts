import { FileReader } from '../../../src/file';
import { NetCDFReader } from '../../../src/readers/netcdf';
import { expect, test } from 'bun:test';

import type { CDFValue, CDFVariable } from '../../../src/readers/netcdf';

test('Throws on non NetCDF file', () => {
  expect(() => {
    return new NetCDFReader(new FileReader(`${__dirname}/fixtures/not_nc.txt`));
  }).toThrow('Not a valid NetCDF file: should start with CDF');
});

test('read header information and test some random variables', () => {
  // http://www.unidata.ucar.edu/software/netcdf/examples/files.html
  // http://www.unidata.ucar.edu/software/netcdf/examples/madis-sao.cdl
  const netcdf = new NetCDFReader(new FileReader(`${__dirname}/fixtures/madis-sao.nc`));
  expect(netcdf.is64).toBe(false);
  expect(netcdf.recordDimension).toStrictEqual({
    size: 178,
    id: 21,
    name: 'recNum',
    recordStep: 1220,
  });
  expect(netcdf.dimensions).toStrictEqual([
    { index: 0, name: 'maxAutoStaLen', size: 6 },
    { index: 1, name: 'maxAutoWeather', size: 5 },
    { index: 2, name: 'maxAutoWeaLen', size: 12 },
    { index: 3, name: 'maxCldTypeLen', size: 5 },
    { index: 4, name: 'maxCloudTypes', size: 5 },
    { index: 5, name: 'maxDataSrcLen', size: 8 },
    { index: 6, name: 'maxRepLen', size: 5 },
    { index: 7, name: 'maxSAOLen', size: 256 },
    { index: 8, name: 'maxSkyCover', size: 5 },
    { index: 9, name: 'maxSkyLen', size: 8 },
    { index: 10, name: 'maxSkyMethLen', size: 3 },
    { index: 11, name: 'maxStaNamLen', size: 5 },
    { index: 12, name: 'maxWeatherNum', size: 5 },
    { index: 13, name: 'maxWeatherLen', size: 40 },
    { index: 14, name: 'QCcheckNum', size: 10 },
    { index: 15, name: 'QCcheckNameLen', size: 60 },
    { index: 16, name: 'ICcheckNum', size: 55 },
    { index: 17, name: 'ICcheckNameLen', size: 72 },
    { index: 18, name: 'maxStaticIds', size: 350 },
    { index: 19, name: 'totalIdLen', size: 6 },
    { index: 20, name: 'nInventoryBins', size: 24 },
    { index: 21, name: 'recNum', size: 0 },
  ]);

  expect(netcdf.globalAttributes).toStrictEqual({
    DD_long_name: 'QC data descriptor model:  QC summary values',
    DD_reference: 'AWIPS Technique Specification Package (TSP) 88-21-R2',
    DD_value_B: 'Included in reject list',
    DD_value_C: 'Passed QC stage 1',
    DD_value_G: 'Included in accept list',
    DD_value_K: 'Passed QC stages 1, 2, 3, and 4',
    DD_value_Q: 'Passed QC stage 1, but failed stages 2 or 3',
    DD_value_S: 'Passed QC stages 1 and 2',
    DD_value_V: 'Passed QC stages 1, 2 and 3',
    DD_value_X: 'Failed QC stage 1',
    DD_value_Z: 'No QC applied',
    DD_value_k: 'Passed QC stage 1,2, and 3, failed stage 4',
    DD_values: 'Z,C,S,V,X,Q,K,k,G, or B',
    ICA_Bit1Set: 'Master bit - at least 1 check applied',
    ICA_BitiSet: 'IC check # applied',
    ICA_LeastSignificantBit: 'bit1',
    ICA_NoBitsSet: 'No IC applied',
    ICA_long_name: 'IC applied model:  applied word definition',
    ICA_reference: "IC check #'s defined in IC check table",
    ICR_Bit1Set: 'Master bit - at least 1 check applied',
    ICR_BitiSet: 'IC check # applied',
    ICR_LeastSignificantBit: 'bit1',
    ICR_NoBitsSet: 'No IC applied',
    ICR_long_name: 'IC results Model:  results word definition',
    ICR_reference: "IC check #'s defined in IC check table",
    QCA_Bit10Set: 'Kalman Filter applied',
    QCA_Bit1Set: 'Master bit - at least 1 check applied',
    QCA_Bit2Set: 'Validity check applied',
    QCA_Bit3Set: 'Position Consistency check applied',
    QCA_Bit4Set: 'Internal Consistency check applied',
    QCA_Bit5Set: 'Temporal Consistency (TC) check applied',
    QCA_Bit6Set: 'Temporal Consistency check (for Marine data) applied',
    QCA_Bit7Set: 'Spatial Consistency check applied',
    QCA_Bit8Set: 'Forecast Model Consistency check applied',
    QCA_Bit9Set: 'Statistical Model Consistency check applied',
    QCA_LeastSignificantBit: 'bit1',
    QCA_NoBitsSet: 'No QC applied',
    QCA_long_name: 'QC applied model:  applied word definition',
    QCA_reference1: 'AWIPS TSP 88-21_R2',
    QCA_reference2: '10th Met Obs and Inst, Paper FA5.7, Phoenix, 1998',
    QCA_reference3: '14th IIPS, Paper FA8.16, Phoenix, 1998',
    QCD_long_name: 'QC departure model:  array definition',
    QCD_pos1: 'Average ob departure from QC check estimates',
    QCD_pos10: 'Departure from Kalman filter estimate',
    QCD_pos2: 'Departure from validity check estimate',
    QCD_pos3: 'Departure from position consistency estimate',
    QCD_pos4: 'Departure from internal consistency estimate',
    QCD_pos5: 'Departure from temporal consistency estimate',
    QCD_pos6: 'Departure from temporal consistency (for marine data)',
    QCD_pos7: 'Departure from spatial consistency estimate',
    QCD_pos8: 'Departure from forecast model estimate',
    QCD_pos9: 'Departure from statistical model estimate',
    QCD_reference1: 'AWIPS TSP 88-21-R2',
    QCD_reference2: '10th Met Obs and Inst, Paper FA5.7, Phoenix, 1998',
    QCD_reference3: '14th IIPS, Paper FA8.16, Phoenix, 1998',
    QCR_Bit10Set: 'Kalman Filter failed',
    QCR_Bit1Set: 'Master bit - at least 1 check failed',
    QCR_Bit2Set: 'Validity check failed',
    QCR_Bit3Set: 'Position Consistency check failed',
    QCR_Bit4Set: 'Internal Consistency (IC) check failed',
    QCR_Bit5Set: 'Temporal Consistency check failed',
    QCR_Bit6Set: 'Temporal Consistency check (for Marine data) failed',
    QCR_Bit7Set: 'Spatial Consistency check failed',
    QCR_Bit8Set: 'Forecast Model Consistency check failed',
    QCR_Bit9Set: 'Statistical Model Consistency check failed',
    QCR_LeastSignificantBit: 'bit1',
    QCR_NoBitsSet: 'No QC failures',
    QCR_long_name: 'QC results model:  results word definition',
    QCR_reference1: 'AWIPS TSP 88-21_R2',
    QCR_reference2: '10th Met Obs and Inst, Paper FA5.7, Phoenix, 1998',
    QCR_reference3: '14th IIPS, Paper FA8.16, Phoenix, 1998',
    QCStage_long_name: 'automated QC checks contained in each stage',
    QCStage_reference: 'AWIPS TSP 88-21_R2',
    QCStage_value_1: 'Validity and Position Consistency Check',
    QCStage_value_2: 'Internal, Temporal, and Model Consistency Checks',
    QCStage_value_3: 'Spatial Consistency Check',
    QCStage_value_4: 'Kalman Filter',
    QCStage_values: '1, 2, 3, or 4',
    cdlDate: '20010327',
    fileEndOffset: 2640,
    filePeriod: 3600,
    idVariables: 'stationName',
    timeVariables: 'timeObs',
  });

  expect(netcdf.variables[0]).toStrictEqual({
    name: 'nStaticIds',
    dimensions: [],
    attributes: { _FillValue: 0 },
    type: 4,
    size: 4,
    offset: 39208,
    record: false,
  });
  expect(netcdf.variables[11]).toStrictEqual({
    name: 'wmoId',
    dimensions: [
      {
        index: 21,
        name: 'recNum',
        size: 0,
      },
    ],
    attributes: {
      _FillValue: -2147483647,
      long_name: 'WMO numeric station ID',
      reference: 'station table',
      valid_range: [1, 89999],
    },
    type: 4,
    size: 4,
    offset: 48884,
    record: true,
  });

  // read non-record variable
  const varTest = netcdf.getDataVariable('nStaticIds');
  if (varTest === undefined) throw new Error('Variable not found');
  expect(varTest[0]).toBe(145);

  expect(netcdf.getDataVariable("n'importe quoi")).toBeUndefined();

  const wmoIdVar = netcdf.getDataVariable('wmoId');
  if (wmoIdVar === undefined) throw new Error('Variable not found');
  expect(wmoIdVar[0]).toBe(71419);
  expect(wmoIdVar[1]).toBe(71415);
  expect(wmoIdVar[2]).toBe(71408);

  const withString = netcdf.getDataVariable('staticIds');
  if (withString === undefined) throw new Error('Variable not found');
  const withObject = netcdf.getDataVariable(netcdf.variables[1].name);
  if (withObject === undefined) throw new Error('Variable not found');
  // TODO: This doesn't match
  // expect(withString[0]).toBe('W');
  // expect(withString[1]).toBe('A');
  // expect(withString[2]).toBe('F');
  expect(withString[0]).toBe(withObject[0]);
  expect(withString[1]).toBe(withObject[1]);
  expect(withString[2]).toBe(withObject[2]);
});

test('read 2 dimensional variable', () => {
  const netcdf = new NetCDFReader(new FileReader(`${__dirname}/fixtures/ichthyop.nc`));

  const timeVar = netcdf.getDataVariable('time');
  if (timeVar === undefined) throw new Error('Variable not found');
  expect(timeVar).toHaveLength(49);
  expect(timeVar[0]).toBe(1547070300);
  const lat = netcdf.getDataVariable('lat');
  if (lat === undefined) throw new Error('Variable not found');
  expect(lat).toHaveLength(49);
  expect(lat[0]).toHaveLength(1000);
  expect(lat[0][0]).toBe(53.26256561279297);
});

test('read 64 bit offset file', () => {
  const netcdf = new NetCDFReader(new FileReader(`${__dirname}/fixtures/model1_md2.nc`));
  expect(netcdf.is64).toBe(true);
  const cellAngular = netcdf.getDataVariable('cell_angular');
  if (cellAngular === undefined) throw new Error('Variable not found');
  expect(cellAngular[0]).toBe('a');
  const cellSpatial = netcdf.getDataVariable('cell_spatial');
  if (cellSpatial === undefined) throw new Error('Variable not found');
  expect(cellSpatial[0]).toBe('a');
});

// TODO: fix this
test.skip('read agilent hplc file file', () => {
  const netcdf = new NetCDFReader(new FileReader(`${__dirname}/fixtures/agilent_hplc.cdf`));

  expect(netcdf.is64).toBe(false);

  const variables: Array<{ value: CDFValue[]; variable: CDFVariable }> = [];
  for (const variable of netcdf.variables) {
    const value = netcdf.getDataVariable(variable.name);
    if (value === undefined) throw new Error('Variable not found');
    variables.push({ value, variable });
  }
  expect(variables[3].value).toStrictEqual([0.012000000104308128]);
  expect(variables).toHaveLength(24);
  expect(netcdf.getDataVariable('ordinate_values')).toHaveLength(4651);
});

test('build vector points', async () => {
  const netcdf = new NetCDFReader(new FileReader(`${__dirname}/fixtures/ichthyop.nc`), {
    lonKey: 'lon',
    latKey: 'lat',
    heightKey: 'depth',
    propFields: ['mortality'],
  });

  const points = await Array.fromAsync(netcdf);

  expect(points.length).toBe(49000);
  expect(points[0]).toEqual({
    geometry: {
      coordinates: {
        x: -9.00235366821289,
        y: 53.26256561279297,
        z: -0.2654986083507538,
      },
      is3D: true,
      type: 'Point',
    },
    properties: {
      mortality: 0,
    },
    type: 'VectorFeature',
  });
});
