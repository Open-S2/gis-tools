import { MMapReader } from '../../src/mmap';
import { Transformer, injectAllDefinitions, injectAllEPSGCodes } from '../../src/proj4';
import { describe, expect, it, test } from 'bun:test';

describe('nagrid BETA2007.gsb', (): void => {
  it('EPSG_31466 -> EPSG_25832', (): void => {
    const transform = new Transformer();
    injectAllDefinitions(transform);
    injectAllEPSGCodes(transform);
    transform.addGridFromReader(
      'BETA2007.gsb',
      new MMapReader(`${__dirname}/fixtures/BETA2007.gsb`),
    );
    transform.setSource('EPSG_31466');
    transform.setDestination('EPSG_25832');
    const forward = transform.forward({ x: 2559552, y: 5670982 });
    expect(forward.x).toBeCloseTo(349757.381712518, 0.01);
    expect(forward.y).toBeCloseTo(5671004.06504954, 0.01);
  });

  it('EPSG_31466 -> EPSG_4326', (): void => {
    const transform = new Transformer();
    injectAllDefinitions(transform);
    injectAllEPSGCodes(transform);
    transform.addGridFromReader(
      'BETA2007.gsb',
      new MMapReader(`${__dirname}/fixtures/BETA2007.gsb`),
    );
    transform.setSource('EPSG_31466');
    transform.setDestination('EPSG_4326');
    const forward = transform.forward({ x: 2559552, y: 5670982 });
    expect(forward.x).toBeCloseTo(6.850861772, 0.0000001);
    expect(forward.y).toBeCloseTo(51.170707759, 0.01);
  });

  it('EPSG_31466 -> EPSG_3857', (): void => {
    const transform = new Transformer();
    injectAllDefinitions(transform);
    injectAllEPSGCodes(transform);
    transform.addGridFromReader(
      'BETA2007.gsb',
      new MMapReader(`${__dirname}/fixtures/BETA2007.gsb`),
    );
    transform.setSource('EPSG_31466');
    transform.setDestination('EPSG_3857');
    const forward = transform.forward({ x: 2559552, y: 5670982 });
    expect(forward.x).toBeCloseTo(762634.443931574, 0.01);
    expect(forward.y).toBeCloseTo(6651545.68026527, 0.01);
  });

  it('EPSG:31466 -> EPSG:3857', (): void => {
    const transform = new Transformer();
    injectAllDefinitions(transform);
    injectAllEPSGCodes(transform);
    transform.addGridFromReader(
      'BETA2007.gsb',
      new MMapReader(`${__dirname}/fixtures/BETA2007.gsb`),
    );
    transform.setSource('EPSG:31466');
    transform.setDestination('EPSG:3857');
    const forward = transform.forward({ x: 2559552, y: 5670982 });
    expect(forward.x).toBeCloseTo(762634.443931574, 0.01);
    expect(forward.y).toBeCloseTo(6651545.68026527, 0.01);
  });
});

test('ntv2', (): void => {
  const transform = new Transformer();
  injectAllDefinitions(transform);
  injectAllEPSGCodes(transform);
  transform.addGridFromReader(
    'ntv2',
    new MMapReader(`${__dirname}/fixtures/ntv2_0_downsampled.gsb`),
  );

  transform.setSource('+proj=longlat +ellps=clrk66 +nadgrids=@ignorable,ntv2,null');
  transform.setDestination('+proj=longlat +datum=WGS84 +no_defs');

  const forwardTests = [
    [-44.382211538462, 40.3768, -44.380749, 40.377457], // just inside the lower limit
    [-87.617788, 59.623262, -87.617659, 59.623441], // just inside the upper limit
    [-44.5, 40.5, -44.498553, 40.500632], // inside the first square
    [-60, 50, -59.999192, 50.000058], // a general point towards the middle of the grid
    [0, 0, 0, 0], // fall back to null
  ];

  for (const [fromLng, fromLat, toLng, toLat] of forwardTests) {
    const forward = transform.forward({ x: fromLng, y: fromLat });
    expect(forward.x).toBeCloseTo(toLng, 0.000001);
    expect(forward.y).toBeCloseTo(toLat, 0.000001);
  }

  const inverseTests = [
    [-44.5, 40.5, -44.498553, 40.500632],
    [-60, 50, -59.999192, 50.000058],
  ];

  for (const [fromLng, fromLat, toLng, toLat] of inverseTests) {
    const inverse = transform.inverse({ x: fromLng, y: fromLat });
    expect(inverse.x).toBeCloseTo(toLng, 0.000001);
    expect(inverse.y).toBeCloseTo(toLat, 0.000001);
  }
});
