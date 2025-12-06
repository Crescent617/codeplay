const std = @import("std");

fn destroyStack() void {
    var trash: [100]u8 = undefined;
    // 往栈上写点垃圾数据
    @memset(&trash, 0xFF);
}

pub fn main() void {
    var ptr: ?*u8 = null;
    {
        var v: u8 = 45;
        ptr = &v;
    }

    destroyStack();
    std.debug.print("Pointer is valid, value: {d}\n", .{ptr.?.*});
}
