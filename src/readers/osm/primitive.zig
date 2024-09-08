const std = @import("std");
const ArrayList = std.ArrayList;
const AutoHashMap = std.AutoHashMap;
const StringHashMap = std.StringHashMap;
const Protobuf = @import("pbf").Protobuf;
const Info = @import("info.zig").Info;
const node = @import("node.zig");
const main = @import("main.zig");
const Options = main.Options;
const OSMReader = main.OSMReader;
const CallBacks = main.CallBacks;
const Node = node.Node;
const NodeGeometry = node.NodeGeometry;
const NodeMember = node.NodeMember;
const NodeHandle = node.NodeHandle;
const DenseNodes = node.DenseNodes;
const way = @import("way.zig");
const Way = way.Way;
const WayMember = way.WayMember;
const WayHandle = way.WayHandle;
const relation = @import("relation.zig");
const Relation = relation.Relation;
const RelationHandle = relation.RelationHandle;
const managers = @import("managers.zig");
const DataManager = managers.DataManager;

pub fn ChangeSetHandle(comptime C: type) type {
    return fn (ctx: C, node: *ChangeSet) anyerror!void;
}

/// We "manually" read PrimitiveGroup data because we want to inject
/// the osmReader, ctx, callbacks, and primitiveBlock into the readLayer function
pub fn processBlock(
    osmReader: *OSMReader,
    ctx: anytype,
    callbacks: *const CallBacks(@TypeOf(ctx)),
    primitiveBlock: *PrimitiveBlock,
) !void {
    var pbf = primitiveBlock.pbf;
    for (primitiveBlock.primitiveGroups_.items) |*pg| {
        // set start position
        pbf.setPos(pg.pos);
        // read message:
        const end = pbf.readValue() + pbf.pos;
        while (pbf.pos < end) {
            const field = pbf.readField();
            const startPos = pbf.pos;
            try pg.readLayer(ctx, callbacks, primitiveBlock, osmReader, field.tag, pbf);
            if (pbf.pos == startPos) pbf.skip(field.type);
        }
    }
}

/// NOTE: currently relations_ are stored, but we don't wait for the Block to store all relations
/// before we start testing primtiveHandle against the data. This is a problem because
/// relations reference eachother at times, and we need to be able to resolve those references
/// before we can run relationHandle against the data. This isn't an important issue since
/// in practice, all relations that reference eachother often produce garbage or unusable data.
/// But it would be *nice* to fix this. Morbidly enough, the "BEST" solution is to treat relations
/// like we do nodes and ways since relations could possibly reference eachother outside their own block.
/// From a practical standpoint, I can't see this being worth the effort or memory/time cost.
pub const PrimitiveBlock = struct {
    pbf: *Protobuf,
    stringtable: StringTable = undefined,
    primitiveGroups_: ArrayList(PrimitiveGroup),
    relations_: AutoHashMap(i64, Relation),
    // Granularity, units of nanodegrees, used to store coordinates in this block.
    granularity: i64 = 100,
    // Offset value between the output coordinates and the granularity grid in units of nanodegrees.
    lat_offset: i64 = 0,
    lon_offset: i64 = 0,
    // Granularity of dates, normally represented in units of milliseconds since the 1970 epoch.
    date_granularity: i32 = 1000,
    options: *const Options,
    manager: *DataManager,
    const Self = @This();

    pub fn Init(pbf: *Protobuf, options_: *const Options, manager_: *DataManager) !PrimitiveBlock {
        const allocator = pbf.arena.allocator();
        var primitiveBlock = PrimitiveBlock{
            .pbf = pbf,
            .primitiveGroups_ = ArrayList(PrimitiveGroup).init(allocator),
            .relations_ = AutoHashMap(i64, Relation).init(allocator),
            .options = options_,
            .manager = manager_,
        };
        try pbf.readFields(PrimitiveBlock, &primitiveBlock, readLayer);
        return primitiveBlock;
    }

    pub fn getNode(self: *Self, index: i64) !?NodeGeometry {
        return try self.manager.getNode(index);
    }

    pub fn getNodeMember(self: *Self, index: i64, allocator: std.mem.Allocator) !?NodeMember {
        return try self.manager.getNodeMember(index, allocator);
    }

    pub fn getWayMember(self: *Self, index: i64, allocator: std.mem.Allocator) !?WayMember {
        return try self.manager.getWayMember(index, allocator);
    }

    pub fn getString(self: *Self, index: usize) []const u8 {
        return self.stringtable.get(index);
    }

    pub fn tags(self: *Self, keys: []usize, values: []usize, allocator: std.mem.Allocator) !StringHashMap([]const u8) {
        var res = StringHashMap([]const u8).init(allocator);
        var i: usize = 0;
        while (i < keys.len) : (i += 1) {
            try res.put(self.stringtable.get(keys[i]), self.stringtable.get(values[i]));
        }
        return res;
    }

    pub fn tagsAsJSONString(self: *Self, keys: []usize, values: []usize, allocator: std.mem.Allocator) !?[]const u8 {
        if (keys.len == 0) return null;
        var res = ArrayList(u8).init(allocator);
        var writer = res.writer();
        try writer.writeByte('{');
        var i: usize = 0;
        while (i < keys.len) : (i += 1) {
            try writer.writeByte('"');
            try writer.writeAll(self.stringtable.get(keys[i]));
            try writer.writeAll("\":\"");
            try writer.writeAll(self.stringtable.get(values[i]));
            try writer.writeByte('"');
            if (i < keys.len - 1) try writer.writeByte(',');
        }
        try writer.writeByte('}');
        return try res.toOwnedSlice();
    }

    fn readLayer(self: *Self, tag: u64, pbf: *Protobuf) !void {
        switch (tag) {
            1 => self.stringtable = try StringTable.Init(pbf),
            2 => try self.primitiveGroups_.append(try PrimitiveGroup.Init(pbf)),
            17 => self.granularity = pbf.readVarint(i64),
            18 => self.date_granularity = pbf.readVarint(i32),
            19 => self.lat_offset = pbf.readVarint(i64),
            20 => self.lon_offset = pbf.readVarint(i64),
            else => unreachable,
        }
    }
};

