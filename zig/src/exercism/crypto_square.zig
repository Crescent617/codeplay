const std = @import("std");
const mem = std.mem;

/// Encodes `plaintext` using the square code. Caller owns the returned memory.
pub fn ciphertext(allocator: mem.Allocator, plaintext: []const u8) mem.Allocator.Error![]u8 {
    var filtered: std.ArrayList(u8) = .{};
    defer filtered.deinit(allocator);

    // Step 1: Normalize the input by removing non-alphanumeric characters and converting to lowercase
    for (plaintext) |c| {
        if (std.ascii.isAlphanumeric(c)) {
            const lower_c = std.ascii.toLower(c);
            try filtered.append(allocator, lower_c);
        }
    }

    const length = filtered.items.len;
    if (length == 0) {
        return try allocator.alloc(u8, 0);
    }

    // Step 2: Determine the size of the square
    var cols = std.math.sqrt(length);
    if (cols * cols < length) {
        cols += 1;
    }
    const rows = (length + cols - 1) / cols;

    // Step 3: Create the ciphertext by reading columns
    var result: std.ArrayList(u8) = .{};
    defer result.deinit(allocator);

    for (0..cols) |col| {
        for (0..rows) |row| {
            const index = row * cols + col;
            try result.append(allocator, if (index < length) filtered.items[index] else ' ');
        }
        if (col != cols - 1) {
            try result.append(allocator, ' ');
        }
    }

    return result.toOwnedSlice(allocator);
}

const testing = std.testing;

test "empty plaintext results in an empty ciphertext" {
    const expected: []const u8 = "";

    const actual = try ciphertext(testing.allocator, "");

    defer testing.allocator.free(actual);

    try testing.expectEqualStrings(expected, actual);
}

test "normalization results in empty plaintext" {
    const expected: []const u8 = "";

    const actual = try ciphertext(testing.allocator, "... --- ...");

    defer testing.allocator.free(actual);

    try testing.expectEqualStrings(expected, actual);
}

test "Lowercase" {
    const expected: []const u8 = "a";

    const actual = try ciphertext(testing.allocator, "A");

    defer testing.allocator.free(actual);

    try testing.expectEqualStrings(expected, actual);
}

test "Remove spaces" {
    const expected: []const u8 = "b";

    const actual = try ciphertext(testing.allocator, "  b ");

    defer testing.allocator.free(actual);

    try testing.expectEqualStrings(expected, actual);
}

test "Remove punctuation" {
    const expected: []const u8 = "1";

    const actual = try ciphertext(testing.allocator, "@1,%!");

    defer testing.allocator.free(actual);

    try testing.expectEqualStrings(expected, actual);
}

test "9 character plaintext results in 3 chunks of 3 characters" {
    const expected: []const u8 = "tsf hiu isn";

    const actual = try ciphertext(testing.allocator, "This is fun!");

    defer testing.allocator.free(actual);

    try testing.expectEqualStrings(expected, actual);
}

test "8 character plaintext results in 3 chunks, the last one with a trailing space" {
    const expected: []const u8 = "clu hlt io ";

    const actual = try ciphertext(testing.allocator, "Chill out.");

    defer testing.allocator.free(actual);

    try testing.expectEqualStrings(expected, actual);
}

test "54 character plaintext results in 8 chunks, the last two with trailing spaces" {
    const expected: []const u8 = "imtgdvs fearwer mayoogo anouuio ntnnlvt wttddes aohghn  sseoau ";

    const actual = try ciphertext(testing.allocator, "If man was meant to stay on the ground, god would have given us roots.");

    defer testing.allocator.free(actual);

    try testing.expectEqualStrings(expected, actual);
}
