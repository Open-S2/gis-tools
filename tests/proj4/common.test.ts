import {
  acosh,
  adjustLat,
  adjustLon,
  adjustZone,
  asinh,
  asinhy,
  asinz,
  clens,
  clensCmplx,
  e0fn,
  e1fn,
  e2fn,
  e3fn,
  fL,
  gN,
  gatg,
  hypot,
  imlfn,
  invlatiso,
  iqsfnz,
  latiso,
  log1py,
  mlfn,
  msfnz,
  phi2z,
  pjEnfn,
  pjInvMlfn,
  pjMlfn,
  qsfnz,
  sign,
  sinh,
  srat,
  tanh,
  tsfnz,
} from '../../src/proj4/common';
import { expect, test } from 'bun:test';

test('acosh', () => {
  expect(acosh(1)).toEqual(0);
  expect(acosh(1.5)).toEqual(0.9624236501192069);
  expect(acosh(2)).toEqual(1.3169578969248166);
  expect(acosh(3)).toEqual(1.7627471740390859);
});

test('adjustLat', () => {
  expect(adjustLat(0)).toEqual(0);
  expect(adjustLat(-10)).toEqual(-6.858407346410207);
  expect(adjustLat(10)).toEqual(6.858407346410207);
});

test('adjustLon', () => {
  expect(adjustLon(0)).toEqual(0);
  expect(adjustLon(-10)).toEqual(-3.7168146928204138);
  expect(adjustLon(10)).toEqual(3.7168146928204138);
});

test('adjustZone', () => {
  expect(adjustZone(undefined, -10)).toEqual(0);
  expect(adjustZone(undefined, 10)).toEqual(60);
  expect(adjustZone(20, 10)).toEqual(20);
  expect(adjustZone(80, 10)).toEqual(60);
  expect(adjustZone(-20, 10)).toEqual(0);
});

test('asinh', () => {
  expect(asinh(0)).toEqual(0);
  expect(asinh(1)).toEqual(0.8813735870195429);
  expect(asinh(-1)).toEqual(-0.8813735870195429);
});

test('asinhy', () => {
  expect(asinhy(0)).toEqual(0);
  expect(asinhy(1)).toEqual(0.881373587019543);
  expect(asinhy(-1)).toEqual(-0.881373587019543);
});

test('asinz', () => {
  expect(asinz(0)).toEqual(0);
  expect(asinz(1)).toEqual(1.5707963267948966);
  expect(asinz(-1)).toEqual(-1.5707963267948966);
});

test('clens', () => {
  expect(clens([0, 1, 2], 5)).toEqual(0.7565545694248642);
  expect(clens([-1, 0, 1], 22)).toEqual(-0.017699844733562918);
  expect(clens([2, 3, 4], 4)).toEqual(-0.6918219227474515);
});

test('clensCmplx', () => {
  expect(clensCmplx([0, 1, 2], 5, 22)).toEqual([2.9959974452904563e28, -3.500023998925367e28]);
  expect(clensCmplx([-1, 0, 1], 22, 4)).toEqual([-2160.422054018843, -81321.41774294688]);
  expect(clensCmplx([2, 3, 4], 4, 5)).toEqual([-3475556.607197254, 5512242.219673926]);
});

test('e0fn', () => {
  expect(e0fn(0)).toEqual(1);
  expect(e0fn(1)).toEqual(0.68359375);
  expect(e0fn(2)).toEqual(0.15625);
});

test('e1fn', () => {
  expect(e1fn(0)).toEqual(0);
  expect(e1fn(1)).toEqual(0.5126953125);
  expect(e1fn(2)).toEqual(1.4765625);
});

test('e2fn', () => {
  expect(e2fn(0)).toEqual(0);
  expect(e2fn(1)).toEqual(0.1025390625);
  expect(e2fn(2)).toEqual(0.5859375);
});

test('e3fn', () => {
  expect(e3fn(0)).toEqual(0);
  expect(e3fn(1)).toEqual(0.011393229166666666);
  expect(e3fn(2)).toEqual(0.09114583333333333);
});

test('fL', () => {
  expect(fL(0, -2)).toEqual(-1.5707963267948966);
  expect(fL(1, 2)).toEqual(1.3017603360460153);
  expect(fL(2, -10)).toEqual(-1.5706147270763458);
});

test('gN', () => {
  expect(gN(0, 1, 2)).toEqual(NaN);
  expect(gN(-1, 0, 1)).toEqual(-1);
  expect(gN(22, 0.1, 1)).toEqual(22.110831935702667);
});

