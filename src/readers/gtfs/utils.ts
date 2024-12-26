/**
 * Convenience method to parse a GTFS date (YYYYMMDD) into a JavaScript Date.
 * Because GTFS dates do not contain timezone info, this function treats them as local dates.
 * @param yyyymmdd - A string in the format YYYYMMDD
 * @returns A JavaScript Date object
 */
export function parseGtfsDate(yyyymmdd: string): Date {
  const year = parseInt(yyyymmdd.slice(0, 4), 10);
  const month = parseInt(yyyymmdd.slice(4, 6), 10) - 1; // zero-based
  const day = parseInt(yyyymmdd.slice(6, 8), 10);

  return new Date(year, month, day);
}
