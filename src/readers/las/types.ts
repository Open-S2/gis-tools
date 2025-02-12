import type { Properties, RGBA } from '../..';

/**
 * VARIABLE LENGTH RECORDS:
 * The Variable Length Records are used to add custom data to the Public Header Block.
 * The `GeoKeyDirectoryTag` is required
 */
export interface LASVariableLengthRecord {
  /** Reserved unsigned short 2 bytes */
  reserved: number;
  /** User ID char[16] 16 bytes */
  userID: string;
  /** Record ID unsigned short 2 bytes */
  recordID: number;
  /** Record Length After Header unsigned short 2 bytes */
  recordLength: number;
  /** Description char[32] 32 bytes */
  description: string;
  /** The data of the record */
  data?: DataView;
}

/**
 * Extended VARIABLE LENGTH RECORDS:
 * The Extended Variable Length Records are used to add custom data to the LAZ Header Block.
 * This record type allows data to be much larger in size.
 */
export interface LASExtendedVariableLengthRecord {
  /** Reserved unsigned short 2 bytes */
  reserved: number;
  /** User ID char[16] 16 bytes */
  userID: string;
  /** Record ID unsigned short 2 bytes */
  recordID: number;
  /** Record Length After Header unsigned short 2 bytes */
  recordLength: number;
  /** Description char[32] 32 bytes */
  description: string;
  /** The data of the record */
  data?: DataView;
}

/**
 * LAS Header Block
 * Any field in the Public Header Block that is not required and is not used must be zero filled.
 */
export interface LASHeader {
  /**
   * File Signature ("LASF") char[4] 4 bytes
   * The file signature must contain the four characters "LASF", and it is required by the LAS
   * specification. These four characters can be checked by user software as a quick look initial
   * determination of file type.
   */
  signature: string;
  /**
   * File Source ID unsigned short 2 bytes
   *
   * File Source ID (Flight Line Number if this file was derived from an original flight line):
   * This field should be set to a value between 1 and 65,535, inclusive. A value of zero (0)
   * is interpreted to mean that an ID has not been assigned. In this case, processing software is
   * free to assign any LAS 1.2 3 valid number. Note that this scheme allows a LIDAR project to
   * contain up to 65,535 unique sources. A source can
   */
  sourceID: number;
  /**
   * Global Encoding unsigned short 2 bytes.
   *
   * The meaning of GPS Time in the Point Records
   * - 0 (not set) -> GPS time in the point record fields is GPS Week Time (the same as previous
   * versions of LAS).
   * - 1 (set) -> GPS Time is standard GPS Time (satellite GPS Time) minus 1 x 109. The offset
   * moves the time back to near zero to improve floating point resolution.
   */
  encoding: number;
  /** Project ID - GUID data 1 unsigned long 4 bytes. 0 means no project ID */
  projectID1: number;
  /** Project ID - GUID data 2 unsigned short 2 byte. 0 means no project ID  */
  projectID2: number;
  /** Project ID - GUID data 3 unsigned short 2 byte. 0 means no project ID  */
  projectID3: number;
  /** Project ID - GUID data 4 unsigned char[8] 8 bytes. 0 means no project ID  */
  projectID4: string;
  /** Version Major unsigned char 1 byte */
  majorVersion: number;
  /** Version Minor unsigned char 1 byte */
  minorVersion: number;
  /** System Identifier char[32] 32 bytes */
  systemIdentifier: string;
  /** Generating Software char[32] 32 bytes */
  generatingSoftware: string;
  /**
   * File Creation Day Year unsigned short 2 bytes. 0 means no creation date
   *
   * Day, expressed as an unsigned short, on which this file was created. Day is computed as the
   * Greenwich Mean Time (GMT) day. January 1 is considered day 1.
   */
  fileCreationDay: number;
  /**
   * File Creation Day Year unsigned short 2 bytes. 0 means no creation date
   *
   * The year, expressed as a four digit number, in which the file was created.
   */
  fileCreationYear: number;
  /**
   * Header Size unsigned short 2 bytes
   *
   * The size, in bytes, of the Public Header Block itself. In the event that the header is extended
   * by a software application through the addition of data at the end of the header, the Header
   * Size field must be updated with the new header size. Extension of the Public Header Block is
   * discouraged; the Variable Length Records should be used whenever possible to add custom header
   * data. In the event a generating software package adds data to the Public Header Block, this
   * data must be placed at the end of the structure and the Header Size must be updated to reflect
   * the new size.
   */
  headerSize: number;
  /**
   * Offset to Point Data unsigned int 4 bytes
   *
   * The actual number of bytes from the beginning of the file to the first field of the first point
   * record data field. This data offset must be updated if any software adds data from the Public
   * Header Block or adds/removes data to/from the Variable Length Records.
   */
  offsetToPoints: number;
  /**
   * Number of Variable Length Records unsigned int 4 bytes
   * This field contains the current number of Variable Length Records. This number must be updated
   * if the number of Variable Length Records changes at any time.
   */
  numVariableLengthRecords: number;
  /**
   * Point Data Format ID unsigned short 1 byte
   *
   * The point data format ID corresponds to the point data record format type.
   * LAS 1.4 defines types 0-10.
   */
  pointDataFormatID: number;
  /** Point Data Record Length unsigned short 2 bytes */
  pointDataRecordLength: number;
  /** Number of point records unsigned long 4 bytes */
  numPoints: number;
  /** Number of points by return unsigned long[5] 20 bytes */
  numPointsByReturn: number[];
  /** X scale factor double 8 bytes */
  xScaleFactor: number;
  /** Y scale factor double 8 bytes */
  yScaleFactor: number;
  /** Z scale factor double 8 bytes */
  zScaleFactor: number;
  /** X offset double 8 bytes */
  xOffset: number;
  /** Y offset double 8 bytes */
  yOffset: number;
  /** Z offset double 8 bytes */
  zOffset: number;
  /** Max X double 8 bytes */
  maxX: number;
  /** Min X double 8 bytes */
  minX: number;
  /** Max Y double 8 bytes */
  maxY: number;
  /** Min Y double 8 bytes */
  minY: number;
  /** Max Z double 8 bytes */
  maxZ: number;
  /** Min Z double 8 bytes */
  minZ: number;
  /**
   * Start of Waveform Data Packet Record - Unsigned long long 8 bytes
   *
   */
  waveformDataPacketOffset: number;
  /**
   * Start of first Extended Variable Length Record - unsigned long long 8 bytes
   */
  extendedVariableLengthRecordOffset: number;
  /**
   * Number of Extended Variable Length Records - unsigned long 4 bytes
   */
  extendedVariableLengthSize: number;
}

