#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    extern crate alloc;
    extern crate std;

    use alloc::{vec, vec::Vec};
    use gistools::parsers::{Buffer, BufferReader, Reader};
    use std::{fs, path::PathBuf};

    #[test]
    fn test_buffer() {
        // new
        let buf = Buffer::default();
        let vec1: Vec<u8> = vec![];
        assert_eq!(vec1, *buf.buf());

        // from
        let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let buf2: Buffer = Buffer::new(vec.clone());
        assert_eq!(vec, *buf2.buf());
    }

    #[test]
    fn test_set_pos() {
        let mut buf = Buffer::default();
        assert_eq!(0, buf.pos());
        buf.set_pos(1);
        assert_eq!(1, buf.pos());
    }

    // len
    #[test]
    fn test_len() {
        let mut buf = Buffer::default();
        assert_eq!(0, buf.len());
        buf.set_u8(1);
        assert_eq!(1, buf.len());
    }

    // is_empty
    #[test]
    fn test_is_empty() {
        let mut buf = Buffer::default();
        assert!(buf.is_empty());
        buf.set_u8(1);
        assert!(!buf.is_empty());
    }

    // get_u8, get_u8_at & set_u8
    #[test]
    fn test_get_u8() {
        let mut buf = Buffer::default();
        buf.set_u8(1);
        buf.set_pos(0);
        assert_eq!(1, buf.get_u8());
        assert_eq!(1, buf.get_u8_at(0));
    }

    // get_i8, get_i8_at & set_i8
    #[test]
    fn test_get_i8() {
        let mut buf = Buffer::default();
        buf.set_i8(1);
        buf.set_i8_at(2, 2);
        buf.set_pos(0);
        assert_eq!(1, buf.get_i8());
        assert_eq!(1, buf.get_i8_at(0));
    }

    // get_u16, get_u16_at & set_u16
    #[test]
    fn test_get_u16() {
        let mut buf = Buffer::default();
        buf.set_u16(1);
        buf.set_pos(0);
        assert_eq!(1, buf.get_u16());
        assert_eq!(1, buf.get_u16_at(0));
    }

    // get_i16, get_i16_at & set_i16
    #[test]
    fn test_get_i16() {
        let mut buf = Buffer::default();
        buf.set_i16(1);
        buf.set_i16_at(4, 4);
        buf.set_pos(0);
        assert_eq!(1, buf.get_i16());
        assert_eq!(1, buf.get_i16_at(0));
    }

    // get_i32, get_i32_at & set_i32
    #[test]
    fn test_get_i32() {
        let mut buf = Buffer::default();
        buf.set_i32(1);
        buf.set_pos(0);
        assert_eq!(1, buf.get_i32());
        assert_eq!(1, buf.get_i32_at(0));
    }

    // get_u32, get_u32_at & set_u32
    #[test]
    fn test_get_u32() {
        let mut buf = Buffer::default();
        buf.set_u32(1);
        buf.set_pos(0);
        assert_eq!(1, buf.get_u32());
        assert_eq!(1, buf.get_u32_at(0));
    }

    // get_f32, get_f32_at & set_f32
    #[test]
    fn test_get_f32() {
        let mut buf = Buffer::default();
        buf.set_f32(1.0);
        buf.set_pos(0);
        assert_eq!(1.0, buf.get_f32());
        assert_eq!(1.0, buf.get_f32_at(0));
    }

    // get_i64, get_i64_at & set_i64
    #[test]
    fn test_get_i64() {
        let mut buf = Buffer::default();
        buf.set_i64(1);
        buf.set_pos(0);
        assert_eq!(1, buf.get_i64());
        assert_eq!(1, buf.get_i64_at(0));
    }

    // get_u64, get_u64_at & set_u64
    #[test]
    fn test_get_u64() {
        let mut buf = Buffer::default();
        buf.set_u64(1);
        buf.set_pos(0);
        assert_eq!(1, buf.get_u64());
        assert_eq!(1, buf.get_u64_at(0));
    }

    // get_f64, get_f64_at & set_f64
    #[test]
    fn test_get_f64() {
        let mut buf = Buffer::default();
        buf.set_f64(1.0);
        buf.set_f64_at(12, 12.0);
        buf.set_pos(0);
        assert_eq!(1.0, buf.get_f64());
        assert_eq!(1.0, buf.get_f64_at(0));
    }

    // decode_varint, read_varint, & write_varint
    #[test]
    fn test_decode_varint() {
        let mut buf = Buffer::default();
        buf.write_varint(1_u16);
        buf.write_varint(19393930202_u64);
        buf.set_pos(0);
        assert_eq!(1, buf.read_varint::<u16>());
        assert_eq!(19393930202, buf.read_varint::<u64>());
        buf.set_pos(0);
        assert_eq!(1, buf.decode_varint());
        assert_eq!(19393930202, buf.decode_varint());
    }

    #[test]
    fn test_copy_from_slice() {
        let bytes = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut buf = Buffer::default();
        buf.copy_from_slice(0, &bytes);
        assert_eq!(bytes, buf.take());
        buf.copy_from_slice(6, &bytes);
        assert_eq!(bytes, buf.take()[6..]);
    }

    // take
    #[test]
    fn test_take() {
        let mut buf = Buffer::default();
        buf.set_u8(1);
        buf.set_u8(2);
        buf.set_u8(3);
        assert_eq!(vec![1, 2, 3], buf.take());
    }

    #[test]
    fn test_buffer_reader() {
        let buffer = b"Hello, world!";
        let reader = BufferReader::from(buffer);
        assert_eq!(reader.parse_string(None, None), "Hello, world!");

        let vec_buff = Vec::<u8>::from(buffer);
        let reader = BufferReader::from(vec_buff);
        assert_eq!(reader.parse_string(None, None), "Hello, world!");
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn test_default_functions() {
        // get expected
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/fixtures/dv.bin");
        let raw_data: Vec<u8> = fs::read(&path).expect("Failed to read file expected");

        let reader = BufferReader::from(&raw_data[..]);

        assert_eq!(reader.tell(), 0);
        assert_eq!(reader.len(), 42);
        assert!(!reader.is_empty());

        let mut offset = 0;

        assert_eq!(reader.uint8(Some(offset)), 255);
        offset += 1;
        assert_eq!(reader.uint16_le(Some(offset)), 65535);
        assert_eq!(reader.uint16_be(Some(offset)), 65535);
        assert_eq!(reader.uint16(Some(offset), Some(true)), 65535);
        assert_eq!(reader.uint16(Some(offset), Some(false)), 65535);
        assert_eq!(reader.f16_le(Some(offset)), 9.1834e-41);
        assert_eq!(reader.f16_be(Some(offset)), 9.1834e-41);
        assert_eq!(reader.f16(Some(offset), Some(true)), 9.1834e-41);
        assert_eq!(reader.f16(Some(offset), Some(false)), 9.1834e-41);
        offset += 2;
        assert_eq!(reader.uint32_le(Some(offset)), 4294967295);
        assert_eq!(reader.uint32_be(Some(offset)), 4294967295);
        assert_eq!(reader.uint32(Some(offset), Some(true)), 4294967295);
        assert_eq!(reader.uint32(Some(offset), Some(false)), 4294967295);
        offset += 4;
        assert_eq!(reader.int8(Some(offset)), -128);
        offset += 1;
        assert_eq!(reader.int16_le(Some(offset)), -32768);
        assert_eq!(reader.int16_be(Some(offset)), 128);
        assert_eq!(reader.int16(Some(offset), Some(true)), -32768);
        assert_eq!(reader.int16(Some(offset), Some(false)), 128);
        offset += 2;
        assert_eq!(reader.int32_le(Some(offset)), -2147483648);
        assert_eq!(reader.int32_be(Some(offset)), 128);
        assert_eq!(reader.int32(Some(offset), Some(true)), -2147483648);
        assert_eq!(reader.int32(Some(offset), Some(false)), 128);
        offset += 4;
        assert_eq!(reader.f32_le(Some(offset)), 3.14);
        assert_eq!(reader.f32_be(Some(offset)), -490.56445);
        assert_eq!(reader.f32(Some(offset), Some(true)), 3.14);
        assert_eq!(reader.f32(Some(offset), Some(false)), -490.56445);
        offset += 4;
        assert_eq!(reader.f64_le(Some(offset)), 3.14159265359);
        assert_eq!(reader.f64_be(Some(offset)), -2.965482352282314e203);
        assert_eq!(reader.f64(Some(offset), Some(true)), 3.14159265359);
        assert_eq!(reader.f64(Some(offset), Some(false)), -2.965482352282314e203);
        offset += 8;
        assert_eq!(reader.uint64_le(Some(offset)), 12345678901234567890);
        assert_eq!(reader.uint64_be(Some(offset)), 15134944594269656235);
        assert_eq!(reader.uint64(Some(offset), Some(true)), 12345678901234567890);
        assert_eq!(reader.uint64(Some(offset), Some(false)), 15134944594269656235);
        offset += 8;
        assert_eq!(reader.int64_le(Some(offset)), -1234567890123456789);
        assert_eq!(reader.int64_be(Some(offset)), -1477718879929115154);
        assert_eq!(reader.int64(Some(offset), Some(true)), -1234567890123456789);
        assert_eq!(reader.int64(Some(offset), Some(false)), -1477718879929115154);

        let slice = reader.slice(Some(4), Some(8));
        assert_eq!(slice, &[255, 255, 255, 128]);
        reader.seek(4);
        assert_eq!(reader.seek_slice(4), &[255, 255, 255, 128]);
        assert_eq!(reader.tell(), 8);
    }
}
