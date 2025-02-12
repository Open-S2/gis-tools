import { U64I64F64 } from '../util';
import { ArithmeticModel, IntegerCompressor, LASWavePacket13, LASpoint10, LASrgba } from '.';

import type { Reader } from '../..';
import type { ArithmeticDecoder, ItemReader, LAZContext } from '.';

const LASZIP_GPSTIME_MULTIMAX = 512;

/** Parse LAZ Point 1.0 */
export class LAZPoint10v1Reader implements ItemReader {
  lastItem = new LASpoint10();
  lastXDiff: Int32Array = new Int32Array([0, 0, 0]); // I32 last_x_diff[3];
  lastYDiff: Int32Array = new Int32Array([0, 0, 0]); // I32 last_yDiff[3];
  lastIncr = 0; // I32 last_incr;
  icDx: IntegerCompressor; // IntegerCompressor* ic_dx;
  icDy: IntegerCompressor; // IntegerCompressor* ic_dy;
  icZ: IntegerCompressor; // IntegerCompressor* ic_z;
  icIntensity: IntegerCompressor; // IntegerCompressor* ic_intensity;
  icScanAngleRank: IntegerCompressor; // IntegerCompressor* ic_scan_angle_rank;
  icPointSourceID: IntegerCompressor; // IntegerCompressor* ic_point_source_ID;
  mChangedValues: ArithmeticModel; // ArithmeticModel* m_changedValues;
  mBitByte: (ArithmeticModel | undefined)[]; // ArithmeticModel* m_bit_byte[256];
  mClassification: (ArithmeticModel | undefined)[]; // ArithmeticModel* m_classification[256];
  mUserData: (ArithmeticModel | undefined)[]; // ArithmeticModel* m_user_data[256];

  /**  @param dec - the arithmetic decoder */
  constructor(readonly dec: ArithmeticDecoder) {
    /* create models and integer compressors */
    this.icDx = new IntegerCompressor(dec, 32); // 32 bits, 1 context
    this.icDy = new IntegerCompressor(dec, 32, 20); // 32 bits, 20 contexts
    this.icZ = new IntegerCompressor(dec, 32, 20); // 32 bits, 20 contexts
    this.icIntensity = new IntegerCompressor(dec, 16);
    this.icScanAngleRank = new IntegerCompressor(dec, 8, 2);
    this.icPointSourceID = new IntegerCompressor(dec, 16);
    this.mChangedValues = new ArithmeticModel(64, false);
    this.mBitByte = new Array(256).fill(undefined);
    this.mClassification = new Array(256).fill(undefined);
    this.mUserData = new Array(256).fill(undefined);
  }

  /**
   * Read in chunk sizes
   * @param _reader - the full data store
   */
  chunkSizes(_reader: Reader): void {}

  /** @param item - the first raw item needs to be injected for future reads */
  init(item: DataView): void {
    /* init state */
    this.lastXDiff = new Int32Array([0, 0, 0]);
    this.lastYDiff = new Int32Array([0, 0, 0]);
    this.lastIncr = 0;

    /* init models and integer compressors */
    this.icDx.initDecompressor();
    this.icDy.initDecompressor();
    this.icZ.initDecompressor();
    this.icIntensity.initDecompressor();
    this.icScanAngleRank.initDecompressor();
    this.icPointSourceID.initDecompressor();
    this.mChangedValues.init();

    /* init "last item" to current item */
    this.lastItem.copyFrom(item, 20);
  }

