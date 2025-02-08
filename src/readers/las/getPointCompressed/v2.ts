import { ArithmeticModel } from '../arithmeticDecoder';
import { IntegerCompressor } from '../integerCompressor';
import { LASpoint10, LASrgba } from '.';
import { U32I32F32, U64I64F64 } from '../util';

import type { ArithmeticDecoder } from '../arithmeticDecoder';
import type { ItemReader } from '.';

const LASZIP_GPSTIME_MULTI = 500;
const LASZIP_GPSTIME_MULTI_MINUS = -10;
const LASZIP_GPSTIME_MULTI_UNCHANGED = LASZIP_GPSTIME_MULTI - LASZIP_GPSTIME_MULTI_MINUS + 1;
const LASZIP_GPSTIME_MULTI_CODE_FULL = LASZIP_GPSTIME_MULTI - LASZIP_GPSTIME_MULTI_MINUS + 2;

const LASZIP_GPSTIME_MULTI_TOTAL = LASZIP_GPSTIME_MULTI - LASZIP_GPSTIME_MULTI_MINUS + 6;

/**
 * Fold a number between 0 and 255
 * @param n - the number to fold
 * @returns - the folded number
 */
function u8Fold(n: number): number {
  return n < 0 ? n + 256 : n > 255 ? n - 256 : n;
}

/**
 * Clamp a number between 0 and 255
 * @param n - the number to clamp
 * @returns - the clamped number
 */
function u8Clamp(n: number): number {
  return n < 0 ? 0 : n > 255 ? 255 : n;
}

/**
 * zero the least significant bit
 * @param n - the number to zero
 * @returns - the zeroed number
 */
function u32ZeroBit0(n: number): number {
  return n & 0xfffffffe;
}

/**
 * for LAS files with the return (r) and the number (n) of
 * returns field correctly populated the mapping should really
 * be only the following.
 *  { 15, 15, 15, 15, 15, 15, 15, 15 },
 *  { 15,  0, 15, 15, 15, 15, 15, 15 },
 *  { 15,  1,  2, 15, 15, 15, 15, 15 },
 *  { 15,  3,  4,  5, 15, 15, 15, 15 },
 *  { 15,  6,  7,  8,  9, 15, 15, 15 },
 *  { 15, 10, 11, 12, 13, 14, 15, 15 },
 *  { 15, 15, 15, 15, 15, 15, 15, 15 },
 *  { 15, 15, 15, 15, 15, 15, 15, 15 }
 * however, some files start the numbering of r and n with 0,
 * only have return counts r, or only have number of return
 * counts n, or mix up the position of r and n. we therefore
 * "complete" the table to also map those "undesired" r & n
 * combinations to different contexts
 * 8 x 8 u8 values
 */
const numberReturnMap = [
  [15, 14, 13, 12, 11, 10, 9, 8],
  [14, 0, 1, 3, 6, 10, 10, 9],
  [13, 1, 2, 4, 7, 11, 11, 10],
  [12, 3, 4, 5, 8, 12, 12, 11],
  [11, 6, 7, 8, 9, 13, 13, 12],
  [10, 10, 11, 12, 13, 14, 14, 13],
  [9, 10, 11, 12, 13, 14, 15, 14],
  [8, 9, 10, 11, 12, 13, 14, 15],
] as const;

/**
 * for LAS files with the return (r) and the number (n) of
 * returns field correctly populated the mapping should really
 * be only the following.
 *  {  0,  7,  7,  7,  7,  7,  7,  7 },
 *  {  7,  0,  7,  7,  7,  7,  7,  7 },
 *  {  7,  1,  0,  7,  7,  7,  7,  7 },
 *  {  7,  2,  1,  0,  7,  7,  7,  7 },
 *  {  7,  3,  2,  1,  0,  7,  7,  7 },
 *  {  7,  4,  3,  2,  1,  0,  7,  7 },
 *  {  7,  5,  4,  3,  2,  1,  0,  7 },
 *  {  7,  6,  5,  4,  3,  2,  1,  0 }
 * however, some files start the numbering of r and n with 0,
 * only have return counts r, or only have number of return
 * counts n, or mix up the position of r and n. we therefore
 * "complete" the table to also map those "undesired" r & n
 * combinations to different contexts
 * 8 x 8 u8 values
 */
