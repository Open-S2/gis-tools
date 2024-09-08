const std = @import("std");
const testing = std.testing;
const assert = std.debug.assert;
const ArrayList = std.ArrayList;
const Thread = std.Thread;
const StringHashMap = std.StringHashMap;
const Protobuf = @import("pbf").Protobuf;
const node = @import("node.zig");
pub const Node = node.Node;
const NodeHandle = node.NodeHandle;
const way = @import("way.zig");
pub const Way = way.Way;
const WayHandle = way.WayHandle;
const relation = @import("relation.zig");
pub const Relation = relation.Relation;
const RelationHandle = relation.RelationHandle;
const fileFormat = @import("fileFormat.zig");
const BlobHeader = fileFormat.BlobHeader;
const Blob = fileFormat.Blob;
const primitive = @import("primitive.zig");
const processBlock = primitive.processBlock;
const PrimitiveBlock = primitive.PrimitiveBlock;
pub const ChangeSet = primitive.ChangeSet;
const ChangeSetHandle = primitive.ChangeSetHandle;
const managers = @import("managers.zig");
const DataManagerType = managers.DataManagerType;
const DataManager = managers.DataManager;
const io = std.io;
const BufferedReader = io.BufferedReader;

pub const TagFilter = struct {
    filters: StringHashMap(?[]const u8),
    nodeFilters: StringHashMap(?[]const u8),
    wayFilters: StringHashMap(?[]const u8),
    relationFilters: StringHashMap(?[]const u8),
    const Self = @This();
    const FilterType = enum {
        All,
        Node,
        Way,
        Relation,
    };

    pub fn Init(allocator: std.mem.Allocator) !Self {
        return .{
            .filters = StringHashMap(?[]const u8).init(allocator),
            .nodeFilters = StringHashMap(?[]const u8).init(allocator),
            .wayFilters = StringHashMap(?[]const u8).init(allocator),
            .relationFilters = StringHashMap(?[]const u8).init(allocator),
        };
    }

    pub fn deinit(self: *Self) void {
        self.filters.deinit();
        self.nodeFilters.deinit();
        self.wayFilters.deinit();
        self.relationFilters.deinit();
    }

    pub fn addFilter(self: *Self, filterType: FilterType, key: []const u8, value: ?[]const u8) !void {
        switch (filterType) {
            .All => try self.filters.put(key, value),
            .Node => try self.nodeFilters.put(key, value),
            .Way => try self.wayFilters.put(key, value),
            .Relation => try self.relationFilters.put(key, value),
        }
    }

    pub fn matchFound(self: *Self, filterType: FilterType, key: []const u8, value: []const u8) bool {
        const curFilters = switch (filterType) {
            .Node => &self.nodeFilters,
            .Way => &self.wayFilters,
            .Relation => &self.relationFilters,
            .All => unreachable,
        };
        // check all filters first
        if (self.filters.contains(key)) {
            const filterValue = self.filters.get(key);
            if (filterValue != null and
                (filterValue.? == null or std.mem.eql(u8, filterValue.?.?, value))) return true;
        }
        // check type-specific filters
        if (curFilters.contains(key)) {
            const filterValue = curFilters.get(key);
            if (filterValue != null and
                (filterValue.? == null or std.mem.eql(u8, filterValue.?.?, value))) return true;
        }
        return false;
    }
};

pub const Options = struct {
    // if true, remove nodes that have no tags
    removeEmptyNodes: bool = true,
    tagFilter: ?*TagFilter = null,
    manager: DataManagerType = .Memory,
    threads: usize = 1,
    mapSize: usize = 128 * 1024 * 1024,
};

pub fn CallBacks(comptime C: type) type {
    return struct {
        nodeHandle: ?NodeHandle(C) = null,
        wayHandle: ?WayHandle(C) = null,
        relationHandle: ?RelationHandle(C) = null,
        changesetHandle: ?ChangeSetHandle(C) = null,
    };
}

const FileReader = io.Reader(std.fs.File, std.fs.File.ReadError, std.fs.File.read);
const BufReaderContainer = BufferedReader(4096, FileReader);
const BufferReader = io.Reader(*BufReaderContainer, std.fs.File.ReadError, BufReaderContainer.read);