/** Enum representing the LAZ Item type */
export const LAZHeaderItemType = {
  BYTE: 0,
  SHORT: 1,
  INT: 2,
  LONG: 3,
  FLOAT: 4,
  DOUBLE: 5,
  POINT10: 6,
  GPSTIME11: 7,
  RGB12: 8,
  WAVEPACKET13: 9,
  POINT14: 10,
  RGB14: 11,
  RGBNIR14: 12,
  WAVEPACKET14: 13,
  BYTE14: 14,
} as const;

/**
 * Enum representing the LAZ Item type
 * - 0: `BYTE` (extra bytes that are appended to a LAS Point Data Record Format 0 to 5)
 * - 1: `SHORT` (reserved, unsupported)
 * - 2: `INT` (reserved, unsupported)
 * - 3: `LONG` (reserved, unsupported)
 * - 4: `FLOAT` (reserved, unsupported)
 * - 5: `DOUBLE` (reserved, unsupported)
 * - 6: `POINT10` (LAS Point Data Record Format 0, containing the core fields that are shared
between LAS Point Data Record Formats 0 to 5)
 * - 7: `GPSTIME11` (the GPS Time field that is added for LAS Point Data Record Formats 1, 3,
4 and 5)
 * - 8: `RGB12` (the R, G and B fields that are added for LAS Point Data Record Formats 2,
3 and 5)
 * - 9: `WAVEPACKET13` (the 7 fields for the Waveform packet that are added for LAS Point Data
Record Formats 4 and 5)
 * - 10: `POINT14` (LAS Point Data Record Format 6, containing the core fields that are shared
between LAS Point Data Record Formats 6 to 10)
 * - 11: `RGB14` (the R, G and B fields that are added for LAS Point Data Record Format 7)
 * - 12: `RGBNIR14` (the R, G, B and NIR (near infrared) fields that are added for LAS Point
Data Record Formats 8 and 10)
 * - 13: `WAVEPACKET14` (the 7 fields for the Waveform packet that are added for LAS Point Data
Record Formast 9 and 10)
 * - 14: `BYTE14` (extra bytes that are appended to a LAS Point Data Record Format 6 to 10)
 * 
 * NOTE: The number in the name, for example in “Point10”, refers to the LAS and LAZ
version where that type got added.
 */
export type LAZHeaderItemType = (typeof LAZHeaderItemType)[keyof typeof LAZHeaderItemType];

/** Enum representing the LAZ Item type */
export const LAZCompressor = {
  NONE: 0,
  POINTWISE: 1,
  POINTWISE_AND_CHUNKED: 2,
  LAYERED_AND_CHUNKED: 3,
} as const;

/**
 * Enum representing the LAZ Item type
 * - 0 = No Compression (Uncompressed Standard LAS file)
 * - 1 = Pointwise compression (only for point types 0 to 5)
 * - 2 = Pointwise and chunked compression (only for point types 0 to 5)
 * - 3 = Layered and chunked compression (only for point types 6 to 10)
 */
export type LAZCompressor = (typeof LAZCompressor)[keyof typeof LAZCompressor];