const numberReturnLevel = [
  [0, 1, 2, 3, 4, 5, 6, 7],
  [1, 0, 1, 2, 3, 4, 5, 6],
  [2, 1, 0, 1, 2, 3, 4, 5],
  [3, 2, 1, 0, 1, 2, 3, 4],
  [4, 3, 2, 1, 0, 1, 2, 3],
  [5, 4, 3, 2, 1, 0, 1, 2],
  [6, 5, 4, 3, 2, 1, 0, 1],
  [7, 6, 5, 4, 3, 2, 1, 0],
] as const;

/** Streaming Median 5 */
export class StreamingMedian5 {
  values: number[] = new Array(5); // i32
  high: boolean = false;

  /** Runs the initialization */
  constructor() {
    this.init();
  }

  /** initialize the first value set */
  init() {
    this.values[0] = this.values[1] = this.values[2] = this.values[3] = this.values[4] = 0;
    this.high = true;
  }
  /**
   * add a new value
   * @param v - the new value to add
   */
  add(v: number): void {
    const { high, values } = this;
    if (high) {
      if (v < values[2]) {
        values[4] = values[3];
        values[3] = values[2];
        if (v < values[0]) {
          values[2] = values[1];
          values[1] = values[0];
          values[0] = v;
        } else if (v < values[1]) {
          values[2] = values[1];
          values[1] = v;
        } else {
          values[2] = v;
        }
      } else {
        if (v < values[3]) {
          values[4] = values[3];
          values[3] = v;
        } else {
          values[4] = v;
        }
        this.high = false;
      }
    } else {
      if (values[2] < v) {
        values[0] = values[1];
        values[1] = values[2];
        if (values[4] < v) {
          values[2] = values[3];
          values[3] = values[4];
          values[4] = v;
        } else if (values[3] < v) {
          values[2] = values[3];
          values[3] = v;
        } else {
          values[2] = v;
        }
      } else {
        if (values[1] < v) {
          values[0] = values[1];
          values[1] = v;
        } else {
          values[0] = v;
        }
        this.high = true;
      }
    }
  }

  /** @returns the median value */
  get(): number {
    return this.values[2];
  }
}

/** LAZ Point10 2.0 Reader */
export class LAZPoint10V2Reader implements ItemReader {
  lastItem = new LASpoint10();
  lastIncr = 0; // I32 last_incr;
  icDx: IntegerCompressor; // IntegerCompressor* icDx;
  icDy: IntegerCompressor; // IntegerCompressor* icDy;
  icZ: IntegerCompressor; // IntegerCompressor* icZ;
  icIntensity: IntegerCompressor; // IntegerCompressor* this.icIntensity;
  lastIntensity: Uint16Array = new Uint16Array(16);
  lastXDiffMedian5: StreamingMedian5[];
  lastYDiffMedian5: StreamingMedian5[];
  lastHeight: Int32Array = new Int32Array(8);
  icPointSourceID: IntegerCompressor; // IntegerCompressor* icPointSourceID;
  mChangedValues: ArithmeticModel; // ArithmeticModel* m_changedValues;
  mBitByte: (ArithmeticModel | undefined)[]; // ArithmeticModel* mBitByte[256];
  mClassification: (ArithmeticModel | undefined)[]; // ArithmeticModel* mClassification[256];
  mUserData: (ArithmeticModel | undefined)[]; // ArithmeticModel* mUserData[256];
  mScanAngleRank: [ArithmeticModel, ArithmeticModel];

  /** @param dec - the arithmetic decoder */
  constructor(readonly dec: ArithmeticDecoder) {
    /* create models and integer compressors */
    this.mChangedValues = new ArithmeticModel(64, false);
    this.icIntensity = new IntegerCompressor(dec, 16, 4);
    this.mScanAngleRank = [new ArithmeticModel(256), new ArithmeticModel(256)];
    this.icPointSourceID = new IntegerCompressor(dec, 16);
    this.mChangedValues = new ArithmeticModel(64, false);
    this.mBitByte = new Array(256).fill(undefined);
    this.mClassification = new Array(256).fill(undefined);
    this.mUserData = new Array(256).fill(undefined);
    this.icDx = new IntegerCompressor(dec, 32, 2); // 32 bits, 2 context
    this.icDy = new IntegerCompressor(dec, 32, 22); // 32 bits, 22 contexts
    this.icZ = new IntegerCompressor(dec, 32, 20); // 32 bits, 20 contexts
    this.lastXDiffMedian5 = new Array(16);
    this.lastYDiffMedian5 = new Array(16);
    for (let i = 0; i < 16; i++) {
      this.lastXDiffMedian5[i] = new StreamingMedian5();
      this.lastYDiffMedian5[i] = new StreamingMedian5();
    }
  }

