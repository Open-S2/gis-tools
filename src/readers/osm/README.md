# OSM Reader

A fast and flexible Zig library for working with OpenStreetMap data.

This library works on all platforms (Linux, macOS and Windows) including in the browser with WASM!

## Example

```zig
const std = @import("std");
const osm = @import("osm");
const TagFilter = osm.TagFilter;
const CallBacks = osm.CallBacks;
const readOSM = osm.readOSM;
const Node = osm.Node;
const Way = osm.Way;
const Relation = osm.Relation;

pub fn main() !void {
    // pull in args
    var args = try std.process.argsWithAllocator(std.heap.c_allocator);
    defer args.deinit();
    const name = args.next();
    const raw_path = args.next();
    if (raw_path == null) {
        std.log.warn("usage: {s} <path>\n", .{name.?});
        return;
    }
    // prep a thread safe allocator
    var threadSafeAlloc = std.heap.ThreadSafeAllocator{ .child_allocator = std.heap.c_allocator };
    var allocator = threadSafeAlloc.allocator();
    // read osm
    try readOSM(
        // "src/osm/fixtures/andorra-latest.osm.pbf",
        raw_path.?,
        TestCTX{ .allocator = allocator },
        CallBacks(TestCTX){
            .nodeHandle = nodeHandle,
            .wayHandle = wayHandle,
            .relationHandle = relationHandle,
        },
        .{ .threads = 4, },
        allocator,
    );
}

const TestCTX = struct {
    allocator: std.mem.Allocator,
};

fn nodeHandle(ctx: TestCTX, n: *Node) anyerror!void {
    var tmpArena = std.heap.ArenaAllocator.init(ctx.allocator);
    defer tmpArena.deinit();
    _ = try n.toGeoJSON(&tmpArena);
}

fn wayHandle(ctx: TestCTX, w: *Way) anyerror!void {
    var tmpArena = std.heap.ArenaAllocator.init(ctx.allocator);
    defer tmpArena.deinit();
    _ = try w.toGeoJSON(&tmpArena, true);
}

fn relationHandle(ctx: TestCTX, r: *Relation) anyerror!void {
    var tmpArena = std.heap.ArenaAllocator.init(ctx.allocator);
    defer tmpArena.deinit();
    _ = try r.toGeoJSON(&tmpArena);
}
```
