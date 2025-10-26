use crate::util::Date;
use core::f64::consts::TAU;
use libm::floor;

/// Time object to track month-day-hour-minute-second
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct TimeStamp {
    /// month
    pub mon: f64,
    /// day
    pub day: f64,
    /// hour
    pub hr: f64,
    /// minute
    pub min: f64,
    /// second
    pub sec: f64,
}

/// procedure days2mdhms
///
/// this procedure converts the day of the year, days, to the equivalent month
/// day, hour, minute and second.
///
/// algorithm     set up array for the number of days per month
/// find leap year - use 1900 because 2000 is a leap year
/// loop through a temp value while the value is < the days
/// perform int conversions to the correct day and month
/// convert remainder into h m s using type conversions
///
/// author        david vallado                  719-573-2600    1 mar 2001
///
/// ## Parameters
/// - `year`: year to date
/// - `days`: day of year
///
/// ## Returns
/// Decomposed information into year, month, day, hour, minute and second
pub fn days2mdhms(year: u16, days: f64) -> TimeStamp {
    let lmonth = [
        31.,
        if year.is_multiple_of(4) { 29. } else { 28. },
        31.,
        30.,
        31.,
        30.,
        31.,
        31.,
        30.,
        31.,
        30.,
        31.,
    ];
    let dayofyr = floor(days);

    //  ----------------- find month and day of month ----------------
    let mut i = 1;
    let mut inttemp = 0.;
    while dayofyr > (inttemp + lmonth[i - 1]) && i < 12 {
        inttemp += lmonth[i - 1];
        i += 1;
    }

    let mon = i;
    let day = dayofyr - inttemp;

    //  ----------------- find hours minutes and seconds -------------
    let mut temp = (days - dayofyr) * 24.0;
    let hr = floor(temp);
    temp = (temp - hr) * 60.0;
    let min = floor(temp);
    let sec = (temp - min) * 60.0;

    TimeStamp { mon: mon as f64, day, hr, min, sec }
}

/// procedure jday
///
/// this procedure finds the julian date given the year, month, day, and time.
/// the julian date is defined by each elapsed day since noon, jan 1, 4713 bc.
///
/// algorithm     calculate the answer in one step for efficiency
///
/// author         david vallado                  719-573-2600    1 mar 2001
///
/// ## Parameters
/// - `year`: year
/// - `mon`: month
/// - `day`: day
/// - `hr`: hour
/// - `min`: minute
/// - `sec`: seconds
/// - `msec`: milliseconds
///
/// ## Returns
/// Julian date
pub fn jday_internal(
    year: f64,
    mon: f64,
    day: f64,
    hr: f64,
    min: f64,
    sec: f64,
    msec: Option<f64>,
) -> f64 {
    let msec = msec.unwrap_or(0.);
    367.0 * year - floor(7. * (year + floor((mon + 9.) / 12.0)) * 0.25)
        + floor((275. * mon) / 9.0)
        + day
        + 1721013.5
        + ((msec / 60000. + sec / 60.0 + min) / 60.0 + hr) / 24.0
}

/// Builds a julian date from the given parameters
///
/// ## Parameters
/// - `date`: date
///
/// ## Returns
/// Julian date
pub fn jday(date: &Date) -> f64 {
    jday_internal(
        date.year as f64,
        date.month as f64,
        date.day as f64,
        date.hour as f64,
        date.minute as f64,
        date.second as f64,
        None,
    )
}

/// function gstime
///
/// this function finds the greenwich sidereal time.
///
/// author         david vallado                  719-573-2600    1 mar 2001
///
/// ## Parameters
/// - `jdut1`: julian date
///
/// ## Returns
/// Greenwich sidereal time
fn gstime_internal(jdut1: f64) -> f64 {
    let tut1 = (jdut1 - 2451545.0) / 36525.0;

    let mut temp = -6.2e-6 * tut1 * tut1 * tut1
        + 0.093104 * tut1 * tut1
        + (876600.0 * 3600.0 + 8640184.812866) * tut1
        + 67310.54841; // # sec
    temp = ((temp.to_radians()) / 240.0) % TAU; // 360/86400 = 1/240, to deg, to rad

    //  ------------------------ check quadrants ---------------------
    if temp < 0.0 {
        temp += TAU;
    }

    temp
}

/// find greenwich sidereal time
///
/// ## Parameters
/// - `time`: julian date
///
/// ## Returns
/// Greenwich sidereal time
pub fn gstime(time: &Date) -> f64 {
    gstime_internal(jday(time))
}
