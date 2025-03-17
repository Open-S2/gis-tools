use alloc::vec::Vec;
use pbf::bit_cast::BitCast;

const MAX_VARINT_LENGTH: usize = u64::BITS as usize * 8 / 7 + 1;
const BIT_SHIFT: [u64; 10] = [0, 7, 14, 21, 28, 35, 42, 49, 56, 63];

/// The `Buffer` struct is used to read and write Buffer messages.
///
/// ## Description
/// This works as a wrapper around a byte buffer. The idea is to have an opinionated reading/writing
/// API for little-endian encodings for various signed and unsigned integer types. It also includes
/// basic varint encoding and decoding.
///
/// ## Usage
///
/// Create a new Buffer instance:
/// ```rs
/// use gistools::util::Buffer;
///
/// let mut buf = Buffer::new();
/// ```
/// Create a Buffer instance from a byte buffer:
/// ```rs
/// use gistools::util::Buffer;
///
/// let mut vec = vec![0x0A, 0x03, 0x74, 0x65, 0x73, 0x74];
/// let mut buf = Buffer::new(vec);
/// // DO STUFF
/// let res = buf.take();
/// ```
#[derive(Default)]
pub struct Buffer {
    buf: Vec<u8>,
    pos: usize,
}
impl From<Vec<u8>> for Buffer {
    fn from(buf: Vec<u8>) -> Buffer {
        Buffer { buf, pos: 0 }
    }
}
impl Buffer {
    /// Create a new Buffer instance.
    pub fn new(buf: Vec<u8>) -> Buffer {
        Buffer { buf, pos: 0 }
    }

    /// Set the position to read from the buffer next.
    pub fn set_pos(&mut self, pos: usize) {
        self.pos = pos;
    }

    /// get the length of the bufer
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    /// check if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// return the current u8 under the buffer
    pub fn get_u8(&mut self) -> u8 {
        let value = self.get_u8_at(self.pos);
        self.pos += 1;

        value
    }

    /// return the current u8 at position
    pub fn get_u8_at(&mut self, pos: usize) -> u8 {
        self.buf[pos]
    }

    /// set the current u8 under the buffer
    pub fn set_u8(&mut self, value: u8) {
        self.set_u8_at(self.pos, value);
        self.pos += 1;
    }

    /// set the current u8 at position
    pub fn set_u8_at(&mut self, pos: usize, value: u8) {
        if pos >= self.buf.len() {
            self.buf.resize(pos + 1, 0);
        }
        self.buf[pos] = value;
    }

    /// return the current i32 under the buffer
    pub fn get_i32(&mut self) -> i32 {
        let value = self.get_i32_at(self.pos);
        // Update the position
        self.pos += 4;

        value
    }

    /// return the current i32 at position
    pub fn get_i32_at(&mut self, pos: usize) -> i32 {
        // Borrow the buffer and slice the next 4 bytes
        let bytes = &self.buf[pos..pos + 4];

        i32::from_le_bytes(bytes.try_into().expect("slice with incorrect length"))
    }

    /// set the current i32 under the buffer
    pub fn set_i32(&mut self, value: i32) {
        self.set_i32_at(self.pos, value);
        self.pos += 4;
    }

    /// set the current i32 at position
    pub fn set_i32_at(&mut self, pos: usize, value: i32) {
        // Borrow the buffer and slice the next 4 bytes
        if pos >= self.buf.len() {
            self.buf.resize(pos + 4, 0);
        }
        let bytes = &mut self.buf[pos..pos + 4];

        bytes.copy_from_slice(&value.to_le_bytes());
    }

    /// return the current u16 under the buffer
    pub fn get_u16(&mut self) -> u16 {
        let value = self.get_u16_at(self.pos);
        // Update the position
        self.pos += 2;

        value
    }

    /// return the current u16 at position
    pub fn get_u16_at(&mut self, pos: usize) -> u16 {
        // Borrow the buffer and slice the next 2 bytes
        let bytes = &self.buf[pos..pos + 2];

        u16::from_le_bytes(bytes.try_into().expect("slice with incorrect length"))
    }

    /// set the current u16 under the buffer
    pub fn set_u16(&mut self, value: u16) {
        self.set_u16_at(self.pos, value);
        self.pos += 2;
    }

    /// set the current u16 at position
    pub fn set_u16_at(&mut self, pos: usize, value: u16) {
        // Borrow the buffer and slice the next 2 bytes
        if pos >= self.buf.len() {
            self.buf.resize(pos + 2, 0);
        }
        let bytes = &mut self.buf[pos..pos + 2];

        bytes.copy_from_slice(&value.to_le_bytes());
    }

    /// return the current u32 under the buffer
    pub fn get_u32(&mut self) -> u32 {
        let value = self.get_u32_at(self.pos);
        // Update the position
        self.pos += 4;

        value
    }

    /// return the current u32 at position
    pub fn get_u32_at(&mut self, pos: usize) -> u32 {
        // Borrow the buffer and slice the next 4 bytes
        let bytes = &self.buf[pos..pos + 4];

        u32::from_le_bytes(bytes.try_into().expect("slice with incorrect length"))
    }

