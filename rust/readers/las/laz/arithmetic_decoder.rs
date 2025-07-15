use crate::{
    parsers::Reader,
    readers::util::{U32I32F32, U64I64F64, ValueType32, ValueType64},
};
use alloc::{rc::Rc, vec, vec::Vec};
use core::cell::RefCell;

// self header byte needs to change in case incompatible change happen
/// The header byte for arithmetic coding
pub const AC_HEADER_BYTE: u8 = 2;
/// The buffer size
pub const AC_BUFFER_SIZE: u32 = 4096;

/// threshold for renormalization
pub const AC_MIN_LENGTH: u32 = 0x01000000;
/// maximum AC interval length
pub const AC_MAX_LENGTH: u32 = 0xffffffff;

// Maximum values for binary models
/// length bits discarded before mult
pub const BM_LENGTH_SHIFT: u32 = 13;
/// for adaptive models
pub const BM_MAX_COUNT: u32 = 1 << BM_LENGTH_SHIFT;

// Maximum values for general models
/// length bits discarded before mult
pub const DM_LENGTH_SHIFT: u32 = 15;
/// for adaptive models
pub const DM_MAX_COUNT: u32 = 1 << DM_LENGTH_SHIFT;

/// A "Corrector" wrapper that handles two type of arithmetic models
#[derive(Debug)]
pub enum Corrector {
    /// ArithmeticModel
    ArithmeticModel(ArithmeticModel),
    /// ArithmeticBitModel
    ArithmeticBitModel(ArithmeticBitModel),
}
impl Corrector {
    /// Initialize the model
    pub fn init(&mut self, table: Option<Vec<u32>>) {
        match self {
            Corrector::ArithmeticModel(m) => m.init(table),
            Corrector::ArithmeticBitModel(m) => m.init(table),
        }
    }
    /// Update the model
    pub fn update(&mut self) {
        match self {
            Corrector::ArithmeticModel(m) => m.update(),
            Corrector::ArithmeticBitModel(m) => m.update(),
        }
    }
    /// Get the model. Most models are ArithmeticModel, so when we ask for a model, we normally
    /// want the arithmetic model. The Bit Model is only the first model.
    pub fn get_model(&mut self) -> Option<&mut ArithmeticModel> {
        match self {
            Corrector::ArithmeticModel(m) => Some(m),
            Corrector::ArithmeticBitModel(_) => None,
        }
    }
    /// Get the bit model. The FIRST model is the bit model.
    pub fn get_bit_model(&mut self) -> Option<&mut ArithmeticBitModel> {
        match self {
            Corrector::ArithmeticModel(_) => None,
            Corrector::ArithmeticBitModel(m) => Some(m),
        }
    }
}
impl From<ArithmeticModel> for Corrector {
    fn from(m: ArithmeticModel) -> Self {
        Corrector::ArithmeticModel(m)
    }
}
impl From<ArithmeticBitModel> for Corrector {
    fn from(m: ArithmeticBitModel) -> Self {
        Corrector::ArithmeticBitModel(m)
    }
}

/// https://github.com/LASzip/LASzip/blob/master/src/arithmeticdecoder.cpp
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ArithmeticDecoder<T: Reader> {
    /// The data to read from
    pub reader: Rc<RefCell<T>>,
    /// The current value
    pub value: u32,
    /// The current length
    pub length: u32,
}
impl<T: Reader> ArithmeticDecoder<T> {
    /// Create a new arithmetic decoder
    pub fn new(reader: Rc<RefCell<T>>) -> Self {
        Self { reader, value: 0, length: AC_MAX_LENGTH }
    }
    /// Initialize the decoder
    /// @param reallyInit - if set to true, initializes the value
    pub fn init(&mut self, really_init: bool) {
        self.length = AC_MAX_LENGTH;
        if really_init {
            self.value = self.reader.borrow().uint32_be(None);
        }
    }

