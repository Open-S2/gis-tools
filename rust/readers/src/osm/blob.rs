use alloc::{string::String, vec::Vec};
use pbf::{ProtoRead, Protobuf};
use util::{CompressionFormat::DeflateRaw, decompress_data};

/// headers have a max size of 64KB
pub const OSM_MAX_HEADER_SIZE: usize = 64 * 1024;
/// blobs have a max size of 32MB
pub const OSM_MAX_BLOB_SIZE: usize = 32 * 1024 * 1024;

/// A file contains an sequence of fileblock headers, each prefixed by
/// their length in network byte order, followed by a data block
/// containing the actual data. Types starting with a "_" are reserved.
/// example: { type: 'OSMHeader', indexdata: null, datasize: 173 }
#[derive(Debug, Default)]
pub struct BlobHeader {
    /// The type of the blob
    pub _type: String,
    /// The index data
    pub indexdata: Vec<u8>,
    /// The size of the data
    pub datasize: u64,
}
/// Read in the contents of the blob header
impl ProtoRead for BlobHeader {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => self._type = pb.read_string(),
            2 => self.indexdata = pb.read_bytes(),
            3 => self.datasize = pb.read_varint(),
            _ => panic!("unknown tag {}", tag),
        }
    }
}

///  STORAGE LAYER: Storing primitives.
/// A Blob is a data block containing the actual data.
#[derive(Debug, Default)]
pub struct Blob {
    /// The raw size (uncompressed length)
    pub raw_size: i32,
    /// The data
    pub data: Vec<u8>,
}
/// Read in the contents of the blob
impl ProtoRead for Blob {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        match tag {
            1 => self.data = pb.read_bytes(),
            2 => self.raw_size = pb.read_varint(),
            // 3 => self.data = decompress_fflate(&pb.read_bytes(), None).unwrap(),
            3 => self.data = decompress_data(&pb.read_bytes(), DeflateRaw).unwrap(),
            4..=6 => panic!("LZMA, bzip2 and LZ4 not supported"),
            _ => panic!("unknown tag {}", tag),
        }
    }
}
