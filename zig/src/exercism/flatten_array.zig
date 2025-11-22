const std = @import("std");
const mem = std.mem;

pub const Box = union(enum) {
    none,
    one: i12,
    many: []const Box,
};

pub fn flatten(allocator: mem.Allocator, box: Box) mem.Allocator.Error![]i12 {
    var result = std.array_list.Managed(i12).init(allocator);

    defer result.deinit();

    try flattenHelper(&result, box);

    return result.toOwnedSlice();
}

fn flattenHelper(result: *std.array_list.Managed(i12), box: Box) !void {
    switch (box) {
        .none => {},
        .one => |value| {
            try result.append(value);
        },
        .many => |boxes| {
            for (boxes) |b| {
                try flattenHelper(result, b);
            }
        },
    }
}
