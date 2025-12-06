//! WASM exports for JavaScript interop.
//!
//! Functions in this module are exported to JavaScript when compiling to WASM.
//! The `export` keyword automatically applies the correct calling convention.

const version = @import("version.zig");
const platform = @import("platform.zig");

extern "env" fn js_log(ptr: [*]const u8, len: usize) void;

/// Initialize Plasma. Returns 0 on success.
pub export fn plasma_init() i32 {
    if (platform.is_wasm) {
        const msg1 = "[plasma] Plasma v" ++ version.VERSION ++ " initialized";
        const msg2 = "[plasma] Build target: " ++ version.BUILD_TARGET;
        const msg3 = "[plasma] Ready to cache your builds!";
        js_log(msg1.ptr, msg1.len);
        js_log(msg2.ptr, msg2.len);
        js_log(msg3.ptr, msg3.len);
    }
    return 0;
}

/// Get null-terminated version string.
pub export fn plasma_version() [*:0]const u8 {
    return version.VERSION ++ "\x00";
}

/// Health check. Returns 1 if healthy.
pub export fn plasma_health() i32 {
    return 1;
}

/// Get version string length.
pub export fn plasma_version_len() usize {
    return version.VERSION.len;
}
