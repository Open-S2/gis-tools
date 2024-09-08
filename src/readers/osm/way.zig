const std = @import("std");
const StringHashMap = std.StringHashMap;
const Protobuf = @import("pbf").Protobuf;
const ArrayList = std.ArrayList;
const Info = @import("info.zig").Info;
const Node = @import("node.zig").Node;
const PrimitiveBlock = @import("primitive.zig").PrimitiveBlock;
const Options = @import("main.zig").Options;
const json = @import("json");
const Feature = json.Feature;
const S2Feature = json.S2Feature;
const convertGeoJSON = @import("../vectors/convert.zig").convertGeoJSON;

pub fn WayHandle(comptime C: type) type {
    return fn (ctx: C, node: *Way) anyerror!void;
}

pub const WayMember = struct {
    id: u64 = 0,
    _refs: []i64,
    propStr: ?[]const u8 = null,
    primitiveBlock: *PrimitiveBlock = undefined,
    const Self = @This();

    pub fn Init(id: i64, refs: []i64, propStr: ?[]const u8) WayMember {
        return WayMember{
            .id = @as(u64, @intCast(id)),
            ._refs = refs,
            .propStr = propStr,
        };
    }

    pub fn nodeRefs(self: Self, allocator: std.mem.Allocator) ![]u64 {
        var res = try ArrayList(u64).initCapacity(allocator, self._refs.len);
        var i: usize = 0;
        var ref: i64 = 0;
        while (i < self._refs.len) : (i += 1) {
            ref += self._refs[i];
            res.appendAssumeCapacity(@as(u64, @intCast(ref)));
        }
        return try res.toOwnedSlice();
    }

    pub fn nodes(self: Self, allocator: std.mem.Allocator) !?json.LineString {
        var res = try ArrayList([]f64).initCapacity(allocator, self._refs.len);
        var i: usize = 0;
        var ref: i64 = 0;
        while (i < self._refs.len) : (i += 1) {
            ref += self._refs[i];
            const node = try self.primitiveBlock.getNode(ref);
            if (node == null) return null;
            var point = try allocator.alloc(f64, 2);
            point[0] = node.?.lon;
            point[1] = node.?.lat;
            res.appendAssumeCapacity(point);
        }
        return try res.toOwnedSlice();
    }

    pub fn isArea(self: Self, maybeArea: bool) bool {
        if (maybeArea and self._refs.len >= 4 and
            self._refs[0] == self._refs[self._refs.len - 1])
        {
            return true;
        }
        return false;
    }

    pub fn geometry(self: Self, arena: *std.heap.ArenaAllocator, maybeArea: bool, areaYes: bool) !?json.Geometry {
        const allocator = arena.allocator();
        const nodes_ = try self.nodes(allocator);
        if (nodes_ == null) return null;
        if (areaYes or self.isArea(maybeArea)) {
            var poly = try allocator.alloc([][]f64, 1);
            poly[0] = nodes_.?;
            return json.Geometry{ .polygon = poly };
        }
        return json.Geometry{ .lineString = nodes_.? };
    }

    pub fn toGeoJSON(self: Self, arena: *std.heap.ArenaAllocator, maybeArea: bool) !?Feature {
        var props = try json.Value.FromString(self.propStr orelse "{}", arena);
        const geo = try self.geometry(arena, maybeArea, props.containsPair("area", "yes"));
        if (geo == null) return null;

        return Feature{
            .id = self.id,
            .geometry = geo.?,
            .properties = props,
        };
    }

    pub fn toS2JSON(self: Self, arena: *std.heap.ArenaAllocator, maybeArea: bool) !?[]S2Feature {
        var geojson = try self.toGeoJSON(arena, maybeArea);
        if (geojson == null) return null;

        return try convertGeoJSON(&(geojson.?), 0.125, arena);
    }
};

