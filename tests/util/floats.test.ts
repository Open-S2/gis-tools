import { describe, expect, it } from 'bun:test';
import { nextDown, nextUp } from '../../src/index.js';

describe('nextUp', () => {
  it('should return the next float', () => {
    expect(nextUp(1)).toEqual(1.0000000000000002);
  });

  it('handles NaN', () => {
    expect(nextUp(NaN)).toEqual(NaN);
  });

  it('infinity', () => {
    expect(nextUp(Infinity)).toEqual(Infinity);
    expect(nextUp(-Infinity)).toEqual(-Infinity);
  });

  it('0 case', () => {
    expect(nextUp(0)).toEqual(Number.MIN_VALUE);
  });
});

describe('nextDown', () => {
  it('should return the next float', () => {
    expect(nextDown(1.0000000000000002)).toEqual(1);
  });

  it('handles NaN', () => {
    expect(nextDown(NaN)).toEqual(NaN);
  });

  it('infinity', () => {
    expect(nextDown(Infinity)).toEqual(Infinity);
    expect(nextDown(-Infinity)).toEqual(-Infinity);
  });

  it('0 case', () => {
    expect(nextDown(0)).toEqual(-Number.MIN_VALUE);
  });
});