    /// @param bits - The number of bits
    /// @returns - The decoded bits
    pub fn read_bits(&mut self, mut bits: u32) -> u32 {
        assert!(bits != 0 && (bits <= 32));

        if bits > 19 {
            let tmp = self.read_short() as u32;
            bits -= 16;
            let tmp1 = self.read_bits(bits) * 65_536;
            return tmp1 | tmp;
        }

        self.length >>= bits;
        let sym = self.value / self.length;
        self.value -= self.length * sym; // update interval

        if self.length < AC_MIN_LENGTH {
            self.renorm_dec_interval(); // renormalization
        }
        if sym >= 1 << bits {
            panic!("4711");
        }

        sym
    }

    /// @param m - The arithmetic bit model
    /// @returns - The decoded bit
    pub fn decode_bit(&mut self, m: &mut ArithmeticBitModel) -> u32 {
        let x = m.bit0_prob * (self.length >> BM_LENGTH_SHIFT); // product l x p0
        let sym = if self.value >= x { 1 } else { 0 }; // decision
        // update & shift interval
        if sym == 0 {
            self.length = x;
            m.bit0_count += 1;
        } else {
            self.value -= x; // shifted interval base = 0
            self.length -= x;
        }

        if self.length < AC_MIN_LENGTH {
            self.renorm_dec_interval(); // renormalization
        }
        m.bits_until_update -= 1;
        if m.bits_until_update == 0 {
            m.update(); // periodic model update
        }

        sym // return data bit value
    }

    /// @param m - The arithmetic model
    /// @returns - The decoded symbol
    pub fn decode_symbol(&mut self, m: &mut ArithmeticModel) -> u32 {
        let mut sym;
        let mut x;
        let mut y = self.length;

        if m.decoder_table_index != NULL_POINTER {
            self.length >>= DM_LENGTH_SHIFT;
            let dv = self.value / self.length;
            let t = dv >> m.table_shift;

            sym = m.distribution[(m.decoder_table_index + t) as usize]; // initial decision based on table look-up
            let mut n = m.distribution[(m.decoder_table_index + t + 1) as usize] + 1;

            while n > sym + 1 {
                // finish with bisection search
                let k = (sym + n) >> 1;
                if m.distribution[k as usize] > dv {
                    n = k;
                } else {
                    sym = k;
                }
            }
            // compute products
            x = m.distribution[sym as usize] * self.length;
            if sym != m.last_symbol {
                y = m.distribution[(sym + 1) as usize] * self.length;
            }
        } else {
            // decode using only multiplications
            sym = 0;
            x = sym;
            self.length >>= DM_LENGTH_SHIFT;
            let mut n = m.symbols;
            let mut k = n >> 1;
            // decode via bisection search
            loop {
                let z = self.length * m.distribution[k as usize];
                if z > self.value {
                    n = k;
                    y = z; // value is smaller
                } else {
                    sym = k;
                    x = z; // value is larger or equal
                }
                k = (sym + n) >> 1;
                if k == sym {
                    break;
                }
            }
        }

        self.value -= x; // update interval
        self.length = y - x;

        if self.length < AC_MIN_LENGTH {
            self.renorm_dec_interval(); // renormalization
        }

        m.distribution[(m.symbol_count_index + sym) as usize] += 1;
        m.symbols_until_update -= 1;
        if m.symbols_until_update == 0 {
            m.update(); // periodic model update
        }
        assert!(sym < m.symbols);

        sym
    }

    /// @returns - The decoded bit
    pub fn read_bit(&mut self) -> u32 {
        self.length >>= 1;
        let sym = self.value / self.length; // decode symbol, change length
        self.value -= self.length * sym; // update interval

        if self.length < AC_MIN_LENGTH {
            self.renorm_dec_interval(); // renormalization
        }
        if sym >= 2 {
            panic!("4711");
        }

        sym
    }

    /// @returns - The decoded byte
    pub fn read_byte(&mut self) -> u8 {
        self.length >>= 8;
        let sym = self.value / self.length; // decode symbol, change length
        self.value -= self.length * sym; // update interval

        if self.length < AC_MIN_LENGTH {
            self.renorm_dec_interval(); // renormalization
        }
        if sym >= 1 << 8 {
            panic!("4711");
        }

        sym as u8
    }

