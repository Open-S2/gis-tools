const std = @import("std");
const ArrayList = std.ArrayList;
const StringHashMap = std.StringHashMap;
const Protobuf = @import("pbf").Protobuf;
const info = @import("info.zig");
const Info = info.Info;
const DenseInfo = info.DenseInfo;
const PrimitiveBlock = @import("primitive.zig").PrimitiveBlock;
const Options = @import("main.zig").Options;
const json = @import("json");
const Feature = json.Feature;
const S2Feature = json.S2Feature;
const S2 = @import("S2");
const S2Latlon = S2.S2LatLng;
const XYZtoFaceST = S2.XYZtoFaceST;

pub fn NodeHandle(comptime C: type) type {
    return fn (ctx: C, node: *Node) anyerror!void;
}

/// Used when either requesting geometry or to store away into a
/// memory manager.
pub const NodeGeometry = packed struct {
    lon: f64,
    lat: f64,
};

/// Nodes are stored away into a memory manager. This Member is
/// created when a relation requests said node.
pub const NodeMember = struct {
    id: u64,
    lon: f64,
    lat: f64,
    propStr: ?[]const u8 = null,
    const Self = @This();

    pub fn Init(
        id_: i64,
        lat_: f64,
        lon_: f64,
        propStr_: ?[]const u8,
    ) NodeMember {
        return NodeMember{
            .id = @as(u64, @intCast(id_)),
            .lon = lon_,
            .lat = lat_,
            .propStr = propStr_,
        };
    }

    pub fn toGeoJSON(self: Self, arena: *std.heap.ArenaAllocator) !Feature {
        const allocator = arena.allocator();
        // coords
        var coords = try allocator.alloc(f64, 2);
        coords[0] = self.lon;
        coords[1] = self.lat;

        return Feature{
            .id = self.id,
            .geometry = json.Geometry{ .point = coords },
            .properties = try json.Value.FromString(self.propStr orelse "{}", arena),
        };
    }

    pub fn toS2JSON(self: Self, arena: *std.heap.ArenaAllocator) !S2Feature {
        const allocator = arena.allocator();
        // update coords to S2
        var coords = try allocator.alloc(f64, 2);
        var ll = S2Latlon.FromDegrees(self.lat, self.lon);
        var s2point = ll.ToPoint();
        const face: u3 = XYZtoFaceST(&s2point, &coords[0], &coords[1]);

        return S2Feature{
            .id = self.id,
            .face = face,
            .geometry = json.Geometry{ .s2point = coords },
            .properties = try json.Value.FromString(self.propStr orelse "{}", arena),
        };
    }
};

pub const Node = struct {
    id: i64 = -1,
    _info: ?Info = null,
    _keys: []usize = &[_]usize{},
    _vals: []usize = &[_]usize{},
    lat: f64 = 0.0,
    lon: f64 = 0.0,
    primitiveBlock: *PrimitiveBlock,
    const Self = @This();

    pub fn Init(pbf: *Protobuf, primitiveBlock_: *PrimitiveBlock) !Node {
        var node = Node{ .primitiveBlock = primitiveBlock_ };
        try pbf.readMessage(Node, &node, readLayer);
        return node;
    }

    pub fn FromDense(
        id_: i64,
        info_: ?Info,
        keys_: []usize,
        vals_: []usize,
        lat_: i64,
        lon_: i64,
        pb_: *PrimitiveBlock,
    ) Node {
        return Node{
            .id = id_,
            ._info = info_,
            ._keys = keys_,
            ._vals = vals_,
            .lat = 0.000000001 * @as(f64, @floatFromInt(pb_.lat_offset + (pb_.granularity * lat_))),
            .lon = 0.000000001 * @as(f64, @floatFromInt(pb_.lat_offset + (pb_.granularity * lon_))),
            .primitiveBlock = pb_,
        };
    }

    pub fn geometry(self: Self) NodeGeometry {
        return NodeGeometry{ .lon = self.lon, .lat = self.lat };
    }

    pub fn isFilterable(self: Self, options: *const Options) bool {
        if (options.removeEmptyNodes and self._keys.len == 0) return true;
        if (options.tagFilter != null) {
            var tagFilter = options.tagFilter.?;
            var i: usize = 0;
            while (i < self._keys.len) : (i += 1) {
                const keyStr = self.primitiveBlock.getString(self._keys[i]);
                const valStr = self.primitiveBlock.getString(self._vals[i]);
                if (tagFilter.matchFound(.Node, keyStr, valStr)) return false;
            }
            // if we make it here, we didn't find any matching tags
            return true;
        }
        return false;
    }

    pub fn info(self: Self) ?Info {
        if (self._info == null) return null;
        self._info.?.primitiveBlock = self.primitiveBlock;
        return self._info;
    }

    pub fn tags(self: Self, allocator: std.mem.Allocator) !StringHashMap([]const u8) {
        return try self.primitiveBlock.tags(self._keys, self._vals, allocator);
    }

    pub fn tagsAsJSONString(self: Self, allocator: std.mem.Allocator) !?[]const u8 {
        return try self.primitiveBlock.tagsAsJSONString(self._keys, self._vals, allocator);
    }

    pub fn hasKey(self: Self, key: []const u8) bool {
        var i: usize = 0;
        while (i < self._keys.len) : (i += 1) {
            if (std.mem.eql(u8, self.primitiveBlock.getString(self._keys[i]), key)) return true;
        }
        return false;
    }

    pub fn hasKeyValue(self: Self, key: []const u8, val: []const u8) bool {
        var i: usize = 0;
        while (i < self._keys.len) : (i += 1) {
            if (std.mem.eql(u8, self.primitiveBlock.getString(self._keys[i]), key) and
                std.mem.eql(u8, self.primitiveBlock.getString(self._vals[i]), val)) return true;
        }
        return false;
    }

    pub fn toGeoJSON(self: Self, arena: *std.heap.ArenaAllocator) !Feature {
        const allocator = arena.allocator();
        // id
        const id: ?u64 = if (self.id >= 0) @as(u64, @intCast(self.id)) else null;
        // properties
        const props = (try self.tagsAsJSONString(allocator)) orelse "{}";
        // coords
        var coords = try allocator.alloc(f64, 2);
        coords[0] = self.lon;
        coords[1] = self.lat;

        return Feature{
            .id = id,
            .geometry = json.Geometry{ .point = coords },
            .properties = try json.Value.FromString(props, arena),
        };
    }

    pub fn toS2JSON(self: Self, arena: *std.heap.ArenaAllocator) !S2Feature {
        const allocator = arena.allocator();
        // id
        const id: ?u64 = if (self.id >= 0) @as(u64, @intCast(self.id)) else null;
        // properties
        const props = (try self.tagsAsJSONString(allocator)) orelse "{}";
        // update coords to S2
        var coords = try allocator.alloc(f64, 2);
        var ll = S2Latlon.FromDegrees(self.lat, self.lon);
        var s2point = ll.ToPoint();
        const face: u3 = XYZtoFaceST(&s2point, &coords[0], &coords[1]);

        return S2Feature{
            .id = id,
            .face = face,
            .geometry = json.Geometry{ .s2point = coords },
            .properties = try json.Value.FromString(props, arena),
        };
    }

    fn readLayer(self: *Self, tag: u64, pbf: *Protobuf) !void {
        const pb = self.primitiveBlock;
        switch (tag) {
            1 => self.id = pbf.readVarint(i64),
            2 => self._keys = try pbf.readPacked(usize),
            3 => self._vals = try pbf.readPacked(usize),
            4 => self._info = try Info.Init(pbf, self.primitiveBlock),
            8 => self.lat = 0.000000001 * @as(f64, @floatFromInt(pb.lat_offset + (pb.granularity * pbf.readSVarint(i64)))),
            9 => self.lon = 0.000000001 * @as(f64, @floatFromInt(pb.lon_offset + (pb.granularity * pbf.readSVarint(i64)))),
            else => unreachable,
        }
    }
};