  /**
   * @param item - the current item to be read into
   * @param context - the current context
   */
  read(item: DataView, context: LAZContext): void {
    // find median difference for x and y from 3 preceding differences
    let medianX;
    if (this.lastXDiff[0] < this.lastXDiff[1]) {
      if (this.lastXDiff[1] < this.lastXDiff[2]) medianX = this.lastXDiff[1];
      else if (this.lastXDiff[0] < this.lastXDiff[2]) medianX = this.lastXDiff[2];
      else medianX = this.lastXDiff[0];
    } else {
      if (this.lastXDiff[0] < this.lastXDiff[2]) medianX = this.lastXDiff[0];
      else if (this.lastXDiff[1] < this.lastXDiff[2]) medianX = this.lastXDiff[2];
      else medianX = this.lastXDiff[1];
    }

    let median_y;
    if (this.lastYDiff[0] < this.lastYDiff[1]) {
      if (this.lastYDiff[1] < this.lastYDiff[2]) median_y = this.lastYDiff[1];
      else if (this.lastYDiff[0] < this.lastYDiff[2]) median_y = this.lastYDiff[2];
      else median_y = this.lastYDiff[0];
    } else {
      if (this.lastYDiff[0] < this.lastYDiff[2]) median_y = this.lastYDiff[0];
      else if (this.lastYDiff[1] < this.lastYDiff[2]) median_y = this.lastYDiff[2];
      else median_y = this.lastYDiff[1];
    }

    // decompress x y z coordinates
    const xDiff = this.icDx.decompress(medianX, context);
    this.lastItem.x += xDiff;
    // we use the number k of bits corrector bits to switch contexts
    let kBits = this.icDx.getK();
    const yDiff = this.icDy.decompress(median_y, { value: kBits < 19 ? kBits : 19 });
    this.lastItem.y += yDiff;
    kBits = Math.trunc((kBits + this.icDy.getK()) / 2);
    this.lastItem.z = this.icZ.decompress(this.lastItem.z, { value: kBits < 19 ? kBits : 19 });

    // decompress which other values have changed
    const changedValues = this.dec.decodeSymbol(this.mChangedValues);

    if (changedValues !== 0) {
      // decompress the intensity if it has changed
      if ((changedValues & 32) !== 0) {
        this.lastItem.intensity = this.icIntensity.decompress(this.lastItem.intensity, context);
      }

      // decompress the edge_of_flight_line, scan_direction_flag, ... if it has changed
      if ((changedValues & 16) !== 0) {
        if (this.mBitByte[this.lastItem.flags] === undefined) {
          this.mBitByte[this.lastItem.flags] = new ArithmeticModel(256, false);
          this.mBitByte[this.lastItem.flags]?.init();
        }
        this.lastItem.flags = this.dec.decodeSymbol(this.mBitByte[this.lastItem.flags]!);
      }

      // decompress the classification ... if it has changed
      if ((changedValues & 8) !== 0) {
        if (this.mClassification[this.lastItem.class] === undefined) {
          this.mClassification[this.lastItem.class] = new ArithmeticModel(256, false);
          this.mClassification[this.lastItem.class]?.init();
        }
        this.lastItem.class = this.dec.decodeSymbol(this.mClassification[this.lastItem.class]!);
      }

      // decompress the scan_angle_rank ... if it has changed
      if ((changedValues & 4) !== 0) {
        this.lastItem.scanAngleRank = this.icScanAngleRank.decompress(this.lastItem.scanAngleRank, {
          value: kBits < 3 ? 1 : 0,
        });
      }

      // decompress the user_data ... if it has changed
      if ((changedValues & 2) !== 0) {
        if (this.mUserData[this.lastItem.userData] === undefined) {
          this.mUserData[this.lastItem.userData] = new ArithmeticModel(256, false);
          this.mUserData[this.lastItem.userData]?.init();
        }
        this.lastItem.userData = this.dec.decodeSymbol(this.mUserData[this.lastItem.userData]!);
      }

      // decompress the point_source_ID ... if it has changed
      if ((changedValues & 1) !== 0) {
        this.lastItem.pointSourceID = this.icPointSourceID.decompress(
          this.lastItem.pointSourceID,
          context,
        );
      }
    }

    // record the difference
    this.lastXDiff[this.lastIncr] = xDiff;
    this.lastYDiff[this.lastIncr] = yDiff;
    this.lastIncr++;
    if (this.lastIncr > 2) this.lastIncr = 0;

    // copy in the last point
    this.lastItem.copyTo(item, 20);
  }
}

/** Parse LAZ GPS Time 1.1v1 */
export class LAZgpstime11v1Reader implements ItemReader {
  mGpstimeMulti: ArithmeticModel;
  mGpstime0diff: ArithmeticModel;
  icGpstime: IntegerCompressor;
  lastItemDiff = 0;
  multiExtremeCounter = 0;
  lastItem = new U64I64F64(0, 'u64');
  /** @param dec - the arithmetic decoder */
  constructor(readonly dec: ArithmeticDecoder) {
    /* create entropy models and integer compressors */
    this.mGpstimeMulti = new ArithmeticModel(LASZIP_GPSTIME_MULTIMAX, false); // dec.createSymbolModel(LASZIP_GPSTIME_MULTIMAX);
    this.mGpstime0diff = new ArithmeticModel(3, false); // dec.createSymbolModel(3);
    this.icGpstime = new IntegerCompressor(dec, 32, 6); // 32 bits, 6 contexts
  }

  /**
   * Read in chunk sizes
   * @param _reader - the full data store
   */
  chunkSizes(_reader: Reader): void {}

  /** @param item - the first raw item needs to be injected for future reads */
  init(item: DataView): void {
    /* init state */
    this.lastItemDiff = 0;
    this.multiExtremeCounter = 0;

    /* init models and integer compressors */
    this.mGpstimeMulti.init();
    this.mGpstime0diff.init();
    this.icGpstime.initDecompressor();

    /* init last item */
    this.lastItem.u64 = item.getBigUint64(0, true);
  }