  /** @param item - the first raw item needs to be injected for future reads */
  init(item: DataView): void {
    let i: number;
    /* init state */
    for (i = 0; i < 16; i++) {
      this.lastXDiffMedian5[i].init();
      this.lastYDiffMedian5[i].init();
      this.lastIntensity[i] = 0;
      this.lastHeight[Math.trunc(i / 2)] = 0;
    }
    /* init models and integer compressors */
    this.mChangedValues.init();
    this.icIntensity.initDecompressor();
    this.mScanAngleRank[0].init();
    this.mScanAngleRank[1].init();
    this.icPointSourceID.initDecompressor();
    for (i = 0; i < 256; i++) {
      this.mBitByte[i]?.init();
      this.mClassification[i]?.init();
      this.mUserData[i]?.init();
    }
    this.icDx.initDecompressor();
    this.icDy.initDecompressor();
    this.icZ.initDecompressor();

    /* init last item */
    this.lastItem.copyFrom(item, 20);

    /* but set intensity to zero */
    this.lastItem.intensity = 0;
  }

  /** @param item - the current item to be read into */
  read(item: DataView): void {
    let r, n, m, l;
    let k_bits;
    let median, diff;
    // decompress which other values have changed
    const changedValues = this.dec.decodeSymbol(this.mChangedValues);
    if (changedValues !== 0) {
      // decompress the edge_of_flight_line, scanDirectionFlag, ... if it has changed
      if ((changedValues & 32) !== 0) {
        if (this.mBitByte[this.lastItem.flags] === undefined) {
          this.mBitByte[this.lastItem.flags] = new ArithmeticModel(256);
          this.mBitByte[this.lastItem.flags]?.init();
        }
        this.lastItem.flags = this.dec.decodeSymbol(this.mBitByte[this.lastItem.flags]!);
      }
      r = this.lastItem.returnNumber;
      n = this.lastItem.numberOfReturns;
      m = numberReturnMap[n][r];
      l = numberReturnLevel[n][r];
      // decompress the intensity if it has changed
      if ((changedValues & 16) !== 0) {
        this.lastItem.intensity = this.icIntensity.decompress(this.lastIntensity[m], {
          value: m < 3 ? m : 3,
        });
        this.lastIntensity[m] = this.lastItem.intensity;
      } else {
        this.lastItem.intensity = this.lastIntensity[m];
      }
      // decompress the classification ... if it has changed
      if ((changedValues & 8) !== 0) {
        if (this.mClassification[this.lastItem.class] === undefined) {
          this.mClassification[this.lastItem.class] = new ArithmeticModel(256);
          this.mClassification[this.lastItem.class]?.init();
        }
        this.lastItem.class = this.dec.decodeSymbol(this.mClassification[this.lastItem.class]!);
      }
      // decompress the scan_angle_rank ... if it has changed
      if ((changedValues & 4) !== 0) {
        const val = this.dec.decodeSymbol(this.mScanAngleRank[this.lastItem.scanDirectionFlag]);
        this.lastItem.scanAngleRank = u8Fold(val + this.lastItem.scanAngleRank);
      }
      // decompress the user_data ... if it has changed
      if ((changedValues & 2) !== 0) {
        if (this.mUserData[this.lastItem.userData] === undefined) {
          this.mUserData[this.lastItem.userData] = new ArithmeticModel(256);
          this.mUserData[this.lastItem.userData]?.init();
        }
        this.lastItem.userData = this.dec.decodeSymbol(this.mUserData[this.lastItem.userData]!);
      }
      // decompress the pointSourceID ... if it has changed
      if ((changedValues & 1) !== 0) {
        this.lastItem.pointSourceID = this.icPointSourceID.decompress(this.lastItem.pointSourceID);
      }
    } else {
      r = this.lastItem.returnNumber;
      n = this.lastItem.numberOfReturns;
      m = numberReturnMap[n][r];
      l = numberReturnLevel[n][r];
    }
    // decompress x coordinate
    median = this.lastXDiffMedian5[m].get();
    diff = this.icDx.decompress(median, { value: n === 1 ? 1 : 0 });
    this.lastItem.x += diff;
    this.lastXDiffMedian5[m].add(diff);
    // decompress y coordinate
    median = this.lastYDiffMedian5[m].get();
    k_bits = this.icDx.getK();
    diff = this.icDy.decompress(median, {
      value: (n === 1 ? 1 : 0) + (k_bits < 20 ? u32ZeroBit0(k_bits) : 20),
    });
    this.lastItem.y += diff;
    this.lastYDiffMedian5[m].add(diff);
    // decompress z coordinate
    k_bits = Math.trunc((this.icDx.getK() + this.icDy.getK()) / 2);
    this.lastItem.z = this.icZ.decompress(this.lastHeight[l], {
      value: (n === 1 ? 1 : 0) + (k_bits < 18 ? u32ZeroBit0(k_bits) : 18),
    });
    this.lastHeight[l] = this.lastItem.z;

    // copy in the last point
    this.lastItem.copyTo(item, 20);
  }
}