/** A LAZ Header Item */
export interface LAZHeaderItem {
  // U16 type: 2 bytes * num_items
  type: LAZHeaderItemType;
  // U16 size: 2 bytes * num_items
  size: number;
  // U16 version: 2 bytes * num_items
  version: number;
}

/** A LAZ Header */
export interface LAZHeader {
  // Compressor unsigned short 2 bytes *
  compressor: LAZCompressor;
  // Coder unsigned short 2 bytes *
  coder: number;
  // Version Major unsigned char 1 byte *
  versionMajor: number;
  // Version Minor unsigned char 1 byte *
  versionMinor: number;
  // Version Revision unsigned short 2 bytes *
  versionRevision: number;
  // Options unsigned long 4 bytes *
  options: number;
  // Chunk Size unsigned long 4 bytes *
  chunkSize: number;
  // Number of special EVLRs signed long long 8 bytes *
  numSpecialEvlrs: number;
  // Offset of special EVLRs signed long long 8 bytes *
  offsetSpecialEvlrs: number;
  // Number of Items unsigned short 2 bytes *
  numItems: number;
  // Item records Array of “Item record” 6 bytes * Number of Items *
  items: LAZHeaderItem[];
}

/** Point Data Record Format */
export type LASFormat =
  | LASFormat0
  | LASFormat1
  | LASFormat2
  | LASFormat3
  | LASFormat4
  | LASFormat5
  | LASFormat6
  | LASFormat7
  | LASFormat8
  | LASFormat9
  | LASFormat10;

/**
 * Point Data Record Format 0 contains the core 20 bytes that are shared by Point Data Record
 * Formats 0 to 5.
 */
export type LASFormat0_5 =
  | LASFormat0
  | LASFormat1
  | LASFormat2
  | LASFormat3
  | LASFormat4
  | LASFormat5;

/** Point Data Record Format 6 to 10 */
export type LASFormat6_10 = LASFormat6 | LASFormat7 | LASFormat8 | LASFormat9 | LASFormat10;

/**
 * Point Data Record Format 0 contains the core 20 bytes that are shared by Point Data Record
 * Formats 0 to 5.
 */
export interface LASFormat0 extends Properties {
  /**
   * 2 bytes
   *
   * The intensity value is the integer representation of the pulse return magnitude. This
   * value is optional and system specific. However, it should always be included if available.
   * Intensity, when included, is always normalized to a 16 bit, unsigned value by multiplying the value
   * by 65,536/(intensity dynamic range of the sensor). For example, if the dynamic range of the
   * sensor is 10 bits, the scaling value would be (65,536/1,024). If intensity is not included, this value
   * must be set to zero. This normalization is required to ensure that data from different sensors can
   * be correctly merged.
   *
   * Please note that the following four fields (Return Number, Number of Returns, Scan Direction
   * Flag and Edge of Flight Line) are bit fields within a single byte.
   */
  intensity: number;
  /**
   * 3 bits
   *
   * The Return Number is the pulse return number for a given output pulse. A given output laser
   * pulse can have many returns, and they must be marked in sequence of return. The first return
   * will have a Return Number of one, the second a Return Number of two, and so on up to five returns
   */
  returnNumber: number;
  /**
   * 3 bits
   *
   * Number of Returns (for this emitted pulse): The Number of Returns is the total number of
   * returns for a given pulse. For example, a laser data point may be return two (Return Number)
   * within a total number of five returns.
   */
  numberOfReturns: number;
  /**
   * 1 bit
   *
   * The Scan Direction Flag denotes the direction at which the scanner mirror was traveling at the
   * time of the output pulse. A bit value of 1 is a positive scan direction, and a
   * bit value of 0 is a negative scan direction (where positive scan direction is a scan moving
   * from the left side of the in-track direction to the right side and negative the opposite).
   */
  scanDirectionFlag: number;
  /**
   * 1 bit
   *
   * The Edge of Flight Line data bit has a value of 1 only when the point is at
   * the end of a scan. It is the last point on a given scan line before it changes direction.
   */
  edgeOfFlightLine: number;
  /**
   * 1 byte
   *
   * Classification in LAS 1.0 was essentially user defined and optional. LAS 1.1
   * defines a standard set of ASPRS classifications. In addition, the field is now mandatory. If a
   * point has never been classified, this byte must be set to zero. There are no user defined classes
   * since both point format 0 and point format 1 supply 8 bits per point for user defined operations.
   */
  classification: string;
  /** Subclass of classification. */
  isSynthetic: boolean;
  /** Subclass of classification. */
  isKeyPoint: boolean;
  /** Subclass of classification. */
  isWithheld: boolean;
  /**
   * 1 byte
   *
   * The Scan Angle Rank is a signed one-byte number with a valid range from -
   * 90 to +90. The Scan Angle Rank is the angle (rounded to the nearest integer in the absolute
   * value sense) at which the laser point was output from the laser system including the roll of the
   * aircraft. The scan angle is within 1 degree of accuracy from +90 to –90 degrees. The scan angle
   * is an angle based on 0 degrees being nadir, and –90 degrees to the left side of the aircraft in the
   * direction of flight.
   */
  scanAngleRank: number;
  /**
   * 1 byte
   *
   * User Data: This field may be used at the user’s discretion
   */
  userData: number;
  /**
   * 2 bytes
   *
   * This value indicates the file from which this point originated. Valid values for
   * this field are 1 to 65,535 inclusive with zero being used for a special case discussed below. The
   * numerical value corresponds to the File Source ID from which this point originated. Zero is
   * reserved as a convenience to system implementers. A Point Source ID of zero implies that this
   * point originated in this file. This implies that processing software should set the Point Source ID
   * equal to the File Source ID of the file containing this point at some time during processing.
   */
  pointSourceID: number;
}