    /// set the current u32 under the buffer
    pub fn set_u32(&mut self, value: u32) {
        self.set_u32_at(self.pos, value);
        self.pos += 4;
    }

    /// set the current u32 at position
    pub fn set_u32_at(&mut self, pos: usize, value: u32) {
        // Borrow the buffer and slice the next 4 bytes
        if pos >= self.buf.len() {
            self.buf.resize(pos + 4, 0);
        }
        let bytes = &mut self.buf[pos..pos + 4];

        bytes.copy_from_slice(&value.to_le_bytes());
    }

    /// return the current i32 under the buffer
    pub fn get_i64(&mut self) -> i64 {
        let value = self.get_i64_at(self.pos);
        // Update the position
        self.pos += 8;

        value
    }

    /// return the current i32 at position
    pub fn get_i64_at(&mut self, pos: usize) -> i64 {
        // Borrow the buffer and slice the next 8 bytes
        let bytes = &self.buf[pos..pos + 8];

        i64::from_le_bytes(bytes.try_into().expect("slice with incorrect length"))
    }

    /// set the current i32 under the buffer
    pub fn set_i64(&mut self, value: i64) {
        self.set_i64_at(self.pos, value);
        self.pos += 8;
    }

    /// set the current i32 at position
    pub fn set_i64_at(&mut self, pos: usize, value: i64) {
        // Borrow the buffer and slice the next 8 bytes
        if pos >= self.buf.len() {
            self.buf.resize(pos + 8, 0);
        }
        let bytes = &mut self.buf[pos..pos + 8];

        bytes.copy_from_slice(&value.to_le_bytes());
    }

    /// return the current u64 under the buffer
    pub fn get_u64(&mut self) -> u64 {
        let value = self.get_u64_at(self.pos);
        // Update the position
        self.pos += 8;

        value
    }

    /// return the current u64 at position
    pub fn get_u64_at(&mut self, pos: usize) -> u64 {
        // Borrow the buffer and slice the next 8 bytes
        let bytes = &self.buf[pos..pos + 8];

        u64::from_le_bytes(bytes.try_into().expect("slice with incorrect length"))
    }

    /// set the current u64 under the buffer
    pub fn set_u64(&mut self, value: u64) {
        self.set_u64_at(self.pos, value);
        self.pos += 8;
    }

    /// set the current u64 at position
    pub fn set_u64_at(&mut self, pos: usize, value: u64) {
        // Borrow the buffer and slice the next 8 bytes
        if pos >= self.buf.len() {
            self.buf.resize(pos + 8, 0);
        }
        let bytes = &mut self.buf[pos..pos + 8];

        bytes.copy_from_slice(&value.to_le_bytes());
    }

    /// Decode a varint from the buffer at the current position.
    pub fn decode_varint(&mut self) -> u64 {
        if self.pos >= self.buf.len() {
            unreachable!();
        }
        let mut val: u64 = 0;

        for (n, shift) in BIT_SHIFT.iter().enumerate().take(MAX_VARINT_LENGTH) {
            let b = self.buf[self.pos] as u64;
            self.pos += 1;
            if n == 0 {
                if b & 0x80 == 0 {
                    return b;
                }
                val = b & 0x7f;
            } else {
                val |= (b & 0x7f) << shift;
            }
            if b < 0x80 {
                break;
            }
        }

        val
    }

    /// Read in a variable size value from the buffer.
    pub fn read_varint<T>(&mut self) -> T
    where
        T: BitCast,
    {
        let val = self.decode_varint();
        T::from_u64(val)
    }

    /// Write a u64 to the buffer.
    pub fn write_varint<T>(&mut self, val: T)
    where
        T: BitCast,
    {
        let mut val = val.to_u64();

        while val >= 0x80 {
            self.buf.push(((val & 0x7f) | 0x80) as u8);
            val >>= 7;
        }
        self.buf.push(val as u8);
    }

    /// When done writing to the buffer, call this function to take ownership
    pub fn take(&mut self) -> Vec<u8> {
        core::mem::take(&mut self.buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_buffer() {
        // new
        let buf = Buffer::default();
        let vec1: Vec<u8> = vec![];
        assert_eq!(vec1, buf.buf.to_vec());

        // from
        let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let buf2: Buffer = Buffer::new(vec.clone());
        assert_eq!(vec, buf2.buf.to_vec());
    }

    #[test]
    fn test_set_pos() {
        let mut buf = Buffer::default();
        assert_eq!(0, buf.pos);
        buf.set_pos(1);
        assert_eq!(1, buf.pos);
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

    // get_u16, get_u16_at & set_u16
    #[test]
    fn test_get_u16() {
        let mut buf = Buffer::default();
        buf.set_u16(1);
        buf.set_pos(0);
        assert_eq!(1, buf.get_u16());
        assert_eq!(1, buf.get_u16_at(0));
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

    // take
    #[test]
    fn test_take() {
        let mut buf = Buffer::default();
        buf.set_u8(1);
        buf.set_u8(2);
        buf.set_u8(3);
        assert_eq!(vec![1, 2, 3], buf.take());
    }
}
