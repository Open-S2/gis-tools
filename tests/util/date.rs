#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::util::Date;

    #[test]
    fn test_date() {
        let date = Date::new_full(2020, 1, 1, 0, 0, 0);
        assert_eq!(date.get_time(), 1577836800000);

        let date = Date::new_full(2020, 2, 29, 12, 30, 45);
        assert_eq!(date.get_time(), 1582979445000);

        let date = Date::new_full(1970, 1, 1, 0, 0, 0);
        assert_eq!(date.get_time(), 0);

        let date = Date::new_full(1970, 1, 1, 1, 1, 1);
        assert_eq!(date.get_time(), 3661000);

        assert_eq!(date.to_iso_string(), "1970-01-01T01:01:01.000Z");
    }

    #[test]
    fn test_now() {
        #[cfg(feature = "std")]
        {
            // In std environment, now() should return a valid current date.
            // We can check that it's a realistic year (e.g., at or past 2026).
            let current_date = Date::now();
            assert!(
                current_date.year >= 2026,
                "Expected current year to be at least 2026, got {}",
                current_date.year
            );
            assert!(current_date.month >= 1 && current_date.month <= 12);
            assert!(current_date.day >= 1 && current_date.day <= 31);
            assert!(current_date.hour < 24);
            assert!(current_date.minute < 60);
            assert!(current_date.second < 60);
        }

        #[cfg(not(feature = "std"))]
        {
            // In no_std environment, now() safely defaults to Unix Epoch (1970-01-01)
            let static_date = Date::now();
            assert_eq!(static_date.year, 1970);
            assert_eq!(static_date.month, 1);
            assert_eq!(static_date.day, 1);
            assert_eq!(static_date.hour, 0);
            assert_eq!(static_date.minute, 0);
            assert_eq!(static_date.second, 0);
        }
    }

    #[test]
    fn test_from_str_milliseconds() {
        // Epoch (0 ms since 1970-01-01T00:00:00Z)
        let d = Date::from("0");
        assert_eq!(d.to_iso_string(), "1970-01-01T00:00:00.000Z");

        // 1 second after epoch
        let d = Date::from("1000");
        assert_eq!(d.to_iso_string(), "1970-01-01T00:00:01.000Z");

        // // Negative ms (before epoch)
        // let d = Date::from("-1000");
        // assert_eq!(d.to_iso_string(), "1969-12-31T23:59:59.000Z");
    }

    #[test]
    fn test_from_str_date_only() {
        let d = Date::from("2022-01-01");
        assert_eq!(d.year, 2022);
        assert_eq!(d.month, 1);
        assert_eq!(d.day, 1);
        assert_eq!(d.hour, 0);
        assert_eq!(d.minute, 0);
        assert_eq!(d.second, 0);
        assert_eq!(d.to_iso_string(), "2022-01-01T00:00:00.000Z");
    }

    #[test]
    fn test_from_str_full_datetime() {
        let d = Date::from("2023-12-31T23:59:59Z");
        assert_eq!(d.year, 2023);
        assert_eq!(d.month, 12);
        assert_eq!(d.day, 31);
        assert_eq!(d.hour, 23);
        assert_eq!(d.minute, 59);
        assert_eq!(d.second, 59);
        assert_eq!(d.to_iso_string(), "2023-12-31T23:59:59.000Z");
    }

    #[test]
    fn test_from_str_leap_year() {
        let d = Date::from("2020-02-29T12:34:56Z");
        assert_eq!(d.year, 2020);
        assert_eq!(d.month, 2);
        assert_eq!(d.day, 29);
        assert_eq!(d.hour, 12);
        assert_eq!(d.minute, 34);
        assert_eq!(d.second, 56);
    }
}