/**
 * Point Data Record Format 1 is the same as Point Data Record Format 0 with the addition of GPS
 * Time.
 */
export interface LASFormat1 extends LASFormat0 {
  /**
   * 8 bytes
   *
   *  The GPS Time is the double floating point time tag value at which the point was
   * acquired. It is GPS Week Time if the Global Encoding low bit is clear and Adjusted Standard GPS
   * Time if the Global Encoding low bit is set (see Global Encoding in the Public Header Block
   * description).
   */
  gpsTime: number;
}

/**
 * Point Data Record Format 2 is the same as Point Data Record Format 0 with the addition of three
 * color channels. These fields are used when "colorizing" a LIDAR point using ancillary data,
 * typically from a camera.
 */
export interface LASFormat2 extends LASFormat0 {
  /**
   * 2 bytes each [R,G,B] (6 bytes total)
   *
   * The Red, Green, Blue values should always be normalized to 16 bit values. For example, when
   * encoding an 8 bit per channel pixel, multiply each channel value by 256 prior to storage in these
   * fields. This normalization allows color values from different camera bit depths to be accurately
   * merged.
   */
  rgba: RGBA;
}

/**
 * Point Data Record Format 3 is the same as Point Data Record Format 2 with the addition of GPS
 * Time.
 * https://github.com/ASPRSorg/LAS/wiki/Waveform-Data-Packet-Descriptors-Explained
 */
export interface LASFormat3 extends LASFormat2 {
  /**
   * 8 bytes
   *
   *  The GPS Time is the double floating point time tag value at which the point was
   * acquired. It is GPS Week Time if the Global Encoding low bit is clear and Adjusted Standard GPS
   * Time if the Global Encoding low bit is set (see Global Encoding in the Public Header Block
   * description).
   */
  gpsTime: number;
}

/** Point Data Record Format 4 adds Wave Packets to Point Data Record Format 1.  */
export interface LASFormat4 extends LASFormat1 {
  /**
   * 1 byte
   *
   * Wave Packet Descriptor Index: This value plus 99 is the Record ID of the Waveform Packet
   * Descriptor and indicates the User Defined Record that describes the waveform packet associated
   * with this LIDAR point. Up to 255 different User Defined Records which describe the waveform
   * packet are supported. A value of zero indicates that there is no waveform data associated with
   * this LIDAR point record.
   */
  wavePacketDescriptorIndex: number;
  /**
   * 8 bytes
   *
   * Byte offset to Waveform Packet Data: The waveform packet data are stored in the LAS file in
   * an Extended Variable Length Record or in an auxiliary WPD file. The Byte Offset represents the
   * location of the start of this LIDAR points’ waveform packet within the waveform data variable
   * length record (or external file) relative to the beginning of the Waveform Packet Data header. The
   * absolute location of the beginning of this waveform packet relative to the beginning of the file is
   * given by:
   * - Start of Waveform Data Packet Record + Byte offset to Waveform Packet Data
   * for waveform packets stored within the LAS file
   * - Byte offset to Waveform Packet Data for data stored in an auxiliary file
   */
  wavePacketOffset: number;
  /**
   * 4 bytes
   *
   * Waveform packet size in bytes: The size, in bytes, of the waveform packet associated with this
   * return. Note that each waveform can be of a different size (even those with the same Waveform
   * Packet Descriptor index) due to packet compression. Also note that waveform packets can be
   * located only via the Byte offset to Waveform Packet Data value since there is no requirement that
   * records be stored sequentially.
   */
  wavePacketLength: number;
  /**
   * 4 bytes
   *
   * Return Point location: The offset in picoseconds (10-12) from the first digitized value to the
   * location within the waveform packet that the associated return pulse was detected.
   */
  waveformLocationReturnPoint: number;
  /**
   * 4 bytes
   *
   * this parameter defines a parametric line equation for extrapolating points
   * along the associated waveform. The position along the wave is given by:
   * `X = X0 + X(t)` where `X` is the spatial position of the derived point, `X0` is the position
   * of the "anchor" point, and t is the time, in picoseconds, relative to the anchor point
   * (i.e. t = zero at the anchor point). The units of X, Y and Z are the units of the coordinate
   * systems of the LAS data. If the coordinate system is geographic, the horizontal units are
   * decimal degrees and the vertical units are meters.
   */
  xT: number;
  /**
   * 4 bytes
   *
   * this parameter defines a parametric line equation for extrapolating points
   * along the associated waveform. The position along the wave is given by:
   * `Y = Y0 + Y(t)` where `Y` is the spatial position of the derived point, `Y0` is the position
   * of the "anchor" point, and t is the time, in picoseconds, relative to the anchor point
   * (i.e. t = zero at the anchor point). The units of X, Y and Z are the units of the coordinate
   * systems of the LAS data. If the coordinate system is geographic, the horizontal units are
   * decimal degrees and the vertical units are meters.
   */
  yT: number;
  /**
   * 4 bytes
   *
   * this parameter defines a parametric line equation for extrapolating points
   * along the associated waveform. The position along the wave is given by:
   * `Z = Z0 + Z(t)` where `Z` is the spatial position of the derived point, `Z0` is the position
   * of the "anchor" point, and t is the time, in picoseconds, relative to the anchor point
   * (i.e. t = zero at the anchor point). The units of X, Y and Z are the units of the coordinate
   * systems of the LAS data. If the coordinate system is geographic, the horizontal units are
   * decimal degrees and the vertical units are meters.
   */
  zT: number;
}

