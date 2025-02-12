import { BufferReader } from '../..';
import {
  ArithmeticDecoder,
  ArithmeticModel,
  IntegerCompressor,
  LASWavePacket13,
  LASZIP_DECOMPRESS_SELECTIVE_ALL,
  LASZIP_DECOMPRESS_SELECTIVE_BYTE0,
  LASZIP_DECOMPRESS_SELECTIVE_CLASSIFICATION,
  LASZIP_DECOMPRESS_SELECTIVE_FLAGS,
  LASZIP_DECOMPRESS_SELECTIVE_GPS_TIME,
  LASZIP_DECOMPRESS_SELECTIVE_INTENSITY,
  LASZIP_DECOMPRESS_SELECTIVE_NIR,
  LASZIP_DECOMPRESS_SELECTIVE_POINT_SOURCE,
  LASZIP_DECOMPRESS_SELECTIVE_RGB,
  LASZIP_DECOMPRESS_SELECTIVE_SCAN_ANGLE,
  LASZIP_DECOMPRESS_SELECTIVE_USER_DATA,
  LASZIP_DECOMPRESS_SELECTIVE_WAVEPACKET,
  LASZIP_DECOMPRESS_SELECTIVE_Z,
  LASpoint14,
  LASrgbaNir,
  number_return_level_8ctx,
  number_return_map_6ctx,
} from '.';
import {
  LAScontextBYTE14,
  LAScontextPOINT14,
  LAScontextRGB14,
  LAScontextRGBNIR14,
  LAScontextWAVEPACKET14,
} from './v3';
import { i16Quantize, i8Clamp, u32ZeroBit0, u8Clamp, u8Fold } from '../util';

import type { Reader } from '../..';
import { ItemReader, LASrgba, LAZContext } from '.';

const LASZIP_GPSTIME_MULTI = 500;
const LASZIP_GPSTIME_MULTI_MINUS = -10;
const LASZIP_GPSTIME_MULTI_CODE_FULL = LASZIP_GPSTIME_MULTI - LASZIP_GPSTIME_MULTI_MINUS + 1;

const LASZIP_GPSTIME_MULTI_TOTAL = LASZIP_GPSTIME_MULTI - LASZIP_GPSTIME_MULTI_MINUS + 5;

/** Parse LAZ Point 1.4v4 */
export class LAZPoint14v4Reader implements ItemReader {
  // readers
  instreamChannelReturnsXY?: Reader;
  instreamZ?: Reader;
  instreamClassification?: Reader;
  instreamFlags?: Reader;
  instreamIntensity?: Reader;
  instreamScanAngle?: Reader;
  instreamUserData?: Reader;
  instreamPointSource?: Reader;
  instreamGpsTime?: Reader;
  // decoders
  decChannelReturnsXY?: ArithmeticDecoder;
  decZ?: ArithmeticDecoder;
  decClassification?: ArithmeticDecoder;
  decFlags?: ArithmeticDecoder;
  decIntensity?: ArithmeticDecoder;
  decScanAngle?: ArithmeticDecoder;
  decUserData?: ArithmeticDecoder;
  decPointSource?: ArithmeticDecoder;
  decGpsTime?: ArithmeticDecoder;
  // states
  changedZ = false;
  changedClassification = false;
  changedFlags = false;
  changedIntensity = false;
  changedScanAngle = false;
  changedUserData = false;
  changedPointSource = false;
  changedGpsTime = false;
  // num bytes values
  numBytesChannelReturnsXY = 0; // U32
  numBytesZ = 0; // U32
  numBytesClassification = 0; // U32
  numBytesFlags = 0; // U32
  numBytesIntensity = 0; // U32
  numBytesScanAngle = 0; // U32
  numBytesUserData = 0; // U32
  numBytesPointSource = 0; // U32
  numBytesGpsTime = 0; // U32
  // request states
  requestedZ = false;
  requestedClassification = false;
  requestedFlags = false;
  requestedIntensity = false;
  requestedScanAngle = false;
  requestedUserData = false;
  requestedPointSource = false;
  requestedGpsTime = false;

  bytes?: DataView;
  numBytesAllocated = 0; // U32

  currentContext = 0; // U32
  contexts = [
    new LAScontextPOINT14(),
    new LAScontextPOINT14(),
    new LAScontextPOINT14(),
    new LAScontextPOINT14(),
  ];
  /**
   * @param dec - the arithmetic decoder
   * @param decompressSelective - which fields to decompress
   */
  constructor(
    readonly dec: ArithmeticDecoder,
    readonly decompressSelective = LASZIP_DECOMPRESS_SELECTIVE_ALL,
  ) {
    for (let c = 0; c < 4; c++) this.contexts[c].mChangedValues[0] = undefined;
    this.requestedZ = Boolean(decompressSelective & LASZIP_DECOMPRESS_SELECTIVE_Z);
    this.requestedClassification = Boolean(
      decompressSelective & LASZIP_DECOMPRESS_SELECTIVE_CLASSIFICATION,
    );
    this.requestedFlags = Boolean(decompressSelective & LASZIP_DECOMPRESS_SELECTIVE_FLAGS);
    this.requestedIntensity = Boolean(decompressSelective & LASZIP_DECOMPRESS_SELECTIVE_INTENSITY);
    this.requestedScanAngle = Boolean(decompressSelective & LASZIP_DECOMPRESS_SELECTIVE_SCAN_ANGLE);
    this.requestedUserData = Boolean(decompressSelective & LASZIP_DECOMPRESS_SELECTIVE_USER_DATA);
    this.requestedPointSource = Boolean(
      decompressSelective & LASZIP_DECOMPRESS_SELECTIVE_POINT_SOURCE,
    );
    this.requestedGpsTime = Boolean(decompressSelective & LASZIP_DECOMPRESS_SELECTIVE_GPS_TIME);
  }

  /**
   * Read in chunk sizes
   * @param reader - the full data store
   */
  chunkSizes(reader: Reader): void {
    this.numBytesChannelReturnsXY = reader.getUint32(undefined, true);
    this.numBytesZ = reader.getUint32(undefined, true);
    this.numBytesClassification = reader.getUint32(undefined, true);
    this.numBytesFlags = reader.getUint32(undefined, true);
    this.numBytesIntensity = reader.getUint32(undefined, true);
    this.numBytesScanAngle = reader.getUint32(undefined, true);
    this.numBytesUserData = reader.getUint32(undefined, true);
    this.numBytesPointSource = reader.getUint32(undefined, true);
    this.numBytesGpsTime = reader.getUint32(undefined, true);
  }

