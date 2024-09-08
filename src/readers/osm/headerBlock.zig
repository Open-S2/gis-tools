const std = @import("std");
const ArrayList = std.ArrayList;
const Protobuf = @import("pbf").Protobuf;
const Info = @import("info.zig").Info;

pub const HeaderBlock = struct {
    bbox: ?HeaderBBox = null,
    // Additional tags to aid in parsing this dataset
    required_features: ArrayList([]const u8),
    optional_features: ArrayList([]const u8),
    writingprogram: ?[]const u8 = null,
    source: ?[]const u8 = null,
    // Tags that allow continuing an Osmosis replication
    // Replication timestamp, expressed in seconds since the epoch,
    // otherwise the same value as in the "timestamp=..." field
    // in the state.txt file used by Osmosis.
    osmosis_replication_timestamp: i64 = -1,
    // Replication sequence number (sequenceNumber in state.txt).
    osmosis_replication_sequence_number: i64,
    // Replication base URL (from Osmosis' configuration.txt file).
    osmosis_replication_base_url: ?[]const u8 = null,
    const Self = @This();

    pub fn Init(pbf: *Protobuf) !HeaderBlock {
        const allocator = pbf.arena.allocator();
        var headerBlock = HeaderBlock{
            .required_features = ArrayList([]const u8).init(allocator),
            .optional_features = ArrayList([]const u8).init(allocator),
        };
        try pbf.readMessage(HeaderBlock, &headerBlock, readLayer);
        return headerBlock;
    }

    fn readLayer(self: *Self, tag: u64, pbf: *Protobuf) !void {
        switch (tag) {
            1 => self.bbox = try HeaderBBox.Init(pbf),
            4 => try self.required_features.append(pbf.readString()),
            5 => try self.optional_features.append(pbf.readString()),
            16 => self.writingprogram = try pbf.readString(),
            17 => self.source = try pbf.readString(),
            32 => self.osmosis_replication_timestamp = pbf.readVarint(i64),
            33 => self.osmosis_replication_sequence_number = pbf.readVarint(i64),
            34 => self.osmosis_replication_base_url = try pbf.readString(),
            else => unreachable,
        }
    }
};

/// The bounding box field in the OSM header. BBOX, as used in the OSM
/// header. Units are always in nanodegrees -- they do not obey
/// granularity rules.
pub const HeaderBBox = struct {
    left: i64 = -1,
    right: i64 = -1,
    top: i64 = -1,
    bottom: i64 = -1,
    const Self = @This();

    pub fn Init(pbf: *Protobuf) !HeaderBBox {
        var headerBBox = HeaderBBox{};
        try pbf.readMessage(HeaderBBox, &headerBBox, readLayer);
        return headerBBox;
    }

    fn readLayer(self: *Self, tag: u64, pbf: *Protobuf) !void {
        switch (tag) {
            1 => self.left = pbf.readVarint(i64),
            2 => self.right = pbf.readVarint(i64),
            3 => self.top = pbf.readVarint(i64),
            4 => self.bottom = pbf.readVarint(i64),
            else => unreachable,
        }
    }
};