/** Point Data Record Format 5 adds Wave Packets to Point Data Record Format 3. */
export interface LASFormat5 extends LASFormat3 {
  /**
   * 1 byte
   *
   * Wave Packet Descriptor Index: This value plus 99 is the Record ID of the Waveform Packet
   * Descriptor and indicates the User Defined Record that describes the waveform packet associated
   * with this LIDAR point. Up to 255 different User Defined Records which describe the waveform
   * packet are supported. A value of zero indicates that there is no waveform data associated with
   * this LIDAR point record.
   */
  wavePacketDescriptorIndex: number;
  /**
   * 8 bytes
   *
   * Byte offset to Waveform Packet Data: The waveform packet data are stored in the LAS file in
   * an Extended Variable Length Record or in an auxiliary WPD file. The Byte Offset represents the
   * location of the start of this LIDAR points’ waveform packet within the waveform data variable
   * length record (or external file) relative to the beginning of the Waveform Packet Data header. The
   * absolute location of the beginning of this waveform packet relative to the beginning of the file is
   * given by:
   * - Start of Waveform Data Packet Record + Byte offset to Waveform Packet Data
   * for waveform packets stored within the LAS file
   * - Byte offset to Waveform Packet Data for data stored in an auxiliary file
   */
  wavePacketOffset: number;
  /**
   * 4 bytes
   *
   * Waveform packet size in bytes: The size, in bytes, of the waveform packet associated with this
   * return. Note that each waveform can be of a different size (even those with the same Waveform
   * Packet Descriptor index) due to packet compression. Also note that waveform packets can be
   * located only via the Byte offset to Waveform Packet Data value since there is no requirement that
   * records be stored sequentially.
   */
  wavePacketLength: number;
  /**
   * 4 bytes
   *
   * Return Point location: The offset in picoseconds (10-12) from the first digitized value to the
   * location within the waveform packet that the associated return pulse was detected.
   */
  waveformLocationReturnPoint: number;
  /**
   * 4 bytes
   *
   * this parameter defines a parametric line equation for extrapolating points
   * along the associated waveform. The position along the wave is given by:
   * `X = X0 + X(t)` where `X` is the spatial position of the derived point, `X0` is the position
   * of the "anchor" point, and t is the time, in picoseconds, relative to the anchor point
   * (i.e. t = zero at the anchor point). The units of X, Y and Z are the units of the coordinate
   * systems of the LAS data. If the coordinate system is geographic, the horizontal units are
   * decimal degrees and the vertical units are meters.
   */
  xT: number;
  /**
   * 4 bytes
   *
   * this parameter defines a parametric line equation for extrapolating points
   * along the associated waveform. The position along the wave is given by:
   * `Y = Y0 + Y(t)` where `Y` is the spatial position of the derived point, `Y0` is the position
   * of the "anchor" point, and t is the time, in picoseconds, relative to the anchor point
   * (i.e. t = zero at the anchor point). The units of X, Y and Z are the units of the coordinate
   * systems of the LAS data. If the coordinate system is geographic, the horizontal units are
   * decimal degrees and the vertical units are meters.
   */
  yT: number;
  /**
   * 4 bytes
   *
   * this parameter defines a parametric line equation for extrapolating points
   * along the associated waveform. The position along the wave is given by:
   * `Z = Z0 + Z(t)` where `Z` is the spatial position of the derived point, `Z0` is the position
   * of the "anchor" point, and t is the time, in picoseconds, relative to the anchor point
   * (i.e. t = zero at the anchor point). The units of X, Y and Z are the units of the coordinate
   * systems of the LAS data. If the coordinate system is geographic, the horizontal units are
   * decimal degrees and the vertical units are meters.
   */
  zT: number;
}

