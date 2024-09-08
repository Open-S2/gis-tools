const std = @import("std");
const io = std.io;
const zlib = std.compress.zlib;
const lzma = std.compress.lzma;
const zstd = std.compress.zstd;
const Protobuf = @import("pbf").Protobuf;
const ArrayList = std.ArrayList;

pub const MAX_HEADER_SIZE = 64 * 1024;

// blobs have a max size of 32MB
pub const MAX_BLOB_SIZE = 32 * 1024 * 1024;

const BlobError = error{
    Deprecated,
    Unsupported,
};

const DecodeType = enum {
    Zlib,
    Lzma,
    Zstd,
};

/// A file contains an sequence of fileblock headers, each prefixed by
/// their length in network byte order, followed by a data block
/// containing the actual data. Types starting with a "_" are reserved.
/// example: { type: 'OSMHeader', indexdata: null, datasize: 173 }
pub const BlobHeader = struct {
    type: []const u8 = undefined,
    indexdata: ?[]u8 = null,
    datasize: i32 = undefined,
    const Self = @This();

    pub fn Init(pbf: *Protobuf) !BlobHeader {
        var blobHeader = BlobHeader{};
        try pbf.readFields(BlobHeader, &blobHeader, BlobHeader.readLayer);
        return blobHeader;
    }

    fn readLayer(self: *Self, tag: u64, pbf: *Protobuf) !void {
        switch (tag) {
            1 => self.type = pbf.readString(),
            2 => self.indexdata = pbf.readBytes(),
            3 => self.datasize = pbf.readVarint(i32),
            else => unreachable,
        }
    }
};

///  STORAGE LAYER: Storing primitives.
pub const Blob = struct {
    raw_size: ?i32 = null,
    data: []u8 = undefined,
    const Self = @This();

    pub fn Init(pbf: *Protobuf) !Blob {
        var blob = Blob{};
        try pbf.readFields(Blob, &blob, Blob.readLayer);
        return blob;
    }

    fn readLayer(self: *Self, tag: u64, pbf: *Protobuf) !void {
        switch (tag) {
            // No compression
            1 => self.data = pbf.readBytes(),
            2 => self.raw_size = pbf.readVarint(i32),
            // ZLIB:
            3 => self.data = try decompress(pbf.readBytes(), DecodeType.Zlib, pbf.arena.allocator()),
            // For LZMA compressed data (optional)
            4 => self.data = try decompress(pbf.readBytes(), DecodeType.Lzma, pbf.arena.allocator()),
            // Formerly used for bzip2 compressed data. Deprecated in 2010.
            5 => return BlobError.Deprecated,
            // NOTE: since Zig currently doesn't have an LZ4 implementation, we can't support this
            // For LZ4 compressed data (optional)
            6 => return BlobError.Unsupported,
            // For ZSTD compressed data (optional)
            7 => self.data = try decompress(pbf.readBytes(), DecodeType.Zstd, pbf.arena.allocator()),
            else => unreachable,
        }
    }
};

fn decompress(data: []u8, decodeType: DecodeType, allocator: std.mem.Allocator) ![]u8 {
    var inStream = io.fixedBufferStream(data);
    switch (decodeType) {
        .Zlib => {
            var decompressStream = try zlib.zlibStream(allocator, inStream.reader());
            defer decompressStream.deinit();
            return try decompressStream.reader().readAllAlloc(allocator, MAX_BLOB_SIZE);
        },
        .Lzma => {
            var decompressStream = try lzma.decompress(allocator, inStream.reader());
            defer decompressStream.deinit();
            return try decompressStream.reader().readAllAlloc(allocator, MAX_BLOB_SIZE);
        },
        .Zstd => {
            var decompressStream = zstd.decompressStream(allocator, inStream.reader());
            defer decompressStream.deinit();
            return try decompressStream.reader().readAllAlloc(allocator, MAX_BLOB_SIZE);
        },
    }
}