pub const Way = struct {
    id: i64 = -1,
    _info: ?Info = null,
    // Parallel arrays
    _keys: []usize = &[_]usize{},
    _vals: []usize = &[_]usize{},
    _refs: []i64 = &[_]i64{}, // DELTA coded
    // Optional infield lat-lon
    // NOTE: I'm not going to bother implementing this.
    _lats: ?[]i64 = null, // optional DELTA coded
    _lons: ?[]i64 = null, // optional DELTA coded
    primitiveBlock: *PrimitiveBlock,
    const Self = @This();

    pub fn Init(pbf: *Protobuf, primitiveBlock_: *PrimitiveBlock) !Way {
        var way = Way{ .primitiveBlock = primitiveBlock_ };
        try pbf.readMessage(Way, &way, Way.readLayer);
        return way;
    }

    pub fn isFilterable(self: Self, options: *const Options) bool {
        if (options.tagFilter != null) {
            var tagFilter = options.tagFilter.?;
            var i: usize = 0;
            while (i < self._keys.len) : (i += 1) {
                const keyStr = self.primitiveBlock.getString(self._keys[i]);
                const valStr = self.primitiveBlock.getString(self._vals[i]);
                if (tagFilter.matchFound(.Way, keyStr, valStr)) return false;
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

    pub fn nodeRefs(self: Self, allocator: std.mem.Allocator) ![]u64 {
        var res = try ArrayList(u64).initCapacity(allocator, self._refs.len);
        var i: usize = 0;
        var ref: i64 = 0;
        while (i < self._refs.len) : (i += 1) {
            ref += self._refs[i];
            res.appendAssumeCapacity(@as(u64, @intCast(ref)));
        }
        return try res.toOwnedSlice();
    }

    pub fn nodes(self: Self, allocator: std.mem.Allocator) !?json.LineString {
        var res = try ArrayList([]f64).initCapacity(allocator, self._refs.len);
        var i: usize = 0;
        var ref: i64 = 0;
        while (i < self._refs.len) : (i += 1) {
            ref += self._refs[i];
            const node = try self.primitiveBlock.getNode(ref);
            if (node == null) return null;
            var point = try allocator.alloc(f64, 2);
            point[0] = node.?.lon;
            point[1] = node.?.lat;
            res.appendAssumeCapacity(point);
        }
        return try res.toOwnedSlice();
    }

    pub fn isArea(self: Self, maybeArea: bool) bool {
        if ((maybeArea and self._refs.len >= 4 and
            self._refs[0] == self._refs[self._refs.len - 1]) or
            self.hasKeyValue("area", "yes"))
        {
            return true;
        }
        return false;
    }

    pub fn geometry(self: Self, arena: *std.heap.ArenaAllocator, maybeArea: bool) !?json.Geometry {
        const allocator = arena.allocator();
        const nodes_ = try self.nodes(allocator);
        if (nodes_ == null) return null;
        if (self.isArea(maybeArea)) {
            var poly = try allocator.alloc([][]f64, 1);
            poly[0] = nodes_.?;
            return json.Geometry{ .polygon = poly };
        }
        return json.Geometry{ .lineString = nodes_.? };
    }

    pub fn toGeoJSON(self: Self, arena: *std.heap.ArenaAllocator, maybeArea: bool) !?Feature {
        const allocator = arena.allocator();
        const id: ?u64 = if (self.id >= 0) @as(u64, @intCast(self.id)) else null;
        const props = (try self.tagsAsJSONString(allocator)) orelse "{}";
        const geo = try self.geometry(arena, maybeArea);
        if (geo == null) return null;

        return Feature{
            .id = id,
            .geometry = geo.?,
            .properties = try json.Value.FromString(props, arena),
        };
    }

    pub fn toS2JSON(self: Self, arena: *std.heap.ArenaAllocator, maybeArea: bool) !?[]S2Feature {
        var geojson = try self.toGeoJSON(arena, maybeArea);
        if (geojson == null) return null;

        return try convertGeoJSON(&(geojson.?), 0.125, arena);
    }

    fn readLayer(self: *Self, tag: u64, pbf: *Protobuf) !void {
        switch (tag) {
            1 => self.id = pbf.readVarint(i64),
            2 => self._keys = try pbf.readPacked(usize),
            3 => self._vals = try pbf.readPacked(usize),
            4 => self._info = try Info.Init(pbf, self.primitiveBlock),
            8 => self._refs = try pbf.readPackedS(i64),
            9 => self._lats = try pbf.readPackedS(i64),
            10 => self._lons = try pbf.readPackedS(i64),
            else => unreachable,
        }
    }
};
