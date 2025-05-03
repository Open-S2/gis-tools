use super::{PMDirectory, PMTileType};
use parsers::Buffer;
use s2json::Face;
use util::CompressionFormat;

/// Store entries for each Face
#[derive(Debug, Clone, Default, PartialEq)]
pub struct S2PMEntries {
    /// The entries for face 0
    pub face_0: PMDirectory,
    /// The entries for face 1
    pub face_1: PMDirectory,
    /// The entries for face 2
    pub face_2: PMDirectory,
    /// The entries for face 3
    pub face_3: PMDirectory,
    /// The entries for face 4
    pub face_4: PMDirectory,
    /// The entries for face 5
    pub face_5: PMDirectory,
}
impl S2PMEntries {
    /// Get the directory for the given face
    pub fn get(&self, face: Face) -> &PMDirectory {
        match face {
            Face::Face0 => &self.face_0,
            Face::Face1 => &self.face_1,
            Face::Face2 => &self.face_2,
            Face::Face3 => &self.face_3,
            Face::Face4 => &self.face_4,
            Face::Face5 => &self.face_5,
        }
    }

    /// Get the mutable directory for the given face
    pub fn get_mut(&mut self, face: Face) -> &mut PMDirectory {
        match face {
            Face::Face0 => &mut self.face_0,
            Face::Face1 => &mut self.face_1,
            Face::Face2 => &mut self.face_2,
            Face::Face3 => &mut self.face_3,
            Face::Face4 => &mut self.face_4,
            Face::Face5 => &mut self.face_5,
        }
    }

    /// Set the directory for the given face
    pub fn set_dir(&mut self, face: Face, dir: PMDirectory) {
        match face {
            Face::Face0 => self.face_0 = dir,
            Face::Face1 => self.face_1 = dir,
            Face::Face2 => self.face_2 = dir,
            Face::Face3 => self.face_3 = dir,
            Face::Face4 => self.face_4 = dir,
            Face::Face5 => self.face_5 = dir,
        }
    }
}

/// The S2PMTiles v1 header size in bytes
pub const S2_PM_HEADER_SIZE_BYTES: usize = 262;
/// The S2PMTiles v1 root directory size in bytes
pub const S2_PM_ROOT_SIZE: usize = 98_304;