  /**
   * @param item - the current item to be read into
   * @param context - the current context
   */
  init(item: DataView, context: LAZContext): void {
    const { reader } = this.dec;
    /* on the first init create instreams and decoders */
    if (this.instreamChannelReturnsXY === undefined) {
      /* create decoders */
      this.decChannelReturnsXY = new ArithmeticDecoder(reader);
      this.decZ = new ArithmeticDecoder(reader);
      this.decClassification = new ArithmeticDecoder(reader);
      this.decFlags = new ArithmeticDecoder(reader);
      this.decIntensity = new ArithmeticDecoder(reader);
      this.decScanAngle = new ArithmeticDecoder(reader);
      this.decUserData = new ArithmeticDecoder(reader);
      this.decPointSource = new ArithmeticDecoder(reader);
      this.decGpsTime = new ArithmeticDecoder(reader);
    }
    /* how many bytes do we need to read */
    let numBytes = this.numBytesChannelReturnsXY; // U32
    if (this.requestedZ) numBytes += this.numBytesZ;
    if (this.requestedClassification) numBytes += this.numBytesClassification;
    if (this.requestedFlags) numBytes += this.numBytesFlags;
    if (this.requestedIntensity) numBytes += this.numBytesIntensity;
    if (this.requestedScanAngle) numBytes += this.numBytesScanAngle;
    if (this.requestedUserData) numBytes += this.numBytesUserData;
    if (this.requestedPointSource) numBytes += this.numBytesPointSource;
    if (this.requestedGpsTime) numBytes += this.numBytesGpsTime;
    /* make sure the buffer is sufficiently large */
    if (numBytes > this.numBytesAllocated) this.numBytesAllocated = numBytes;
    /* load the requested bytes and init the corresponding instreams and decoders */
    numBytes = 0;
    this.bytes = reader.seekSlice(this.numBytesChannelReturnsXY);
    this.instreamChannelReturnsXY = new BufferReader(this.bytes.buffer);
    this.decChannelReturnsXY = new ArithmeticDecoder(this.instreamChannelReturnsXY);
    this.decChannelReturnsXY.init();
    numBytes += this.numBytesChannelReturnsXY;
    if (this.requestedZ) {
      if (this.numBytesZ !== 0) {
        this.bytes = reader.seekSlice(this.numBytesZ);
        this.instreamZ = new BufferReader(this.bytes.buffer);
        this.decZ = new ArithmeticDecoder(this.instreamZ);
        this.decZ.init();
        numBytes += this.numBytesZ;
        this.changedZ = true;
      } else {
        this.instreamZ = undefined;
        this.changedZ = false;
      }
    } else {
      if (this.numBytesZ !== 0) {
        reader.seek(reader.tell() + this.numBytesZ);
      }
      this.changedZ = false;
    }
    if (this.requestedClassification) {
      if (this.numBytesClassification !== 0) {
        this.bytes = reader.seekSlice(this.numBytesClassification);
        this.instreamClassification = new BufferReader(this.bytes.buffer);
        this.decClassification = new ArithmeticDecoder(this.instreamClassification);
        this.decClassification.init();
        numBytes += this.numBytesClassification;
        this.changedClassification = true;
      } else {
        this.instreamClassification = undefined;
        this.changedClassification = false;
      }
    } else {
      if (this.numBytesClassification !== 0) {
        reader.seek(reader.tell() + this.numBytesClassification);
      }
      this.changedClassification = false;
    }
    if (this.requestedFlags) {
      if (this.numBytesFlags !== 0) {
        this.bytes = reader.seekSlice(this.numBytesFlags);
        this.instreamFlags = new BufferReader(this.bytes.buffer);
        this.decFlags = new ArithmeticDecoder(this.instreamFlags);
        this.decFlags.init();
        numBytes += this.numBytesFlags;
        this.changedFlags = true;
      } else {
        this.instreamFlags = undefined;
        this.changedFlags = false;
      }
    } else {
      if (this.numBytesFlags !== 0) {
        reader.seek(reader.tell() + this.numBytesFlags);
      }
      this.changedFlags = false;
    }
    if (this.requestedIntensity) {
      if (this.numBytesIntensity !== 0) {
        this.bytes = reader.seekSlice(this.numBytesIntensity);
        this.instreamIntensity = new BufferReader(this.bytes.buffer);
        this.decIntensity = new ArithmeticDecoder(this.instreamIntensity);
        this.decIntensity.init();
        numBytes += this.numBytesIntensity;
        this.changedIntensity = true;
      } else {
        this.instreamIntensity = undefined;
        this.changedIntensity = false;
      }
    } else {
      if (this.numBytesIntensity !== 0) {
        reader.seek(reader.tell() + this.numBytesIntensity);
      }
      this.changedIntensity = false;
    }
    if (this.requestedScanAngle) {
      if (this.numBytesScanAngle !== 0) {
        this.bytes = reader.seekSlice(this.numBytesScanAngle);
        this.instreamScanAngle = new BufferReader(this.bytes.buffer);
        this.decScanAngle = new ArithmeticDecoder(this.instreamScanAngle);
        this.decScanAngle.init();
        numBytes += this.numBytesScanAngle;
        this.changedScanAngle = true;
      } else {
        this.instreamScanAngle = undefined;
        this.changedScanAngle = false;
      }
    } else {
      if (this.numBytesScanAngle !== 0) {
        reader.seek(reader.tell() + this.numBytesScanAngle);
      }
      this.changedScanAngle = false;
    }
    if (this.requestedUserData) {
      if (this.numBytesUserData !== 0) {
        this.bytes = reader.seekSlice(this.numBytesUserData);
        this.instreamUserData = new BufferReader(this.bytes.buffer);
        this.decUserData = new ArithmeticDecoder(this.instreamUserData);
        this.decUserData.init();
        numBytes += this.numBytesUserData;
        this.changedUserData = true;
      } else {
        this.instreamUserData = undefined;
        this.changedUserData = false;
      }
    } else {
      if (this.numBytesUserData !== 0) {
        reader.seek(reader.tell() + this.numBytesUserData);
      }
      this.changedUserData = false;
    }
    if (this.requestedPointSource) {
      if (this.numBytesPointSource !== 0) {
        this.bytes = reader.seekSlice(this.numBytesPointSource);
        this.instreamPointSource = new BufferReader(this.bytes.buffer);
        this.decPointSource = new ArithmeticDecoder(this.instreamPointSource);
        this.decPointSource.init();
        numBytes += this.numBytesPointSource;
        this.changedPointSource = true;
      } else {
        this.instreamPointSource = undefined;
        this.changedPointSource = false;
      }
    } else {
      if (this.numBytesPointSource !== 0) {
        reader.seek(reader.tell() + this.numBytesPointSource);
      }
      this.changedPointSource = false;
    }
    if (this.requestedGpsTime) {
      if (this.numBytesGpsTime !== 0) {
        this.bytes = reader.seekSlice(this.numBytesGpsTime);
        this.instreamGpsTime = new BufferReader(this.bytes.buffer);
        this.decGpsTime = new ArithmeticDecoder(this.instreamGpsTime);
        this.decGpsTime.init();
        numBytes += this.numBytesGpsTime;
        this.changedGpsTime = true;
      } else {
        this.instreamGpsTime = undefined;
        this.changedGpsTime = false;
      }
    } else {
      if (this.numBytesGpsTime !== 0) {
        reader.seek(reader.tell() + this.numBytesGpsTime);
      }
      this.changedGpsTime = false;
    }
    /* mark the four scanner channel contexts as unused */
    for (let c = 0; c < 4; c++) {
      this.contexts[c].unused = true;
    }
    /* set scanner channel as current context */
    const currItem = new LASpoint14(item);
    this.currentContext = currItem.scannerChannel;
    context.value = this.currentContext; // the POINT14 reader sets context for all other items
    /* create and init models and decompressors */
    this.#createAndInitModelsAndDecompressors(this.currentContext, currItem);
  }

