const std = @import("std");
const StringHashMap = std.StringHashMap;
const ArrayList = std.ArrayList;
const Protobuf = @import("pbf").Protobuf;
const Info = @import("info.zig").Info;
const Node = @import("node.zig").Node;
const NodeMember = @import("node.zig").NodeMember;
const Way = @import("way.zig").Way;
const WayMember = @import("way.zig").WayMember;
const PrimitiveBlock = @import("primitive.zig").PrimitiveBlock;
const Options = @import("main.zig").Options;
const json = @import("json");
const Feature = json.Feature;
const S2Feature = json.S2Feature;
const convertGeoJSON = @import("../vectors/convert.zig").convertGeoJSON;
const buildArea = @import("areaBuilder.zig").buildArea;

pub fn RelationHandle(comptime C: type) type {
    return fn (ctx: C, node: *Relation) anyerror!void;
}

pub const MemberType = enum(u8) {
    Node = 0,
    Way = 1,
    Relation = 2,
};

pub const RoleType = enum {
    Area,
    Outer,
    Inner,
    From,
    To,
    Via,
    Label,
    AdminCentre,
    Stop,
    Empty,
    Other,
    const roles = [_][]const u8{
        "area",
        "outer",
        "inner",
        "from",
        "to",
        "via",
        "label",
        "admin_centre",
        "stop",
    };
    const Self = @This();
    pub fn Init(name: []const u8) RoleType {
        if (name.len == 0) {
            return RoleType.Empty;
        }
        for (roles, 0..) |v, i| {
            if (std.mem.eql(u8, v, name)) {
                return @as(RoleType, @enumFromInt(@as(u3, @truncate(i))));
            }
        }
        return RoleType.Other;
    }
};

pub const Item = union(enum) {
    node: NodeMember,
    way: WayMember,
    relation: *Relation,
};

pub const Member = struct {
    role: RoleType,
    item: Item,
};

pub const MemberFeature = struct {
    role: RoleType,
    item: Feature,
};

pub const MemberS2Feature = struct {
    role: RoleType,
    item: S2Feature,
};