  /** @param item - the current item to be read into */
  read(item: DataView): void {
    let multi;
    if (this.lastItemDiff === 0) {
      // if the last integer difference was zero
      multi = this.dec.decodeSymbol(this.mGpstime0diff);
      if (multi === 1) {
        // the difference can be represented with 32 bits
        this.lastItemDiff = this.icGpstime.decompress(0, { value: 0 });
        this.lastItem.i64 += BigInt(this.lastItemDiff);
      } else if (multi === 2) {
        // the difference is huge
        this.lastItem.u64 = this.dec.readInt64();
      }
    } else {
      multi = this.dec.decodeSymbol(this.mGpstimeMulti);

      if (multi < LASZIP_GPSTIME_MULTIMAX - 2) {
        let gpstimeDiff; // I32
        if (multi === 1) {
          gpstimeDiff = this.icGpstime.decompress(this.lastItemDiff, { value: 1 });
          this.lastItemDiff = gpstimeDiff;
          this.multiExtremeCounter = 0;
        } else if (multi === 0) {
          gpstimeDiff = this.icGpstime.decompress(Math.trunc(this.lastItemDiff / 4), { value: 2 });
          this.multiExtremeCounter++;
          if (this.multiExtremeCounter > 3) {
            this.lastItemDiff = gpstimeDiff;
            this.multiExtremeCounter = 0;
          }
        } else if (multi < 10) {
          gpstimeDiff = this.icGpstime.decompress(multi * this.lastItemDiff, { value: 3 });
        } else if (multi < 50) {
          gpstimeDiff = this.icGpstime.decompress(multi * this.lastItemDiff, { value: 4 });
        } else {
          gpstimeDiff = this.icGpstime.decompress(multi * this.lastItemDiff, { value: 5 });
          if (multi === LASZIP_GPSTIME_MULTIMAX - 3) {
            this.multiExtremeCounter++;
            if (this.multiExtremeCounter > 3) {
              this.lastItemDiff = gpstimeDiff;
              this.multiExtremeCounter = 0;
            }
          }
        }
        this.lastItem.i64 += BigInt(gpstimeDiff);
      } else if (multi < LASZIP_GPSTIME_MULTIMAX - 1) {
        this.lastItem.u64 = this.dec.readInt64();
      }
    }

    // *((I64*)item) = lastItem.i64;
    item.setBigInt64(0, this.lastItem.i64, true);
  }
}

/** Parse LAZ RGB 1.2v1 */
export class LAZrgb12v1Reader implements ItemReader {
  lastItem = new LASrgba();
  mByteUsed: ArithmeticModel;
  icRgb: IntegerCompressor;
  /** @param dec - the arithmetic decoder */
  constructor(readonly dec: ArithmeticDecoder) {
    /* create models and integer compressors */
    this.mByteUsed = new ArithmeticModel(64, false);
    this.icRgb = new IntegerCompressor(dec, 8, 6);
  }

  /**
   * Read in chunk sizes
   * @param _reader - the full data store
   */
  chunkSizes(_reader: Reader): void {}

  /** @param item - the first raw item needs to be injected for future reads */
  init(item: DataView): void {
    /* init models and integer compressors */
    this.mByteUsed.init();
    this.icRgb.initDecompressor();

    this.lastItem.copyFrom(item, 6);
  }

  /** @param item - the current item to be read into */
  read(item: DataView): void {
    const currItem = new LASrgba(item);
    const sym = this.dec.decodeSymbol(this.mByteUsed);
    if ((sym & (1 << 0)) !== 0)
      currItem.r = this.icRgb.decompress(this.lastItem.r & 255, { value: 0 });
    else currItem.r = this.lastItem.r & 0xff;
    if ((sym & (1 << 1)) !== 0)
      currItem.r |= this.icRgb.decompress(this.lastItem.r >> 8, { value: 1 }) << 8;
    else currItem.r |= this.lastItem.r & 0xff00;
    if ((sym & (1 << 2)) !== 0)
      currItem.g = this.icRgb.decompress(this.lastItem.g & 255, { value: 2 });
    else currItem.g = this.lastItem.g & 0xff;
    if ((sym & (1 << 3)) !== 0)
      currItem.g |= this.icRgb.decompress(this.lastItem.g >> 8, { value: 3 }) << 8;
    else currItem.g |= this.lastItem.g & 0xff00;
    if ((sym & (1 << 4)) !== 0)
      currItem.b = this.icRgb.decompress(this.lastItem.b & 255, { value: 4 });
    else currItem.b = this.lastItem.b & 0xff;
    if ((sym & (1 << 5)) !== 0)
      currItem.b |= this.icRgb.decompress(this.lastItem.b >> 8, { value: 5 }) << 8;
    else currItem.b |= this.lastItem.b & 0xff00;

    currItem.copyTo(this.lastItem.data, 6);
  }
}