  /**
   * @param item - the current item to be read into
   * @param context - the current context
   */
  read(item: DataView, context: LAZContext): void {
    const { contexts } = this;
    // get last
    let lastItem = new LASpoint14(contexts[this.currentContext].lastItem);
    ////////////////////////////////////////
    // decompress returnsXY layer
    ////////////////////////////////////////
    // create single (3) / first (1) / last (2) / intermediate (0) context from last point return
    let lpr = lastItem.returnNumber === 1 ? 1 : 0; // (I32) first?
    lpr += lastItem.returnNumber >= lastItem.numberOfReturns ? 2 : 0; // last?
    // add info whether the GPS time changed in the last return to the context
    lpr += lastItem.gpsTimeChange !== 0 ? 4 : 0;
    // decompress which values have changed with last point return context (I32)
    const changedValues = this.decChannelReturnsXY!.decodeSymbol(
      contexts[this.currentContext].mChangedValues[lpr]!,
    );
    // if scanner channel has changed
    if ((changedValues & (1 << 6)) !== 0) {
      const diff = this.decChannelReturnsXY!.decodeSymbol(
        contexts[this.currentContext].mScannerChannel!,
      ); // (U32) curr = last + (sym + 1)
      const scannerChannel = (this.currentContext + diff + 1) % 4; // U32
      // maybe create and init entropy models and integer compressors
      if (contexts[scannerChannel].unused) {
        // create and init entropy models and integer decompressors
        this.#createAndInitModelsAndDecompressors(
          scannerChannel,
          new LASpoint14(contexts[this.currentContext].lastItem),
        );
      }
      // switch context to current scanner channel
      this.currentContext = scannerChannel;
      // get last for new context
      lastItem = new LASpoint14(contexts[this.currentContext].lastItem);
      lastItem.scannerChannel = scannerChannel;
    }
    context.value = this.currentContext; // the POINT14 reader sets context for all other items
    // determine changed attributes
    const pointSourceChange = Boolean(changedValues & (1 << 5));
    const gpsTimeChange = Boolean(changedValues & (1 << 4));
    const scanAngleChange = Boolean(changedValues & (1 << 3));
    // get last return counts
    const lastN = lastItem.numberOfReturns; // U32
    const lastR = lastItem.returnNumber; // U32
    // if number of returns is different we decompress it
    let n: number;
    if ((changedValues & (1 << 2)) !== 0) {
      if (contexts[this.currentContext].mNumberOfReturns[lastN] === undefined) {
        contexts[this.currentContext].mNumberOfReturns[lastN] = new ArithmeticModel(16);
        contexts[this.currentContext].mNumberOfReturns[lastN]!.init();
      }
      n = this.decChannelReturnsXY!.decodeSymbol(
        contexts[this.currentContext].mNumberOfReturns[lastN]!,
      );
      lastItem.numberOfReturns = n;
    } else {
      n = lastN;
    }
    // how is the return number different
    let r: number;
    if ((changedValues & 3) === 0) {
      // same return number
      r = lastR;
    } else if ((changedValues & 3) === 1) {
      // return number plus 1 mod 16
      r = (lastR + 1) % 16;
      lastItem.returnNumber = r;
    } else if ((changedValues & 3) === 2) {
      // return number minus 1 mod 16
      r = (lastR + 15) % 16;
      lastItem.returnNumber = r;
    } else {
      // the return number difference is bigger than +1 / -1 so we decompress how it is different
      if (gpsTimeChange) {
        // if the GPS time has changed
        if (contexts[this.currentContext].mReturnNumber[lastR] === undefined) {
          contexts[this.currentContext].mReturnNumber[lastR] = new ArithmeticModel(16);
          contexts[this.currentContext].mReturnNumber[lastR]!.init();
        }
        r = this.decChannelReturnsXY!.decodeSymbol(
          contexts[this.currentContext].mReturnNumber[lastR]!,
        );
      } // if the GPS time has not changed
      else {
        // I32
        const sym = this.decChannelReturnsXY!.decodeSymbol(
          contexts[this.currentContext].mReturnNumberGpsSame!,
        );
        r = (lastR + (sym + 2)) % 16;
      }
      lastItem.returnNumber = r;
    }
    // set legacy return counts and number of returns
    if (n > 7) {
      if (r > 6) {
        if (r >= n) {
          lastItem.legacyReturnNumber = 7;
        } else {
          lastItem.legacyReturnNumber = 6;
        }
      } else {
        lastItem.legacyReturnNumber = r;
      }
      lastItem.legacyNumberOfReturns = 7;
    } else {
      lastItem.legacyReturnNumber = r;
      lastItem.legacyNumberOfReturns = n;
    }
    // get return map m and return level l context for current point
    const m = number_return_map_6ctx[n][r]; // U32
    const l = number_return_level_8ctx[n][r]; // U32
    // create single (3) / first (1) / last (2) / intermediate (0) return context for current point
    let cpr = r === 1 ? 2 : 0; // first ? (I32)
    cpr += r >= n ? 1 : 0; // last ?
    let kBits: number; // U32
    let median, diff; // I32
    // decompress X coordinate
    median =
      contexts[this.currentContext].lastXDiffMedian5[(m << 1) | Number(gpsTimeChange)]!.get();
    diff = contexts[this.currentContext].icdX!.decompress(median, { value: Number(n === 1) });
    lastItem.x += diff;
    contexts[this.currentContext].lastXDiffMedian5[(m << 1) | Number(gpsTimeChange)]!.add(diff);
    // decompress Y coordinate
    median =
      contexts[this.currentContext].lastYDiffMedian5[(m << 1) | Number(gpsTimeChange)]!.get();
    kBits = contexts[this.currentContext].icdX!.getK();
    diff = contexts[this.currentContext].icdY!.decompress(median, {
      value: Number(n === 1) + (kBits < 20 ? u32ZeroBit0(kBits) : 20),
    });
    lastItem.y += diff;
    contexts[this.currentContext].lastYDiffMedian5[(m << 1) | Number(gpsTimeChange)]!.add(diff);
    ////////////////////////////////////////
    // decompress Z layer (if changed and requested)
    ////////////////////////////////////////
    if (this.changedZ) {
      // if the Z coordinate should be decompressed and changes within this chunk
      kBits =
        (contexts[this.currentContext].icdX!.getK() + contexts[this.currentContext].icdY!.getK()) /
        2;
      lastItem.z = contexts[this.currentContext].icZ!.decompress(
        contexts[this.currentContext].lastZ[l],
        { value: Number(n === 1) + (kBits < 18 ? u32ZeroBit0(kBits) : 18) },
      );
      contexts[this.currentContext].lastZ[l] = lastItem.z;
    }
    ////////////////////////////////////////
    // decompress classifications layer (if changed and requested)
    ////////////////////////////////////////
    if (this.changedClassification) {
      // if the classification should be decompressed and changes within this chunk
      const last_classification = lastItem.classification; // U32
      const ccc = ((last_classification & 0x1f) << 1) + (cpr === 3 ? 1 : 0); // I32
      if (contexts[this.currentContext].mClassification[ccc] === undefined) {
        contexts[this.currentContext].mClassification[ccc] = new ArithmeticModel(256);
        contexts[this.currentContext].mClassification[ccc]!.init();
      }
      lastItem.classification = this.decClassification!.decodeSymbol(
        contexts[this.currentContext].mClassification[ccc]!,
      );
      // update the legacy copy
      if (lastItem.classification < 32) {
        lastItem.legacyClassification = lastItem.classification;
      } else {
        lastItem.legacyClassification = 0;
      }
    }
    ////////////////////////////////////////
    // decompress flags layer (if changed and requested)
    ////////////////////////////////////////
    if (this.changedFlags) {
      // if the flags should be decompressed and change within this chunk
      // U32
      const lastFlags =
        (lastItem.edgeOfFlightLine << 5) |
        (lastItem.scanDirectionFlag << 4) |
        lastItem.classificationFlags;
      if (contexts[this.currentContext].mFlags[lastFlags] === undefined) {
        contexts[this.currentContext].mFlags[lastFlags] = new ArithmeticModel(64);
        contexts[this.currentContext].mFlags[lastFlags]!.init();
      }
      const flags = this.decFlags!.decodeSymbol(contexts[this.currentContext].mFlags[lastFlags]!);
      lastItem.edgeOfFlightLine = (flags & (1 << 5)) !== 0 ? 1 : 0;
      lastItem.scanDirectionFlag = (flags & (1 << 4)) !== 0 ? 1 : 0;
      lastItem.classificationFlags = flags & 0x0f;
      // legacy copies
      lastItem.legacyFlags = flags & 0x07;
    }
    ////////////////////////////////////////
    // decompress intensity layer (if changed and requested)
    ////////////////////////////////////////
    if (this.changedIntensity) {
      // if the intensity should be decompressed and changes within this chunk
      // U16
      const intensity = contexts[this.currentContext].icIntensity!.decompress(
        contexts[this.currentContext].lastIntensity[(cpr << 1) | Number(gpsTimeChange)],
        { value: cpr },
      );
      contexts[this.currentContext].lastIntensity[(cpr << 1) | Number(gpsTimeChange)] = intensity;
      lastItem.intensity = intensity;
    }
    ////////////////////////////////////////
    // decompress scanAngle layer (if changed and requested)
    ////////////////////////////////////////
    if (this.changedScanAngle) {
      // if the scan angle should be decompressed and changes within this chunk
      if (scanAngleChange) {
        // if the scan angle has actually changed
        lastItem.scanAngle = contexts[this.currentContext].icScanAngle!.decompress(
          lastItem.scanAngle,
          { value: Number(gpsTimeChange) },
        ); // if the GPS time has changed
        lastItem.legacyScanAngleRank = i8Clamp(i16Quantize(0.006 * lastItem.scanAngle));
      }
    }
    ////////////////////////////////////////
    // decompress userData layer (if changed and requested)
    ////////////////////////////////////////
    if (this.changedUserData) {
      // if the user data should be decompressed and changes within this chunk
      const index = Math.trunc(lastItem.userData / 4);
      if (contexts[this.currentContext].mUserData![index] === undefined) {
        contexts[this.currentContext].mUserData[index] = new ArithmeticModel(256);
        contexts[this.currentContext].mUserData[index]!.init();
      }
      lastItem.userData = this.decUserData!.decodeSymbol(
        contexts[this.currentContext].mUserData[index]!,
      );
    }
    ////////////////////////////////////////
    // decompress point_source layer (if changed and requested)
    ////////////////////////////////////////
    if (this.changedPointSource) {
      // if the point source ID should be decompressed and changes within this chunk
      if (pointSourceChange) {
        // if the point source ID has actually changed
        lastItem.pointSourceID = contexts[this.currentContext].icPointSourceID!.decompress(
          lastItem.pointSourceID,
        );
      }
    }
    ////////////////////////////////////////
    // decompress gpsTime layer (if changed and requested)
    ////////////////////////////////////////
    if (this.changedGpsTime) {
      // if the GPS time should be decompressed and changes within this chunk
      if (gpsTimeChange) {
        // if the GPS time has actually changed
        this.#readGpsTime();
        lastItem.gpsTime =
          contexts[this.currentContext].lastGpstime[contexts[this.currentContext].last].f64;
      }
    }
    // copy the last item
    lastItem.copyTo(item, 45);
    // remember if the last point had a gpsTimeChange
    lastItem.gpsTimeChange = Number(gpsTimeChange);
  }

  /** Read the GPS Time */
  #readGpsTime(): void {
    const { contexts } = this;
    let multi: number; // I32
    if (contexts[this.currentContext].lastGpstimeDiff[contexts[this.currentContext].last] === 0) {
      // if the last integer difference was zero
      multi = this.decGpsTime!.decodeSymbol(contexts[this.currentContext].mGpstime0diff!);
      if (multi === 0) {
        // the difference can be represented with 32 bits
        contexts[this.currentContext].lastGpstimeDiff[contexts[this.currentContext].last] =
          contexts[this.currentContext].icGpstime!.decompress(0, { value: 0 });
        contexts[this.currentContext].lastGpstime[contexts[this.currentContext].last].i64 += BigInt(
          contexts[this.currentContext].lastGpstimeDiff[contexts[this.currentContext].last],
        );
        contexts[this.currentContext].multiExtremeCounter[contexts[this.currentContext].last] = 0;
      } else if (multi === 1) {
        // the difference is huge
        contexts[this.currentContext].next = (contexts[this.currentContext].next + 1) & 3;
        contexts[this.currentContext].lastGpstime[contexts[this.currentContext].next].u64 =
          contexts[this.currentContext].icGpstime!.decompress(
            Number(
              contexts[this.currentContext].lastGpstime[contexts[this.currentContext].last].u64 >>
                32n,
            ),
            { value: 8 },
          );
        contexts[this.currentContext].lastGpstime[contexts[this.currentContext].next].u64 =
          contexts[this.currentContext].lastGpstime[contexts[this.currentContext].next].u64 << 32n;
        contexts[this.currentContext].lastGpstime[contexts[this.currentContext].next].u64 |= BigInt(
          this.decGpsTime!.readInt(),
        );
        contexts[this.currentContext].last = contexts[this.currentContext].next;
        contexts[this.currentContext].lastGpstimeDiff[contexts[this.currentContext].last] = 0;
        contexts[this.currentContext].multiExtremeCounter[contexts[this.currentContext].last] = 0;
      } // we switch to another sequence
      else {
        contexts[this.currentContext].last = (contexts[this.currentContext].last + multi - 1) & 3;
        this.#readGpsTime();
      }
    } else {
      multi = this.decGpsTime!.decodeSymbol(contexts[this.currentContext].mGpstimeMulti!);
      if (multi === 1) {
        contexts[this.currentContext].lastGpstime[contexts[this.currentContext].last].i64 += BigInt(
          contexts[this.currentContext].icGpstime!.decompress(
            contexts[this.currentContext].lastGpstimeDiff[contexts[this.currentContext].last],
            { value: 1 },
          ),
        );
        contexts[this.currentContext].multiExtremeCounter[contexts[this.currentContext].last] = 0;
      } else if (multi < LASZIP_GPSTIME_MULTI_CODE_FULL) {
        let gpstimeDiff: number; // I32
        if (multi === 0) {
          gpstimeDiff = contexts[this.currentContext].icGpstime!.decompress(0, { value: 7 });
          contexts[this.currentContext].multiExtremeCounter[contexts[this.currentContext].last]++;
          if (
            contexts[this.currentContext].multiExtremeCounter[contexts[this.currentContext].last] >
            3
          ) {
            contexts[this.currentContext].lastGpstimeDiff[contexts[this.currentContext].last] =
              gpstimeDiff;
            contexts[this.currentContext].multiExtremeCounter[contexts[this.currentContext].last] =
              0;
          }
        } else if (multi < LASZIP_GPSTIME_MULTI) {
          if (multi < 10)
            gpstimeDiff = contexts[this.currentContext].icGpstime!.decompress(
              multi *
                contexts[this.currentContext].lastGpstimeDiff[contexts[this.currentContext].last],
              { value: 2 },
            );
          else
            gpstimeDiff = contexts[this.currentContext].icGpstime!.decompress(
              multi *
                contexts[this.currentContext].lastGpstimeDiff[contexts[this.currentContext].last],
              { value: 3 },
            );
        } else if (multi === LASZIP_GPSTIME_MULTI) {
          gpstimeDiff = contexts[this.currentContext].icGpstime!.decompress(
            LASZIP_GPSTIME_MULTI *
              contexts[this.currentContext].lastGpstimeDiff[contexts[this.currentContext].last],
            { value: 4 },
          );
          contexts[this.currentContext].multiExtremeCounter[contexts[this.currentContext].last]++;
          if (
            contexts[this.currentContext].multiExtremeCounter[contexts[this.currentContext].last] >
            3
          ) {
            contexts[this.currentContext].lastGpstimeDiff[contexts[this.currentContext].last] =
              gpstimeDiff;
            contexts[this.currentContext].multiExtremeCounter[contexts[this.currentContext].last] =
              0;
          }
        } else {
          multi = LASZIP_GPSTIME_MULTI - multi;
          if (multi > LASZIP_GPSTIME_MULTI_MINUS) {
            gpstimeDiff = contexts[this.currentContext].icGpstime!.decompress(
              multi *
                contexts[this.currentContext].lastGpstimeDiff[contexts[this.currentContext].last],
              { value: 5 },
            );
          } else {
            gpstimeDiff = contexts[this.currentContext].icGpstime!.decompress(
              LASZIP_GPSTIME_MULTI_MINUS *
                contexts[this.currentContext].lastGpstimeDiff[contexts[this.currentContext].last],
              { value: 6 },
            );
            contexts[this.currentContext].multiExtremeCounter[contexts[this.currentContext].last]++;
            if (
              contexts[this.currentContext].multiExtremeCounter[
                contexts[this.currentContext].last
              ] > 3
            ) {
              contexts[this.currentContext].lastGpstimeDiff[contexts[this.currentContext].last] =
                gpstimeDiff;
              contexts[this.currentContext].multiExtremeCounter[
                contexts[this.currentContext].last
              ] = 0;
            }
          }
        }
        contexts[this.currentContext].lastGpstime[contexts[this.currentContext].last].i64 +=
          BigInt(gpstimeDiff);
      } else if (multi === LASZIP_GPSTIME_MULTI_CODE_FULL) {
        contexts[this.currentContext].next = (contexts[this.currentContext].next + 1) & 3;
        contexts[this.currentContext].lastGpstime[contexts[this.currentContext].next].u64 =
          contexts[this.currentContext].icGpstime!.decompress(
            Number(
              contexts[this.currentContext].lastGpstime[contexts[this.currentContext].last].u64 >>
                32n,
            ),
            { value: 8 },
          );
        contexts[this.currentContext].lastGpstime[contexts[this.currentContext].next].u64 =
          contexts[this.currentContext].lastGpstime[contexts[this.currentContext].next].u64 << 32n;
        contexts[this.currentContext].lastGpstime[contexts[this.currentContext].next].u64 |= BigInt(
          this.decGpsTime!.readInt(),
        );
        contexts[this.currentContext].last = contexts[this.currentContext].next;
        contexts[this.currentContext].lastGpstimeDiff[contexts[this.currentContext].last] = 0;
        contexts[this.currentContext].multiExtremeCounter[contexts[this.currentContext].last] = 0;
      } else if (multi >= LASZIP_GPSTIME_MULTI_CODE_FULL) {
        contexts[this.currentContext].last =
          (contexts[this.currentContext].last + multi - LASZIP_GPSTIME_MULTI_CODE_FULL) & 3;
        this.#readGpsTime();
      }
    }
  }

  /**
   * @param context - the current context
   * @param item - the current item to be read from
   */
  #createAndInitModelsAndDecompressors(context: number, item: LASpoint14) {
    const { contexts } = this;
    let i: number; // I32
    /* first create all entropy models and integer decompressors (if needed) */
    if (contexts[context].mChangedValues[0] === undefined) {
      /* for the channelReturnsXY layer */
      contexts[context].mChangedValues[0] = new ArithmeticModel(128);
      contexts[context].mChangedValues[1] = new ArithmeticModel(128);
      contexts[context].mChangedValues[2] = new ArithmeticModel(128);
      contexts[context].mChangedValues[3] = new ArithmeticModel(128);
      contexts[context].mChangedValues[4] = new ArithmeticModel(128);
      contexts[context].mChangedValues[5] = new ArithmeticModel(128);
      contexts[context].mChangedValues[6] = new ArithmeticModel(128);
      contexts[context].mChangedValues[7] = new ArithmeticModel(128);
      contexts[context].mScannerChannel = new ArithmeticModel(3);
      for (i = 0; i < 16; i++) {
        contexts[context].mNumberOfReturns[i] = undefined;
        contexts[context].mReturnNumber[i] = undefined;
      }
      contexts[context].mReturnNumberGpsSame = new ArithmeticModel(13);
      contexts[context].icdX = new IntegerCompressor(this.decChannelReturnsXY!, 32, 2); // 32 bits, 2 context
      contexts[context].icdY = new IntegerCompressor(this.decChannelReturnsXY!, 32, 22); // 32 bits, 22 contexts
      /* for the Z layer */
      contexts[context].icZ = new IntegerCompressor(this.decZ!, 32, 20); // 32 bits, 20 contexts
      /* for the classification layer */
      /* for the flags layer */
      /* for the userData layer */
      for (i = 0; i < 64; i++) {
        contexts[context].mClassification[i] = undefined;
        contexts[context].mFlags[i] = undefined;
        contexts[context].mUserData[i] = undefined;
      }
      /* for the intensity layer */
      contexts[context].icIntensity = new IntegerCompressor(this.decIntensity!, 16, 4);
      /* for the scanAngle layer */
      contexts[context].icScanAngle = new IntegerCompressor(this.decScanAngle!, 16, 2);
      /* for the pointSourceID layer */
      contexts[context].icPointSourceID = new IntegerCompressor(this.decPointSource!, 16);
      /* for the gpsTime layer */
      contexts[context].mGpstimeMulti = new ArithmeticModel(LASZIP_GPSTIME_MULTI_TOTAL);
      contexts[context].mGpstime0diff = new ArithmeticModel(5);
      contexts[context].icGpstime = new IntegerCompressor(this.decGpsTime!, 32, 9); // 32 bits, 9 contexts
    }
    /* then init entropy models and integer compressors */
    /* for the channelReturnsXY layer */
    contexts[context].mChangedValues[0]!.init();
    contexts[context].mChangedValues[1]!.init();
    contexts[context].mChangedValues[2]!.init();
    contexts[context].mChangedValues[3]!.init();
    contexts[context].mChangedValues[4]!.init();
    contexts[context].mChangedValues[5]!.init();
    contexts[context].mChangedValues[6]!.init();
    contexts[context].mChangedValues[7]!.init();
    contexts[context].mScannerChannel!.init();
    for (i = 0; i < 16; i++) {
      if (contexts[context].mNumberOfReturns[i] !== undefined)
        contexts[context].mNumberOfReturns[i]!.init();
      if (contexts[context].mReturnNumber[i] !== undefined)
        contexts[context].mReturnNumber[i]!.init();
    }
    contexts[context].mReturnNumberGpsSame!.init();
    contexts[context].icdX!.initDecompressor();
    contexts[context].icdY!.initDecompressor();
    for (i = 0; i < 12; i++) {
      contexts[context].lastXDiffMedian5[i]!.init();
      contexts[context].lastYDiffMedian5[i]!.init();
    }
    /* for the Z layer */
    contexts[context].icZ!.initDecompressor();
    for (i = 0; i < 8; i++) {
      contexts[context].lastZ[i] = item.z;
    }
    /* for the classification layer, flags layer, and userData layer */
    for (i = 0; i < 64; i++) {
      if (contexts[context].mClassification[i] !== undefined)
        contexts[context].mClassification[i]!.init();
      if (contexts[context].mFlags[i] !== undefined) contexts[context].mFlags[i]!.init();
      if (contexts[context].mUserData[i] !== undefined) contexts[context].mUserData[i]!.init();
    }
    /* for the intensity layer */
    contexts[context].icIntensity!.initDecompressor();
    for (i = 0; i < 8; i++) {
      contexts[context].lastIntensity[i] = item.intensity;
    }
    /* for the scanAngle layer */
    contexts[context].icScanAngle!.initDecompressor();
    /* for the pointSourceID layer */
    contexts[context].icPointSourceID!.initDecompressor();
    /* for the gpsTime layer */
    contexts[context].mGpstimeMulti!.init();
    contexts[context].mGpstime0diff!.init();
    contexts[context].icGpstime!.initDecompressor();
    contexts[context].last = 0;
    contexts[context].next = 0;
    contexts[context].lastGpstimeDiff[0] = 0;
    contexts[context].lastGpstimeDiff[1] = 0;
    contexts[context].lastGpstimeDiff[2] = 0;
    contexts[context].lastGpstimeDiff[3] = 0;
    contexts[context].multiExtremeCounter[0] = 0;
    contexts[context].multiExtremeCounter[1] = 0;
    contexts[context].multiExtremeCounter[2] = 0;
    contexts[context].multiExtremeCounter[3] = 0;
    contexts[context].lastGpstime[0].f64 = item.gpsTime;
    contexts[context].lastGpstime[1].u64 = 0;
    contexts[context].lastGpstime[2].u64 = 0;
    contexts[context].lastGpstime[3].u64 = 0;
    /* init current context from last item */
    item.copyTo(contexts[context].lastItem, 45);
    new LASpoint14(contexts[context].lastItem).gpsTimeChange = 0;
    contexts[context].unused = false;
  }
}

