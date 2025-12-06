//! Plasma - Multi-Layer Build Cache Infrastructure
//!
//! Entry point for both native and WASM builds.

const std = @import("std");
const builtin = @import("builtin");

const platform = @import("platform.zig");
const version = @import("version.zig");

// Force WASM exports to be included in the binary.
// Using comptime block ensures the linker doesn't optimize them out.
const exports = @import("exports.zig");
comptime {
    _ = &exports.plasma_init;
    _ = &exports.plasma_version;
    _ = &exports.plasma_health;
    _ = &exports.plasma_version_len;
}

pub fn main() void {
    if (!platform.is_wasm) {
        runNative();
    }
}

fn runNative() void {
    // Zig 0.15 requires explicit buffer management for writers.
    // Using unbuffered output (empty slice) for direct OS writes.
    var writer = std.fs.File.stdout().writer(&.{});
    const stdout = &writer.interface;

    stdout.print("[plasma] Plasma v{s}\n", .{version.VERSION}) catch {};
    stdout.print("[plasma] Build target: {s}\n", .{version.BUILD_TARGET}) catch {};
    stdout.print("[plasma] \n", .{}) catch {};
    stdout.print("[plasma] This is the Zig implementation of Plasma.\n", .{}) catch {};
    stdout.print("[plasma] Run with --help for usage information.\n", .{}) catch {};

    var args = std.process.args();
    _ = args.skip();

    while (args.next()) |arg| {
        if (std.mem.eql(u8, arg, "--help") or std.mem.eql(u8, arg, "-h")) {
            printHelp(stdout);
            return;
        }
        if (std.mem.eql(u8, arg, "--version") or std.mem.eql(u8, arg, "-V")) {
            stdout.print("[plasma] plasma {s}\n", .{version.VERSION}) catch {};
            return;
        }
    }
}

fn printHelp(writer: *std.Io.Writer) void {
    const help =
        \\Usage: plasma [COMMAND] [OPTIONS]
        \\
        \\Commands:
        \\  daemon      Start the local cache daemon
        \\  server      Start the regional cache server
        \\  run         Execute a cached script
        \\  config      Configuration utilities
        \\  health      Health check and diagnostics
        \\
        \\Options:
        \\  -h, --help     Print help information
        \\  -V, --version  Print version information
        \\
        \\Examples:
        \\  plasma daemon                  Start local cache daemon
        \\  plasma run build.sh            Run cached build script
        \\
        \\For more information, visit: https://github.com/plasmaorg/plasma
    ;
    writer.print("{s}\n", .{help}) catch {};
}

test "version is valid" {
    try std.testing.expect(version.VERSION.len > 0);
}
