#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use core::f64::consts::TAU;
    use gistools::space::util::time::{days2mdhms, gstime, jday_internal};
    use gistools::util::Date;

    #[test]
    fn test_days2mdhms() {
        // Example: 2024 is a leap year
        let ts = days2mdhms(2024, 32.75); // 32nd day -> Feb 1, 18:00
        assert_eq!(ts.mon, 2.0);
        assert_eq!(ts.day, 1.0);
        assert_eq!(ts.hr, 18.0);
        assert_eq!(ts.min, 0.0);
        // assert_relative_eq!(ts.sec, 0.0, epsilon = 1e-10);
        assert_eq!(ts.sec, 0.0);
    }

    #[test]
    fn test_jday_internal() {
        // Expected from Vallado example: Jan 1, 2000, 12h UT = 2451545.0
        let jd = jday_internal(2000.0, 1.0, 1.0, 12.0, 0.0, 0.0, None);
        // assert_relative_eq!(jd, 2451545.0, epsilon = 1e-6);
        assert_eq!(jd, 2451545.0);
    }

    #[test]
    fn test_gstime() {
        // Using Vallado’s canonical example:
        // Jan 1, 2000, 12h UT1 → GST = 280.46061837° = 4.894961 rad
        let date = Date { year: 2000, month: 1, day: 1, hour: 12, minute: 0, second: 0 };
        let gst = gstime(&date);
        // assert_relative_eq!(gst, 4.894961, epsilon = 1e-5);
        assert_eq!(gst, 4.894961212823059);

        // Ensure GST wraps properly between 0 and 2π
        assert!(gst >= 0.0 && gst <= TAU);
    }
}