/** Parse LAZ RGB 1.4v4 */
export class LAZrgb14v4Reader implements ItemReader {
  instreamRGB?: Reader;
  decRGB?: ArithmeticDecoder;
  changedRGB = false;
  numBytesRGB = 0; // U32
  requestedRGB: boolean;
  bytes?: DataView;
  numBytesAllocated = 0; // U32
  currentContext = 0; // U32
  contexts = [
    new LAScontextRGB14(),
    new LAScontextRGB14(),
    new LAScontextRGB14(),
    new LAScontextRGB14(),
  ];
  /**
   * @param dec - the arithmetic decoder
   * @param decompressSelective - which fields to decompress
   */
  constructor(
    readonly dec: ArithmeticDecoder,
    readonly decompressSelective = LASZIP_DECOMPRESS_SELECTIVE_ALL,
  ) {
    this.requestedRGB = Boolean(decompressSelective & LASZIP_DECOMPRESS_SELECTIVE_RGB);
    for (let c = 0; c < 4; c++) this.contexts[c].mByteUsed = undefined;
  }

  /**
   * Read in the chunk size
   * @param reader - the data block to read from
   */
  chunkSizes(reader: Reader): void {
    this.numBytesRGB = reader.getUint32(undefined, true);
  }

  /**
   * @param item - the current item
   * @param context - the current context
   */
  init(item: DataView, context: LAZContext): void {
    const { reader } = this.dec;
    /* on the first init create instreams and decoders */
    if (this.instreamRGB === undefined) {
      /* create decoders */
      this.decRGB = new ArithmeticDecoder(reader);
    }
    /* make sure the buffer is sufficiently large */
    if (this.numBytesRGB > this.numBytesAllocated) {
      this.numBytesAllocated = this.numBytesRGB;
    }
    /* load the requested bytes and init the corresponding instreams an decoders */
    if (this.requestedRGB) {
      if (this.numBytesRGB !== 0) {
        this.bytes = reader.seekSlice(this.numBytesRGB);
        this.instreamRGB = new BufferReader(this.bytes.buffer);
        this.decRGB = new ArithmeticDecoder(this.instreamRGB);
        this.decRGB.init();
        this.changedRGB = true;
      } else {
        this.instreamRGB = undefined;
        this.changedRGB = false;
      }
    } else {
      if (this.numBytesRGB !== 0) {
        reader.seek(reader.tell() + this.numBytesRGB);
      }
      this.changedRGB = false;
    }
    /* mark the four scanner channel contexts as unused */
    for (let c = 0; c < 4; c++) this.contexts[c].unused = true;
    /* set scanner channel as current context */
    this.currentContext = context.value; // all other items use context set by POINT14 reader
    /* create and init models and decompressors */
    this.#createAndInitModelsAndDecompressors(this.currentContext, new LASrgba(item));
  }