/**
 * Point Data Record Format 6 contains the core 30 bytes that are shared by Point Data Record
 * Formats 6 to 10. The difference to the core 20 bytes of Point Data Record Formats 0 to 5 is that
 * there are more bits for return numbers in order to support up to 15 returns, there are more bits for
 * point classifications to support up to 256 classes, there is a higher precision scan angle (16 bits
 * instead of 8), and the GPS time is mandatory.
 */
export interface LASFormat6 extends Properties {
  /**
   * 2 bytes
   *
   * The intensity value is the integer representation of the pulse return magnitude. This
   * value is optional and system specific. However, it should always be included if available.
   * Intensity, when included, is always normalized to a 16 bit, unsigned value by multiplying the value
   * by 65,536/(intensity dynamic range of the sensor). For example, if the dynamic range of the
   * sensor is 10 bits, the scaling value would be (65,536/1,024). If intensity is not included, this value
   * must be set to zero. This normalization is required to ensure that data from different sensors can
   * be correctly merged.
   *
   * Please note that the following four fields (Return Number, Number of Returns, Scan Direction
   * Flag and Edge of Flight Line) are bit fields within a single byte.
   */
  intensity: number;
  /**
   * 4 bits (bits 0 - 3)
   *
   * Return Number: The Return Number is the pulse return number for a given output pulse. A
   * given output laser pulse can have many returns, and they must be marked in sequence of return.
   * The first return will have a Return Number of one, the second a Return Number of two, and so on
   * up to fifteen returns. The Return Number must be between 1 and the Number of Returns,
   * inclusive.
   */
  returnNumber: number;
  /**
   * 4 bits (bits 0 - 3)
   *
   * Number of Returns (given pulse): The Number of Returns is the total number of returns for a
   * given pulse. For example, a laser data point may be return two (Return Number) within a total
   * number of up to fifteen returns.
   */
  numberOfReturns: number;
  /**
   * 4 bits (bits 0 - 3)
   *
   * Classification Flags: Classification flags are used to indicate special characteristics associated
   * with the point. The bit definitions are:
   * - 0 **Synthetic** If set then this point was created by a technique other than LIDAR collection
   * such as digitized from a photogrammetric stereo model or by traversing a waveform.
   * - 1 **Key-point** If set, this point is considered to be a model key-point and thus generally
   * should not be withheld in a thinning algorithm.
   * - 2 **Withheld** If set, this point should not be included in processing (synonymous with Deleted).
   * - 3 **Overlap** If set, this point is within the overlap region of two or more swaths or takes.
   * Setting this bit is not mandatory (unless, of course, it is mandated by a particular delivery
   * specification) but allows Classification of overlap points to be preserved.
   */
  classificationFlag: string;
  /**
   * 2 bits (bits 4 - 5)
   *
   * Scanner Channel: Scanner Channel is used to indicate the channel (scanner head) of a multichannel
   * system. Channel 0 is used for single scanner systems. Up to four channels are supported (0-3).
   */
  scannerChannel: number;
  /**
   * 1 bit (bit 6)
   *
   * Scan Direction Flag: The Scan Direction Flag denotes the direction at which the scanner mirror
   * was traveling at the time of the output pulse. A bit value of 1 is a positive scan direction, and a bit
   * value of 0 is a negative scan direction (where positive scan direction is a scan moving from the
   * left side of the in-track direction to the right side and negative the opposite).
   */
  scanDirectionFlag: number;
  /**
   * 1 bit (bit 7)
   *
   * Edge of Flight Line: The Edge of Flight Line data bit has a value of 1 only when the point is at
   * the end of a scan. It is the last point on a given scan line before it changes direction or the mirror
   * facet changes. Note that this field has no meaning for 360° Field of View scanners (such as
   * Mobile LIDAR scanners) and should not be set.
   */
  edgeOfFlightLine: number;
  /**
   * 1 byte
   *
   * ASPRS Standard LIDAR Point Classes (See {@link toLASClassification14})
   */
  classification: string;
  /**
   * 1 byte
   *
   * User Data: This field may be used at the user’s discretion
   */
  userData: number;
  /**
   * 2 bytes
   *
   * Scan Angle: The Scan Angle is a signed short that represents the rotational position of the
   * emitted laser pulse with respect to the vertical of the coordinate system of the data. Down in the
   * data coordinate system is the 0.0 position. Each increment represents 0.006 degrees.
   * CounterClockwise rotation, as viewed from the rear of the sensor, facing in the along-track (positive
   * trajectory) direction, is positive. The maximum value in the positive sense is 30,000 (180 degrees
   * which is up in the coordinate system of the data). The maximum value in the negative direction is
   * -30.000 which is also directly up.
   */
  scanAngle: number;
  /**
   * 2 bytes
   *
   * This value indicates the file from which this point originated. Valid values for
   * this field are 1 to 65,535 inclusive with zero being used for a special case discussed below. The
   * numerical value corresponds to the File Source ID from which this point originated. Zero is
   * reserved as a convenience to system implementers. A Point Source ID of zero implies that this
   * point originated in this file. This implies that processing software should set the Point Source ID
   * equal to the File Source ID of the file containing this point at some time during processing.
   */
  pointSourceID: number;
  /**
   * 8 bytes
   *
   *  The GPS Time is the double floating point time tag value at which the point was
   * acquired. It is GPS Week Time if the Global Encoding low bit is clear and Adjusted Standard GPS
   * Time if the Global Encoding low bit is set (see Global Encoding in the Public Header Block
   * description).
   */
  gpsTime: number;
}

