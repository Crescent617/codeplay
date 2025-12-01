const std = @import("std");

pub fn main() !void {
    const stdout_file = std.fs.File.stdout();

    var buffer: [1024]u8 = undefined;
    var stdout_writer = stdout_file.writer(&buffer);

    try writeData(&stdout_writer.interface);
    try stdout_writer.interface.flush(); // this could also be done in `writeData`
}

fn writeData(writer: *std.Io.Writer) std.Io.Writer.Error!void {
    try writer.writeAll("Hello, 0.15.1!\n");
}
