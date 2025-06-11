use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub};
use libm::pow;
use s2json::{GetM, MValue, MValueCompatible, ValuePrimitive, ValueType};
use serde::{Deserialize, Serialize};

use crate::parsers::Reader;

/// Gamma correction
const GAMMA: f64 = 2.2;

/// Convert from u8 sRGB (gamma-encoded) to linear space
pub fn gamma_to_linear(n: f64) -> f64 {
    pow(n / 255., 1. / GAMMA)
}

/// Convert from linear space to u8 sRGB (gamma-encoded)
pub fn linear_to_gamma(n: f64) -> f64 {
    pow(n, GAMMA) * 255.
}

/// RGBA data in 0->1 range floats
/// These values remove gamma-corrected values so that you can apply maths on them
/// This means the RGBA values are in linear space
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, ValuePrimitive)]
pub struct RGBA {
    /// Gamma corrected Red between 0 and 1
    pub r: f64,
    /// Gamma corrected Green between 0 and 1
    pub g: f64,
    /// Gamma corrected Blue between 0 and 1
    pub b: f64,
    /// Opacity between 0 and 1 (not gamma corrected as opacity is linear)
    pub a: f64,
}
impl Default for RGBA {
    fn default() -> Self {
        Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
    }
}
impl RGBA {
    /// Create a new RGBA value
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }

    /// Create a new RGBA value from gamma-corrected values
    pub fn from_gamma(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self::new(pow(r, 1. / GAMMA), pow(g, 1. / GAMMA), pow(b, 1. / GAMMA), a)
    }

    /// Convert RGBA to gamma-corrected values
    pub fn to_gamma(self) -> (f64, f64, f64, f64) {
        (pow(self.r, GAMMA), pow(self.g, GAMMA), pow(self.b, GAMMA), self.a)
    }

    /// Create a new RGBA value from 4 u8 values
    pub fn from_u8s(r: u8, g: u8, b: u8, a: u8) -> Self {
        let max_u8 = u8::MAX as f64;
        // store the gamma corrected value
        let r = (r as f64) / max_u8;
        let g = (g as f64) / max_u8;
        let b = (b as f64) / max_u8;
        let a = (a as f64) / max_u8;
        Self::from_gamma(r, g, b, a)
    }

    /// Convert RGBA to 4 u8 values in Gamma corrected 0->255 range
    pub fn to_u8s(self) -> (u8, u8, u8, u8) {
        let (r, g, b, a) = self.to_gamma();
        let max_u8 = u8::MAX as f64;
        (
            (r * max_u8).round() as u8,
            (g * max_u8).round() as u8,
            (b * max_u8).round() as u8,
            (a * max_u8).round() as u8,
        )
    }

    /// Create a new RGBA value from 4 u16 values
    pub fn from_reader<R: Reader>(reader: &R, offset: Option<u64>) -> Self {
        let offset = offset.unwrap_or(reader.tell());
        let r = reader.uint16_le(Some(offset));
        let g = reader.uint16_le(Some(offset + 2));
        let b = reader.uint16_le(Some(offset + 4));
        RGBA::from_u16s(r, g, b, u16::MAX)
    }

    /// Create a new RGBA value from 4 u16 values
    pub fn from_u16s(r: u16, g: u16, b: u16, a: u16) -> Self {
        let max_u16 = u16::MAX as f64;
        let r = (r as f64) / max_u16;
        let g = (g as f64) / max_u16;
        let b = (b as f64) / max_u16;
        let a = (a as f64) / max_u16;
        Self::from_gamma(r, g, b, a)
    }

    /// Convert RGBA to 4 u16 values in Gamma corrected 0->65,535 range
    pub fn to_u16s(&self) -> (u16, u16, u16, u16) {
        let (r, g, b, a) = self.to_gamma();
        let max_u16 = u16::MAX as f64;
        (
            (r * max_u16).round() as u16,
            (g * max_u16).round() as u16,
            (b * max_u16).round() as u16,
            (a * max_u16).round() as u16,
        )
    }

    /// Convert a 32-bit integer to RGBA
    pub fn from_u32(value: u32) -> Self {
        let r = ((value >> 24) & 0xFF) as u8;
        let g = ((value >> 16) & 0xFF) as u8;
        let b = ((value >> 8) & 0xFF) as u8;
        let a = (value & 0xFF) as u8;
        Self::from_u8s(r, g, b, a)
    }

    /// Convert RGBA to a 32-bit integer (Big-endian: 0xRRGGBBAA)
    /// big-endian is more common in graphics formats like BMP, PNG, etc.
    pub fn to_u32(&self) -> u32 {
        let max_u8 = u8::MAX as f64;
        let (r, g, b, a) = self.to_gamma();
        ((r * max_u8).round() as u32) << 24
            | ((g * max_u8).round() as u32) << 16
            | ((b * max_u8).round() as u32) << 8
            | ((a * max_u8).round() as u32)
    }

    /// Convert an unsigned 64-bit integer to RGBA
    pub fn from_u64(value: u64) -> Self {
        let r = ((value >> 48) & 0xFFFF) as u16;
        let g = ((value >> 32) & 0xFFFF) as u16;
        let b = ((value >> 16) & 0xFFFF) as u16;
        let a = (value & 0xFFFF) as u16;
        Self::from_u16s(r, g, b, a)
    }

    /// Convert RGBA to an unsigned 64-bit integer
    pub fn to_u64(&self) -> u64 {
        let max_u16 = u16::MAX as f64;
        let (r, g, b, a) = self.to_gamma();
        ((r * max_u16).round() as u64) << 48
            | ((g * max_u16).round() as u64) << 32
            | ((b * max_u16).round() as u64) << 16
            | ((a * max_u16).round() as u64)
    }
}
impl GetM<RGBA> for RGBA {
    fn m(&self) -> Option<&RGBA> {
        Some(self)
    }
}
impl Add<RGBA> for RGBA {
    type Output = RGBA;
    fn add(self, rhs: RGBA) -> Self::Output {
        RGBA::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b, self.a + rhs.a)
    }
}
impl AddAssign<RGBA> for RGBA {
    fn add_assign(&mut self, rhs: RGBA) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.a += rhs.a;
    }
}
impl AddAssign<f64> for RGBA {
    fn add_assign(&mut self, rhs: f64) {
        self.r += rhs;
        self.g += rhs;
        self.b += rhs;
        self.a += rhs;
    }
}
impl Sub<RGBA> for RGBA {
    type Output = RGBA;
    fn sub(self, rhs: RGBA) -> Self::Output {
        RGBA::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b, self.a - rhs.a)
    }
}
impl Mul<RGBA> for RGBA {
    type Output = RGBA;
    fn mul(self, rhs: RGBA) -> Self::Output {
        RGBA::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b, self.a * rhs.a)
    }
}
impl MulAssign<RGBA> for RGBA {
    fn mul_assign(&mut self, rhs: RGBA) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
        self.a *= rhs.a;
    }
}
impl MulAssign<f64> for RGBA {
    fn mul_assign(&mut self, rhs: f64) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
        self.a *= rhs;
    }
}
impl Div<RGBA> for RGBA {
    type Output = RGBA;
    fn div(self, rhs: RGBA) -> Self::Output {
        RGBA::new(self.r / rhs.r, self.g / rhs.g, self.b / rhs.b, self.a / rhs.a)
    }
}
impl DivAssign<RGBA> for RGBA {
    fn div_assign(&mut self, rhs: RGBA) {
        self.r /= rhs.r;
        self.g /= rhs.g;
        self.b /= rhs.b;
        self.a /= rhs.a;
    }
}
impl DivAssign<f64> for RGBA {
    fn div_assign(&mut self, rhs: f64) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
        self.a /= rhs;
    }
}
impl PartialEq<f64> for RGBA {
    fn eq(&self, rhs: &f64) -> bool {
        self.r == *rhs || self.g == *rhs || self.b == *rhs || self.a == *rhs
    }
}
impl MValueCompatible for RGBA {}
impl From<MValue> for RGBA {
    fn from(mvalue: MValue) -> Self {
        let r = mvalue.get("r").unwrap().to_prim().unwrap().to_u64().unwrap() as u8;
        let g = mvalue.get("g").unwrap().to_prim().unwrap().to_u64().unwrap() as u8;
        let b = mvalue.get("b").unwrap().to_prim().unwrap().to_u64().unwrap() as u8;
        let a = mvalue.get("a").unwrap().to_prim().unwrap().to_u64().unwrap() as u8;
        RGBA::from_u8s(r, g, b, a)
    }
}
impl From<&MValue> for RGBA {
    fn from(mvalue: &MValue) -> Self {
        let r = mvalue.get("r").unwrap().to_prim().unwrap().to_u64().unwrap() as u8;
        let g = mvalue.get("g").unwrap().to_prim().unwrap().to_u64().unwrap() as u8;
        let b = mvalue.get("b").unwrap().to_prim().unwrap().to_u64().unwrap() as u8;
        let a = mvalue.get("a").unwrap().to_prim().unwrap().to_u64().unwrap() as u8;
        RGBA::from_u8s(r, g, b, a)
    }
}
impl From<RGBA> for MValue {
    fn from(value: RGBA) -> Self {
        let (r, g, b, a) = value.to_u8s();
        MValue::from([
            ("r".into(), (r as u64).into()),
            ("g".into(), (g as u64).into()),
            ("b".into(), (b as u64).into()),
            ("a".into(), (a as u64).into()),
        ])
    }
}
impl From<RGBA> for ValueType {
    fn from(value: RGBA) -> Self {
        let (r, g, b, a) = value.to_u8s();
        ValueType::Nested(MValue::from([
            ("r".into(), (r as u64).into()),
            ("g".into(), (g as u64).into()),
            ("b".into(), (b as u64).into()),
            ("a".into(), (a as u64).into()),
        ]))
    }
}
impl From<&ValueType> for RGBA {
    fn from(value: &ValueType) -> Self {
        let ValueType::Nested(mvalue) = value else {
            panic!("Expected nested value type");
        };
        let r = mvalue.get("r").unwrap().to_prim().unwrap().to_u64().unwrap() as u8;
        let g = mvalue.get("g").unwrap().to_prim().unwrap().to_u64().unwrap() as u8;
        let b = mvalue.get("b").unwrap().to_prim().unwrap().to_u64().unwrap() as u8;
        let a = mvalue.get("a").unwrap().to_prim().unwrap().to_u64().unwrap() as u8;
        RGBA::from_u8s(r, g, b, a)
    }
}
