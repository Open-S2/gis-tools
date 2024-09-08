const std = @import("std");
const testing = std.testing;
const ArrayList = std.ArrayList;
const json = @import("json");
const Feature = json.Feature;
const Point = json.Point;
const Polygon = json.Polygon;
const LineString = json.LineString;
const MultiPolygon = json.MultiPolygon;
const relation = @import("relation.zig");
const Member = relation.Member;
const RoleType = relation.RoleType;
const WayMember = @import("way.zig").WayMember;

/// Given a group of Members whose type is "way",
/// build a polygon or multipolygon Feature.
pub fn buildArea(members: []*Member, id: ?u64, properties: json.Value, arena: *std.heap.ArenaAllocator) !?Feature {
    // prep variables
    const allocator = arena.allocator();
    const polygons = ArrayList(Polygon).init(allocator);
    const currentPolygon = ArrayList(LineString).init(allocator);
    const currentRing = ArrayList(Point).init(allocator);

    // prepare step: members are stored out of order
    sortMembers(members);

    for (members) |member| {
        // Using "isClockwise", depending on whether the ring is outer or inner,
        // we may need to reverse the order of the points. Every time we find the
        // first and last point are the same, close out the ring, add it to the current
        // polygon, and start a new ring. if the current polygon is NOT empty, we store
        // it in the polygons list and start a new one before adding the completed ring.
        // NOTE: Due to the nature of OSM data, it is possible that resulting ring is reversed.
        // Check against the current ring to see if the way needs to be edited.
        //
        // grab the geometry from the member
        var geometry = try member.item.way.nodes(allocator);
        if (geometry == null) return null;
        // store in current ring, checking current rings order
        if (currentRing.items.len == 0) {
            try currentRing.appendSlice(geometry.?);
        } else {
            try currentRing.appendSlice(geometry.?[1..]);
        }
        // if current rings first and last point are the same, close out the ring
        if (equalPoints(currentRing.items[0], currentRing.items[currentRing.items.len - 1])) {
            // if ring is outer and clockwise, reverse the order of the points
            // also if the ring is an inner and counter-clockwise, reverse the order of the points
            if (member.role == RoleType.Outer and isClockwise(currentRing.items) or
                member.role == RoleType.Inner and !isClockwise(currentRing.items))
            {
                std.mem.reverse(Point, currentRing.items);
            }
            // add the ring to the current polygon. If member role is outer and
            // currentPolygon already has data, we need to store the current poly and
            // start a new polygon.
            // If the member role is inner, we can add the ring to the
            // current polygon.
            if (member.role == RoleType.Outer and currentPolygon.items.len > 0) {
                try polygons.append(try currentPolygon.toOwnedSlice());
            }
            try currentPolygon.append(try currentRing.toOwnedSlice());
        }
    }

    // Last step is to build:
    // flush the current polygon if it exists
    if (currentPolygon.items.len > 0) {
        try polygons.append(try currentPolygon.toOwnedSlice());
    }
    // grab the polys and return a feature
    const polys = try polygons.toOwnedSlice();
    if (polys.len == 1) {
        return Feature{
            .id = id,
            .geometry = .{ .polygon = polys[0] },
            .properties = properties,
        };
    } else {
        return Feature{
            .id = id,
            .geometry = .{ .multiPolygon = polys },
            .properties = properties,
        };
    }
}

fn equalPoints(a: []f64, b: []f64) bool {
    return a[0] == b[0] and a[1] == b[1];
}

/// If the returned sum < 0, the ring is counter-clockwise.
/// If the returned sum > 0, the ring is clockwise.
fn isClockwise(ring: LineString) bool {
    var sum: f64 = 0;
    if (ring.len < 4) return false;
    var i: usize = 1;
    var prev: Point = ring[0];
    var curr: Point = undefined;
    while (i < ring.len) : (i += 1) {
        curr = ring[i];
        sum += (curr[0] - prev[0]) * (curr[1] + prev[1]);
        prev = curr;
    }
    return sum > 0;
}

/// osm throws relation members out of order, so we need to not only sort them
/// but also check if the first and last points of each way follow the same direction.
fn sortMembers(members: []*Member) void {
    if (members.len < 3) return;
    var i: usize = 0;
    while (i < members.len - 1) : (i += 1) {
        const curWay = members[i].*.item.way;
        const curFirstPoint = curWay._refs[0];
        const curLastPoint = curWay._refs[curWay._refs.len - 1];
        // if current way is already self closing break
        if (curFirstPoint == curLastPoint) break;
        var j = i + 1;
        while (j < members.len) : (j += 1) {
            const nextWay = members[j].*.item.way;
            const nextFirstPoint = nextWay._refs[0];
            const nextLastPoint = nextWay._refs[nextWay._refs.len - 1];
            // if we find a match between any of the points, swap the member positions
            // if curFirstPoint == nextFirstPoint or curLastPoint == nextLastPoint
            // swap the _refs order
            const equalFirst = curFirstPoint == nextFirstPoint;
            const equalLast = curLastPoint == nextLastPoint;
            const equalFirstLast = curFirstPoint == nextLastPoint;
            const equalLastFirst = curLastPoint == nextFirstPoint;
            if (equalFirst or equalLast or equalFirstLast or equalLastFirst) {
                if (equalFirst) {
                    std.mem.reverse(i64, curWay._refs);
                } else if (equalLast) {
                    std.mem.reverse(i64, nextWay._refs);
                } else if (equalFirstLast) {
                    std.mem.reverse(i64, curWay._refs);
                    std.mem.reverse(i64, nextWay._refs);
                }
                // we want to move the found member to be next to the current member
                if (i + 1 != j) std.mem.swap(Member, members[i + 1], members[j]);
                break;
            }
        }
    }
}