  /**
   * @param item - the current item
   * @param context - the current context
   */
  read(item: DataView, context: LAZContext): void {
    const { contexts } = this;
    const curItem = new LASrgba(item);
    let lastItem = new LASrgba(contexts[this.currentContext].lastItem);
    // check for context switch
    if (this.currentContext !== context.value) {
      this.currentContext = context.value; // all other items use context set by POINT14 reader
      if (contexts[this.currentContext].unused) {
        this.#createAndInitModelsAndDecompressors(this.currentContext, lastItem);
      }
      lastItem = new LASrgba(contexts[this.currentContext].lastItem);
    }

    // decompress
    if (this.changedRGB) {
      let corr: number; // U8
      let diff = 0; // I32
      const sym = this.decRGB!.decodeSymbol(contexts[this.currentContext].mByteUsed!); // U32
      if ((sym & (1 << 0)) !== 0) {
        corr = this.decRGB!.decodeSymbol(contexts[this.currentContext].mRgbDiff0!);
        curItem.r = u8Fold(corr + (lastItem.r & 255));
      } else {
        curItem.r = lastItem.r & 0xff;
      }
      if ((sym & (1 << 1)) !== 0) {
        corr = this.decRGB!.decodeSymbol(contexts[this.currentContext].mRgbDiff1!);
        curItem.r |= u8Fold(corr + (lastItem.r >> 8)) << 8;
      } else {
        curItem.r |= lastItem.r & 0xff00;
      }
      if ((sym & (1 << 6)) !== 0) {
        diff = (curItem.r & 0x00ff) - (lastItem.r & 0x00ff);
        if ((sym & (1 << 2)) !== 0) {
          corr = this.decRGB!.decodeSymbol(contexts[this.currentContext].mRgbDiff2!);
          curItem.g = u8Fold(corr + u8Clamp(diff + (lastItem.g & 255)));
        } else {
          curItem.g = lastItem.g & 0xff;
        }
        if ((sym & (1 << 4)) !== 0) {
          corr = this.decRGB!.decodeSymbol(contexts[this.currentContext].mRgbDiff4!);
          diff = Math.trunc((diff + ((curItem.g & 0x00ff) - (lastItem.g & 0x00ff))) / 2);
          curItem.b = u8Fold(corr + u8Clamp(diff + (lastItem.b & 255)));
        } else {
          curItem.b = lastItem.b & 0xff;
        }
        diff = (curItem.r >> 8) - (lastItem.r >> 8);
        if ((sym & (1 << 3)) !== 0) {
          corr = this.decRGB!.decodeSymbol(contexts[this.currentContext].mRgbDiff3!);
          curItem.g |= u8Fold(corr + u8Clamp(diff + (lastItem.g >> 8))) << 8;
        } else {
          curItem.g |= lastItem.g & 0xff00;
        }
        if ((sym & (1 << 5)) !== 0) {
          corr = this.decRGB!.decodeSymbol(contexts[this.currentContext].mRgbDiff5!);
          diff = Math.trunc((diff + ((curItem.g >> 8) - (lastItem.g >> 8))) / 2);
          curItem.b |= u8Fold(corr + u8Clamp(diff + (lastItem.b >> 8))) << 8;
        } else {
          curItem.b |= lastItem.b & 0xff00;
        }
      } else {
        curItem.g = curItem.r;
        curItem.b = curItem.r;
      }
      curItem.copyTo(lastItem.data, 6);
    } else {
      lastItem.copyTo(curItem.data, 6);
    }
  }

