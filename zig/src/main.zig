const std = @import("std");
const zig = @import("zig");

const ShapeVTable = struct {
    area: *const fn (*const Shape) f64,
};

const Shape = struct {
    vtable: *const ShapeVTable,

    fn area(self: *const Shape) f64 {
        return self.vtable.area(self);
    }
};

const Shape2 = struct {
    ctx: *anyopaque,
    vtable: *const struct {
        area: *const fn (self: *anyopaque) f64,
    },

    fn area(self: *const Shape2) f64 {
        return self.vtable.area(self.ctx);
    }
};

const Circle = struct {
    radius: f64,
    interface: Shape,

    fn area(shape: *const Shape) f64 {
        const self: *const Circle = @fieldParentPtr("interface", shape);
        return std.math.pi * self.radius * self.radius;
    }

    fn init(radius: f64) Circle {
        return .{
            .radius = radius,
            .interface = .{ .vtable = &.{
                .area = area,
            } },
        };
    }

    fn area2(self: *const anyopaque) f64 {
        const self_circle: *const Circle = @ptrCast(@alignCast(self));
        return std.math.pi * self_circle.radius * self_circle.radius;
    }

    fn asShape(self: *Circle) Shape2 {
        return Shape2{
            .ctx = self,
            .vtable = &.{
                .area = area2,
            },
        };
    }
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};

    defer _ = gpa.deinit();

    const alloc = gpa.allocator();

    var my_circle = try alloc.create(Circle);
    defer alloc.destroy(my_circle);

    my_circle.* = Circle.init(5.0);

    const area = my_circle.interface.area();
    std.debug.print("Area of the circle: {}\n", .{area});

    const shape2 = my_circle.asShape();
    const area2 = shape2.area();
    std.debug.print("Area of the circle (Shape2): {}\n", .{area2});
}
