const std = @import("std");
const Protobuf = @import("pbf").Protobuf;
const Node = @import("node.zig").Node;
const PrimitiveBlock = @import("primitive.zig").PrimitiveBlock;

// Optional metadata that may be included into each primitive.
pub const Info = struct {
    version: ?i32 = null,
    _timestamp: ?i64 = null, // (millisec_stamp = timestamp*date_granularity.)
    _changeset: ?i64 = null,
    uid: ?i32 = null,
    user_sid: ?i32 = null, // String IDs for usernames.
    // The visible flag is used to store history information. It indicates that
    // the current object version has been created by a delete operation on the
    // OSM API.
    // When a writer sets this flag, it MUST add a required_features tag with
    // value "HistoricalInformation" to the HeaderBlock.
    // If this flag is not available for some object it MUST be assumed to be
    // true if the file has the required_features tag "HistoricalInformation"
    // set.
    visible: ?bool = null,
    primitiveBlock: *PrimitiveBlock,
    const Self = @This();

    pub fn Init(pbf: *Protobuf, primitiveBlock_: *PrimitiveBlock) !Info {
        var info = Info{ .primitiveBlock = primitiveBlock_ };
        try pbf.readMessage(Info, &info, readLayer);
        return info;
    }

    pub fn FromDense(
        version_: i32,
        timestamp_: i64,
        changeset_: i64,
        uid_: i32,
        user_sid_: i32,
        visible_: ?bool,
        primitiveBlock_: *PrimitiveBlock,
    ) Info {
        return Info{
            .version = version_,
            ._timestamp = timestamp_,
            ._changeset = changeset_,
            .uid = uid_,
            .user_sid = user_sid_,
            .visible = visible_,
            .primitiveBlock = primitiveBlock_,
        };
    }

    pub fn timeStamp(self: *Self) ?i64 {
        if (self._timestamp == null) return null;
        return self._timestamp + self.primitiveBlock.date_granularity;
    }

    pub fn changeset(self: *Self) ?i64 {
        if (self._changeset == null) return null;
        return self.primitiveBlock.stringtable.get(self._changeset);
    }

    pub fn user(self: *Self) ?[]const u8 {
        if (self.user_sid == null) return null;
        return self.primitiveBlock.stringtable.get(self.user_sid);
    }

    fn readLayer(self: *Self, tag: u64, pbf: *Protobuf) !void {
        switch (tag) {
            1 => self.version = pbf.readVarint(i32),
            2 => self._timestamp = pbf.readVarint(i64),
            3 => self._changeset = pbf.readVarint(i64),
            4 => self.uid = pbf.readSVarint(i32),
            5 => self.user_sid = pbf.readVarint(i32),
            6 => self.visible = pbf.readVarint(bool),
            else => unreachable,
        }
    }
};

/// DenseInfo
pub const DenseInfo = struct {
    version: []i32 = undefined,
    _timestamp: []i64 = undefined, // DELTA coded (millisec_stamp = timestamp*date_granularity.)
    _changeset: []i64 = undefined,
    uid: []i32 = undefined, // DELTA coded
    user_sid: []i32 = undefined, // String IDs for usernames. DELTA coded
    // The visible flag is used to store history information. It indicates that
    // the current object version has been created by a delete operation on the
    // OSM API.
    // When a writer sets this flag, it MUST add a required_features tag with
    // value "HistoricalInformation" to the HeaderBlock.
    // If this flag is not available for some object it MUST be assumed to be
    // true if the file has the required_features tag "HistoricalInformation"
    // set.
    visible: ?[]bool = null,
    primitiveBlock: *PrimitiveBlock,
    const Self = @This();

    pub fn Init(pbf: *Protobuf, primitiveBlock_: *PrimitiveBlock) !DenseInfo {
        var denseNode = DenseInfo{ .primitiveBlock = primitiveBlock_ };
        try pbf.readMessage(DenseInfo, &denseNode, readLayer);
        return denseNode;
    }

    pub fn infos(self: Self, allocator: std.mem.Allocator) ![]Info {
        var res = try allocator.alloc(Info, self.version.len);
        var i: usize = 0;
        var curTimestamp: i64 = 0;
        var curUid: i32 = 0;
        while (i < self.version.len) : (i += 1) {
            curTimestamp += self._timestamp[i];
            curUid += self.uid[i];
            res[i] = Info.FromDense(
                self.version[i],
                curTimestamp,
                self._changeset[i],
                curUid,
                self.user_sid[i],
                self.getVisible(i),
                self.primitiveBlock,
            );
        }
        return res;
    }

    fn getVisible(self: Self, i: usize) ?bool {
        if (self.visible == null) return null;
        return self.visible.?[i];
    }

    fn readLayer(self: *Self, tag: u64, pbf: *Protobuf) !void {
        switch (tag) {
            1 => self.version = try pbf.readPacked(i32),
            2 => self._timestamp = try pbf.readPackedS(i64),
            3 => self._changeset = try pbf.readPackedS(i64),
            4 => self.uid = try pbf.readPackedS(i32),
            5 => self.user_sid = try pbf.readPackedS(i32),
            6 => self.visible = try pbf.readPacked(bool),
            else => unreachable,
        }
    }
};