/** Parse LAZ wavepacket 1.3v1 */
export class LAZwavepacket13v1Reader implements ItemReader {
  lastItem = new LASWavePacket13();
  mPacketIndex: ArithmeticModel;
  mOffsetDiff: [ArithmeticModel, ArithmeticModel, ArithmeticModel, ArithmeticModel];
  icOffsetDiff: IntegerCompressor;
  icPacketSize: IntegerCompressor;
  icReturnPoint: IntegerCompressor;
  icXyz: IntegerCompressor;
  lastDiff32 = 0;
  symLastOffsetDiff = 0;

  /**  @param dec - the arithmetic decoder */
  constructor(readonly dec: ArithmeticDecoder) {
    /* create models and integer compressors */
    this.mPacketIndex = new ArithmeticModel(256, false);
    this.mOffsetDiff = [
      new ArithmeticModel(4, false),
      new ArithmeticModel(4, false),
      new ArithmeticModel(4, false),
      new ArithmeticModel(4, false),
    ];
    this.icOffsetDiff = new IntegerCompressor(dec, 32);
    this.icPacketSize = new IntegerCompressor(dec, 32);
    this.icReturnPoint = new IntegerCompressor(dec, 32);
    this.icXyz = new IntegerCompressor(dec, 32, 3);
  }

  /**
   * Read in chunk sizes
   * @param _reader - the full data store
   */
  chunkSizes(_reader: Reader): void {}

  /** @param item - the first raw item needs to be injected for future reads */
  init(item: DataView): void {
    /* init state */
    this.lastDiff32 = 0;
    this.symLastOffsetDiff = 0;

    /* init models and integer compressors */
    this.mPacketIndex.init();
    for (const m of this.mOffsetDiff) m.init();
    this.icOffsetDiff.initDecompressor();
    this.icPacketSize.initDecompressor();
    this.icReturnPoint.initDecompressor();
    this.icXyz.initDecompressor();

    this.lastItem.copyFrom(item, 28);
  }

  /** @param item - the current item to be read into */
  read(item: DataView): void {
    const thisItemM = new LASWavePacket13(item);
    thisItemM.index = this.dec.decodeSymbol(this.mPacketIndex);

    this.symLastOffsetDiff = this.dec.decodeSymbol(this.mOffsetDiff[this.symLastOffsetDiff]);

    if (this.symLastOffsetDiff === 0) {
      thisItemM.offset = this.lastItem.offset;
    } else if (this.symLastOffsetDiff === 1) {
      thisItemM.offset = this.lastItem.offset + this.lastItem.packetSize;
    } else if (this.symLastOffsetDiff === 2) {
      this.lastDiff32 = this.icOffsetDiff.decompress(this.lastDiff32);
      thisItemM.offset = this.lastItem.offset + this.lastDiff32;
    } else {
      thisItemM.offset = this.dec.readInt64();
    }

    thisItemM.packetSize = this.icPacketSize.decompress(this.lastItem.packetSize);
    thisItemM.returnPoint = this.icReturnPoint.decompress(this.lastItem.returnPoint);
    thisItemM.x = this.icXyz.decompress(this.lastItem.x, { value: 0 });
    thisItemM.y = this.icXyz.decompress(this.lastItem.y, { value: 1 });
    thisItemM.z = this.icXyz.decompress(this.lastItem.z, { value: 2 });

    thisItemM.copyTo(this.lastItem.data, 29);
  }
}

/** Parse LAZ byte 1.0v1 */
export class LAZbyte10v1Reader implements ItemReader {
  lastItem: Uint8Array;
  icByte: IntegerCompressor;
  /**
   * @param dec - the arithmetic decoder
   * @param number - the number of bytes to read at a time
   */
  constructor(
    readonly dec: ArithmeticDecoder,
    readonly number: number,
  ) {
    this.icByte = new IntegerCompressor(dec, 8, number);
    this.lastItem = new Uint8Array(number);
  }

  /**
   * Read in chunk sizes
   * @param _reader - the full data store
   */
  chunkSizes(_reader: Reader): void {}

  /** @param item - the first raw item needs to be injected for future reads */
  init(item: DataView): void {
    /* init models and integer compressors */
    this.icByte.initDecompressor();

    const itemUint8 = new Uint8Array(item.buffer, item.byteOffset, this.number);
    this.lastItem.set(itemUint8);
  }

  /** @param item - the current item to be read into */
  read(item: DataView): void {
    const thisItem = new Uint8Array(item.buffer, item.byteOffset, this.number);
    for (let i = 0; i < this.number; i++) {
      thisItem[i] = this.icByte.decompress(this.lastItem[i], { value: i });
    }
    this.lastItem.set(thisItem);
  }
}