/** Parse LAZ GPS Time 1.1v2 */
export class LAZgpstime11v2Reader implements ItemReader {
  mGpstimeMulti: ArithmeticModel;
  mGpstime0diff: ArithmeticModel;
  icGpstime: IntegerCompressor;
  last = 0; // U32
  next = 0; // U32
  lastGpstime: [U64I64F64, U64I64F64, U64I64F64, U64I64F64] = [
    new U64I64F64(0, 'u64'),
    new U64I64F64(0, 'u64'),
    new U64I64F64(0, 'u64'),
    new U64I64F64(0, 'u64'),
  ];
  lastGpstimeDiff: [number, number, number, number] = [0, 0, 0, 0]; // I32
  multiExtremeCounter: [number, number, number, number] = [0, 0, 0, 0]; // I32

  /** @param dec - the arithmetic decoder */
  constructor(readonly dec: ArithmeticDecoder) {
    /* create entropy models and integer compressors */
    this.mGpstimeMulti = new ArithmeticModel(LASZIP_GPSTIME_MULTI_TOTAL);
    this.mGpstime0diff = new ArithmeticModel(6);
    this.icGpstime = new IntegerCompressor(dec, 32, 9); // 32 bits, 9 contexts
  }

  /** @param item - the first raw item needs to be injected for future reads */
  init(item: DataView): void {
    /* init state */
    this.last = 0;
    this.next = 0;
    this.lastGpstimeDiff = [0, 0, 0, 0];
    this.multiExtremeCounter = [0, 0, 0, 0];

    /* init models and integer compressors */
    this.mGpstimeMulti.init();
    this.mGpstime0diff.init();
    this.icGpstime.initDecompressor();

    /* init last item */
    this.lastGpstime[0].u64 = item.getBigUint64(0, true); // *((const U64*)item);
    this.lastGpstime[1].u64 = 0;
    this.lastGpstime[2].u64 = 0;
    this.lastGpstime[3].u64 = 0;
  }