pub const OSMReader = struct {
    options: *const Options,
    memoryManager: DataManager,
    threads: ArrayList(Thread),
    reader: *BufferReader,
    file: *std.fs.File,
    fileSize: u64,
    arena: *std.heap.ArenaAllocator,
    // when reading from a file
    readMut: std.Thread.Mutex = .{},
    // when checking if all nodes have been read
    nodeMut: std.Thread.Mutex = .{},
    nodeCond: std.Thread.Condition = std.Thread.Condition{},
    // when checking if all ways have been read
    wayMut: std.Thread.Mutex = .{},
    wayCond: std.Thread.Condition = std.Thread.Condition{},
    // count of threads that parsed all nodes
    parsedNodes: usize = 0,
    // count of threads that parsed all ways
    parsedWays: usize = 0,
    const Self = @This();
    pub fn Init(
        file_: *std.fs.File,
        reader_: *BufferReader,
        options_: *const Options,
        arena_: *std.heap.ArenaAllocator,
    ) !OSMReader {
        return .{
            .options = options_,
            .memoryManager = switch (options_.manager) {
                .Memory => .{ .memory = try managers.MemoryManager.Init(arena_) },
                .File => .{ .file = try managers.FileManager.Init(options_.mapSize) },
            },
            .threads = ArrayList(Thread).init(arena_.allocator()),
            .reader = reader_,
            .file = file_,
            .fileSize = try file_.getEndPos(),
            .arena = arena_,
            .parsedNodes = if (options_.threads == 1) 1 else 0,
        };
    }

    pub fn deinit(self: *OSMReader) void {
        self.threads.deinit();
        self.memoryManager.deinit();
        defer self.file.close();
    }

    /// OSM data comes in as a series of blocks. Each block is a blob that contains a
    /// serialized primitive block. The primitive block contains a list of nodes, ways,
    /// and relations. However, the order is always the same, nodes will always come first.
    /// This means as soon as we hit a way or relation (and we are using threads) we need
    /// to wait for all other threads to finish reading in nodes.
    pub fn awaitNodeBlock(self: *Self) void {
        if (self.parsedNodes >= self.options.threads) return;
        self.nodeMut.lock();
        defer self.nodeMut.unlock();
        self.parsedNodes += 1;
        // if we are single threaded, we don't need to wait for the node lock
        while (self.parsedNodes < self.options.threads) {
            self.nodeCond.wait(&self.nodeMut);
        }
        // the last thread to finish readings nodes will signal the other threads to continue
        self.nodeCond.signal();
    }

    /// to expound upon the above function, after nodes are ways. So we need to wait for all
    /// threads to finish reading nodes and ways before we can process relations.
    pub fn awaitWayBlock(self: *Self) void {
        if (self.parsedWays >= self.options.threads) return;
        self.wayMut.lock();
        defer self.wayMut.unlock();
        self.parsedWays += 1;
        // if we are single threaded, we don't need to wait for the way lock
        while (self.parsedWays < self.options.threads) {
            self.wayCond.wait(&self.wayMut);
        }
        // the last thread to finish readings ways will signal the other threads to continue
        self.wayCond.signal();
    }

    pub fn processBlocksMulti(self: *Self, ctx: anytype, callbacks: *const CallBacks(@TypeOf(ctx))) !void {
        var t: usize = 0;
        while (t < self.options.threads) : (t += 1) {
            // Start a new thread and pass the context to the worker thread.
            try self.threads.append(try std.Thread.spawn(.{}, OSMReader.processBlocks, .{ self, ctx, callbacks }));
        }
        // lastly wait for threads to join
        for (self.threads.items) |thread| thread.join();
    }

    pub fn processBlocks(self: *Self, ctx: anytype, callbacks: *const CallBacks(@TypeOf(ctx))) !void {
        while (try self.next(false)) |blob| {
            try self.readBlob(blob, ctx, callbacks);
        }
        // corner case: a thread never hits a way or relation (because the dataset is small),
        // so it never signals the other threads
        self.awaitNodeBlock();
        self.awaitWayBlock();
    }

    /// NOTE: If null is returned, all memory is already freed. However,if a
    /// blob is returned, it is the caller's responsibility to free the memory.
    pub fn next(self: *Self, isHeader: bool) !?[]u8 {
        // ensure we don't read file contents at the same time
        self.readMut.lock();
        defer self.readMut.unlock();
        // if we've already read all the data, return null
        if (try self.file.getPos() >= self.fileSize) return null;

        // prepare a local allocator
        const allocator = self.arena.child_allocator;
        var localArena = std.heap.ArenaAllocator.init(allocator);
        defer localArena.deinit();
        const localAllocator = localArena.allocator();

        // STEP 1: Get blob size
        // read length of current blob
        const length = try self.reader.readInt(i32, .Big);
        const blobHeaderData = try localAllocator.alloc(u8, @as(usize, @intCast(length)));
        _ = try self.reader.read(blobHeaderData);
        // build a blob header
        var pbf = try Protobuf.init(blobHeaderData, &localArena);
        const blobHeader = try BlobHeader.Init(&pbf);
        // get size of actual blob
        const compressedBlobLength = blobHeader.datasize;

        // STEP 2: Get blob data
        const compressedBlobData = try allocator.alloc(u8, @as(usize, @intCast(compressedBlobLength)));
        const readBytes = try self.reader.read(compressedBlobData);
        assert(readBytes == compressedBlobLength);

        // if we're just reading the header, we can stop here
        if (isHeader) {
            allocator.free(compressedBlobData);
            return null;
        }

        return compressedBlobData;
    }

    fn readBlob(
        self: *Self,
        compressedBlobData: []u8,
        ctx: anytype,
        callbacks: *const CallBacks(@TypeOf(ctx)),
    ) !void {
        const allocator = self.arena.child_allocator;
        defer allocator.free(compressedBlobData);

        var localArena = std.heap.ArenaAllocator.init(allocator);
        defer localArena.deinit();
        // Blob data is PBF encoded and ?compressed, so we need to parse & decompress it first
        var pbf = try Protobuf.init(compressedBlobData, &localArena);
        const blob = try Blob.Init(&pbf);
        pbf = try Protobuf.init(blob.data, &localArena);

        // Parse the PrimitiveBlock and read its contents.
        // all nodes/ways/relations that can be filtered already are on invocation.
        var primitiveBlock = try PrimitiveBlock.Init(
            &pbf,
            self.options,
            &self.memoryManager,
        );
        try processBlock(self, ctx, callbacks, &primitiveBlock);
    }
};