/**
 * Point Data Record Format 7 is the same as Point Data Record Format 6 with the addition of three
 * RGB color channels. These fields are used when “colorizing” a LIDAR point using ancillary data,
 * typically from a camera.
 */
export interface LASFormat7 extends LASFormat6 {
  /**
   * 2 bytes each [R,G,B] (6 bytes total)
   *
   * The Red, Green, Blue values should always be normalized to 16 bit values. For example, when
   * encoding an 8 bit per channel pixel, multiply each channel value by 256 prior to storage in these
   * fields. This normalization allows color values from different camera bit depths to be accurately
   * merged.
   */
  rgba: RGBA;
}

/**
 * Point Data Record Format 8 is the same as Point Data Record Format 7 with the addition of a
 * NIR (near infrared) channel.
 */
export interface LASFormat8 extends LASFormat7 {
  /**
   * 2 bytes
   *
   * NIR: The NIR (near infrared) channel value associated with this point.
   *
   * Note that Red, Green, Blue and NIR values should always be normalized to 16 bit values. For
   * example, when encoding an 8 bit per channel pixel, multiply each channel value by 256 prior to
   * storage in these fields. This normalization allows color values from different camera bit depths to
   * be accurately merged.
   */
  nir: number;
}

/** Point Data Record Format 9 adds Wave Packets to Point Data Record Format 6. */
export interface LASFormat9 extends LASFormat6 {
  /**
   * 1 byte
   *
   * Wave Packet Descriptor Index: This value plus 99 is the Record ID of the Waveform Packet
   * Descriptor and indicates the User Defined Record that describes the waveform packet associated
   * with this LIDAR point. Up to 255 different User Defined Records which describe the waveform
   * packet are supported. A value of zero indicates that there is no waveform data associated with
   * this LIDAR point record.
   */
  wavePacketDescriptorIndex: number;
  /**
   * 8 bytes
   *
   * Byte offset to Waveform Packet Data: The waveform packet data are stored in the LAS file in
   * an Extended Variable Length Record or in an auxiliary WPD file. The Byte Offset represents the
   * location of the start of this LIDAR points’ waveform packet within the waveform data variable
   * length record (or external file) relative to the beginning of the Waveform Packet Data header. The
   * absolute location of the beginning of this waveform packet relative to the beginning of the file is
   * given by:
   * - Start of Waveform Data Packet Record + Byte offset to Waveform Packet Data
   * for waveform packets stored within the LAS file
   * - Byte offset to Waveform Packet Data for data stored in an auxiliary file
   */
  wavePacketOffset: number;
  /**
   * 4 bytes
   *
   * Waveform packet size in bytes: The size, in bytes, of the waveform packet associated with this
   * return. Note that each waveform can be of a different size (even those with the same Waveform
   * Packet Descriptor index) due to packet compression. Also note that waveform packets can be
   * located only via the Byte offset to Waveform Packet Data value since there is no requirement that
   * records be stored sequentially.
   */
  wavePacketLength: number;
  /**
   * 4 bytes
   *
   * Return Point location: The offset in picoseconds (10-12) from the first digitized value to the
   * location within the waveform packet that the associated return pulse was detected.
   */
  waveformLocationReturnPoint: number;
  /**
   * 4 bytes
   *
   * this parameter defines a parametric line equation for extrapolating points
   * along the associated waveform. The position along the wave is given by:
   * `X = X0 + X(t)` where `X` is the spatial position of the derived point, `X0` is the position
   * of the "anchor" point, and t is the time, in picoseconds, relative to the anchor point
   * (i.e. t = zero at the anchor point). The units of X, Y and Z are the units of the coordinate
   * systems of the LAS data. If the coordinate system is geographic, the horizontal units are
   * decimal degrees and the vertical units are meters.
   */
  xT: number;
  /**
   * 4 bytes
   *
   * this parameter defines a parametric line equation for extrapolating points
   * along the associated waveform. The position along the wave is given by:
   * `Y = Y0 + Y(t)` where `Y` is the spatial position of the derived point, `Y0` is the position
   * of the "anchor" point, and t is the time, in picoseconds, relative to the anchor point
   * (i.e. t = zero at the anchor point). The units of X, Y and Z are the units of the coordinate
   * systems of the LAS data. If the coordinate system is geographic, the horizontal units are
   * decimal degrees and the vertical units are meters.
   */
  yT: number;
  /**
   * 4 bytes
   *
   * this parameter defines a parametric line equation for extrapolating points
   * along the associated waveform. The position along the wave is given by:
   * `Z = Z0 + Z(t)` where `Z` is the spatial position of the derived point, `Z0` is the position
   * of the "anchor" point, and t is the time, in picoseconds, relative to the anchor point
   * (i.e. t = zero at the anchor point). The units of X, Y and Z are the units of the coordinate
   * systems of the LAS data. If the coordinate system is geographic, the horizontal units are
   * decimal degrees and the vertical units are meters.
   */
  zT: number;
}

