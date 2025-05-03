#[cfg(test)]
// #[coverage(off)]
mod tests {
    use geometry::S2CellId;

    #[test]
    #[allow(clippy::useless_conversion)]
    fn test_u64_u64() {
        let val: u64 = 12345;
        let converted: u64 = val.into();
        assert_eq!(converted, val);
    }

    #[test]
    fn test_u64_s2cellid() {
        let val: u64 = 12345;
        let cell_id: S2CellId = val.into();
        let converted: u64 = cell_id.into();
        assert_eq!(converted, val);
    }
}