  /**
   * @param context - the current context
   * @param item - the current item to be read from
   */
  #createAndInitModelsAndDecompressors(context: number, item: LASrgba): void {
    const { contexts } = this;
    /* first create all entropy models (if needed) */
    if (contexts[context].mByteUsed === undefined) {
      contexts[context].mByteUsed = new ArithmeticModel(128);
      contexts[context].mRgbDiff0 = new ArithmeticModel(256);
      contexts[context].mRgbDiff1 = new ArithmeticModel(256);
      contexts[context].mRgbDiff2 = new ArithmeticModel(256);
      contexts[context].mRgbDiff3 = new ArithmeticModel(256);
      contexts[context].mRgbDiff4 = new ArithmeticModel(256);
      contexts[context].mRgbDiff5 = new ArithmeticModel(256);
    }
    /* then init entropy models */
    contexts[context].mByteUsed!.init();
    contexts[context].mRgbDiff0!.init();
    contexts[context].mRgbDiff1!.init();
    contexts[context].mRgbDiff2!.init();
    contexts[context].mRgbDiff3!.init();
    contexts[context].mRgbDiff4!.init();
    contexts[context].mRgbDiff5!.init();
    /* init current context from item */
    item.copyTo(contexts[context].lastItem, 6);
    contexts[context].unused = false;
  }
}

/** Parse LAZ RGB NIR 1.4v3 */
export class LAZrgbNir14v4Reader implements ItemReader {
  instreamRGB?: Reader;
  instreamNIR?: Reader;
  decRGB?: ArithmeticDecoder;
  decNIR?: ArithmeticDecoder;
  changedRGB = false;
  changedNIR = false;
  numBytesRGB = 0; // U32
  numBytesNIR = 0; // U32
  requestedRGB: boolean;
  requestedNIR: boolean;
  bytes?: DataView;
  numBytesAllocated = 0; // U32
  currentContext = 0; // U32
  contexts = [
    new LAScontextRGBNIR14(),
    new LAScontextRGBNIR14(),
    new LAScontextRGBNIR14(),
    new LAScontextRGBNIR14(),
  ];
  /**
   * @param dec - the arithmetic decoder
   * @param decompressSelective - the decompress selective
   */
  constructor(
    readonly dec: ArithmeticDecoder,
    readonly decompressSelective = LASZIP_DECOMPRESS_SELECTIVE_ALL,
  ) {
    this.requestedRGB = Boolean(decompressSelective & LASZIP_DECOMPRESS_SELECTIVE_RGB);
    this.requestedNIR = Boolean(decompressSelective & LASZIP_DECOMPRESS_SELECTIVE_NIR);
    /* mark the four scanner channel contexts as uninitialized */
    for (let c = 0; c < 4; c++) {
      this.contexts[c].mRgbBytesUsed = undefined;
      this.contexts[c].mNirBytesUsed = undefined;
    }
  }

  /** @param reader - the data block to read from */
  chunkSizes(reader: Reader): void {
    /* read bytes per layer */
    this.numBytesRGB = reader.getUint32(undefined, true);
    this.numBytesNIR = reader.getUint32(undefined, true);
  }

  /**
   * @param item - the current item
   * @param context - the current context
   */
  init(item: DataView, context: LAZContext): void {
    const { reader } = this.dec;
    if (this.instreamRGB === undefined) {
      /* create decoders */
      this.decRGB = new ArithmeticDecoder(reader);
      this.decNIR = new ArithmeticDecoder(reader);
    }
    /* how many bytes do we need to read */
    let numBytes = 0; // U32
    if (this.requestedRGB) numBytes += this.numBytesRGB;
    if (this.requestedNIR) numBytes += this.numBytesNIR;
    /* make sure the buffer is sufficiently large */
    if (numBytes > this.numBytesAllocated) this.numBytesAllocated = numBytes;
    /* load the requested bytes and init the corresponding instreams an decoders */
    numBytes = 0;
    if (this.requestedRGB) {
      if (this.numBytesRGB !== 0) {
        this.bytes = reader.seekSlice(this.numBytesRGB);
        numBytes += this.numBytesRGB;
        this.instreamRGB = new BufferReader(this.bytes.buffer);
        this.decRGB = new ArithmeticDecoder(this.instreamRGB);
        this.decRGB.init();
        this.changedRGB = true;
      } else {
        this.instreamRGB = undefined;
        this.changedRGB = false;
      }
    } else {
      if (this.numBytesRGB !== 0) {
        reader.seek(reader.tell() + this.numBytesRGB);
      }
      this.changedRGB = false;
    }
    if (this.requestedNIR) {
      if (this.numBytesNIR !== 0) {
        this.bytes = reader.seekSlice(this.numBytesNIR);
        this.instreamNIR = new BufferReader(this.bytes.buffer);
        this.decNIR = new ArithmeticDecoder(this.instreamNIR);
        this.decNIR.init();
        this.changedNIR = true;
      } else {
        this.instreamNIR = undefined;
        this.changedNIR = false;
      }
    } else {
      if (this.numBytesNIR !== 0) {
        reader.seek(reader.tell() + this.numBytesNIR);
      }
      this.changedNIR = false;
    }
    /* mark the four scanner channel contexts as unused */
    for (let c = 0; c < 4; c++) this.contexts[c].unused = true;
    /* set scanner channel as current context */
    this.currentContext = context.value; // all other items use context set by POINT14 reader
    /* create and init models and decompressors */
    this.#createAndInitModelsAndDecompressors(this.currentContext, new LASrgbaNir(item));
  }