/** Point Data Record Format 10 adds Wave Packets to Point Data Record Format 7. */
export interface LASFormat10 extends LASFormat7 {
  /**
   * 1 byte
   *
   * Wave Packet Descriptor Index: This value plus 99 is the Record ID of the Waveform Packet
   * Descriptor and indicates the User Defined Record that describes the waveform packet associated
   * with this LIDAR point. Up to 255 different User Defined Records which describe the waveform
   * packet are supported. A value of zero indicates that there is no waveform data associated with
   * this LIDAR point record.
   */
  wavePacketDescriptorIndex: number;
  /**
   * 8 bytes
   *
   * Byte offset to Waveform Packet Data: The waveform packet data are stored in the LAS file in
   * an Extended Variable Length Record or in an auxiliary WPD file. The Byte Offset represents the
   * location of the start of this LIDAR points’ waveform packet within the waveform data variable
   * length record (or external file) relative to the beginning of the Waveform Packet Data header. The
   * absolute location of the beginning of this waveform packet relative to the beginning of the file is
   * given by:
   * - Start of Waveform Data Packet Record + Byte offset to Waveform Packet Data
   * for waveform packets stored within the LAS file
   * - Byte offset to Waveform Packet Data for data stored in an auxiliary file
   */
  wavePacketOffset: number;
  /**
   * 4 bytes
   *
   * Waveform packet size in bytes: The size, in bytes, of the waveform packet associated with this
   * return. Note that each waveform can be of a different size (even those with the same Waveform
   * Packet Descriptor index) due to packet compression. Also note that waveform packets can be
   * located only via the Byte offset to Waveform Packet Data value since there is no requirement that
   * records be stored sequentially.
   */
  wavePacketLength: number;
  /**
   * 4 bytes
   *
   * Return Point location: The offset in picoseconds (10-12) from the first digitized value to the
   * location within the waveform packet that the associated return pulse was detected.
   */
  waveformLocationReturnPoint: number;
  /**
   * 4 bytes
   *
   * this parameter defines a parametric line equation for extrapolating points
   * along the associated waveform. The position along the wave is given by:
   * `X = X0 + X(t)` where `X` is the spatial position of the derived point, `X0` is the position
   * of the "anchor" point, and t is the time, in picoseconds, relative to the anchor point
   * (i.e. t = zero at the anchor point). The units of X, Y and Z are the units of the coordinate
   * systems of the LAS data. If the coordinate system is geographic, the horizontal units are
   * decimal degrees and the vertical units are meters.
   */
  xT: number;
  /**
   * 4 bytes
   *
   * this parameter defines a parametric line equation for extrapolating points
   * along the associated waveform. The position along the wave is given by:
   * `Y = Y0 + Y(t)` where `Y` is the spatial position of the derived point, `Y0` is the position
   * of the "anchor" point, and t is the time, in picoseconds, relative to the anchor point
   * (i.e. t = zero at the anchor point). The units of X, Y and Z are the units of the coordinate
   * systems of the LAS data. If the coordinate system is geographic, the horizontal units are
   * decimal degrees and the vertical units are meters.
   */
  yT: number;
  /**
   * 4 bytes
   *
   * this parameter defines a parametric line equation for extrapolating points
   * along the associated waveform. The position along the wave is given by:
   * `Z = Z0 + Z(t)` where `Z` is the spatial position of the derived point, `Z0` is the position
   * of the "anchor" point, and t is the time, in picoseconds, relative to the anchor point
   * (i.e. t = zero at the anchor point). The units of X, Y and Z are the units of the coordinate
   * systems of the LAS data. If the coordinate system is geographic, the horizontal units are
   * decimal degrees and the vertical units are meters.
   */
  zT: number;
}