/// Group of OSMPrimitives. All primitives in a group must be the same type.
pub const PrimitiveGroup = struct {
    // pos where to start reading this groups message in the PBF
    pos: usize,
    const Self = @This();

    pub fn Init(pbf: *Protobuf) !PrimitiveGroup {
        return PrimitiveGroup{ .pos = pbf.pos };
    }

    fn readLayer(
        _: *Self,
        ctx: anytype,
        callbacks: *const CallBacks(@TypeOf(ctx)),
        pb: *PrimitiveBlock,
        osmReader: *OSMReader,
        tag: u64,
        pbf: *Protobuf,
    ) !void {
        const nodeHandle = callbacks.nodeHandle;
        const wayHandle = callbacks.wayHandle;
        const relationHandle = callbacks.relationHandle;
        const changesetHandle = callbacks.changesetHandle;
        const options = pb.options;
        switch (tag) {
            1 => {
                var n = try Node.Init(pbf, pb);
                if (wayHandle != null or relationHandle != null) {
                    // ONLY store tags if relations are needed
                    const tags = if (relationHandle != null) try n.tagsAsJSONString(pbf.arena.allocator()) else null;
                    try pb.manager.putNode(
                        n.id,
                        n.geometry(),
                        tags,
                    );
                }
                if (nodeHandle != null and !n.isFilterable(options)) try nodeHandle.?(ctx, &n);
            },
            2 => {
                var dn = try DenseNodes.Init(pbf, pb);
                const dnNodes = try dn.nodes(pbf.arena.allocator());
                for (dnNodes) |*n| {
                    if (wayHandle != null or relationHandle != null) {
                        const tags = if (relationHandle != null) try n.tagsAsJSONString(pbf.arena.allocator()) else null;
                        try pb.manager.putNode(n.id, n.geometry(), tags);
                    }
                    if (nodeHandle != null and !n.isFilterable(options)) try nodeHandle.?(ctx, n);
                }
            },
            3 => {
                osmReader.awaitNodeBlock();
                var w = try Way.Init(pbf, pb);
                if (relationHandle != null) {
                    try pb.manager.putWay(w.id, w._refs, try w.tagsAsJSONString(pbf.arena.allocator()));
                }
                if (wayHandle != null and !w.isFilterable(options)) try wayHandle.?(ctx, &w);
            },
            4 => {
                osmReader.awaitNodeBlock();
                osmReader.awaitWayBlock();
                var r = try Relation.Init(pbf, pb);
                if (relationHandle != null and !r.isFilterable(options)) {
                    try pb.relations_.put(r.id, r);
                    try relationHandle.?(ctx, &r);
                }
            },
            5 => {
                var c = try ChangeSet.Init(pbf);
                if (changesetHandle != null) try changesetHandle.?(ctx, &c);
            },
            else => unreachable,
        }
    }
};

/// String table, contains the common strings in each block.
/// Note that we reserve index '0' as a delimiter, so the entry at that
/// index in the table is ALWAYS blank and unused.
/// NOTE: OSM isn't safe and allows " inside of strings, so we have to replace them with '
/// NOTE: OSM isn't safe and allows \ at the end of strings, so we have to remove them so it can be properly parsed.
pub const StringTable = struct {
    s: ArrayList([]const u8),
    const Self = @This();

    pub fn Init(pbf: *Protobuf) !StringTable {
        var stringTable = StringTable{
            .s = ArrayList([]const u8).init(pbf.arena.allocator()),
        };
        try pbf.readMessage(StringTable, &stringTable, readLayer);
        return stringTable;
    }

    pub fn get(self: *Self, index: usize) []const u8 {
        return self.s.items[index];
    }

    fn readLayer(self: *Self, tag: u64, pbf: *Protobuf) !void {
        switch (tag) {
            1 => {
                // unfortunately we have to clean up the strings here
                const bytes = pbf.readBytes();
                // convert all " to ' and don't allow \ at the end of the string
                for (bytes, 0..) |*b, idx| {
                    if (b.* == '"') b.* = '\'';
                    if (b.* == '\\' and idx == bytes.len - 1) b.* = ' ';
                }
                try self.s.append(bytes);
            },
            else => unreachable,
        }
    }
};

/// This is kept for backwards compatibility but not used anywhere.
pub const ChangeSet = struct {
    id: i64 = 0,
    const Self = @This();

    pub fn Init(pbf: *Protobuf) !ChangeSet {
        var changeSet = ChangeSet{};
        try pbf.readMessage(ChangeSet, &changeSet, readLayer);
        return changeSet;
    }

    fn readLayer(self: *Self, tag: u64, pbf: *Protobuf) !void {
        switch (tag) {
            1 => self.id = pbf.readVarint(i64),
            else => unreachable,
        }
    }
};
