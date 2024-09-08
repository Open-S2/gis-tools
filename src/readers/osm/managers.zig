const std = @import("std");
const lmdb = @import("lmdb");
const Environment = lmdb.Environment;
const node = @import("node.zig");
const NodeGeometry = node.NodeGeometry;
const NodeMember = node.NodeMember;
const way = @import("way.zig");
const WayMember = way.WayMember;

pub const DataManagerType = enum {
    File,
    Memory,
};

pub const DataManager = union(enum) {
    file: FileManager,
    memory: MemoryManager,
    const Self = @This();

    pub fn deinit(self: *Self) void {
        switch (self.*) {
            .file => |*f| f.deinit(),
            .memory => |*m| m.deinit(),
        }
    }

    pub fn getNode(self: *Self, key: i64) !?NodeGeometry {
        switch (self.*) {
            .file => |*f| return try f.getNode(key),
            .memory => |*m| return m.getNode(key),
        }
    }

    pub fn getNodeMember(self: *Self, key: i64, allocator: std.mem.Allocator) !?NodeMember {
        switch (self.*) {
            .file => |*f| return try f.getNodeMember(key, allocator),
            .memory => |*m| return m.getNodeMember(key),
        }
    }

    pub fn putNode(self: *Self, key: i64, val: NodeGeometry, props: ?[]const u8) !void {
        switch (self.*) {
            .file => |*f| try f.putNode(key, val, props),
            .memory => |*m| try m.putNode(key, val, props),
        }
    }

    pub fn getWayMember(self: *Self, key: i64, allocator: std.mem.Allocator) !?WayMember {
        switch (self.*) {
            .file => |*f| return try f.getWay(key, allocator),
            .memory => |*m| return m.getWay(key),
        }
    }

    pub fn putWay(self: *Self, key: i64, val: []i64, props: ?[]const u8) !void {
        switch (self.*) {
            .file => |*f| try f.putWay(key, val, props),
            .memory => |*m| try m.putWay(key, val, props),
        }
    }
};

fn clearExistingFiles() void {
    std.fs.deleteFileAbsolute("/tmp/node_env_/lock.mdb") catch {};
    std.fs.deleteFileAbsolute("/tmp/node_env_/data.mdb") catch {};
    std.fs.deleteDirAbsolute("/tmp/node_env_") catch {};
    std.fs.deleteFileAbsolute("/tmp/node_prop_/lock.mdb") catch {};
    std.fs.deleteFileAbsolute("/tmp/node_prop_/data.mdb") catch {};
    std.fs.deleteDirAbsolute("/tmp/node_prop_") catch {};
    std.fs.deleteFileAbsolute("/tmp/way_env_/lock.mdb") catch {};
    std.fs.deleteFileAbsolute("/tmp/way_env_/data.mdb") catch {};
    std.fs.deleteDirAbsolute("/tmp/way_env_") catch {};
    std.fs.deleteFileAbsolute("/tmp/way_prop_/lock.mdb") catch {};
    std.fs.deleteFileAbsolute("/tmp/way_prop_/data.mdb") catch {};
    std.fs.deleteDirAbsolute("/tmp/way_prop_") catch {};
}

