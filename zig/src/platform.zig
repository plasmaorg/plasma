//! Platform detection and abstraction layer.
//!
//! Provides compile-time constants and platform-specific implementations
//! for native and WASM targets.

const builtin = @import("builtin");

/// True when compiling for WebAssembly.
pub const is_wasm = builtin.target.cpu.arch == .wasm32;

/// True when compiling for a native target with full OS support.
pub const is_native = !is_wasm;
