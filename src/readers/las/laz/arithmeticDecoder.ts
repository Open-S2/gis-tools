/* this header byte needs to change in case incompatible change happen */
export const AC_HEADER_BYTE = 2;
export const AC_BUFFER_SIZE = 4096;

export const AC__MinLength = 0x01000000; // threshold for renormalization
export const AC__MaxLength = 0xffffffff; // maximum AC interval length

// Maximum values for binary models
export const BM__LengthShift = 13; // length bits discarded before mult.
export const BM__MaxCount = 1 << BM__LengthShift; // for adaptive models

// Maximum values for general models
export const DM__LengthShift = 15; // length bits discarded before mult.
export const DM__MaxCount = 1 << DM__LengthShift; // for adaptive models

/**
 *
 */
export class ArithmeticModel {
  distribution: number[] = [];
  symbol_count_index = 0;
  decoder_table_index = 0;
  total_count = 0;
  update_cycle = 0;
  symbols_until_update = 0;
  last_symbol = 0;
  table_size = 0;
  table_shift = 0;
  /**
   * @param symbols
   * @param compress
   */
  constructor(
    private symbols: number,
    private compress: boolean,
  ) {}

  /**
   * @param table
   */
  init(table: number[] | undefined): number {
    if (this.distribution.length === 0) {
      if (this.symbols < 2 || this.symbols > 1 << 11) return -1; // invalid number of symbols
      this.last_symbol = this.symbols - 1;
      if (!this.compress && this.symbols > 16) {
        let table_bits = 3;
        while (this.symbols > 1 << (table_bits + 2)) ++table_bits;
        this.table_size = 1 << table_bits;
        this.table_shift = DM__LengthShift - table_bits;
        this.distribution = new Array(2 * this.symbols + this.table_size + 2);
        this.decoder_table_index = 2 * this.symbols;
      } else {
        // small alphabet: no table needed
        this.decoder_table_index = 0;
        this.table_size = this.table_shift = 0;
        this.distribution = new Array(2 * this.symbols);
      }
      if (this.distribution.length === 0) return -1; // "cannot allocate model memory");
      this.symbol_count_index = this.symbols;
    }

    this.total_count = 0;
    this.update_cycle = this.symbols;
    if (table !== undefined)
      for (let k = 0; k < this.symbols; k++)
        this.distribution[this.symbol_count_index + k] = table[k];
    else for (let k = 0; k < this.symbols; k++) this.distribution[this.symbol_count_index + k] = 1;

    this.update();
    this.symbols_until_update = this.update_cycle = (this.symbols + 6) >> 1;

    return 0;
  }

  /**
   *
   */
  update(): void {
    // halve counts when a threshold is reached
    if ((this.total_count += this.update_cycle) > DM__MaxCount) {
      this.total_count = 0;
      for (let n = 0; n < this.symbols; n++) {
        this.total_count += this.distribution[this.symbol_count_index + n] =
          (this.distribution[this.symbol_count_index + n] + 1) >> 1;
      }
    }

    // compute cumulative distribution, decoder table
    let k: number,
      sum = 0,
      s = 0;
    const scale = 0x80000000 / this.total_count;

    if (this.compress || this.table_size === 0) {
      for (k = 0; k < this.symbols; k++) {
        this.distribution[k] = (scale * sum) >> (31 - DM__LengthShift);
        sum += this.distribution[this.symbol_count_index + k];
      }
    } else {
      for (k = 0; k < this.symbols; k++) {
        this.distribution[k] = (scale * sum) >> (31 - DM__LengthShift);
        sum += this.distribution[this.symbol_count_index + k];
        const w = this.distribution[k] >> this.table_shift;
        while (s < w) this.distribution[this.decoder_table_index + ++s] = k - 1;
      }
      this.distribution[this.decoder_table_index] = 0;
      while (s <= this.table_size)
        this.distribution[this.decoder_table_index + ++s] = this.symbols - 1;
    }

    // set frequency of model updates
    this.update_cycle = (5 * this.update_cycle) >> 2;
    const max_cycle = (this.symbols + 6) << 3;
    if (this.update_cycle > max_cycle) this.update_cycle = max_cycle;
    this.symbols_until_update = this.update_cycle;
  }
}

/**
 *
 */
export class ArithmeticBitModel {
  // start with frequent updates
  update_cycle = 4;
  bits_until_update = 4;
  // initialization to equiprobable model
  bit_0_prob = 1 << (BM__LengthShift - 1);
  bit_0_count = 1;
  bit_count = 2;

  /**
   *
   */
  update(): void {
    // halve counts when a threshold is reached
    if ((this.bit_count += this.update_cycle) > BM__MaxCount) {
      this.bit_count = (this.bit_count + 1) >> 1;
      this.bit_0_count = (this.bit_0_count + 1) >> 1;
      if (this.bit_0_count === this.bit_count) ++this.bit_count;
    }

    // compute scaled bit 0 probability
    const scale = 0x80000000 / this.bit_count;
    this.bit_0_prob = (this.bit_0_count * scale) >> (31 - BM__LengthShift);

    // set frequency of model updates
    this.update_cycle = (5 * this.update_cycle) >> 2;
    if (this.update_cycle > 64) this.update_cycle = 64;
    this.bits_until_update = this.update_cycle;
  }
}