  /**
   * @param item - the current item
   * @param context - the current context
   */
  read(item: DataView, context: LAZContext): void {
    const { contexts } = this;
    const curItem = new LASrgbaNir(item);
    let lastItem = new LASrgbaNir(contexts[this.currentContext].lastItem);
    // check for context switch
    if (this.currentContext !== context.value) {
      this.currentContext = context.value; // all other items use context set by POINT14 reader
      if (contexts[this.currentContext].unused) {
        this.#createAndInitModelsAndDecompressors(this.currentContext, lastItem);
      }
      lastItem = new LASrgbaNir(contexts[this.currentContext].lastItem);
    }
    // decompress
    ////////////////////////////////////////
    // decompress RGB layer
    ////////////////////////////////////////
    if (this.changedRGB) {
      let corr: number; // U8
      let diff = 0; // I32
      const sym = this.decRGB!.decodeSymbol(contexts[this.currentContext].mRgbBytesUsed!); // U32
      if ((sym & (1 << 0)) !== 0) {
        corr = this.decRGB!.decodeSymbol(contexts[this.currentContext].mRgbDiff0!);
        curItem.r = u8Fold(corr + (lastItem.r & 255));
      } else {
        curItem.r = lastItem.r & 0xff;
      }
      if ((sym & (1 << 1)) !== 0) {
        corr = this.decRGB!.decodeSymbol(contexts[this.currentContext].mRgbDiff1!);
        curItem.r |= u8Fold(corr + (lastItem.r >> 8)) << 8;
      } else {
        curItem.r |= lastItem.r & 0xff00;
      }
      if ((sym & (1 << 6)) !== 0) {
        diff = (curItem.r & 0x00ff) - (lastItem.r & 0x00ff);
        if ((sym & (1 << 2)) !== 0) {
          corr = this.decRGB!.decodeSymbol(contexts[this.currentContext].mRgbDiff2!);
          curItem.g = u8Fold(corr + u8Clamp(diff + (lastItem.g & 255)));
        } else {
          curItem.g = lastItem.g & 0xff;
        }
        if ((sym & (1 << 4)) !== 0) {
          corr = this.decRGB!.decodeSymbol(contexts[this.currentContext].mRgbDiff4!);
          diff = Math.trunc((diff + ((curItem.g & 0x00ff) - (lastItem.g & 0x00ff))) / 2);
          curItem.b = u8Fold(corr + u8Clamp(diff + (lastItem.b & 255)));
        } else {
          curItem.b = lastItem.b & 0xff;
        }
        diff = (curItem.r >> 8) - (lastItem.r >> 8);
        if ((sym & (1 << 3)) !== 0) {
          corr = this.decRGB!.decodeSymbol(contexts[this.currentContext].mRgbDiff3!);
          curItem.g |= u8Fold(corr + u8Clamp(diff + (lastItem.g >> 8))) << 8;
        } else {
          curItem.g |= lastItem.g & 0xff00;
        }
        if ((sym & (1 << 5)) !== 0) {
          corr = this.decRGB!.decodeSymbol(contexts[this.currentContext].mRgbDiff5!);
          diff = Math.trunc((diff + ((curItem.g >> 8) - (lastItem.g >> 8))) / 2);
          curItem.b |= u8Fold(corr + u8Clamp(diff + (lastItem.b >> 8))) << 8;
        } else {
          curItem.b |= lastItem.b & 0xff00;
        }
      } else {
        curItem.g = curItem.r;
        curItem.b = curItem.r;
      }
      curItem.copyTo(lastItem.data, 6);
    } else {
      lastItem.copyTo(item, 6);
    }
    ////////////////////////////////////////
    // decompress NIR layer
    ////////////////////////////////////////
    if (this.changedNIR) {
      let corr; // U8
      const sym = this.decNIR!.decodeSymbol(contexts[this.currentContext].mNirBytesUsed!); // U32
      if ((sym & (1 << 0)) !== 0) {
        corr = this.decNIR!.decodeSymbol(contexts[this.currentContext].mNirDiff0!);
        curItem.nir = u8Fold(corr + (lastItem.nir & 255));
      } else {
        curItem.nir = lastItem.nir & 0xff;
      }
      if ((sym & (1 << 1)) !== 0) {
        corr = this.decNIR!.decodeSymbol(contexts[this.currentContext].mNirDiff1!);
        curItem.nir |= u8Fold(corr + (lastItem.nir >> 8)) << 8;
      } else {
        curItem.nir |= lastItem.nir & 0xff00;
      }
      lastItem.nir = curItem.nir;
    } else {
      curItem.nir = lastItem.nir;
    }
  }

  /**
   * @param context - the current context
   * @param item - the current item
   */
  #createAndInitModelsAndDecompressors(context: number, item: LASrgbaNir): void {
    const { contexts } = this;
    if (this.requestedRGB) {
      if (contexts[context].mRgbBytesUsed === undefined) {
        contexts[context].mRgbBytesUsed = new ArithmeticModel(128);
        contexts[context].mRgbDiff0 = new ArithmeticModel(256);
        contexts[context].mRgbDiff1 = new ArithmeticModel(256);
        contexts[context].mRgbDiff2 = new ArithmeticModel(256);
        contexts[context].mRgbDiff3 = new ArithmeticModel(256);
        contexts[context].mRgbDiff4 = new ArithmeticModel(256);
        contexts[context].mRgbDiff5 = new ArithmeticModel(256);
      }
      /* then init entropy models */
      contexts[context].mRgbBytesUsed!.init();
      contexts[context].mRgbDiff0!.init();
      contexts[context].mRgbDiff1!.init();
      contexts[context].mRgbDiff2!.init();
      contexts[context].mRgbDiff3!.init();
      contexts[context].mRgbDiff4!.init();
      contexts[context].mRgbDiff5!.init();
    }
    if (this.requestedNIR) {
      if (contexts[context].mNirBytesUsed === undefined) {
        contexts[context].mNirBytesUsed = new ArithmeticModel(4);
        contexts[context].mNirDiff0 = new ArithmeticModel(256);
        contexts[context].mNirDiff1 = new ArithmeticModel(256);
      }
      /* then init entropy models */
      contexts[context].mNirBytesUsed!.init();
      contexts[context].mNirDiff0!.init();
      contexts[context].mNirDiff1!.init();
    }
    /* init current context from item */
    item.copyTo(contexts[context].lastItem, 8);
    contexts[context].unused = false;
  }
}

/** Parse LAZ wavepacket 1.4v4 */
export class LAZwavepacket14v4Reader implements ItemReader {
  instreamWavepacket?: Reader;
  decWavepacket?: ArithmeticDecoder;
  changedWavepacket = false;
  numBytesWavepacket = 0; // U32
  requestedWavepacket: boolean;
  bytes?: DataView;
  numBytesAllocated = 0; // U32
  currentContext = 0; // U32
  contexts = [
    new LAScontextWAVEPACKET14(),
    new LAScontextWAVEPACKET14(),
    new LAScontextWAVEPACKET14(),
    new LAScontextWAVEPACKET14(),
  ];
  /**
   * @param dec - the arithmetic decoder
   * @param decompressSelective - the decompress selective (filter)
   */
  constructor(
    private dec: ArithmeticDecoder,
    readonly decompressSelective = LASZIP_DECOMPRESS_SELECTIVE_ALL,
  ) {
    this.requestedWavepacket = Boolean(
      decompressSelective & LASZIP_DECOMPRESS_SELECTIVE_WAVEPACKET,
    );
    /* mark the four scanner channel contexts as uninitialized */
    for (let c = 0; c < 4; c++) this.contexts[c].mPacketIndex = undefined;
  }

  /** @param reader - the data block to read from */
  chunkSizes(reader: Reader): void {
    this.numBytesWavepacket = reader.getUint32(undefined, true);
  }

  /**
   * @param item - the current item to read in
   * @param context - the current context
   */
  init(item: DataView, context: LAZContext): void {
    const { reader } = this.dec;
    if (this.instreamWavepacket === undefined) {
      /* create decoders */
      this.decWavepacket = new ArithmeticDecoder(reader);
    }
    /* make sure the buffer is sufficiently large */
    if (this.numBytesWavepacket > this.numBytesAllocated) {
      this.numBytesAllocated = this.numBytesWavepacket;
    }
    /* load the requested bytes and init the corresponding instreams an decoders */
    if (this.requestedWavepacket) {
      if (this.numBytesWavepacket !== 0) {
        this.bytes = reader.seekSlice(this.numBytesWavepacket);
        this.instreamWavepacket = new BufferReader(this.bytes.buffer);
        this.decWavepacket = new ArithmeticDecoder(this.instreamWavepacket);
        this.decWavepacket.init();
        this.changedWavepacket = true;
      } else {
        this.instreamWavepacket = undefined;
        this.changedWavepacket = false;
      }
    } else {
      if (this.numBytesWavepacket !== 0) {
        reader.seek(reader.tell() + this.numBytesWavepacket);
      }
      this.changedWavepacket = false;
    }
    /* mark the four scanner channel contexts as unused */
    for (let c = 0; c < 4; c++) {
      this.contexts[c].unused = true;
    }
    /* set scanner channel as current context */
    this.currentContext = context.value; // all other items use context set by POINT14 reader
    /* create and init models and decompressors */
    this.#createAndInitModelsAndDecompressors(this.currentContext, new LASWavePacket13(item));
  }

  /**
   * @param item - the current item
   * @param context - the current context
   */
  read(item: DataView, context: LAZContext): void {
    const { contexts } = this;
    const currItem = new LASWavePacket13(item);
    let lastItem = new LASWavePacket13(contexts[this.currentContext].lastItem);
    // check for context switch
    if (this.currentContext !== context.value) {
      this.currentContext = context.value; // all other items use context set by POINT14 reader
      if (contexts[this.currentContext].unused) {
        this.#createAndInitModelsAndDecompressors(this.currentContext, lastItem);
      }
      lastItem = new LASWavePacket13(contexts[this.currentContext].lastItem);
    }
    // decompress
    if (this.changedWavepacket) {
      currItem.index = this.decWavepacket!.decodeSymbol(
        contexts[this.currentContext].mPacketIndex!,
      );
      contexts[this.currentContext].symLastOffsetDiff = this.decWavepacket!.decodeSymbol(
        contexts[this.currentContext].mOffsetDiff[contexts[this.currentContext].symLastOffsetDiff]!,
      );
      if (contexts[this.currentContext].symLastOffsetDiff === 0) {
        currItem.offset = lastItem.offset;
      } else if (contexts[this.currentContext].symLastOffsetDiff === 1) {
        currItem.offset = lastItem.offset + lastItem.packetSize;
      } else if (contexts[this.currentContext].symLastOffsetDiff === 2) {
        contexts[this.currentContext].lastDiff32 = contexts[
          this.currentContext
        ].icOffsetDiff!.decompress(contexts[this.currentContext].lastDiff32);
        currItem.offset = lastItem.offset + contexts[this.currentContext].lastDiff32;
      } else {
        currItem.offset = this.decWavepacket!.readInt64();
      }
      currItem.packetSize = contexts[this.currentContext].icPacketSize!.decompress(
        lastItem.packetSize,
      );
      currItem.returnPoint = contexts[this.currentContext].icReturnPoint!.decompress(
        lastItem.returnPoint,
      );
      currItem.x = contexts[this.currentContext].icXyz!.decompress(lastItem.x, { value: 0 });
      currItem.y = contexts[this.currentContext].icXyz!.decompress(lastItem.y, { value: 1 });
      currItem.z = contexts[this.currentContext].icXyz!.decompress(lastItem.z, { value: 2 });

      currItem.copyTo(lastItem.data, 29);
    }
  }

