use alloc::{fmt, format, string::String};

/// Helper function to check if a year is a leap year
const fn is_leap_year(year: u16) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
}

/// Days in each month for non-leap and leap years
const DAYS_IN_MONTH: [[u16; 12]; 2] = [
    [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31], // Non-leap year
    [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31], // Leap year
];

/// # Date Structure
///
/// ## Description
/// Convenience Date structure to model like a Javascript Date object.
///
/// ## Usage
///
/// The methods you have access to:
/// - [`Date::new`]: Create a new Date
/// - [`Date::new_full`]: Creates a full Date
/// - [`Date::from_time`]: Create date given number of milliseconds since 1970-01-01T00:00:00Z (UTC)
/// - [`Date::set_time`]: Set the date fields from milliseconds since 1970-01-01T00:00:00Z
/// - [`Date::get_time`]: Returns the number of milliseconds since 1970-01-01T00:00:00Z (UTC)
/// - [`Date::to_iso_string`]: Returns a string representing the Date in ISO 8601 extended format.
///
/// ```rust
/// use gistools::util::Date;
///
/// let date = Date::new(2022, 1, 1);
/// assert_eq!(date.to_iso_string(), "2022-01-01T00:00:00.000Z");
/// ```
#[derive(Debug, PartialEq, Ord, PartialOrd, Eq, Clone, Default)]
pub struct Date {
    /// Year
    pub year: u16,
    /// Month
    pub month: u8,
    /// Day
    pub day: u8,
    /// Hour
    pub hour: u8,
    /// Minute
    pub minute: u8,
    /// Second
    pub second: u8,
}
impl Date {
    /// Creates a new Date
    pub fn new(year: u16, month: u8, day: u8) -> Date {
        Date { year, month, day, hour: 0, minute: 0, second: 0 }
    }

    /// Creates a full Date
    pub fn new_full(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Date {
        Date { year, month, day, hour, minute, second }
    }

    /// Create date given number of milliseconds since 1970-01-01T00:00:00Z (UTC)
    pub fn from_time(time: i64) -> Date {
        let mut date = Date::default();
        date.set_time(time);
        date
    }

    /// Set the date fields from milliseconds since 1970-01-01T00:00:00Z
    pub fn set_time(&mut self, time: i64) {
        // Break into days and remaining milliseconds
        let mut days = time / 86_400_000;
        let mut ms_remaining = time % 86_400_000;
        if ms_remaining < 0 {
            ms_remaining += 86_400_000;
            days -= 1;
        }

        // Determine year
        let mut year = 1970;
        loop {
            let year_days = if is_leap_year(year) { 366 } else { 365 };
            if days < year_days {
                break;
            }
            days -= year_days;
            year += 1;
        }

        // Determine month
        let leap = is_leap_year(year) as usize;
        let mut month = 0;
        while days >= DAYS_IN_MONTH[leap][month] as i64 {
            days -= DAYS_IN_MONTH[leap][month] as i64;
            month += 1;
        }

        self.year = year;
        self.month = (month + 1) as u8;
        self.day = (days + 1) as u8;

        // Convert remaining milliseconds to time of day
        self.hour = (ms_remaining / 3_600_000) as u8;
        self.minute = ((ms_remaining % 3_600_000) / 60_000) as u8;
        self.second = ((ms_remaining % 60_000) / 1_000) as u8;
    }

    /// Returns the number of milliseconds since 1970-01-01T00:00:00Z (UTC)
    pub fn get_time(&self) -> i64 {
        let mut days = 0;

        // Sum up days for all previous years
        for y in 1970..self.year {
            days += if is_leap_year(y) { 366 } else { 365 };
        }

        // Sum up days for previous months in the current year
        let leap = is_leap_year(self.year) as usize;
        for m in 0..(self.month as usize - 1) {
            days += DAYS_IN_MONTH[leap][m % 12] as i64;
        }

        // Add days of the current month
        days += self.day as i64 - 1;

        // Convert to milliseconds
        days * 86_400_000
            + (self.hour as i64 * 3_600_000)
            + (self.minute as i64 * 60_000)
            + (self.second as i64 * 1_000)
    }

    /// Returns a string representing the Date in ISO 8601 extended format.
    pub fn to_iso_string(&self) -> String {
        format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.000Z",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}
impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}{:02}{:02}", self.year, self.month + 1, self.day)
    }
}