pub const Relation = struct {
    id: i64 = -1,
    // Parallel arrays.
    _keys: []usize = &[_]usize{},
    _vals: []usize = &[_]usize{},
    _info: ?Info = null,
    // Parallel arrays
    roles_sid: []usize = &[_]usize{}, // This should have been defined as uint32 for consistency, but it is now too late to change it
    memids: []i64 = &[_]i64{}, // DELTA encoded
    types: []MemberType = &[_]MemberType{},
    primitiveBlock: *PrimitiveBlock,
    const Self = @This();

    pub fn Init(pbf: *Protobuf, primitiveBlock_: *PrimitiveBlock) !Relation {
        var relation = Relation{ .primitiveBlock = primitiveBlock_ };
        try pbf.readMessage(Relation, &relation, Relation.readLayer);
        return relation;
    }

    pub fn isFilterable(self: Self, options: *const Options) bool {
        if (options.tagFilter != null) {
            var tagFilter = options.tagFilter.?;
            var i: usize = 0;
            while (i < self._keys.len) : (i += 1) {
                const keyStr = self.primitiveBlock.getString(self._keys[i]);
                const valStr = self.primitiveBlock.getString(self._vals[i]);
                if (tagFilter.matchFound(.Relation, keyStr, valStr)) return false;
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

    /// Each member can be node, way or relation.
    pub fn members(self: Self, allocator: std.mem.Allocator) !?[]Member {
        var res = try ArrayList(Member).initCapacity(allocator, self.memids.len);
        var i: usize = 0;
        var memid: i64 = 0;
        while (i < self.memids.len) : (i += 1) {
            memid += self.memids[i];
            const role = RoleType.Init(self.primitiveBlock.getString(self.roles_sid[i]));
            var item: ?Item = null;
            const curType = self.types[i];
            switch (curType) {
                MemberType.Node => {
                    const node = try self.primitiveBlock.getNodeMember(memid, allocator);
                    if (node == null) item = null else item = Item{ .node = node.? };
                },
                MemberType.Way => {
                    var way = try self.primitiveBlock.getWayMember(memid, allocator);
                    if (way == null) {
                        item = null;
                    } else {
                        // be sure to update hte primitive block before use
                        way.?.primitiveBlock = self.primitiveBlock;
                        item = Item{ .way = way.? };
                    }
                },
                MemberType.Relation => {
                    const relation = self.primitiveBlock.relations_.getPtr(memid);
                    if (relation == null) item = null else item = Item{ .relation = relation.? };
                },
            }
            if (item == null) {
                // if the relation is missing nodes or ways we just give up,
                // but if it's missing a relation we can still return the rest.
                if (curType != MemberType.Relation) {
                    res.deinit();
                    return null;
                }
            } else {
                res.appendAssumeCapacity(Member{ .role = role, .item = item.? });
            }
        }
        return try res.toOwnedSlice();
    }

    /// convert members to geoJSON. members are grouped by type if applicable.
    /// (outer + inner are grouped together to be a polygon/multipolygon for instance)
    pub fn toGeoJSON(self: Self, arena: *std.heap.ArenaAllocator) !?[]MemberFeature {
        const allocator = arena.allocator();
        // pull in members
        const mmbrs = try self.members(allocator);
        if (mmbrs == null) return null;
        var res = ArrayList(MemberFeature).init(allocator);

        // group area members on our way
        var areaMembers = ArrayList(*Member).init(allocator);
        defer areaMembers.deinit();

        for (mmbrs.?) |*mmbr| {
            switch (mmbr.item) {
                .node => |n| try res.append(MemberFeature{ .role = mmbr.role, .item = try n.toGeoJSON(arena) }),
                .way => |*w| {
                    if (mmbr.role == RoleType.Outer or mmbr.role == RoleType.Inner) {
                        try areaMembers.append(mmbr);
                    } else {
                        const feat = try w.toGeoJSON(arena, true);
                        if (feat != null) try res.append(MemberFeature{ .role = mmbr.role, .item = feat.? });
                    }
                },
                .relation => |r| {
                    r.primitiveBlock = self.primitiveBlock;
                    const geo = try r.toGeoJSON(arena);
                    if (geo != null) try res.appendSlice(geo.?);
                },
            }
        }

        // add area object
        if (areaMembers.items.len > 0) {
            const id: ?u64 = if (self.id >= 0) @as(u64, @intCast(self.id)) else null;
            const props = (try self.tagsAsJSONString(allocator)) orelse "{}";
            const properties = try json.Value.FromString(props, arena);
            const amSlice = try areaMembers.toOwnedSlice();
            const item = try buildArea(amSlice, id, properties, arena);
            if (item == null) return null;
            try res.append(MemberFeature{
                .role = RoleType.Area,
                .item = item.?,
            });
        }

        return try res.toOwnedSlice();
    }

    /// convert members to S2JSON. members are grouped by type if applicable.
    /// (outer + inner are grouped together to be a s2polygon/s2multipolygon for instance)
    pub fn toS2JSON(self: Self, arena: *std.heap.ArenaAllocator) !?[]MemberS2Feature {
        const geojson = try self.toGeoJSON(arena);
        if (geojson == null) return null;
        var res = try ArrayList(MemberS2Feature).initCapacity(arena.allocator(), geojson.?.len);

        for (geojson.?) |*member| {
            const s2jsons = try convertGeoJSON(&member.item, 0.125, arena);
            for (s2jsons) |s2j| {
                try res.append(MemberS2Feature{ .role = member.role, .item = s2j });
            }
        }

        return try res.toOwnedSlice();
    }

    fn readLayer(self: *Self, tag: u64, pbf: *Protobuf) !void {
        switch (tag) {
            1 => self.id = pbf.readVarint(i64),
            2 => self._keys = try pbf.readPacked(usize),
            3 => self._vals = try pbf.readPacked(usize),
            4 => self._info = try Info.Init(pbf, self.primitiveBlock),
            8 => self.roles_sid = try pbf.readPacked(usize),
            9 => self.memids = try pbf.readPackedS(i64),
            10 => self.types = try pbf.readPacked(MemberType),
            else => unreachable,
        }
    }
};