test('gatg', () => {
  expect(gatg([0, 1, 2], 5)).toEqual(3.9368820025419042);
  expect(gatg([-1, 0, 1], 22)).toEqual(22.035381662040646);
  expect(gatg([2, 3, 4], 4)).toEqual(1.4926930952250719);
});

test('hypot', () => {
  expect(hypot(0, 0)).toEqual(0);
  expect(hypot(0, 1)).toEqual(1);
  expect(hypot(1, 0)).toEqual(1);
  expect(hypot(1, 1)).toEqual(Math.SQRT2);
});

test('imlfn', () => {
  expect(imlfn(0, 1, 2, 3, 4)).toEqual(0);
  expect(() => {
    imlfn(1, 0, 1, 2, 3);
  }).toThrowError('IMLFN-CONV:Latitude failed to converge after 15 iterations');
});

test('invlatiso', () => {
  expect(invlatiso(0, 1)).toEqual(0.8657694832396587);
  expect(invlatiso(1, 0)).toEqual(0);
  expect(invlatiso(1, 1)).toEqual(1.5707963267948966);
});

test('iqsfnz', () => {
  expect(() => iqsfnz(0, 1)).toThrowError(
    'IQSFN-CONV:Latitude failed to converge after 30 iterations',
  );
  expect(iqsfnz(0.0001, 0.01)).toEqual(0.005000020883607367);
  // expect(iqsfnz(22, -0.0000000001)).toEqual(-9.921875000000001e-9);
});

test('latiso', () => {
  expect(latiso(0.001, 1, Math.sin(1))).toEqual(1.2261903294123335);
});

test('log1py', () => {
  expect(log1py(0)).toEqual(0);
  expect(log1py(1)).toEqual(0.6931471805599453);
});

test('mlfn', () => {
  expect(mlfn(0, 1, 2, 3, 4)).toEqual(1.151570206066359);
});

test('msfnz', () => {
  expect(msfnz(0.0001, Math.sin(0.01), Math.cos(0.01))).toEqual(0.9999500004171653);
});

test('phi2z', () => {
  expect(phi2z(0, 0)).toEqual(1.5707963267948966);
  expect(phi2z(1, 0.00000000000001)).toEqual(1.5707963267948966);
  expect(phi2z(0.00000000000001, 1)).toEqual(0);
  expect(() => phi2z(0.00000000000001, NaN)).toThrowError('phi2z has NoConvergence');
});

test('pjEnfn', () => {
  expect(pjEnfn(0)).toEqual([1, 0, 0, 0, 0]);
  expect(pjEnfn(0.001)).toEqual([
    0.9997499531054581, 0.0007499531054580689, 4.687369720458984e-7, 3.6457763671875e-10,
    3.0761718750000005e-13,
  ]);
});

test('pjInvMlfn', () => {
  expect(pjInvMlfn(22, 0.001, [1, 0, 0, 0, 0])).toEqual(22);
  expect(() =>
    pjInvMlfn(
      22,
      40,
      [
        0.9997499531054581, 0.0007499531054580689, 4.687369720458984e-7, 3.6457763671875e-10,
        3.0761718750000005e-13,
      ],
    ),
  ).toThrowError('cass:pj_inv_mlfn: Convergence error');
});

test('pjMlfn', () => {
  expect(pjMlfn(22, 0, 1, [1, 0, 0, 0, 0])).toEqual(22);
});

test('qsfnz', () => {
  expect(qsfnz(0, 0)).toEqual(0);
  expect(qsfnz(1, 0)).toEqual(0);
});

test('sign', () => {
  expect(sign(0)).toEqual(1);
  expect(sign(1)).toEqual(1);
  expect(sign(-1)).toEqual(-1);
});

test('sinh', () => {
  expect(sinh(0)).toEqual(0);
  expect(sinh(1)).toEqual(1.1752011936438014);
  expect(sinh(-1)).toEqual(-1.1752011936438014);
});

test('srat', () => {
  expect(srat(0, 0.1)).toEqual(1);
  expect(srat(1, 0.1)).toEqual(0);
});

test('tanh', () => {
  expect(tanh(0)).toEqual(0);
  expect(tanh(1)).toEqual(0.7615941559557649);
  expect(tanh(-1)).toEqual(-0.7615941559557649);
});

test('tsfnz', () => {
  expect(tsfnz(0, 0.01234, Math.sin(0.01234))).toEqual(0.9877355162326474);
  expect(tsfnz(1, 0.01234, Math.sin(0.01234))).toEqual(0.9999999999999999);
});