    /// @returns - The decoded short
    pub fn read_short(&mut self) -> u16 {
        self.length >>= 16;
        let sym = self.value / self.length; // decode symbol, change length
        self.value -= self.length * sym; // update interval

        if self.length < AC_MIN_LENGTH {
            self.renorm_dec_interval(); // renormalization
        }
        if sym >= 65_536 {
            panic!("4711");
        }

        sym as u16
    }

    /// @returns - The decoded int
    pub fn read_int(&mut self) -> u32 {
        let lower_int = self.read_short() as u32;
        let upper_int = self.read_short() as u32;
        (upper_int << 16) | lower_int
    }

    /// @returns - The decoded float
    pub fn read_float(&mut self) -> f32 {
        // danger in float reinterpretation
        let u32i32f32 = U32I32F32::new(self.read_int(), ValueType32::U32);
        u32i32f32.f32()
    }

    /// @returns - The decoded int64
    pub fn read_int64(&mut self) -> u64 {
        let lower_int = self.read_int() as u64;
        let upper_int = self.read_int() as u64;
        (upper_int << 32) | lower_int
    }

    /// @returns - The decoded double
    pub fn read_double(&mut self) -> f64 {
        let u64i64f64 = U64I64F64::new(self.read_int64(), ValueType64::U64);
        u64i64f64.f64()
    }

    /// Renormalize the decoder interval
    pub fn renorm_dec_interval(&mut self) {
        loop {
            let byte = self.reader.borrow().uint8(None) as u32;
            self.value = (self.value << 8) | byte;
            self.length <<= 8;
            if self.length >= AC_MIN_LENGTH {
                break;
            }
        }
    }
}

const NULL_POINTER: u32 = u32::MAX;

/// Arithmetic Model
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ArithmeticModel {
    /// The distribution
    pub distribution: Vec<u32>,
    /// The symbol count index
    pub symbol_count_index: u32,
    /// The decoder table index
    pub decoder_table_index: u32,
    /// The total count
    pub total_count: u32,
    /// The update cycle
    pub update_cycle: u32,
    /// The symbols until update
    pub symbols_until_update: u32,
    /// The last symbol
    pub last_symbol: u32,
    /// The table size
    pub table_size: u32,
    /// The table shift
    pub table_shift: u32,
    /// The symbols
    pub symbols: u32,
    /// The compress
    pub compress: bool,
}
impl ArithmeticModel {
    /// Create a new ArithmeticModel
    pub fn new(symbols: u32, compress: bool) -> Self {
        Self {
            distribution: Vec::new(),
            symbol_count_index: 0,
            decoder_table_index: NULL_POINTER,
            total_count: 0,
            update_cycle: 0,
            symbols_until_update: 0,
            last_symbol: 0,
            table_size: 0,
            table_shift: 0,
            symbols,
            compress,
        }
    }

    /// Initialize the model
    pub fn init(&mut self, table: Option<Vec<u32>>) {
        if self.distribution.is_empty() {
            if self.symbols < 2 || self.symbols > (1 << 11) {
                panic!("invalid number of symbols");
            }
            self.last_symbol = self.symbols - 1;
            if !self.compress && self.symbols > 16 {
                let mut table_bits = 3;
                while self.symbols > 1 << (table_bits + 2) {
                    table_bits += 1;
                }
                self.table_size = 1 << table_bits;
                self.table_shift = DM_LENGTH_SHIFT - table_bits;
                self.distribution = vec![0; (2 * self.symbols + self.table_size + 2) as usize];
                self.decoder_table_index = 2 * self.symbols;
            } else {
                // small alphabet: no table needed
                self.decoder_table_index = NULL_POINTER;
                self.table_shift = 0;
                self.table_size = 0;
                self.distribution = vec![0; 2 * self.symbols as usize];
            }
            self.symbol_count_index = self.symbols;
        }

        self.total_count = 0;
        self.update_cycle = self.symbols;
        if let Some(table) = table {
            for k in 0..self.symbols {
                self.distribution[(self.symbol_count_index + k) as usize] = table[k as usize];
            }
        } else {
            for k in 0..self.symbols {
                self.distribution[(self.symbol_count_index + k) as usize] = 1;
            }
        }

        self.update();
        self.update_cycle = (self.symbols + 6) >> 1;
        self.symbols_until_update = self.update_cycle;
    }

