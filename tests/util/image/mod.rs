#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::parsers::RGBA;

    #[test]
    fn test_default() {
        let rgba = RGBA::default();
        assert_eq!(rgba.r, 0.);
        assert_eq!(rgba.g, 0.);
        assert_eq!(rgba.b, 0.);
        assert_eq!(rgba.a, 1.);

        let rgba: RGBA = Default::default();
        assert_eq!(rgba.r, 0.);
        assert_eq!(rgba.g, 0.);
        assert_eq!(rgba.b, 0.);
        assert_eq!(rgba.a, 1.);
    }

    #[test]
    fn test_rgba() {
        let rgba = RGBA::new(1., 2., 3., 4.);
        assert_eq!(rgba.r, 1.);
        assert_eq!(rgba.g, 2.);
        assert_eq!(rgba.b, 3.);
        assert_eq!(rgba.a, 4.);

        // from u8
        let rgba = RGBA::from_u8s(1, 2, 3, 4);
        let (r, g, b, a) = rgba.to_u8s();
        assert_eq!(r, 1);
        assert_eq!(g, 2);
        assert_eq!(b, 3);
        assert_eq!(a, 4);

        // from u16
        let rgba = RGBA::from_u16s(1, 2, 3, 4);
        let (r, g, b, a) = rgba.to_u16s();
        assert_eq!(r, 1);
        assert_eq!(g, 2);
        assert_eq!(b, 3);
        assert_eq!(a, 4);
    }

    #[test]
    fn test_rgba_to_from_u32() {
        let rgba = RGBA::from_u8s(1, 2, 3, 4);
        assert_eq!(rgba.to_u32(), 0x01020304);

        let rgba = RGBA::from_u32(0x01020304);
        let (r, g, b, a) = rgba.to_u8s();
        assert_eq!(r, 1);
        assert_eq!(g, 2);
        assert_eq!(b, 3);
        assert_eq!(a, 4);
    }

    #[test]
    fn test_rgba_to_from_u64() {
        let rgba = RGBA::from_u16s(1, 2, 3, 4);
        assert_eq!(rgba.to_u64(), 0x0001000200030004);

        let rgba = RGBA::from_u64(0x0001000200030004);
        let (r, g, b, a) = rgba.to_u16s();
        assert_eq!(r, 1);
        assert_eq!(g, 2);
        assert_eq!(b, 3);
        assert_eq!(a, 4);
    }

    #[test]
    fn test_rgba_add() {
        let rgba1 = RGBA::new(0.1, 0.2, 0.3, 0.4);
        let rgba2 = RGBA::new(0.1, 0.2, 0.3, 0.4);
        let rgba3 = rgba1 + rgba2;
        assert_eq!(rgba3.r, 0.2);
        assert_eq!(rgba3.g, 0.4);
        assert_eq!(rgba3.b, 0.6);
        assert_eq!(rgba3.a, 0.8);
    }

    #[test]
    fn test_rgba_sub() {
        let rgba1 = RGBA::new(0.1, 0.2, 0.3, 0.4);
        let rgba2 = RGBA::new(0.1, 0.2, 0.3, 0.4);
        let rgba3 = rgba1 - rgba2;
        assert_eq!(rgba3.r, 0.0);
        assert_eq!(rgba3.g, 0.0);
        assert_eq!(rgba3.b, 0.0);
        assert_eq!(rgba3.a, 0.0);
    }

    #[test]
    fn test_rgba_mul() {
        let rgba1 = RGBA::new(0.1, 0.2, 0.3, 0.4);
        let rgba2 = RGBA::new(0.1, 0.2, 0.3, 0.4);
        let rgba3 = rgba1 * rgba2;
        assert_eq!(rgba3.r, 0.010000000000000002);
        assert_eq!(rgba3.g, 0.04000000000000001);
        assert_eq!(rgba3.b, 0.09);
        assert_eq!(rgba3.a, 0.16000000000000003);
    }

    #[test]
    fn test_rgba_div() {
        let rgba1 = RGBA::new(0.1, 0.2, 0.3, 0.4);
        let rgba2 = RGBA::new(0.1, 0.2, 0.3, 0.4);
        let rgba3 = rgba1 / rgba2;
        assert_eq!(rgba3.r, 1.0);
        assert_eq!(rgba3.g, 1.0);
        assert_eq!(rgba3.b, 1.0);
        assert_eq!(rgba3.a, 1.0);
    }
}