/// S2PMTiles v3 header storing basic archive-level information.
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct S2PMHeader {
    /// True if this is an S2PMTiles v1, otherwise PMTiles v3
    pub is_s2: bool,
    /// versioning used for the s2-pmtiles spec
    pub version: u8,
    /// the offset in the archive of the root directory for Face 0
    pub root_directory_offset: u64,
    /// the length of the root directory for Face 0
    pub root_directory_length: u64,
    /// the offset in the archive of the JSON metadata
    pub metadata_offset: u64,
    /// the length of the JSON metadata
    pub metadata_length: u64,
    /// the offset in the archive of the leaf directory for Face 0
    pub leaf_directory_offset: u64,
    /// the length of the leaf directory
    pub leaf_directory_length: u64,
    /// the offset in the archive of the tile data
    pub data_offset: u64,
    /// the length of the tile data
    pub data_length: u64,
    /// number of tiles addressed
    pub n_addressed_tiles: u64,
    /// number of tile entries
    pub n_tile_entries: u64,
    /// number of tile contents
    pub n_tile_contents: u64,
    /// if the archive is clustered or not
    pub clustered: bool,
    /// if the archive is compressed or not
    /// NOTE: deprecated and only `Compression::None` is supported
    pub internal_compression: CompressionFormat,
    /// what kind of compression is used for the tile data
    pub tile_compression: CompressionFormat,
    /// what kind of compression is used for the metadata
    pub tile_type: PMTileType,
    /// the min zoom level
    pub min_zoom: u8,
    /// the max zoom level
    pub max_zoom: u8,
    /// the min longitude
    pub min_longitude: f32,
    /// the min latitude
    pub min_latitude: f32,
    /// the max longitude
    pub max_longitude: f32,
    /// the max latitude
    pub max_latitude: f32,
    /// the center zoom level
    pub center_zoom: u8,
    /// the center longitude
    pub center_longitude: f32,
    /// the center latitude
    pub center_latitude: f32,
    /// the offset in the archive of the root directory for Face 1
    pub root_directory_offset1: u64,
    /// the length of the root directory for Face 1
    pub root_directory_length1: u64,
    /// the offset in the archive of the root directory for Face 2
    pub root_directory_offset2: u64,
    /// the length of the root directory for Face 2
    pub root_directory_length2: u64,
    /// the offset in the archive of the root directory for Face 3
    pub root_directory_offset3: u64,
    /// the length of the root directory for Face 3
    pub root_directory_length3: u64,
    /// the offset in the archive of the root directory for Face 4
    pub root_directory_offset4: u64,
    /// the length of the root directory for Face 4
    pub root_directory_length4: u64,
    /// the offset in the archive of the root directory for Face 5
    pub root_directory_offset5: u64,
    /// the length of the root directory for Face 5
    pub root_directory_length5: u64,
    /// the offset in the archive of the leaf directory for Face 1
    pub leaf_directory_offset1: u64,
    /// the length of the leaf directory for Face 1
    pub leaf_directory_length1: u64,
    /// the offset in the archive of the leaf directory for Face 2
    pub leaf_directory_offset2: u64,
    /// the length of the leaf directory for Face 2
    pub leaf_directory_length2: u64,
    /// the offset in the archive of the leaf directory for Face 3
    pub leaf_directory_offset3: u64,
    /// the length of the leaf directory for Face 3
    pub leaf_directory_length3: u64,
    /// the offset in the archive of the leaf directory for Face 4
    pub leaf_directory_offset4: u64,
    /// the length of the leaf directory for Face 4
    pub leaf_directory_length4: u64,
    /// the offset in the archive of the leaf directory for Face 5
    pub leaf_directory_offset5: u64,
    /// the length of the leaf directory for Face 5
    pub leaf_directory_length5: u64,
}
impl S2PMHeader {
    /// Convert a buffer into a S2Header
    pub fn from_bytes(buffer: &mut Buffer) -> Self {
        let ess = buffer.get_u8_at(0);
        let two = buffer.get_u8_at(1);
        let is_s2 = ess == 83 && two == 50;
        Self {
            is_s2,
            version: buffer.get_u8_at(7),
            root_directory_offset: buffer.get_u64_at(8),
            root_directory_length: buffer.get_u64_at(16),
            metadata_offset: buffer.get_u64_at(24),
            metadata_length: buffer.get_u64_at(32),
            leaf_directory_offset: buffer.get_u64_at(40),
            leaf_directory_length: buffer.get_u64_at(48),
            data_offset: buffer.get_u64_at(56),
            data_length: buffer.get_u64_at(64),
            n_addressed_tiles: buffer.get_u64_at(72),
            n_tile_entries: buffer.get_u64_at(80),
            n_tile_contents: buffer.get_u64_at(88),
            clustered: buffer.get_u8_at(96) == 1,
            internal_compression: CompressionFormat::from(buffer.get_u8_at(97)),
            tile_compression: CompressionFormat::from(buffer.get_u8_at(98)),
            tile_type: PMTileType::from(buffer.get_u8_at(99)),
            min_zoom: buffer.get_u8_at(100),
            max_zoom: buffer.get_u8_at(101),
            min_longitude: if is_s2 { 0.0 } else { (buffer.get_i32_at(102) as f32) / 10_000_000.0 },
            min_latitude: if is_s2 { 0.0 } else { (buffer.get_i32_at(106) as f32) / 10_000_000.0 },
            max_longitude: if is_s2 { 0.0 } else { (buffer.get_i32_at(110) as f32) / 10_000_000.0 },
            max_latitude: if is_s2 { 0.0 } else { (buffer.get_i32_at(114) as f32) / 10_000_000.0 },
            center_zoom: if is_s2 { 0 } else { buffer.get_u8_at(118) },
            center_longitude: if is_s2 {
                0.0
            } else {
                (buffer.get_i32_at(119) as f32) / 10_000_000.0
            },
            center_latitude: if is_s2 {
                0.0
            } else {
                (buffer.get_i32_at(123) as f32) / 10_000_000.0
            },
            root_directory_offset1: if is_s2 { buffer.get_u64_at(102) } else { 0 },
            root_directory_length1: if is_s2 { buffer.get_u64_at(110) } else { 0 },
            root_directory_length2: if is_s2 { buffer.get_u64_at(118) } else { 0 },
            root_directory_offset2: if is_s2 { buffer.get_u64_at(126) } else { 0 },
            root_directory_offset3: if is_s2 { buffer.get_u64_at(134) } else { 0 },
            root_directory_length3: if is_s2 { buffer.get_u64_at(142) } else { 0 },
            root_directory_offset4: if is_s2 { buffer.get_u64_at(150) } else { 0 },
            root_directory_length4: if is_s2 { buffer.get_u64_at(158) } else { 0 },
            root_directory_offset5: if is_s2 { buffer.get_u64_at(166) } else { 0 },
            root_directory_length5: if is_s2 { buffer.get_u64_at(174) } else { 0 },
            leaf_directory_offset1: if is_s2 { buffer.get_u64_at(182) } else { 0 },
            leaf_directory_length1: if is_s2 { buffer.get_u64_at(190) } else { 0 },
            leaf_directory_offset2: if is_s2 { buffer.get_u64_at(198) } else { 0 },
            leaf_directory_length2: if is_s2 { buffer.get_u64_at(206) } else { 0 },
            leaf_directory_offset3: if is_s2 { buffer.get_u64_at(214) } else { 0 },
            leaf_directory_length3: if is_s2 { buffer.get_u64_at(222) } else { 0 },
            leaf_directory_offset4: if is_s2 { buffer.get_u64_at(230) } else { 0 },
            leaf_directory_length4: if is_s2 { buffer.get_u64_at(238) } else { 0 },
            leaf_directory_offset5: if is_s2 { buffer.get_u64_at(246) } else { 0 },
            leaf_directory_length5: if is_s2 { buffer.get_u64_at(254) } else { 0 },
        }
    }

