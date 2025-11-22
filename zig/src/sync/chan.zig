const std = @import("std");

pub const ChanErr = error{
    ChannelClosed,
};

pub fn ReceiverVTable(comptime T: type) type {
    return struct {
        recv: *const fn (self: *anyopaque) ?T,
    };
}

pub fn Receiver(comptime T: type) type {
    return struct {
        ctx: *anyopaque,
        vtable: *const ReceiverVTable(T),

        pub fn recv(self: *@This()) ?T {
            return self.vtable.recv(self.ctx);
        }
    };
}

pub fn SenderVTable(comptime T: type) type {
    return struct {
        send: *const fn (self: *anyopaque, v: T) ChanErr!void,
    };
}

pub fn Sender(comptime T: type) type {
    return struct {
        ctx: *anyopaque,
        vtable: *const SenderVTable(T),

        pub fn send(self: *@This(), v: T) ChanErr!void {
            return self.vtable.send(self.ctx, v);
        }
    };
}

pub fn Chan(comptime T: type) type {
    return struct {
        const Self = @This();

        const rx_vt: *const ReceiverVTable(T) = &.{
            .recv = recv,
        };
        const tx_vt: *const SenderVTable(T) = &.{
            .send = send,
        };

        mutex: std.Thread.Mutex = .{},
        send_cv: std.Thread.Condition = .{},
        recv_cv: std.Thread.Condition = .{},
        closed: bool = false,

        buf: []T,
        head: usize = 0,
        tail: usize = 0,
        count: usize = 0,

        pub fn init(allocator: std.mem.Allocator, capacity: usize) !*Self {
            const self = try allocator.create(Self);
            self.* = .{
                .buf = try allocator.alloc(T, capacity),
            };
            return self;
        }

        pub fn deinit(self: *Self, allocator: std.mem.Allocator) void {
            allocator.free(self.buf);
            allocator.destroy(self);
        }

        fn push(self: *Self, v: T) void {
            self.buf[self.tail] = v;
            self.tail = (self.tail + 1) % self.buf.len;
            self.count += 1;
        }

        fn pop(self: *Self) T {
            const v = self.buf[self.head];
            self.head = (self.head + 1) % self.buf.len;
            self.count -= 1;
            return v;
        }

        /// 阻塞发送，close 后发送返回 error
        fn send(sender: *anyopaque, v: T) !void {
            const self: *Self = @ptrCast(@alignCast(sender));
            self.mutex.lock();
            defer self.mutex.unlock();

            if (self.closed)
                return ChanErr.ChannelClosed;

            while (self.count == self.buf.len) {
                self.send_cv.wait(&self.mutex);
                if (self.closed) {
                    return ChanErr.ChannelClosed;
                }
            }

            self.push(v);
            self.recv_cv.signal();
        }

        /// 阻塞接收；若关闭并读空则返回 null
        fn recv(r: *anyopaque) ?T {
            const self: *Self = @ptrCast(@alignCast(r));

            self.mutex.lock();
            defer self.mutex.unlock();

            while (self.count == 0 and !self.closed) {
                self.recv_cv.wait(&self.mutex);
            }

            // close 且且没数据：返回 null
            if (self.count == 0 and self.closed)
                return null;

            const v = self.pop();
            self.send_cv.signal();
            return v;
        }

        pub fn rx(self: *Self) Receiver(T) {
            return .{
                .ctx = self,
                .vtable = rx_vt,
            };
        }

        pub fn tx(self: *Self) Sender(T) {
            return .{
                .ctx = self,
                .vtable = tx_vt,
            };
        }

        /// 关闭 channel：唤醒所有 send/recv
        pub fn close(self: *Self) void {
            self.mutex.lock();
            self.closed = true;
            self.mutex.unlock();

            self.send_cv.broadcast();
            self.recv_cv.broadcast();
        }
    };
}

test "chan" {
    const ch = try Chan(i32).init(std.testing.allocator, 2);
    defer ch.deinit(std.testing.allocator);

    var tx = ch.tx();
    var rx = ch.rx();

    try tx.send(10);
    try tx.send(20);

    const v1 = rx.recv().?;
    try std.testing.expectEqual(v1, 10);

    const v2 = rx.recv().?;
    try std.testing.expectEqual(v2, 20);

    ch.close();

    try std.testing.expectError(
        ChanErr.ChannelClosed,
        tx.send(30),
    );

    const v3 = rx.recv();
    try std.testing.expectEqual(v3, null);
}
