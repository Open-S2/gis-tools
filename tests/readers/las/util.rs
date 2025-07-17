#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::readers::util::{
        IntoValue64, U32I32F32, U64I64F64, ValueType32, ValueType64, i8_clamp,
    };

    #[test]
    fn test_las_util_clamp() {
        assert_eq!(i8_clamp(-300), -128);
        assert_eq!(i8_clamp(300), 127);
        assert_eq!(i8_clamp(0), 0);
    }

    #[test]
    fn test_las_u64_i64_f64() {
        let mut test = U64I64F64::default();

        test.set_i64(22);
        assert_eq!(test.i64(), 22);

        test.set(33 as i64, ValueType64::I64);
        assert_eq!(test.i64(), 33);

        test.set_f64(44.44);
        assert_eq!(test.f64(), 44.44);

        test.set(55.55, ValueType64::F64);
        assert_eq!(test.f64(), 55.55);
    }

    #[test]
    fn u64_into_value64() {
        let val: u64 = 0xDEADBEEFCAFEBABE;
        assert_eq!(val.into_u64(), val);
        assert_eq!(val.into_i64(), val as i64);
        assert_eq!(val.into_f64(), val as f64);
    }

    #[test]
    fn i64_into_value64() {
        let val: i64 = -1234567890123456789;
        assert_eq!(val.into_u64(), val as u64);
        assert_eq!(val.into_i64(), val);
        assert_eq!(val.into_f64(), val as f64);
    }

    #[test]
    fn f64_into_value64() {
        let val: f64 = -98765.4321;
        let bits = val.to_bits();
        assert_eq!(val.into_u64(), bits);
        assert_eq!(val.into_i64(), bits as i64);
        assert_eq!(val.into_f64(), val);
    }

    #[test]
    fn round_trip_f64() {
        let val: f64 = std::f64::consts::PI;
        let bits = val.to_bits();
        assert_eq!(f64::from_bits(bits), val);
        assert_eq!(val.into_u64(), bits);
        assert_eq!(f64::from_bits(val.into_u64()), val);
    }

    #[test]
    fn edge_cases() {
        let zero_f = 0.0f64;
        let neg_zero_f = -0.0f64;
        assert_ne!(zero_f.to_bits(), neg_zero_f.to_bits());

        let nan = f64::NAN;
        assert!(nan.into_f64().is_nan());
        assert_ne!(nan.into_u64(), 0);
    }

    #[test]
    fn test_u32i32f32_from_value_types() {
        // U32 input
        let val_u32 = 123456789u32;
        let data = U32I32F32::new(val_u32, ValueType32::U32);
        assert_eq!(data.u32(), val_u32);
        assert_eq!(data.i32(), val_u32 as i32);
        assert_eq!(data.f32(), 1.6535997e-34);

        // I32 input
        let val_i32 = -6543210i32;
        let data = U32I32F32::new(val_i32, ValueType32::I32);
        assert_eq!(data.i32(), val_i32);
        assert_eq!(data.u32(), val_i32 as u32);
        assert!(data.f32().is_nan());

        // F32 input
        let val_f32 = 42.875f32;
        let data = U32I32F32::new(val_f32, ValueType32::F32);
        assert_eq!(data.f32(), val_f32);
        assert_eq!(data.u32(), val_f32.to_bits());
        assert_eq!(data.i32(), val_f32.to_bits() as i32);
    }

    #[test]
    fn test_u32i32f32_direct_setters_and_getters() {
        let mut data = U32I32F32::new(0u32, ValueType32::U32);

        data.set_u32(0xCAFEBABE);
        assert_eq!(data.u32(), 0xCAFEBABE);
        assert_eq!(data.i32(), i32::from_le_bytes(0xCAFEBABEu32.to_le_bytes()));
        assert_eq!(data.f32(), f32::from_le_bytes(0xCAFEBABEu32.to_le_bytes()));

        data.set_i32(-202020202);
        assert_eq!(data.i32(), -202020202);
        assert_eq!(data.u32(), (-202020202i32) as u32);
        assert_eq!(data.f32(), -3.8887773e31);

        let val_f32 = -19.625f32;
        data.set_f32(val_f32);
        assert_eq!(data.f32(), val_f32);
        assert_eq!(data.u32(), val_f32.to_bits());
        assert_eq!(data.i32(), val_f32.to_bits() as i32);
    }
}
