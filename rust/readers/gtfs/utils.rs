use crate::util::Date;

/// Convenience method to parse a GTFS date (YYYYMMDD) into a JavaScript Date.
/// Because GTFS dates do not contain timezone info, this function treats them as local dates.
///
/// ## Parameters
/// - `yyyymmdd`: A string in the format YYYYMMDD
///
/// ## Returns
/// A JavaScript Date object
pub fn parse_gtfs_date(yyyymmdd: &str) -> Option<Date> {
    let year = yyyymmdd[0..4].parse::<u16>().ok()?;
    let month = yyyymmdd[4..6].parse::<u8>().ok()? - 1; // zero-based
    let day = yyyymmdd[6..8].parse::<u8>().ok()?;

    Some(Date::new(year, month, day))
}