    /// Update the model
    pub fn update(&mut self) {
        // halve counts when a threshold is reached
        self.total_count += self.update_cycle;
        if self.total_count > DM_MAX_COUNT {
            self.total_count = 0;
            for n in 0..self.symbols {
                self.distribution[(self.symbol_count_index + n) as usize] =
                    (self.distribution[(self.symbol_count_index + n) as usize] + 1) >> 1;
                self.total_count += self.distribution[(self.symbol_count_index + n) as usize];
            }
        }

        // compute cumulative distribution, decoder table
        let mut sum = 0;
        let mut s = 0;
        let scale = 0x80000000 / self.total_count;

        if self.compress || self.table_size == 0 {
            for k in 0..self.symbols {
                self.distribution[k as usize] =
                    ((scale as u64 * sum as u64) >> (31 - DM_LENGTH_SHIFT)) as u32;
                sum += self.distribution[(self.symbol_count_index + k) as usize];
            }
        } else {
            for k in 0..self.symbols {
                self.distribution[k as usize] =
                    ((scale as u64 * sum as u64) >> (31 - DM_LENGTH_SHIFT)) as u32;
                sum += self.distribution[(self.symbol_count_index + k) as usize];
                let w = self.distribution[k as usize] >> self.table_shift;
                while s < w {
                    s += 1;
                    self.distribution[(self.decoder_table_index + s) as usize] = k - 1;
                }
            }
            self.distribution[self.decoder_table_index as usize] = 0;
            while s <= self.table_size {
                s += 1;
                self.distribution[(self.decoder_table_index + s) as usize] = self.symbols - 1;
            }
        }

        // set frequency of model updates
        self.update_cycle = (5 * self.update_cycle) >> 2;
        let max_cycle = (self.symbols + 6) << 3;
        if self.update_cycle > max_cycle {
            self.update_cycle = max_cycle;
        }
        self.symbols_until_update = self.update_cycle;
    }
}

/// Arithmetic Bit Model
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ArithmeticBitModel {
    // start with frequent updates
    /// update cycle
    pub update_cycle: u32,
    /// bits until update
    pub bits_until_update: u32,
    // initialization to equiprobable model
    /// bit 0 probability
    pub bit0_prob: u32,
    /// bit 0 count
    pub bit0_count: u32,
    /// bit count
    pub bit_count: u32,
}

impl ArithmeticBitModel {
    /// Create a new ArithmeticBitModel
    pub fn new() -> Self {
        Self {
            update_cycle: 4,
            bits_until_update: 4,
            bit0_prob: 1 << (BM_LENGTH_SHIFT - 1),
            bit0_count: 1,
            bit_count: 2,
        }
    }

    /// Initialize the model
    pub fn init(&mut self, _table: Option<Vec<u32>>) {
        self.update_cycle = 4;
        self.bits_until_update = 4;
        self.bit0_prob = 1 << (BM_LENGTH_SHIFT - 1);
        self.bit0_count = 1;
        self.bit_count = 2;
    }

    /// Update the model
    pub fn update(&mut self) {
        // halve counts when a threshold is reached
        self.bit_count += self.update_cycle;
        if self.bit_count > BM_MAX_COUNT {
            self.bit_count = (self.bit_count + 1) >> 1;
            self.bit0_count = (self.bit0_count + 1) >> 1;
            if self.bit0_count == self.bit_count {
                self.bit_count += 1;
            }
        }
        // compute scaled bit 0 probability
        let scale = 0x80000000 / self.bit_count;
        self.bit0_prob = (self.bit0_count * scale) >> (31 - BM_LENGTH_SHIFT);
        // set frequency of model updates
        self.update_cycle = (5 * self.update_cycle) >> 2;
        if self.update_cycle > 64 {
            self.update_cycle = 64;
        }
        self.bits_until_update = self.update_cycle;
    }
}
