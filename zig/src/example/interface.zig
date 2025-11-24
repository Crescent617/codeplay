const std = @import("std");

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

test "interface example" {
    var circle = Circle.init(5.0);
    const shape: *const Shape = &circle.interface;
    std.debug.assert(std.math.approxEqRel(f64, shape.area(), 78.53981633974483, 0.00001));

    const shape2 = circle.asShape();
    std.debug.assert(std.math.approxEqRel(f64, shape2.area(), 78.53981633974483, 0.00001));
}