test "sortMethods" {
    const ref1 = [_]i64{ 1, 2, 3 };
    const way1 = WayMember{ .primitiveBlock = undefined, ._refs = &ref1 };
    const m1 = Member{ .role = RoleType.Outer, .item = .{ .way = way1 } };
    const ref2 = [_]i64{ 3, 4, 5 };
    const way2 = WayMember{ .primitiveBlock = undefined, ._refs = &ref2 };
    const m2 = Member{ .role = RoleType.Outer, .item = .{ .way = way2 } };
    const ref3 = [_]i64{ 5, 6, 7 };
    const way3 = WayMember{ .primitiveBlock = undefined, ._refs = &ref3 };
    const m3 = Member{ .role = RoleType.Outer, .item = .{ .way = way3 } };
    const ref4 = [_]i64{ 1, 8, 7 };
    const way4 = WayMember{ .primitiveBlock = undefined, ._refs = &ref4 };
    const m4 = Member{ .role = RoleType.Outer, .item = .{ .way = way4 } };
    // add inner
    const ref5 = [_]i64{ 11, 22, 33 };
    const way5 = WayMember{ .primitiveBlock = undefined, ._refs = &ref5 };
    const m5 = Member{ .role = RoleType.Inner, .item = .{ .way = way5 } };
    const ref6 = [_]i64{ 55, 44, 33 };
    const way6 = WayMember{ .primitiveBlock = undefined, ._refs = &ref6 };
    const m6 = Member{ .role = RoleType.Inner, .item = .{ .way = way6 } };
    const ref7 = [_]i64{ 55, 66, 77 };
    const way7 = WayMember{ .primitiveBlock = undefined, ._refs = &ref7 };
    const m7 = Member{ .role = RoleType.Inner, .item = .{ .way = way7 } };
    const ref8 = [_]i64{ 77, 88, 11 };
    const way8 = WayMember{ .primitiveBlock = undefined, ._refs = &ref8 };
    const m8 = Member{ .role = RoleType.Inner, .item = .{ .way = way8 } };

    const members = [_]*Member{ &m1, &m3, &m4, &m2, &m8, &m6, &m7, &m5 };
    sortMembers(&members);
    // outer (tests equalFirst and equalLast)
    try testing.expectEqualSlices(i64, &[_]i64{ 3, 2, 1 }, members[0].*.item.way._refs);
    try testing.expectEqualSlices(i64, &[_]i64{ 1, 8, 7 }, members[1].*.item.way._refs);
    try testing.expectEqualSlices(i64, &[_]i64{ 7, 6, 5 }, members[2].*.item.way._refs);
    try testing.expectEqualSlices(i64, &[_]i64{ 5, 4, 3 }, members[3].*.item.way._refs);
    // inner (tests equalFirstLast)
    try testing.expectEqualSlices(i64, &[_]i64{ 11, 88, 77 }, members[4].*.item.way._refs);
    try testing.expectEqualSlices(i64, &[_]i64{ 77, 66, 55 }, members[5].*.item.way._refs);
    try testing.expectEqualSlices(i64, &[_]i64{ 55, 44, 33 }, members[6].*.item.way._refs);
    try testing.expectEqualSlices(i64, &[_]i64{ 33, 22, 11 }, members[7].*.item.way._refs);
}

test "signedArea outer (ccw) is negative" {
    const p1 = [_]f64{ 0, 0 };
    const p2 = [_]f64{ 1, 0 };
    const p3 = [_]f64{ 1, 1 };
    const p4 = [_]f64{ 0, 1 };
    const ls = [_][]f64{ &p1, &p2, &p3, &p4, &p1 };
    try testing.expect(!isClockwise(&ls));
}

test "signedArea inner (cw) is positive" {
    const p1 = [_]f64{ 0, 0 };
    const p2 = [_]f64{ 0, 1 };
    const p3 = [_]f64{ 1, 1 };
    const p4 = [_]f64{ 1, 0 };
    const ls = [_][]f64{ &p1, &p2, &p3, &p4, &p1 };
    try testing.expect(isClockwise(&ls));
}
