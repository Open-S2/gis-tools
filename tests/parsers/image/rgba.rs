#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::parsers::{RGBA, gamma_to_linear, linear_to_gamma};
    use s2json::{GetM, MValue, ValueType};

    #[test]
    fn gammas() {
        // gamma -> lin
        assert_eq!(gamma_to_linear(0.), 0.);
        assert_eq!(gamma_to_linear(55.49759841012791), 0.5);

        // lin -> gamma
        assert_eq!(linear_to_gamma(0.), 0.);
        assert_eq!(linear_to_gamma(0.5), 55.49759841012791);
    }

    #[test]
    fn test_from_hex() {
        // rgb
        let c = RGBA::from_hex("#FF0000");
        assert_eq!(c, RGBA::from_u8s(255, 0, 0, 255));
        let c = RGBA::from_hex("00FF00");
        assert_eq!(c, RGBA::from_u8s(0, 255, 0, 255));
        let c = RGBA::from_hex("#0000FF");
        assert_eq!(c, RGBA::from_u8s(0, 0, 255, 255));

        // rgba
        let c = RGBA::from_hex("#FF000080");
        assert_eq!(c, RGBA::from_u8s(255, 0, 0, 128));
        let c = RGBA::from_hex("00FF0080");
        assert_eq!(c, RGBA::from_u8s(0, 255, 0, 128));
        let c = RGBA::from_hex("0000FF00");
        assert_eq!(c, RGBA::from_u8s(0, 0, 255, 0));
    }

    #[test]
    #[should_panic(expected = "Invalid hex color")]
    fn test_from_hex_invalid_length_short() {
        RGBA::from_hex("#123");
    }

    #[test]
    #[should_panic(expected = "Invalid hex color")]
    fn test_from_hex_invalid_length_long() {
        RGBA::from_hex("#1234567890");
    }

    #[test]
    #[should_panic]
    fn test_from_hex_invalid_chars() {
        RGBA::from_hex("#GGHHII");
    }

    #[test]
    fn test_mul_two_rgba() {
        let mut c1 = RGBA::from_u8s(255, 0, 0, 255);
        let c2 = RGBA::from_u8s(0, 255, 0, 255);
        assert_eq!(c1 * c2, RGBA::from_u8s(0, 0, 0, 255));
        c1 *= c2;
        assert_eq!(c1, RGBA::from_u8s(0, 0, 0, 255));
    }

    #[test]
    fn test_rgba_to_mvalue() {
        let c1 = RGBA::from_u8s(255, 0, 0, 255);
        let mvalue: MValue = c1.into();
        assert_eq!(mvalue.get("r").unwrap().to_prim().unwrap().to_u64().unwrap(), 255);
        assert_eq!(mvalue.get("g").unwrap().to_prim().unwrap().to_u64().unwrap(), 0);
        assert_eq!(mvalue.get("b").unwrap().to_prim().unwrap().to_u64().unwrap(), 0);
        assert_eq!(mvalue.get("a").unwrap().to_prim().unwrap().to_u64().unwrap(), 255);

        let m = c1.m();
        assert_eq!(m, Some(&c1));

        let c2: RGBA = mvalue.clone().into();
        assert_eq!(c2.to_u8s(), (255, 0, 0, 255));

        let value: ValueType = c1.into();
        let value_rgba: RGBA = (&value).into();
        assert_eq!(value_rgba.to_u8s(), (255, 0, 0, 255));
    }
}