  /**
   * @param context - the current context
   * @param item - the current item
   */
  #createAndInitModelsAndDecompressors(context: number, item: LASWavePacket13): void {
    const { contexts } = this;
    if (this.requestedWavepacket) {
      if (contexts[context].mPacketIndex === undefined) {
        contexts[context].mPacketIndex = new ArithmeticModel(256);
        contexts[context].mOffsetDiff[0] = new ArithmeticModel(4);
        contexts[context].mOffsetDiff[1] = new ArithmeticModel(4);
        contexts[context].mOffsetDiff[2] = new ArithmeticModel(4);
        contexts[context].mOffsetDiff[3] = new ArithmeticModel(4);
        contexts[context].icOffsetDiff = new IntegerCompressor(this.decWavepacket!, 32);
        contexts[context].icPacketSize = new IntegerCompressor(this.decWavepacket!, 32);
        contexts[context].icReturnPoint = new IntegerCompressor(this.decWavepacket!, 32);
        contexts[context].icXyz = new IntegerCompressor(this.decWavepacket!, 32, 3);
      }
      /* then init entropy models */
      contexts[context].mPacketIndex!.init();
      contexts[context].mOffsetDiff[0]!.init();
      contexts[context].mOffsetDiff[1]!.init();
      contexts[context].mOffsetDiff[2]!.init();
      contexts[context].mOffsetDiff[3]!.init();
      contexts[context].icOffsetDiff!.initDecompressor();
      contexts[context].icPacketSize!.initDecompressor();
      contexts[context].icReturnPoint!.initDecompressor();
      contexts[context].icXyz!.initDecompressor();
    }
    /* init current context from item */
    contexts[context].lastDiff32 = 0;
    contexts[context].symLastOffsetDiff = 0;
    item.copyTo(contexts[context].lastItem, 29);
    contexts[context].unused = false;
  }
}

/** Parse LAZ RGB 1.4v4 */
export class LAZbyte14v4Reader implements ItemReader {
  instreamBytes?: Reader[];
  decBytes: (ArithmeticDecoder | undefined)[] = [];
  numBytesBytes: number[];
  changedBytes: boolean[];
  requestedBytes: boolean[];
  bytes?: DataView;
  numBytesAllocated = 0;
  currentContext = 0;
  contexts = [
    new LAScontextBYTE14(),
    new LAScontextBYTE14(),
    new LAScontextBYTE14(),
    new LAScontextBYTE14(),
  ];
  /**
   * @param dec - the arithmetic decoder
   * @param number - the number of bytes to read
   * @param decompressSelective - the selective decompression flags
   */
  constructor(
    readonly dec: ArithmeticDecoder,
    readonly number: number,
    readonly decompressSelective = LASZIP_DECOMPRESS_SELECTIVE_ALL,
  ) {
    this.numBytesBytes = new Array(number).fill(0);
    this.changedBytes = new Array(number).fill(false);
    this.requestedBytes = new Array(number).fill(false);

    for (let i = 0; i < number; i++) {
      this.numBytesBytes[i] = 0;
      this.changedBytes[i] = false;

      if (i > 15) {
        // currently only the first 16 extra bytes can be selectively decompressed
        this.requestedBytes[i] = true;
      } else {
        this.requestedBytes[i] = Boolean(
          decompressSelective & (LASZIP_DECOMPRESS_SELECTIVE_BYTE0 << i),
        );
      }
    }

    /* mark the four scanner channel contexts as uninitialized */
    for (let c = 0; c < 4; c++) this.contexts[c].mBytes = [];
    this.currentContext = 0;
  }

  /**
   * Read in the chunk sizes
   * @param reader - the data block to read from
   */
  chunkSizes(reader: Reader): void {
    for (let i = 0; i < this.number; i++) this.numBytesBytes[i] = reader.getUint32();
  }

  /**
   * Initialize the byte reader
   * @param item - the first raw item needs to be injected for future reads
   * @param context - the current context
   */
  init(item: DataView, context: LAZContext): void {
    const { reader } = this.dec;
    let i: number;
    /* on the first init create instreams and decoders */
    if (this.instreamBytes === undefined) {
      /* create instream pointer array */
      this.instreamBytes = new Array(this.number);
      /* create decoder pointer array */
      this.decBytes = new Array(this.number);
      /* create layer decoders */
      for (i = 0; i < this.number; i++) this.decBytes[i] = new ArithmeticDecoder(reader);
    }
    /* how many bytes do we need to read */
    let numBytes = 0; // U32
    for (i = 0; i < this.number; i++) {
      if (this.requestedBytes[i]) numBytes += this.numBytesBytes[i];
    }
    /* make sure the buffer is sufficiently large */
    if (numBytes > this.numBytesAllocated) this.numBytesAllocated = numBytes;
    /* load the requested bytes and init the corresponding instreams an decoders */
    numBytes = 0;
    for (i = 0; i < this.number; i++) {
      if (this.requestedBytes[i]) {
        if (this.numBytesBytes[i] !== 0) {
          this.bytes = reader.seekSlice(this.numBytesBytes[i]);
          this.instreamBytes[i] = new BufferReader(this.bytes.buffer);
          this.decBytes[i] = new ArithmeticDecoder(this.instreamBytes[i]);
          this.decBytes[i]!.init();
          numBytes += this.numBytesBytes[i];
          this.changedBytes[i] = true;
        } else {
          this.decBytes[i] = undefined;
          this.changedBytes[i] = false;
        }
      } else {
        if (this.numBytesBytes[i] !== 0) reader.seek(reader.tell() + this.numBytesBytes[i]);
        this.changedBytes[i] = false;
      }
    }
    /* mark the four scanner channel contexts as unused */
    for (let c = 0; c < 4; c++) this.contexts[c].unused = true;
    /* set scanner channel as current context */
    this.currentContext = context.value; // all other items use context set by POINT14 reader
    /* create and init models and decompressors */
    this.#createAndInitModelsAndDecompressors(this.currentContext, item);
  }

  /**
   * Read the next item
   * @param item - the next item
   * @param context - the current context
   */
  read(item: DataView, context: LAZContext): void {
    const { contexts } = this;
    // get last
    let lastItem = contexts[this.currentContext].lastItem;
    // check for context switch
    if (this.currentContext !== context.value) {
      this.currentContext = context.value; // all other items use context set by POINT14 reader
      if (contexts[this.currentContext].unused) {
        this.#createAndInitModelsAndDecompressors(this.currentContext, lastItem);
      }
      lastItem = contexts[this.currentContext].lastItem;
    }
    // decompress
    for (let i = 0; i < this.number; i++) {
      if (this.changedBytes[i]) {
        // I32
        const value =
          lastItem.getUint8(i) +
          this.decBytes[i]!.decodeSymbol(contexts[this.currentContext].mBytes[i]!);
        item.setUint8(i, u8Fold(value));
        lastItem.setUint8(i, item.getUint8(i));
      } else {
        item.setUint8(i, lastItem.getUint8(i));
      }
    }
  }

  /**
   * @param context - the current context
   * @param item - the current item
   */
  #createAndInitModelsAndDecompressors(context: number, item: DataView): void {
    const { contexts } = this;
    let i: number;

    /* first create all entropy models and last items (if needed) */
    if (contexts[context].mBytes.length === 0) {
      contexts[context].mBytes = new Array(this.number);
      for (i = 0; i < this.number; i++) {
        contexts[context].mBytes[i] = new ArithmeticModel(256);
        contexts[context].mBytes[i]!.init();
      }
      /* create last item */
      contexts[context].lastItem = new DataView(new ArrayBuffer(this.number));
    }
    /* then init entropy models */
    for (i = 0; i < this.number; i++) contexts[context].mBytes[i]!.init();
    /* init current context from item */
    new Uint8Array(contexts[context].lastItem.buffer).set(new Uint8Array(item.buffer));

    contexts[context].unused = false;
  }
}