pub const FileManager = struct {
    nodeEnv: Environment(i64, NodeGeometry),
    nodeProperties: Environment(i64, []const u8), // JSON string
    wayEnv: Environment(i64, []u8),
    wayProperties: Environment(i64, []const u8), // JSON string
    pub fn Init(mapSize: usize) !FileManager {
        clearExistingFiles();
        var bufNodeEnv: [std.fs.MAX_PATH_BYTES]u8 = undefined;
        var bufNodeProp: [std.fs.MAX_PATH_BYTES]u8 = undefined;
        var bufWayEnv: [std.fs.MAX_PATH_BYTES]u8 = undefined;
        var bufWayProp: [std.fs.MAX_PATH_BYTES]u8 = undefined;
        try std.fs.makeDirAbsolute("/tmp/node_env_");
        try std.fs.makeDirAbsolute("/tmp/node_prop_");
        try std.fs.makeDirAbsolute("/tmp/way_env_");
        try std.fs.makeDirAbsolute("/tmp/way_prop_");
        const nodeEnvPath = try std.fs.realpath("/tmp/node_env_/", &bufNodeEnv);
        const nodePropPath = try std.fs.realpath("/tmp/node_prop_/", &bufNodeProp);
        const wayEnvPath = try std.fs.realpath("/tmp/way_env_/", &bufWayEnv);
        const wayPropPath = try std.fs.realpath("/tmp/way_prop_/", &bufWayProp);
        return FileManager{
            .nodeEnv = try Environment(i64, NodeGeometry).init(nodeEnvPath, .{}),
            .nodeProperties = try Environment(i64, []const u8).init(nodePropPath, .{}),
            .wayEnv = try Environment(i64, []u8).init(wayEnvPath, .{
                .map_size = mapSize,
            }),
            .wayProperties = try Environment(i64, []const u8).init(wayPropPath, .{}),
        };
    }

    pub fn deinit(self: *FileManager) void {
        clearExistingFiles();
        self.nodeEnv.deinit();
    }

    pub fn getNode(self: *FileManager, key: i64) !?NodeGeometry {
        const tx = try self.nodeEnv.begin(.{});
        errdefer tx.deinit();
        defer tx.commit() catch {};

        const db = try tx.open(.{});
        defer db.close(self.nodeEnv);

        return try tx.getMaybeNull(db, key);
    }

    pub fn getNodeProperties(self: *FileManager, key: i64) !?[]const u8 {
        const tx = try self.nodeProperties.begin(.{});
        errdefer tx.deinit();
        defer tx.commit() catch {};

        const db = try tx.open(.{});
        defer db.close(self.nodeProperties);

        return try tx.getMaybeNull(db, key);
    }

    pub fn getNodeMember(self: *FileManager, key: i64, allocator: std.mem.Allocator) !?NodeMember {
        const geometry = try self.getNode(key);
        if (geometry == null) return null;
        const props = try self.getNodeProperties(key);
        if (props != null) {
            const propsClone = try allocator.alloc(u8, props.?.len);
            @memcpy(propsClone, props.?);
            return NodeMember.Init(key, geometry.?.lat, geometry.?.lon, propsClone);
        }
        return NodeMember.Init(key, geometry.?.lat, geometry.?.lon, null);
    }

    pub fn putNodeGeometry(self: *FileManager, key: i64, val: NodeGeometry) !void {
        const tx = try self.nodeEnv.begin(.{});
        errdefer tx.deinit();
        defer tx.commit() catch {};

        const db = try tx.open(.{});
        defer db.close(self.nodeEnv);

        try tx.put(db, key, val, .{});
    }

    pub fn putNodeProperties(self: *FileManager, key: i64, val: []const u8) !void {
        const tx = try self.nodeProperties.begin(.{});
        errdefer tx.deinit();
        defer tx.commit() catch {};

        const db = try tx.open(.{});
        defer db.close(self.nodeProperties);

        try tx.put(db, key, val, .{});
    }

    pub fn putNode(self: *FileManager, key: i64, val: NodeGeometry, props: ?[]const u8) !void {
        try self.putNodeGeometry(key, val);
        if (props != null) try self.putNodeProperties(key, props.?);
    }

    pub fn getWayProperties(self: *FileManager, key: i64) !?[]const u8 {
        const tx = try self.wayProperties.begin(.{});
        errdefer tx.deinit();
        defer tx.commit() catch {};

        const db = try tx.open(.{});
        defer db.close(self.wayProperties);

        return try tx.getMaybeNull(db, key);
    }

    pub fn getWayRefs(self: *FileManager, key: i64) !?[]u8 {
        const tx = try self.wayEnv.begin(.{});
        errdefer tx.deinit();
        defer tx.commit() catch {};

        const db = try tx.open(.{});
        defer db.close(self.wayEnv);

        return try tx.getMaybeNull(db, key);
    }

    pub fn getWay(self: *FileManager, key: i64, allocator: std.mem.Allocator) !?WayMember {
        const refs_ = try self.getWayRefs(key);
        if (refs_ == null) return null;
        const valSlice = @as([*]i64, @ptrCast(@alignCast(refs_.?)))[0..refs_.?.len];
        const refs = try allocator.alloc(i64, valSlice.len);
        @memcpy(refs, valSlice);
        // get the way properties
        const props = try self.getWayProperties(key);
        if (props != null) {
            const propsClone = try allocator.alloc(u8, props.?.len);
            @memcpy(propsClone, props.?);
            return WayMember.Init(key, refs, propsClone);
        }
        return WayMember.Init(key, refs, null);
    }

    pub fn putWayGeometry(self: *FileManager, key: i64, val: []i64) !void {
        const tx = try self.wayEnv.begin(.{});
        errdefer tx.deinit();
        defer tx.commit() catch {};

        const db = try tx.open(.{});
        defer db.close(self.wayEnv);

        const valSlice = @as([*]u8, @ptrCast(@alignCast(val)))[0 .. val.len * @sizeOf(i64)];
        tx.put(db, key, valSlice, .{}) catch |err| {
            std.debug.print("ERROR: {}\n", .{err});
            return err;
        };
    }

    pub fn putWayProperties(self: *FileManager, key: i64, val: []const u8) !void {
        const tx = try self.wayProperties.begin(.{});
        errdefer tx.deinit();
        defer tx.commit() catch {};

        const db = try tx.open(.{});
        defer db.close(self.wayProperties);

        try tx.put(db, key, val, .{});
    }

    pub fn putWay(self: *FileManager, key: i64, val: []i64, props: ?[]const u8) !void {
        try self.putWayGeometry(key, val);
        if (props != null) try self.putWayProperties(key, props.?);
    }
};

