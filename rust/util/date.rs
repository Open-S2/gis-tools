use alloc::{format, string::String};

/// Helper function to check if a year is a leap year
const fn is_leap_year(year: u16) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Days in each month for non-leap and leap years
const DAYS_IN_MONTH: [[u16; 12]; 2] = [
    [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31], // Non-leap year
    [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31], // Leap year
];

/// Convenience Date structure to model like a Javascript Date object.
#[derive(Debug, PartialEq, Ord, PartialOrd, Eq, Clone, Default)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
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
            days += DAYS_IN_MONTH[leap][m] as i64;
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
