import { deg2rad, twoPi } from './constants';

/** Time object to track month-day-hour-minute-second */
export interface TimeStamp {
  mon: number;
  day: number;
  hr: number;
  minute: number;
  sec: number;
}

/**
 * -----------------------------------------------------------------------------
 *
 * procedure days2mdhms
 *
 * this procedure converts the day of the year, days, to the equivalent month
 * day, hour, minute and second.
 *
 * algorithm     : set up array for the number of days per month
 * find leap year - use 1900 because 2000 is a leap year
 * loop through a temp value while the value is < the days
 * perform int conversions to the correct day and month
 * convert remainder into h m s using type conversions
 *
 * author        : david vallado                  719-573-2600    1 mar 2001
 *
 * inputs          description                    range / units
 * year        - year                           1900 .. 2100
 * days        - julian day of the year         0.0  .. 366.0
 *
 * outputs       :
 * mon         - month                          1 .. 12
 * day         - day                            1 .. 28,29,30,31
 * hr          - hour                           0 .. 23
 * min         - minute                         0 .. 59
 * sec         - second                         0.0 .. 59.999
 *
 * locals        :
 * dayofyr     - day of year
 * temp        - temporary extended values
 * inttemp     - temporary int value
 * i           - index
 * lmonth[12]  - int array containing the number of days per month
 *
 * coupling      :
 * none.
 * @param year - year to date
 * @param days - day of year
 * @returns - Decomposed information into year, month, day, hour, minute and second
 */
