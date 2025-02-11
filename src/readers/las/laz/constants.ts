export const LASZIP_DECOMPRESS_SELECTIVE_ALL = 0xffffffff;

export const LASZIP_DECOMPRESS_SELECTIVE_CHANNEL_RETURNS_XY = 0x00000000;
export const LASZIP_DECOMPRESS_SELECTIVE_Z = 0x00000001;
export const LASZIP_DECOMPRESS_SELECTIVE_CLASSIFICATION = 0x00000002;
export const LASZIP_DECOMPRESS_SELECTIVE_FLAGS = 0x00000004;
export const LASZIP_DECOMPRESS_SELECTIVE_INTENSITY = 0x00000008;
export const LASZIP_DECOMPRESS_SELECTIVE_SCAN_ANGLE = 0x00000010;
export const LASZIP_DECOMPRESS_SELECTIVE_USER_DATA = 0x00000020;
export const LASZIP_DECOMPRESS_SELECTIVE_POINT_SOURCE = 0x00000040;
export const LASZIP_DECOMPRESS_SELECTIVE_GPS_TIME = 0x00000080;
export const LASZIP_DECOMPRESS_SELECTIVE_RGB = 0x00000100;
export const LASZIP_DECOMPRESS_SELECTIVE_NIR = 0x00000200;
export const LASZIP_DECOMPRESS_SELECTIVE_WAVEPACKET = 0x00000400;
export const LASZIP_DECOMPRESS_SELECTIVE_BYTE0 = 0x00010000;
export const LASZIP_DECOMPRESS_SELECTIVE_BYTE1 = 0x00020000;
export const LASZIP_DECOMPRESS_SELECTIVE_BYTE2 = 0x00040000;
export const LASZIP_DECOMPRESS_SELECTIVE_BYTE3 = 0x00080000;
export const LASZIP_DECOMPRESS_SELECTIVE_BYTE4 = 0x00100000;
export const LASZIP_DECOMPRESS_SELECTIVE_BYTE5 = 0x00200000;
export const LASZIP_DECOMPRESS_SELECTIVE_BYTE6 = 0x00400000;
export const LASZIP_DECOMPRESS_SELECTIVE_BYTE7 = 0x00800000;
export const LASZIP_DECOMPRESS_SELECTIVE_EXTRA_BYTES = 0xffff0000;

export const LASZIP_GPSTIME_MULTIMAX = 512;

export const LASZIP_GPSTIME_MULTI = 500;
export const LASZIP_GPSTIME_MULTI_MINUS = -10;
export const LASZIP_GPSTIME_MULTI_CODE_FULL = 511; // (LASZIP_GPSTIME_MULTI - LASZIP_GPSTIME_MULTI_MINUS + 1)

export const LASZIP_GPSTIME_MULTI_TOTAL = 515; // (LASZIP_GPSTIME_MULTI - LASZIP_GPSTIME_MULTI_MINUS + 5)

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
export const numberReturnMap = [
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
export const numberReturnLevel = [
  [0, 1, 2, 3, 4, 5, 6, 7],
  [1, 0, 1, 2, 3, 4, 5, 6],
  [2, 1, 0, 1, 2, 3, 4, 5],
  [3, 2, 1, 0, 1, 2, 3, 4],
  [4, 3, 2, 1, 0, 1, 2, 3],
  [5, 4, 3, 2, 1, 0, 1, 2],
  [6, 5, 4, 3, 2, 1, 0, 1],
  [7, 6, 5, 4, 3, 2, 1, 0],
] as const;

/** 6 context map. U8 c[16][16] */
export const number_return_map_6ctx = [
  [0, 1, 2, 3, 4, 5, 3, 4, 4, 5, 5, 5, 5, 5, 5, 5],
  [1, 0, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
  [2, 1, 2, 4, 4, 4, 4, 4, 4, 4, 4, 3, 3, 3, 3, 3],
  [3, 3, 4, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4],
  [4, 3, 4, 4, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4],
  [5, 3, 4, 4, 4, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4],
  [3, 3, 4, 4, 4, 4, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4],
  [4, 3, 4, 4, 4, 4, 4, 5, 4, 4, 4, 4, 4, 4, 4, 4],
  [4, 3, 4, 4, 4, 4, 4, 4, 5, 4, 4, 4, 4, 4, 4, 4],
  [5, 3, 4, 4, 4, 4, 4, 4, 4, 5, 4, 4, 4, 4, 4, 4],
  [5, 3, 4, 4, 4, 4, 4, 4, 4, 4, 5, 4, 4, 4, 4, 4],
  [5, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 4, 4, 4],
  [5, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 4, 4],
  [5, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 4],
  [5, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5],
  [5, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5],
] as const;

/** 8 context map. U8 c[16][16] */
export const number_return_level_8ctx = [
  [0, 1, 2, 3, 4, 5, 6, 7, 7, 7, 7, 7, 7, 7, 7, 7],
  [1, 0, 1, 2, 3, 4, 5, 6, 7, 7, 7, 7, 7, 7, 7, 7],
  [2, 1, 0, 1, 2, 3, 4, 5, 6, 7, 7, 7, 7, 7, 7, 7],
  [3, 2, 1, 0, 1, 2, 3, 4, 5, 6, 7, 7, 7, 7, 7, 7],
  [4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 6, 7, 7, 7, 7, 7],
  [5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 6, 7, 7, 7, 7],
  [6, 5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 6, 7, 7, 7],
  [7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 6, 7, 7],
  [7, 7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 6, 7],
  [7, 7, 7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 6],
  [7, 7, 7, 7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5],
  [7, 7, 7, 7, 7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 3, 4],
  [7, 7, 7, 7, 7, 7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 3],
  [7, 7, 7, 7, 7, 7, 7, 6, 5, 4, 3, 2, 1, 0, 1, 2],
  [7, 7, 7, 7, 7, 7, 7, 7, 6, 5, 4, 3, 2, 1, 0, 1],
  [7, 7, 7, 7, 7, 7, 7, 7, 7, 6, 5, 4, 3, 2, 1, 0],
] as const;
