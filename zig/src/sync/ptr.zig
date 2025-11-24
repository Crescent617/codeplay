const std = @import("std");

pub fn Arc(comptime T: type) type {
    return struct {
        inner: *Inner,

        const Self = @This();
        const Inner = struct {
            ref_cnt: std.atomic.Value(usize) = .init(1),
            value: T,
        };

        /// Allocate and initialize an Arc
        pub fn init(allocator: std.mem.Allocator, v: T) !Self {
            const inner = try allocator.create(Inner);
            inner.* = .{
                .value = v,
            };
            return .{ .inner = inner };
        }

        /// Increase reference count
        pub fn retain(self: Self) Self {
            _ = self.inner.ref_cnt.fetchAdd(1, .acq_rel);
            return self;
        }

        /// Release one reference; free if count hits zero
        pub fn release(self: Self, allocator: std.mem.Allocator) void {
            const prev = self.inner.ref_cnt.fetchSub(1, .acq_rel);
            if (prev == 1) {
                if (@hasDecl(T, "deinit")) {
                    self.inner.value.deinit();
                }
                allocator.destroy(self.inner);
            }
        }

        /// Get a pointer to the inner value
        pub fn get(self: Self) *T {
            return &self.inner.value;
        }
    };
}
