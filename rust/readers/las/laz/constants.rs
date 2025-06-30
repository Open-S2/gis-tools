/// LASZIP_DECOMPRESS_SELECTIVE
pub const LASZIP_DECOMPRESS_SELECTIVE_ALL: u32 = 0xffffffff;

// /// LASZIP_DECOMPRESS_SELECTIVE XY
// pub const LASZIP_DECOMPRESS_SELECTIVE_CHANNEL_RETURNS_XY: u32 = 0x00000000;
/// LASZIP_DECOMPRESS_SELECTIVE Z
pub const LASZIP_DECOMPRESS_SELECTIVE_Z: u32 = 0x00000001;
/// LASZIP_DECOMPRESS_SELECTIVE Classification
pub const LASZIP_DECOMPRESS_SELECTIVE_CLASSIFICATION: u32 = 0x00000002;
/// LASZIP_DECOMPRESS_SELECTIVE Flags
pub const LASZIP_DECOMPRESS_SELECTIVE_FLAGS: u32 = 0x00000004;
/// LASZIP_DECOMPRESS_SELECTIVE Intensity
pub const LASZIP_DECOMPRESS_SELECTIVE_INTENSITY: u32 = 0x00000008;
/// LASZIP_DECOMPRESS_SELECTIVE Scan Angle
pub const LASZIP_DECOMPRESS_SELECTIVE_SCAN_ANGLE: u32 = 0x00000010;
/// LASZIP_DECOMPRESS_SELECTIVE User Data
pub const LASZIP_DECOMPRESS_SELECTIVE_USER_DATA: u32 = 0x00000020;
/// LASZIP_DECOMPRESS_SELECTIVE Point Source
pub const LASZIP_DECOMPRESS_SELECTIVE_POINT_SOURCE: u32 = 0x00000040;
/// LASZIP_DECOMPRESS_SELECTIVE GPS Time
pub const LASZIP_DECOMPRESS_SELECTIVE_GPS_TIME: u32 = 0x00000080;
/// LASZIP_DECOMPRESS_SELECTIVE RGB
pub const LASZIP_DECOMPRESS_SELECTIVE_RGB: u32 = 0x00000100;
/// LASZIP_DECOMPRESS_SELECTIVE NIR
pub const LASZIP_DECOMPRESS_SELECTIVE_NIR: u32 = 0x00000200;
/// LASZIP_DECOMPRESS_SELECTIVE Wave Packet
pub const LASZIP_DECOMPRESS_SELECTIVE_WAVEPACKET: u32 = 0x00000400;
/// LASZIP_DECOMPRESS_SELECTIVE Bytes 0
pub const LASZIP_DECOMPRESS_SELECTIVE_BYTE0: u32 = 0x00010000;
// /// LASZIP_DECOMPRESS_SELECTIVE Bytes 1
// pub const LASZIP_DECOMPRESS_SELECTIVE_BYTE1: u32 = 0x00020000;
// /// LASZIP_DECOMPRESS_SELECTIVE Bytes 2
// pub const LASZIP_DECOMPRESS_SELECTIVE_BYTE2: u32 = 0x00040000;
// /// LASZIP_DECOMPRESS_SELECTIVE Bytes 3
// pub const LASZIP_DECOMPRESS_SELECTIVE_BYTE3: u32 = 0x00080000;
// /// LASZIP_DECOMPRESS_SELECTIVE Bytes 4
// pub const LASZIP_DECOMPRESS_SELECTIVE_BYTE4: u32 = 0x00100000;
// /// LASZIP_DECOMPRESS_SELECTIVE Bytes 5
// pub const LASZIP_DECOMPRESS_SELECTIVE_BYTE5: u32 = 0x00200000;
// /// LASZIP_DECOMPRESS_SELECTIVE Bytes 6
// pub const LASZIP_DECOMPRESS_SELECTIVE_BYTE6: u32 = 0x00400000;
// /// LASZIP_DECOMPRESS_SELECTIVE Bytes 7
// pub const LASZIP_DECOMPRESS_SELECTIVE_BYTE7: u32 = 0x00800000;
// /// LASZIP_DECOMPRESS_SELECTIVE Extra Bytes
// pub const LASZIP_DECOMPRESS_SELECTIVE_EXTRA_BYTES: u32 = 0xffff0000;

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
pub const NUMBER_RETURN_MAP: [[u8; 8]; 8] = [
    [15, 14, 13, 12, 11, 10, 9, 8],
    [14, 0, 1, 3, 6, 10, 10, 9],
    [13, 1, 2, 4, 7, 11, 11, 10],
    [12, 3, 4, 5, 8, 12, 12, 11],
    [11, 6, 7, 8, 9, 13, 13, 12],
    [10, 10, 11, 12, 13, 14, 14, 13],
    [9, 10, 11, 12, 13, 14, 15, 14],
    [8, 9, 10, 11, 12, 13, 14, 15],
];

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
pub const NUMBER_RETURN_LEVEL: [[u8; 8]; 8] = [
    [0, 1, 2, 3, 4, 5, 6, 7],
    [1, 0, 1, 2, 3, 4, 5, 6],
    [2, 1, 0, 1, 2, 3, 4, 5],
    [3, 2, 1, 0, 1, 2, 3, 4],
    [4, 3, 2, 1, 0, 1, 2, 3],
    [5, 4, 3, 2, 1, 0, 1, 2],
    [6, 5, 4, 3, 2, 1, 0, 1],
    [7, 6, 5, 4, 3, 2, 1, 0],
];

/** 6 context map. U8 c[16][16] */
pub const NUMBER_RETURN_MAP_6CTX: [[u8; 16]; 16] = [
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
];

/** 8 context map. U8 c[16][16] */
pub const NUMBER_RETURN_LEVEL_8CTX: [[u8; 16]; 16] = [
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
];
