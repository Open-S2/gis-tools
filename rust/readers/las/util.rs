/// Fold a number between 0 and 255
/// @param n - the number to fold
/// @returns - the folded number
pub fn u8_fold(n: u32) -> u8 {
    if n > 255 { (n - 256) as u8 } else { n as u8 }
}

/// Clamp a number between 0 and 255
/// @param n - the number to clamp
/// @returns - the clamped number
pub fn u8_clamp(n: u32) -> u8 {
    if n > 255 { 255 } else { n as u8 }
}

/// Clamp a number between -128 and 127
/// @param n - the number to clamp
/// @returns - the clamped number
pub fn i8_clamp(n: i32) -> i8 {
    if n <= -128 {
        -128
    } else if n >= 127 {
        127
    } else {
        n as i8
    }
}

/// zero the least significant bit
/// @param n - the number to zero
/// @returns - the zeroed number
pub fn u32_zero_bit0(n: u32) -> u32 {
    n & 0xfffffffe
}

/// Quantize a signed 16-bit number
/// @param n - the number to quantize
/// @returns - the quantized number
pub fn i16_quantize(n: f64) -> i16 {
    if (n) >= 0. { ((n) + 0.5) as i16 } else { ((n) - 0.5) as i16 }
}

/// A special buffer that stores a 64-bit number and can be converted to different types
/// respecting bit positions
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct U64I64F64 {
    bytes: [u8; 8],
}

impl U64I64F64 {
    /// Creates a new U64I64F64
    pub fn new(value: impl IntoValue64, ty: ValueType64) -> Self {
        let mut out = Self { bytes: [0; 8] };
        out.set(value, ty);
        out
    }
    /// Sets the value
    pub fn set(&mut self, value: impl IntoValue64, ty: ValueType64) {
        match ty {
            ValueType64::U64 => self.bytes = value.into_u64().to_le_bytes(),
            ValueType64::I64 => self.bytes = value.into_i64().to_le_bytes(),
            ValueType64::F64 => self.bytes = value.into_f64().to_le_bytes(),
        }
    }
    /// Get the u64
    pub fn u64(&self) -> u64 {
        u64::from_le_bytes(self.bytes)
    }
    /// Set u64
    pub fn set_u64(&mut self, value: u64) {
        self.bytes = value.to_le_bytes();
    }
    /// Get i64
    pub fn i64(&self) -> i64 {
        i64::from_le_bytes(self.bytes)
    }
    /// Set i64
    pub fn set_i64(&mut self, value: i64) {
        self.bytes = value.to_le_bytes();
    }
    /// Get f64
    pub fn f64(&self) -> f64 {
        f64::from_le_bytes(self.bytes)
    }
    /// Set f64
    pub fn set_f64(&mut self, value: f64) {
        self.bytes = value.to_le_bytes();
    }
}

/// A special buffer that stores a 64-bit number and can be converted to different types
/// respecting bit positions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType64 {
    /// Unsigned 64-bit number
    U64,
    /// Signed 64-bit number
    I64,
    /// 64-bit float
    F64,
}
/// A trait for converting to different types
pub trait IntoValue64 {
    /// Converts to u64
    fn into_u64(self) -> u64;
    /// Converts to i64
    fn into_i64(self) -> i64;
    /// Converts to f64
    fn into_f64(self) -> f64;
}
impl IntoValue64 for u64 {
    fn into_u64(self) -> u64 {
        self
    }
    fn into_i64(self) -> i64 {
        self as i64
    }
    fn into_f64(self) -> f64 {
        self as f64
    }
}
impl IntoValue64 for i64 {
    fn into_u64(self) -> u64 {
        self as u64
    }
    fn into_i64(self) -> i64 {
        self
    }
    fn into_f64(self) -> f64 {
        self as f64
    }
}
impl IntoValue64 for f64 {
    fn into_u64(self) -> u64 {
        self.to_bits()
    }
    fn into_i64(self) -> i64 {
        self.to_bits() as i64
    }
    fn into_f64(self) -> f64 {
        self
    }
}

/// A special buffer that stores a 32-bit number and can be converted to different types
/// respecting bit positions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U32I32F32 {
    bytes: [u8; 4],
}

impl U32I32F32 {
    /// Creates a new U32I32F32
    pub fn new(value: impl IntoValue32, ty: ValueType32) -> Self {
        let mut out = Self { bytes: [0; 4] };
        out.set(value, ty);
        out
    }
    /// Sets the value
    pub fn set(&mut self, value: impl IntoValue32, ty: ValueType32) {
        match ty {
            ValueType32::U32 => self.bytes = value.into_u32().to_le_bytes(),
            ValueType32::I32 => self.bytes = value.into_i32().to_le_bytes(),
            ValueType32::F32 => self.bytes = value.into_f32().to_le_bytes(),
        }
    }
    /// Get the u32
    pub fn u32(&self) -> u32 {
        u32::from_le_bytes(self.bytes)
    }
    /// Set u32
    pub fn set_u32(&mut self, value: u32) {
        self.bytes = value.to_le_bytes();
    }
    /// Get i32
    pub fn i32(&self) -> i32 {
        i32::from_le_bytes(self.bytes)
    }
    /// Set i32
    pub fn set_i32(&mut self, value: i32) {
        self.bytes = value.to_le_bytes();
    }
    /// Get f32
    pub fn f32(&self) -> f32 {
        f32::from_le_bytes(self.bytes)
    }
    /// Set f32
    pub fn set_f32(&mut self, value: f32) {
        self.bytes = value.to_le_bytes();
    }
}

/// A enum for converting to different types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType32 {
    /// Unsigned 32-bit number
    U32,
    /// Signed 32-bit number
    I32,
    /// 32-bit float
    F32,
}
/// A trait for converting to different types
pub trait IntoValue32 {
    /// Converts to u32
    fn into_u32(self) -> u32;
    /// Converts to i32
    fn into_i32(self) -> i32;
    /// Converts to f32
    fn into_f32(self) -> f32;
}

impl IntoValue32 for u32 {
    fn into_u32(self) -> u32 {
        self
    }
    fn into_i32(self) -> i32 {
        self as i32
    }
    fn into_f32(self) -> f32 {
        self as f32
    }
}
impl IntoValue32 for i32 {
    fn into_u32(self) -> u32 {
        self as u32
    }
    fn into_i32(self) -> i32 {
        self
    }
    fn into_f32(self) -> f32 {
        self as f32
    }
}
impl IntoValue32 for f32 {
    fn into_u32(self) -> u32 {
        self.to_bits()
    }
    fn into_i32(self) -> i32 {
        self.to_bits() as i32
    }
    fn into_f32(self) -> f32 {
        self
    }
}