  /** @param item - the current item to be read into */
  read(item: DataView): void {
    let multi: number; // I32
    if (this.lastGpstimeDiff[this.last] === 0) {
      // if the last integer difference was zero
      multi = this.dec.decodeSymbol(this.mGpstime0diff);
      if (multi === 1) {
        // the difference can be represented with 32 bits
        this.lastGpstimeDiff[this.last] = this.icGpstime.decompress(0, { value: 0 });
        this.lastGpstime[this.last].i64 += BigInt(this.lastGpstimeDiff[this.last]);
        this.multiExtremeCounter[this.last] = 0;
      } else if (multi === 2) {
        // the difference is huge
        this.next = (this.next + 1) & 3;
        const value = new U32I32F32(Number(this.lastGpstime[this.last].u64 >> 32n), 'u32');
        this.lastGpstime[this.next].u64 = this.icGpstime.decompress(Number(value.i32), {
          value: 8,
        });
        this.lastGpstime[this.next].u64 = BigInt(this.lastGpstime[this.next].u64) << 32n;
        const int = new U32I32F32(this.dec.readInt(), 'u32');
        this.lastGpstime[this.next].u64 |= BigInt(int.u32);
        this.last = this.next;
        this.lastGpstimeDiff[this.last] = 0;
        this.multiExtremeCounter[this.last] = 0;
      } else if (multi > 2) {
        // we switch to another sequence
        this.last = (this.last + multi - 2) & 3;
        this.read(item);
      }
    } else {
      multi = this.dec.decodeSymbol(this.mGpstimeMulti);
      if (multi === 1) {
        this.lastGpstime[this.last].i64 += BigInt(
          this.icGpstime.decompress(this.lastGpstimeDiff[this.last], { value: 1 }),
        );
        this.multiExtremeCounter[this.last] = 0;
      } else if (multi < LASZIP_GPSTIME_MULTI_UNCHANGED) {
        let gpstimeDiff;
        if (multi === 0) {
          gpstimeDiff = this.icGpstime.decompress(0, { value: 7 });
          this.multiExtremeCounter[this.last]++;
          if (this.multiExtremeCounter[this.last] > 3) {
            this.lastGpstimeDiff[this.last] = gpstimeDiff;
            this.multiExtremeCounter[this.last] = 0;
          }
        } else if (multi < LASZIP_GPSTIME_MULTI) {
          if (multi < 10)
            gpstimeDiff = this.icGpstime.decompress(multi * this.lastGpstimeDiff[this.last], {
              value: 2,
            });
          else
            gpstimeDiff = this.icGpstime.decompress(multi * this.lastGpstimeDiff[this.last], {
              value: 3,
            });
        } else if (multi === LASZIP_GPSTIME_MULTI) {
          gpstimeDiff = this.icGpstime.decompress(
            LASZIP_GPSTIME_MULTI * this.lastGpstimeDiff[this.last],
            { value: 4 },
          );
          this.multiExtremeCounter[this.last]++;
          if (this.multiExtremeCounter[this.last] > 3) {
            this.lastGpstimeDiff[this.last] = gpstimeDiff;
            this.multiExtremeCounter[this.last] = 0;
          }
        } else {
          multi = LASZIP_GPSTIME_MULTI - multi;
          if (multi > LASZIP_GPSTIME_MULTI_MINUS) {
            gpstimeDiff = this.icGpstime.decompress(multi * this.lastGpstimeDiff[this.last], {
              value: 5,
            });
          } else {
            gpstimeDiff = this.icGpstime.decompress(
              LASZIP_GPSTIME_MULTI_MINUS * this.lastGpstimeDiff[this.last],
              { value: 6 },
            );
            this.multiExtremeCounter[this.last]++;
            if (this.multiExtremeCounter[this.last] > 3) {
              this.lastGpstimeDiff[this.last] = gpstimeDiff;
              this.multiExtremeCounter[this.last] = 0;
            }
          }
        }
        this.lastGpstime[this.last].i64 += BigInt(gpstimeDiff);
      } else if (multi === LASZIP_GPSTIME_MULTI_CODE_FULL) {
        this.next = (this.next + 1) & 3;
        const value = new U32I32F32(Number(this.lastGpstime[this.last].u64 >> 32n), 'u32');
        this.lastGpstime[this.next].u64 = this.icGpstime.decompress(Number(value.i32), {
          value: 8,
        });
        this.lastGpstime[this.next].u64 = this.lastGpstime[this.next].u64 << 32n;
        const int = new U32I32F32(this.dec.readInt(), 'u32');
        this.lastGpstime[this.next].u64 |= BigInt(int.u32);
        this.last = this.next;
        this.lastGpstimeDiff[this.last] = 0;
        this.multiExtremeCounter[this.last] = 0;
      } else if (multi >= LASZIP_GPSTIME_MULTI_CODE_FULL) {
        this.last = (this.last + multi - LASZIP_GPSTIME_MULTI_CODE_FULL) & 3;
        this.read(item);
      }
    }

    item.setBigInt64(0, this.lastGpstime[this.last].i64, true);
  }
}

/** Parse LAZ RGB 1.2v2 */
export class LAZrgb12v2 implements ItemReader {
  lastItem = new LASrgba();
  mByteUsed = new ArithmeticModel(128);
  mRgbDiff0 = new ArithmeticModel(256);
  mRgbDiff1 = new ArithmeticModel(256);
  mRgbDiff2 = new ArithmeticModel(256);
  mRgbDiff3 = new ArithmeticModel(256);
  mRgbDiff4 = new ArithmeticModel(256);
  mRgbDiff5 = new ArithmeticModel(256);

  /** @param dec - the arithmetic decoder */
  constructor(readonly dec: ArithmeticDecoder) {}