pub const MemoryManager = struct {
    nodeMap: std.AutoHashMap(i64, NodeMember),
    wayMap: std.AutoHashMap(i64, WayMember),
    mutex: std.Thread.Mutex = .{},
    arena: *std.heap.ArenaAllocator,
    pub fn Init(arena: *std.heap.ArenaAllocator) !MemoryManager {
        return MemoryManager{
            .nodeMap = std.AutoHashMap(i64, NodeMember).init(arena.allocator()),
            .wayMap = std.AutoHashMap(i64, WayMember).init(arena.allocator()),
            .arena = arena,
        };
    }

    pub fn deinit(self: *MemoryManager) void {
        self.nodeMap.deinit();
    }

    pub fn getNode(self: *MemoryManager, key: i64) ?NodeGeometry {
        const member = self.nodeMap.get(key);
        if (member == null) return null;
        return NodeGeometry{ .lat = member.?.lat, .lon = member.?.lon };
    }

    pub fn getNodeMember(self: *MemoryManager, key: i64) ?NodeMember {
        return self.nodeMap.get(key);
    }

    pub fn putNode(self: *MemoryManager, key: i64, val: NodeGeometry, props: ?[]const u8) !void {
        self.mutex.lock();
        defer self.mutex.unlock();
        var clone: ?[]const u8 = null;
        if (props != null) {
            var allocator = self.arena.allocator();
            const cl = try allocator.alloc(u8, props.?.len);
            @memcpy(cl, props.?);
            clone = cl;
        }
        const nodeMember: NodeMember = NodeMember.Init(key, val.lat, val.lon, clone);
        try self.nodeMap.put(key, nodeMember);
    }

    pub fn getWay(self: *MemoryManager, key: i64) ?WayMember {
        return self.wayMap.get(key);
    }

    pub fn putWay(self: *MemoryManager, key: i64, val: []i64, props: ?[]const u8) !void {
        self.mutex.lock();
        defer self.mutex.unlock();
        var allocator = self.arena.allocator();
        var propClone: ?[]const u8 = null;
        if (props != null) {
            const pc = try allocator.alloc(u8, props.?.len);
            @memcpy(pc, props.?);
            propClone = pc;
        }
        const valClone = try allocator.alloc(i64, val.len);
        @memcpy(valClone, val);
        const wayMember = WayMember.Init(key, valClone, propClone);
        try self.wayMap.put(key, wayMember);
    }
};
