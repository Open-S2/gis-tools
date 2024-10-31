import { invjday } from '../../src';
import { expect, test } from 'bun:test';

test('invjday', () => {
  expect(invjday(2000, true)).toEqual([-4707, 7, 6, 12, 0, 0]);
  const testDate = new Date(Date.UTC(2024, 6 - 1, 2, 4, 22));
  expect(invjday(toJulianDate(testDate), false)).toEqual(new Date('2024-06-01T12:00:00.000Z'));
  expect(invjday(toJulianDate(testDate), true)).toEqual([2024, 6, 1, 12, 0, 0]);
});

/**
 * @param date - default: new Date()
 * @returns - julian day
 */
function toJulianDate(date = new Date()) {
  // @ts-expect-error - i don't care its just for testing
  const julianDay = Math.floor((date - new Date(Date.UTC(2000, 0, 1, 12))) / 86400000) + 2451545;
  return julianDay;
}
