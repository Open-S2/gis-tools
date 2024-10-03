import {
  compare,
  fromFaceST,
  fromLonLat,
  fromS2Point,
  toCell,
} from '../../src/dataStructures/uint64';
import { describe, expect, it, test } from 'bun:test';

test('compare', () => {
  expect(compare({ low: 0, high: 0 }, { low: 0, high: 0 })).toEqual(0);
  expect(compare({ low: 0, high: 0 }, { low: 0, high: 1 })).toEqual(-1);
  expect(compare({ low: 0, high: 0 }, { low: 1, high: 0 })).toEqual(-1);
  expect(compare({ low: 0, high: 0 }, { low: 1, high: 1 })).toEqual(-1);
  expect(compare({ low: 0, high: 1 }, { low: 0, high: 0 })).toEqual(1);
  expect(compare({ low: 0, high: 1 }, { low: 0, high: 1 })).toEqual(0);
  expect(compare({ low: 0, high: 1 }, { low: 1, high: 0 })).toEqual(1);
  expect(compare({ low: 0, high: 1 }, { low: 1, high: 1 })).toEqual(-1);
  expect(compare({ low: 1, high: 0 }, { low: 0, high: 0 })).toEqual(1);
  expect(compare({ low: 1, high: 0 }, { low: 1, high: 0 })).toEqual(0);
});

test('fromFaceST', () => {
  expect(fromFaceST(0, 0, 0)).toEqual({ low: 1, high: 0 });
  expect(fromFaceST(1, 0.5, 0.5)).toEqual({ low: 1, high: 805306368 });
  expect(fromFaceST(2, 1, 1)).toEqual({ low: 1431655765, high: 1431655765 });
});

test('fromLonLat', () => {
  expect(fromLonLat(0, 0)).toEqual({ low: 1, high: 268435456 });
  expect(fromLonLat(90, 0)).toEqual({ low: 1, high: 805306368 });
  expect(fromLonLat(-90, 45)).toEqual({ low: 1, high: 2281701376 });
});

test('fromS2Point', () => {
  expect(fromS2Point([0, 1, 0])).toEqual({ low: 1, high: 805306368 });
  expect(fromS2Point([1, 0, 0])).toEqual({ low: 1, high: 268435456 });
  expect(fromS2Point([0, 0, 1])).toEqual({ low: 1, high: 1342177280 });
});

describe('toCell', () => {
  it('number inputs', () => {
    expect(toCell(0)).toEqual({ low: 0, high: 0 });
    expect(toCell(1)).toEqual({ low: 1, high: 0 });
    expect(toCell(1234567890)).toEqual({ low: 1234567890, high: 0 });
    expect(toCell(12345678900000)).toEqual({ low: 1942891296, high: 2874 });
  });
  it('bigint inputs', () => {
    expect(toCell(0n)).toEqual({ low: 0, high: 0 });
    expect(toCell(1n)).toEqual({ low: 1, high: 0 });
    expect(toCell(1234567890n)).toEqual({ low: 1234567890, high: 0 });
    expect(toCell(12345678900000n)).toEqual({ low: 1942891296, high: 2874 });
  });
  it('Uint64Cell inputs return themselves', () => {
    const cell = { low: 1234, high: 5678 };
    expect(toCell(cell)).toEqual(cell);
  });
});
