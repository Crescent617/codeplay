const std = @import("std");

pub const BucketId = enum {
    one,
    two,
};

pub const Result = struct {
    moves: u64,
    goal_bucket: BucketId,
    other_bucket: u64,
};

const Solver = struct {
    bucket_one: i64,
    bucket_two: i64,
    goal: u64,
    initial_bucket: BucketId,
    visited: *std.AutoHashMap([2]i64, void),
    allocator: std.mem.Allocator,

    fn solve(self: @This(), bucket1: i64, bucket2: i64, g: u64) !?Result {
        if (g > self.bucket_one and g > self.bucket_two)
            return null;

        const State = struct {
            b1: i64,
            b2: i64,
            moves: u64 = 1,
        };

        var queue: std.ArrayList(State) = .{};
        defer queue.deinit(self.allocator);

        try queue.append(self.allocator, State{ .b1 = bucket1, .b2 = bucket2, .moves = 1 });

        var tmp_queue: std.ArrayList(State) = .{};
        defer tmp_queue.deinit(self.allocator);

        while (queue.items.len > 0) {
            for (queue.items) |state| {
                const b1 = state.b1;
                const b2 = state.b2;
                if (b1 == g) {
                    return .{
                        .moves = state.moves,
                        .goal_bucket = .one,
                        .other_bucket = @intCast(b2),
                    };
                } else if (b2 == g) {
                    return .{
                        .moves = state.moves,
                        .goal_bucket = .two,
                        .other_bucket = @intCast(b1),
                    };
                }

                const next_states = [_]State{
                    // Fill bucket one
                    .{ .b1 = self.bucket_one, .b2 = b2 },
                    // Fill bucket two
                    .{ .b1 = b1, .b2 = self.bucket_two },
                    // Empty bucket one
                    .{ .b1 = 0, .b2 = b2 },
                    // Empty bucket two
                    .{ .b1 = b1, .b2 = 0 },
                    // Pour bucket one into bucket two
                    .{
                        .b1 = @max(0, b1 - (self.bucket_two - b2)),
                        .b2 = @min(self.bucket_two, b1 + b2),
                    },
                    // Pour bucket two into bucket one
                    .{
                        .b1 = @min(self.bucket_one, b1 + b2),
                        .b2 = @max(0, b2 - (self.bucket_one - b1)),
                    },
                };

                for (next_states) |ns| {
                    const nb1 = ns.b1;
                    const nb2 = ns.b2;

                    if (self.initial_bucket == .one and ns.b1 == 0 and ns.b2 == self.bucket_two) continue;
                    if (self.initial_bucket == .two and ns.b1 == self.bucket_one and ns.b2 == 0) continue;

                    if (self.visited.get(.{ nb1, nb2 }) != null) {
                        continue;
                    }
                    try self.visited.put(.{ nb1, nb2 }, {});
                    try tmp_queue.append(self.allocator, .{ .b1 = nb1, .b2 = nb2, .moves = state.moves + 1 });
                }
            }

            queue.clearRetainingCapacity();
            try queue.appendSlice(self.allocator, tmp_queue.items);

            tmp_queue.clearRetainingCapacity();
        }

        return null;
    }
};

pub fn measure(bucket_one: u64, bucket_two: u64, goal: u64, start_bucket: BucketId) ?Result {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    const allocator = gpa.allocator();

    var visited = std.AutoHashMap([2]i64, void).init(allocator);
    defer visited.deinit();

    var solver = Solver{
        .bucket_one = @intCast(bucket_one),
        .bucket_two = @intCast(bucket_two),
        .goal = goal,
        .initial_bucket = start_bucket,
        .visited = &visited,
        .allocator = allocator,
    };

    return switch (start_bucket) {
        .one => solver.solve(@intCast(bucket_one), 0, goal),
        .two => solver.solve(0, @intCast(bucket_two), goal),
    } catch null;
}

const testing = std.testing;

test "Measure using bucket one of size 3 and bucket two of size 5 - start with bucket one" {
    if (measure(3, 5, 1, .one)) |result| {
        try testing.expectEqual(4, result.moves);

        try testing.expectEqual(.one, result.goal_bucket);

        try testing.expectEqual(5, result.other_bucket);
    } else {
        try testing.expect(false);
    }
}

test "Measure using bucket one of size 3 and bucket two of size 5 - start with bucket two" {
    if (measure(3, 5, 1, .two)) |result| {
        try testing.expectEqual(8, result.moves);

        try testing.expectEqual(.two, result.goal_bucket);

        try testing.expectEqual(3, result.other_bucket);
    } else {
        try testing.expect(false);
    }
}

test "Measure using bucket one of size 7 and bucket two of size 11 - start with bucket one" {
    if (measure(7, 11, 2, .one)) |result| {
        try testing.expectEqual(14, result.moves);

        try testing.expectEqual(.one, result.goal_bucket);

        try testing.expectEqual(11, result.other_bucket);
    } else {
        try testing.expect(false);
    }
}

test "Measure using bucket one of size 7 and bucket two of size 11 - start with bucket two" {
    if (measure(7, 11, 2, .two)) |result| {
        try testing.expectEqual(18, result.moves);

        try testing.expectEqual(.two, result.goal_bucket);

        try testing.expectEqual(7, result.other_bucket);
    } else {
        try testing.expect(false);
    }
}

test "Measure one step using bucket one of size 1 and bucket two of size 3 - start with bucket two" {
    if (measure(1, 3, 3, .two)) |result| {
        try testing.expectEqual(1, result.moves);

        try testing.expectEqual(.two, result.goal_bucket);

        try testing.expectEqual(0, result.other_bucket);
    } else {
        try testing.expect(false);
    }
}

test "Measure using bucket one of size 2 and bucket two of size 3 - start with bucket one and end with bucket two" {
    if (measure(2, 3, 3, .one)) |result| {
        try testing.expectEqual(2, result.moves);

        try testing.expectEqual(.two, result.goal_bucket);

        try testing.expectEqual(2, result.other_bucket);
    } else {
        try testing.expect(false);
    }
}

test "Measure using bucket one much bigger than bucket two" {
    if (measure(5, 1, 2, .one)) |result| {
        try testing.expectEqual(6, result.moves);

        try testing.expectEqual(.one, result.goal_bucket);

        try testing.expectEqual(1, result.other_bucket);
    } else {
        try testing.expect(false);
    }
}

test "Measure using bucket one much smaller than bucket two" {
    if (measure(3, 15, 9, .one)) |result| {
        try testing.expectEqual(6, result.moves);

        try testing.expectEqual(.two, result.goal_bucket);

        try testing.expectEqual(0, result.other_bucket);
    } else {
        try testing.expect(false);
    }
}

test "Not possible to reach the goal" {
    const result = measure(6, 15, 5, .one);

    try testing.expectEqual(result, null);
}

test "With the same buckets but a different goal, then it is possible" {
    if (measure(6, 15, 9, .one)) |result| {
        try testing.expectEqual(10, result.moves);

        try testing.expectEqual(.two, result.goal_bucket);

        try testing.expectEqual(0, result.other_bucket);
    } else {
        try testing.expect(false);
    }
}

test "Goal larger than both buckets is impossible" {
    const result = measure(5, 7, 8, .one);

    try testing.expectEqual(result, null);
}