    /// Convert a S2Header into a buffer
    pub fn to_bytes(&self) -> Buffer {
        let mut buffer = Buffer::default();

        // default id
        buffer.set_u8(b'S');
        buffer.set_u8(b'2');
        // Version number at position 7
        buffer.set_u8_at(7, 1);

        // Root directory offset and length at positions 8 and 16
        buffer.set_u64_at(8, self.root_directory_offset);
        buffer.set_u64_at(16, self.root_directory_length);

        // JSON metadata offset and length at positions 24 and 32
        buffer.set_u64_at(24, self.metadata_offset);
        buffer.set_u64_at(32, self.metadata_length);

        // Leaf directory offset and optional length at positions 40 and 48
        buffer.set_u64_at(40, self.leaf_directory_offset);
        buffer.set_u64_at(48, self.leaf_directory_length);

        // Tile data offset and optional length at positions 56 and 64
        buffer.set_u64_at(56, self.data_offset);
        buffer.set_u64_at(64, self.data_length);

        // Number of addressed tiles, tile entries, and tile contents at positions 72, 80, and 88
        buffer.set_u64_at(72, self.n_addressed_tiles);
        buffer.set_u64_at(80, self.n_tile_entries);
        buffer.set_u64_at(88, self.n_tile_contents);

        // Flags and types at positions 96 through 101
        buffer.set_u8_at(96, if self.clustered { 1 } else { 0 });
        buffer.set_u8_at(97, self.internal_compression.into());
        buffer.set_u8_at(98, self.tile_compression.into());
        buffer.set_u8_at(99, self.tile_type.into());
        buffer.set_u8_at(100, self.min_zoom);
        buffer.set_u8_at(101, self.max_zoom);

        // set the remaining root directory offsets and lengths
        buffer.set_u64_at(102, self.root_directory_offset1);
        buffer.set_u64_at(110, self.root_directory_length1);
        buffer.set_u64_at(118, self.root_directory_length2);
        buffer.set_u64_at(126, self.root_directory_offset2);
        buffer.set_u64_at(134, self.root_directory_offset3);
        buffer.set_u64_at(142, self.root_directory_length3);
        buffer.set_u64_at(150, self.root_directory_offset4);
        buffer.set_u64_at(158, self.root_directory_length4);
        buffer.set_u64_at(166, self.root_directory_offset5);
        buffer.set_u64_at(174, self.root_directory_length5);

        // set the remaining leaf directory offsets and lengths
        buffer.set_u64_at(182, self.leaf_directory_offset1);
        buffer.set_u64_at(190, self.leaf_directory_length1);
        buffer.set_u64_at(198, self.leaf_directory_offset2);
        buffer.set_u64_at(206, self.leaf_directory_length2);
        buffer.set_u64_at(214, self.leaf_directory_offset3);
        buffer.set_u64_at(222, self.leaf_directory_length3);
        buffer.set_u64_at(230, self.leaf_directory_offset4);
        buffer.set_u64_at(238, self.leaf_directory_length4);
        buffer.set_u64_at(246, self.leaf_directory_offset5);
        buffer.set_u64_at(254, self.leaf_directory_length5);

        buffer
    }

    /// Get the root directory offset for a given face
    pub fn get_root_offset(&self, face: Face) -> u64 {
        match face {
            Face::Face0 => self.root_directory_offset,
            Face::Face1 => self.root_directory_offset1,
            Face::Face2 => self.root_directory_offset2,
            Face::Face3 => self.root_directory_offset3,
            Face::Face4 => self.root_directory_offset4,
            Face::Face5 => self.root_directory_offset5,
        }
    }

    /// Get the root directory length for a given face
    pub fn get_root_length(&self, face: Face) -> u64 {
        match face {
            Face::Face0 => self.root_directory_length,
            Face::Face1 => self.root_directory_length1,
            Face::Face2 => self.root_directory_length2,
            Face::Face3 => self.root_directory_length3,
            Face::Face4 => self.root_directory_length4,
            Face::Face5 => self.root_directory_length5,
        }
    }
}
