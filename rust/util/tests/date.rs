#[cfg(test)]
// #[coverage(off)]
mod tests {
    use util::Date;

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
}