pub fn readOSM(
    input: [:0]const u8,
    ctx: anytype,
    callbacks: CallBacks(@TypeOf(ctx)),
    options: Options,
    allocator: std.mem.Allocator,
) !void {
    assert(options.threads > 0);
    assert(options.threads <= try Thread.getCpuCount());
    // setup primary variables
    var globalArena = std.heap.ArenaAllocator.init(allocator);
    defer globalArena.deinit();

    // pull in the file and prep a reader
    var file = try std.fs.cwd().openFile(input, .{});
    var bufReader = std.io.bufferedReader(file.reader());
    var reader = bufReader.reader();

    // initialize the OSMReader
    var osmReader = try OSMReader.Init(&file, &reader, &options, &globalArena);
    defer osmReader.deinit();

    // read off the headerBlock first
    _ = try osmReader.next(true);
    // now we have the OSMReader read the rest of the file which is a series of blobs
    // as PrimitiveBlocks. We create a distinction of whether we're using multiple threads
    // or not so that we can use this module in WASM.
    if (options.threads > 1) {
        try osmReader.processBlocksMulti(ctx, &callbacks);
    } else {
        try osmReader.processBlocks(ctx, &callbacks);
    }
}

test "parse a small OSM file" {
    var threadSafeTestingAlloc = std.heap.ThreadSafeAllocator{ .child_allocator = testing.allocator };
    const allocator = threadSafeTestingAlloc.allocator();
    try readOSM(
        "src/osm/fixtures/andorra-latest.osm.pbf",
        TestCTX{ .allocator = allocator },
        CallBacks(TestCTX){
            .nodeHandle = testNodeHandle,
            .wayHandle = testWayHandle,
            .relationHandle = testRelationHandle,
        },
        .{
            .threads = 4,
            // .manager = .File,
        },
        allocator,
    );
}

const TestCTX = struct {
    allocator: std.mem.Allocator,
};

fn testNodeHandle(ctx: TestCTX, n: *Node) anyerror!void {
    // _ = n.geometry();
    // var tags = try n.tags(ctx.allocator);
    // defer tags.deinit();
    // var tagStr = try n.tagsAsJSONString(ctx.allocator);
    // defer ctx.allocator.free(tagStr);

    var tmpArena = std.heap.ArenaAllocator.init(ctx.allocator);
    defer tmpArena.deinit();
    _ = try n.toGeoJSON(&tmpArena);
    // _ = try n.toS2JSON(&tmpArena);
}

fn testWayHandle(ctx: TestCTX, w: *Way) anyerror!void {
    // var nodes = try w.nodes(ctx.allocator);
    // if (nodes != null) {
    //     defer ctx.allocator.free(nodes.?);
    // }

    var tmpArena = std.heap.ArenaAllocator.init(ctx.allocator);
    defer tmpArena.deinit();
    _ = try w.toGeoJSON(&tmpArena, true);
    // _ = try w.toS2JSON(&tmpArena, true);
}

fn testRelationHandle(ctx: TestCTX, r: *Relation) anyerror!void {
    // var members = try r.members(ctx.allocator);
    // if (members != null) {
    //     defer ctx.allocator.free(members.?);
    // }

    var tmpArena = std.heap.ArenaAllocator.init(ctx.allocator);
    defer tmpArena.deinit();
    _ = try r.toGeoJSON(&tmpArena);
    // _ = try r.toS2JSON(&tmpArena);
}