// Used to densly represent a sequence of nodes that do not have any tags.
// We represent these nodes columnwise as five columns: ID's, lats, and
// lons, all delta coded. When metadata is not omitted,
// We encode keys & vals for all nodes as a single array of integers
// containing key-stringid and val-stringid, using a stringid of 0 as a
// delimiter between nodes.
//    ( (<keyid> <valid>)* '0' )*
pub const DenseNodes = struct {
    _id: []i64 = undefined, // DELTA coded
    denseinfo: ?DenseInfo = null,
    _lat: []i64 = undefined, // DELTA coded
    _lon: []i64 = undefined, // DELTA coded
    // Special packing of keys and vals into one array. May be empty if all nodes in this block are tagless.
    keys_vals: []usize = undefined,
    primitiveBlock: *PrimitiveBlock,
    const Self = @This();

    pub fn Init(pbf: *Protobuf, primitiveBlock_: *PrimitiveBlock) !DenseNodes {
        var denseNodes = DenseNodes{ .primitiveBlock = primitiveBlock_ };
        try pbf.readMessage(DenseNodes, &denseNodes, readLayer);
        return denseNodes;
    }

    pub fn nodes(self: Self, allocator: std.mem.Allocator) ![]Node {
        var res = try ArrayList(Node).initCapacity(allocator, self._id.len);
        const infoMap = if (self.denseinfo != null)
            try self.denseinfo.?.infos(allocator)
        else
            null;
        var i: usize = 0;
        var j: usize = 0;
        var curId: i64 = 0;
        var curLat: i64 = 0;
        var curLon: i64 = 0;
        while (i < self._id.len) : (i += 1) {
            const curInfo: ?Info = if (infoMap != null)
                infoMap.?[i]
            else
                null;
            curId += self._id[i];
            curLat += self._lat[i];
            curLon += self._lon[i];
            var keys = ArrayList(usize).init(allocator);
            var vals = ArrayList(usize).init(allocator);
            if (self.keys_vals.len > 0) {
                while (self.keys_vals[j] != 0) : (j += 2) {
                    try keys.append(self.keys_vals[j]);
                    try vals.append(self.keys_vals[j + 1]);
                }
                j += 1;
            }
            const node = Node.FromDense(
                curId,
                curInfo,
                try keys.toOwnedSlice(),
                try vals.toOwnedSlice(),
                curLat,
                curLon,
                self.primitiveBlock,
            );
            res.appendAssumeCapacity(node);
        }

        return try res.toOwnedSlice();
    }

    fn readLayer(self: *Self, tag: u64, pbf: *Protobuf) !void {
        switch (tag) {
            1 => self._id = try pbf.readPackedS(i64),
            5 => self.denseinfo = try DenseInfo.Init(pbf, self.primitiveBlock),
            8 => self._lat = try pbf.readPackedS(i64),
            9 => self._lon = try pbf.readPackedS(i64),
            10 => self.keys_vals = try pbf.readPacked(usize),
            else => unreachable,
        }
    }
};