  /** @param item - the first raw item needs to be injected for future reads */
  init(item: DataView): void {
    /* init models and integer compressors */
    this.mByteUsed.init();
    this.mRgbDiff0.init();
    this.mRgbDiff1.init();
    this.mRgbDiff2.init();
    this.mRgbDiff3.init();
    this.mRgbDiff4.init();
    this.mRgbDiff5.init();

    /* init last item */
    this.lastItem.copyFrom(item, 6);
  }

  /**
   * read in the next item
   * @param item - the next item
   */
  read(item: DataView): void {
    const currItem = new LASrgba(item);
    currItem.copyFrom(this.lastItem.data as DataView, 6);
    let corr: number;
    let diff = 0;
    const sym = this.dec.decodeSymbol(this.mByteUsed);
    if ((sym & 1) !== 0) {
      corr = this.dec.decodeSymbol(this.mRgbDiff0);
      currItem.r = u8Fold(corr + (this.lastItem.r & 255));
    } else {
      currItem.r = this.lastItem.r & 0xff;
    }
    if ((sym & (1 << 1)) !== 0) {
      corr = this.dec.decodeSymbol(this.mRgbDiff1);
      currItem.r |= u8Fold(corr + (this.lastItem.r >> 8)) << 8;
    } else {
      currItem.r |= this.lastItem.r & 0xff00;
    }
    if ((sym & (1 << 6)) !== 0) {
      diff = (currItem.r & 0x00ff) - (this.lastItem.r & 0x00ff);
      if ((sym & (1 << 2)) !== 0) {
        corr = this.dec.decodeSymbol(this.mRgbDiff2);
        currItem.g = u8Fold(corr + u8Clamp(diff + (this.lastItem.g & 255)));
      } else {
        currItem.g = this.lastItem.g & 0xff;
      }
      if ((sym & (1 << 4)) !== 0) {
        corr = this.dec.decodeSymbol(this.mRgbDiff4);
        diff = Math.trunc((diff + ((currItem.g & 0x00ff) - (this.lastItem.g & 0x00ff))) / 2);
        currItem.b = u8Fold(corr + u8Clamp(diff + (this.lastItem.b & 255)));
      } else {
        currItem.b = this.lastItem.b & 0xff;
      }
      diff = (currItem.r >> 8) - (this.lastItem.r >> 8);
      if ((sym & (1 << 3)) !== 0) {
        corr = this.dec.decodeSymbol(this.mRgbDiff3);
        currItem.g |= u8Fold(corr + u8Clamp(diff + (this.lastItem.g >> 8))) << 8;
      } else {
        currItem.g |= this.lastItem.g & 0xff00;
      }
      if ((sym & (1 << 5)) !== 0) {
        corr = this.dec.decodeSymbol(this.mRgbDiff5);
        diff = Math.trunc((diff + ((currItem.g >> 8) - (this.lastItem.g >> 8))) / 2);
        currItem.b |= u8Fold(corr + u8Clamp(diff + (this.lastItem.b >> 8))) << 8;
      } else {
        currItem.b |= this.lastItem.b & 0xff00;
      }
    } else {
      currItem.g = currItem.r;
      currItem.b = currItem.r;
    }

    currItem.copyTo(this.lastItem.data, 6);
  }
}

/** LAZ byte reader V2 */
export class LAZbyteV2 implements ItemReader {
  mByte: ArithmeticModel[];
  lastItem: Uint8Array;
  /**
   * @param dec - the arithmetic decoder
   * @param number - the number of bytes to read
   */
  constructor(
    readonly dec: ArithmeticDecoder,
    readonly number: number,
  ) {
    /* create models and integer compressors */
    this.mByte = new Array<ArithmeticModel>(number).map(() => new ArithmeticModel(256));
    /* create last item */
    this.lastItem = new Uint8Array(number);
  }

  /** @param item - the first raw item needs to be injected for future reads */
  init(item: DataView): void {
    /* init models and integer compressors */
    for (let i = 0; i < this.number; i++) this.mByte[i].init();

    /* init last item */
    const itemUint8 = new Uint8Array(item.buffer, item.byteOffset, this.number);
    this.lastItem.set(itemUint8);
  }

  /** @param item - the current item to be read into */
  read(item: DataView): void {
    let value: number;
    const itemUint8 = new Uint8Array(item.buffer, item.byteOffset, this.number);
    for (let i = 0; i < this.number; i++) {
      value = this.lastItem[i] + this.dec.decodeSymbol(this.mByte[i]);
      itemUint8[i] = u8Fold(value);
    }
    // update lastItem
    this.lastItem.set(itemUint8);
  }
}
