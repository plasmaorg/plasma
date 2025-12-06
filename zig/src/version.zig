//! Version and build information.

const builtin = @import("builtin");
const platform = @import("platform.zig");

/// Semantic version of Plasma.
pub const VERSION = "0.1.0";

/// Human-readable build target string.
pub const BUILD_TARGET = if (platform.is_wasm)
    "wasm32-freestanding"
else
    @tagName(builtin.target.cpu.arch) ++ "-" ++ @tagName(builtin.target.os.tag);