export function days2mdhms(year: number, days: number): TimeStamp {
  const lmonth = [31, year % 4 === 0 ? 29 : 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
  const dayofyr = Math.floor(days);

  //  ----------------- find month and day of month ----------------
  let i = 1;
  let inttemp = 0;
  while (dayofyr > inttemp + lmonth[i - 1] && i < 12) {
    inttemp += lmonth[i - 1];
    i += 1;
  }

  const mon = i;
  const day = dayofyr - inttemp;

  //  ----------------- find hours minutes and seconds -------------
  let temp = (days - dayofyr) * 24.0;
  const hr = Math.floor(temp);
  temp = (temp - hr) * 60.0;
  const minute = Math.floor(temp);
  const sec = (temp - minute) * 60.0;

  return {
    mon,
    day,
    hr,
    minute,
    sec,
  };
}

/**
 *
 *  procedure jday
 *
 *  this procedure finds the julian date given the year, month, day, and time.
 *  the julian date is defined by each elapsed day since noon, jan 1, 4713 bc.
 *
 *  algorithm     : calculate the answer in one step for efficiency
 *
 *  author        : david vallado                  719-573-2600    1 mar 2001
 *
 *  inputs          description                    range / units
 *  year        - year                           1900 .. 2100
 *  mon         - month                          1 .. 12
 *  day         - day                            1 .. 28,29,30,31
 *  hr          - universal time hour            0 .. 23
 *  min         - universal time min             0 .. 59
 *  sec         - universal time sec             0.0 .. 59.999
 *
 *  outputs       :
 *  jd          - julian date                    days from 4713 bc
 *
 *  locals        :
 *  none.
 *
 *  coupling      :
 *  none.
 *
 *  references    :
 *  vallado       2007, 189, alg 14, ex 3-14
 * @param year - year
 * @param mon - month
 * @param day - day
 * @param hr - hour
 * @param minute - minute
 * @param sec - seconds
 * @param msec - milliseconds
 * @returns - Julian date
 */
function jdayInternal(
  year: number,
  mon: number,
  day: number,
  hr: number,
  minute: number,
  sec: number,
  msec = 0,
): number {
  return (
    367.0 * year -
    Math.floor(7 * (year + Math.floor((mon + 9) / 12.0)) * 0.25) +
    Math.floor((275 * mon) / 9.0) +
    day +
    1721013.5 +
    ((msec / 60000 + sec / 60.0 + minute) / 60.0 + hr) / 24.0 // ut in days
    // # - 0.5*sgn(100.0*year + mon - 190002.5) + 0.5;
  );
}

/**
 * Builds a julian date from the given parameters
 * @param year - year
 * @param mon - month
 * @param day - day
 * @param hr - hour
 * @param min - minute
 * @param sec - seconds
 * @param msec - milliseconds
 * @returns - Julian date
 */
export function jday(
  year: number | Date,
  mon = 0,
  day = 0,
  hr = 0,
  min = 0,
  sec = 0,
  msec = 0,
): number {
  if (year instanceof Date) {
    const date = year;
    return jdayInternal(
      date.getUTCFullYear(),
      date.getUTCMonth() + 1, // Note, this function requires months in range 1-12.
      date.getUTCDate(),
      date.getUTCHours(),
      date.getUTCMinutes(),
      date.getUTCSeconds(),
      date.getUTCMilliseconds(),
    );
  }

  return jdayInternal(year, mon, day, hr, min, sec, msec);
}

/**
 *                           procedure invjday
 *
 *  this procedure finds the year, month, day, hour, minute and second
 *  given the julian date. tu can be ut1, tdt, tdb, etc.
 *
 *  algorithm     : set up starting values
 *                  find leap year - use 1900 because 2000 is a leap year
 *                  find the elapsed days through the year in a loop
 *                  call routine to find each individual value
 *
 *  author        : david vallado                  719-573-2600    1 mar 2001
 *
 *  inputs          description                    range / units
 *    jd          - julian date                    days from 4713 bc
 *
 *  outputs       :
 *    year        - year                           1900 .. 2100
 *    mon         - month                          1 .. 12
 *    day         - day                            1 .. 28,29,30,31
 *    hr          - hour                           0 .. 23
 *    min         - minute                         0 .. 59
 *    sec         - second                         0.0 .. 59.999
 *
 *  locals        :
 *    days        - day of year plus fractional
 *                  portion of a day               days
 *    tu          - julian centuries from 0 h
 *                  jan 0, 1900
 *    temp        - temporary double values
 *    leapyrs     - number of leap years from 1900
 *
 *  coupling      :
 *    days2mdhms  - finds month, day, hour, minute and second given days and year
 *
 *  references    :
 *    vallado       2007, 208, alg 22, ex 3-13
 * @param jd - julian date
 * @param asArray - if true, return an array, otherwise return a Date
 * @returns - Date or [year, month, day, hour, minute, seconds]
 */
export function invjday(
  jd: number,
  asArray: boolean,
): Date | [number, number, number, number, number, number] {
  // --------------- find year and days of the year -
  const temp = jd - 2415019.5;
  const tu = temp / 365.25;
  let year = 1900 + Math.floor(tu);
  let leapyrs = Math.floor((year - 1901) * 0.25);

  // optional nudge by 8.64x10-7 sec to get even outputs
  let days = temp - ((year - 1900) * 365.0 + leapyrs) + 0.00000000001;

  // ------------ check for case of beginning of a year -----------
  if (days < 1.0) {
    year -= 1;
    leapyrs = Math.floor((year - 1901) * 0.25);
    days = temp - ((year - 1900) * 365.0 + leapyrs);
  }

  // ----------------- find remaing data  -------------------------
  const mdhms = days2mdhms(year, days);

  const { mon, day, hr, minute } = mdhms;

  const sec = mdhms.sec - 0.000000864;

  if (asArray) {
    return [year, mon, day, hr, minute, Math.floor(sec)];
  }

  return new Date(Date.UTC(year, mon - 1, day, hr, minute, Math.floor(sec)));
}

/**
 *                           function gstime
 *
 *  this function finds the greenwich sidereal time.
 *
 *  author        : david vallado                  719-573-2600    1 mar 2001
 *
 *  inputs          description                    range / units
 *    jdut1       - julian date in ut1             days from 4713 bc
 *
 *  outputs       :
 *    gstime      - greenwich sidereal time        0 to 2pi rad
 *
 *  locals        :
 *    temp        - temporary variable for doubles   rad
 *    tut1        - julian centuries from the
 *                  jan 1, 2000 12 h epoch (ut1)
 *
 *  coupling      :
 *    none
 *
 *  references    :
 *    vallado       2004, 191, eq 3-45
 * @param jdut1 - julian date
 * @returns - greenwich sidereal time
 */
function gstimeInternal(jdut1: number): number {
  const tut1 = (jdut1 - 2451545.0) / 36525.0;

  let temp =
    -6.2e-6 * tut1 * tut1 * tut1 +
    0.093104 * tut1 * tut1 +
    (876600.0 * 3600 + 8640184.812866) * tut1 +
    67310.54841; // # sec
  temp = ((temp * deg2rad) / 240.0) % twoPi; // 360/86400 = 1/240, to deg, to rad

  //  ------------------------ check quadrants ---------------------
  if (temp < 0.0) temp += twoPi;

  return temp;
}

/**
 * find greenwich sidereal time
 * @param time - julian date
 * @returns - greenwich sidereal time
 */
export function gstime(time: Date | number): number {
  if (time instanceof Date) return gstimeInternal(jday(time));
  return gstimeInternal(time);
}
