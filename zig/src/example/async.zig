// Requires Zig 0.16.0 or later.

const std = @import("std");
const Io = std.Io;
const Allocator = std.mem.Allocator;
const assert = std.debug.assert;

fn juicyMain(io: Io) !void {
    var queue: Io.Queue([]const u8) = .init(&.{});

    var producer_task = try io.concurrent(producer, .{
        io, &queue, "never gonna give you up",
    });
    defer producer_task.cancel(io) catch {};

    var producer_task2 = try io.concurrent(producer, .{
        io, &queue, "never gonna give you up!!",
    });
    defer producer_task2.cancel(io) catch {};

    var consumer_task = try io.concurrent(consumer, .{ io, &queue });
    defer _ = consumer_task.cancel(io) catch {};

    const result = try consumer_task.await(io);
    std.debug.print("message received: {s} {}\n", .{ result, @TypeOf(result) });
}

fn producer(
    io: Io,
    queue: *Io.Queue([]const u8),
    flavor_text: []const u8,
) !void {
    try queue.putOne(io, flavor_text);
}

fn consumer(
    io: Io,
    queue: *Io.Queue([]const u8),
) ![]const u8 {
    return queue.getOne(io);
}

pub fn main() !void {
    // Set up allocator.
    var debug_allocator: std.heap.DebugAllocator(.{}) = .init;
    defer assert(debug_allocator.deinit() == .ok);
    const gpa = debug_allocator.allocator();

    // Set up our I/O implementation.
    var threaded: std.Io.Threaded = .init(gpa);
    defer threaded.deinit();
    const io = threaded.io();

    return juicyMain(io);
}
