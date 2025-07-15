use super::arithmetic_decoder::{
    ArithmeticBitModel, ArithmeticDecoder, ArithmeticModel, Corrector,
};
use crate::parsers::Reader;
use alloc::{rc::Rc, vec, vec::Vec};
use core::cell::RefCell;

const I32_MIN: i32 = 0x80000000u32 as i32;
const I32_MAX: i32 = 0x7fffffffu32 as i32;

/// https://github.com/LASzip/LASzip/blob/master/src/integercompressor.cpp
#[derive(Debug)]
pub struct IntegerCompressor<T: Reader> {
    /// Arithmetic decoder
    pub dec: Rc<RefCell<ArithmeticDecoder<T>>>,
    k: u32,
    corr_bits: u32,
    corr_range: u32,
    /// Corrector minimum
    pub corr_min: i32,
    /// Corrector maximum
    pub corr_max: i32,
    m_bits: Vec<ArithmeticModel>,
    m_corrector: Vec<Corrector>,
    // user defined options
    bits: u32,
    contexts: u32,
    bits_high: u32,
    range: u32,
}
impl<T: Reader> IntegerCompressor<T> {
    /// Create a new integer compressor
    pub fn new(
        dec: Rc<RefCell<ArithmeticDecoder<T>>>,
        bits: Option<u32>,
        contexts: Option<u32>,
        bits_high: Option<u32>,
        range: Option<u32>,
    ) -> IntegerCompressor<T> {
        let mut ic = IntegerCompressor {
            dec,
            k: 0,
            corr_bits: 0,
            corr_range: 0,
            corr_min: 0,
            corr_max: 0,
            m_bits: vec![],
            m_corrector: vec![],
            bits: bits.unwrap_or(16),
            contexts: contexts.unwrap_or(1),
            bits_high: bits_high.unwrap_or(8),
            range: range.unwrap_or(0),
        };

        if ic.range != 0 {
            // the corrector's significant bits and range
            ic.corr_bits = 0;
            ic.corr_range = ic.range;
            while ic.range != 0 {
                ic.range >>= 1;
                ic.corr_bits += 1;
            }
            if ic.corr_range == 1 << (ic.corr_bits - 1) {
                ic.corr_bits -= 1;
            }
            // the corrector must fall into ic interval
            ic.corr_min = -((ic.corr_range / 2) as i32);
            ic.corr_max = ic.corr_min + ic.corr_range as i32 - 1;
        } else if ic.bits != 0 && ic.bits < 32 {
            ic.corr_bits = ic.bits;
            ic.corr_range = 1 << ic.bits;
            // the corrector must fall into ic interval
            ic.corr_min = -((ic.corr_range / 2) as i32);
            ic.corr_max = ic.corr_min + ic.corr_range as i32 - 1;
        } else {
            ic.corr_bits = 32;
            ic.corr_range = 0;
            // the corrector must fall into ic interval
            ic.corr_min = I32_MIN;
            ic.corr_max = I32_MAX;
        }

        ic
    }

    /// Get the K value in the Compressor
    pub fn get_k(&self) -> u32 {
        self.k
    }

    /// Initialize the decompressor
    pub fn init_decompressor(&mut self) {
        // maybe create the models
        if self.m_bits.is_empty() {
            self.m_bits = Vec::with_capacity(self.contexts as usize);
            for _ in 0..self.contexts {
                self.m_bits.push(ArithmeticModel::new(self.corr_bits + 1, false));
            }
            self.m_corrector.push(ArithmeticBitModel::new().into());
            for i in 1..=self.corr_bits {
                if i <= self.bits_high {
                    self.m_corrector.push(ArithmeticModel::new(1 << i, false).into());
                } else {
                    self.m_corrector.push(ArithmeticModel::new(1 << self.bits_high, false).into());
                }
            }
        }
        // certainly init the models
        for i in 0..self.contexts as usize {
            self.m_bits[i].init(None);
        }
        for i in 0..=self.corr_bits as usize {
            self.m_corrector[i].init(None);
        }
    }

    /// @param pred - the predicted value
    /// @param context - the context DEFAULTS TO 0 IF NOT GIVEN
    /// @returns - the decompressed value
    pub fn decompress(&mut self, pred: i32, context: u32) -> i32 {
        let mut real = pred + self.read_corrector(context);
        if real < 0 {
            real += self.corr_range as i32;
        } else if real as u32 >= self.corr_range {
            real -= self.corr_range as i32;
        }
        real
    }

    /// @param m_bits - the arithmetic model
    /// @returns - the corrector
    fn read_corrector(&mut self, context: u32) -> i32 {
        let mut c: i32 = 0; // I32
        let mut dec = self.dec.borrow_mut();

        // decode within which interval the corrector is falling
        self.k = dec.decode_symbol(&mut self.m_bits[context as usize]);

        // decode the exact location of the corrector within the interval
        if self.k != 0 {
            // then c is either smaller than 0 or bigger than 1
            if self.k < 32 {
                if self.k <= self.bits_high {
                    // for small k we can do self in one step
                    // decompress c with the range coder
                    if let Some(model) = self.m_corrector[self.k as usize].get_model() {
                        c = dec.decode_symbol(model) as i32;
                    }
                } else {
                    // for larger k we need to do self in two steps
                    let k1 = self.k as i32 - self.bits_high as i32;
                    // decompress higher bits with table
                    if let Some(model) = self.m_corrector[self.k as usize].get_model() {
                        c = dec.decode_symbol(model) as i32;
                    }
                    // read lower bits raw
                    let c1 = dec.read_bits(k1 as u32) as i32;
                    // put the corrector back together
                    c = (c << k1) | c1;
                }
                // translate c back into its correct interval
                if c >= (1 << (self.k - 1)) {
                    // if c is in the interval [ 2^(k-1)  ...  + 2^k - 1 ]
                    // so we translate c back into the interval [ 2^(k-1) + 1  ...  2^k ] by adding 1
                    c += 1;
                } else {
                    // otherwise c is in the interval [ 0 ...  + 2^(k-1) - 1 ]
                    // so we translate c back into the interval [ - (2^k - 1)  ...  - (2^(k-1)) ] by subtracting (2^k - 1)
                    // c -= (1 << self.k) - 1;
                    let offset = (1u32 << self.k) - 1;
                    c = c.wrapping_sub(offset as i32);
                }
            } else {
                c = self.corr_min;
            }
        } else {
            // then c is either 0 or 1
            if let Some(model) = self.m_corrector[0].get_bit_model() {
                c = dec.decode_bit(model) as i32;
            }
        }

        c
    }
}
