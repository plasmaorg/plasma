# Plasma Implementation Plan - Zig Migration

> **Status**: Migration In Progress
> **Last Updated**: 2025-12-06
> **Current Phase**: Phase 0 - Zig Hello World
> **Target**: Full rewrite in Zig with WASM/browser support

This document tracks the migration from Rust to Zig. Each phase is designed as a single PR that teaches specific Zig concepts while building toward the complete system.

---

## Migration Overview

### Why Zig?

1. **WASM-first design**: Zig has first-class WASM support with `zig build -Dtarget=wasm32-wasi`
2. **No hidden control flow**: Explicit allocators make WASM memory management predictable
3. **C interop**: Can reuse existing C libraries or expose C API easily
4. **Comptime**: Powerful compile-time execution for protocol generation
5. **Small binaries**: Important for browser delivery

### WASM Considerations

The browser environment has significant constraints that affect architecture:

| Feature | Native | WASM/Browser |
|---------|--------|--------------|
| File System | Full access | IndexedDB / Origin Private FS |
| Networking | TCP/UDP sockets | Fetch API only |
| Threads | Native threads | Web Workers |
| gRPC | Native tonic | gRPC-Web via HTTP |
| Storage | RocksDB | IndexedDB wrapper |
| Crypto | OpenSSL/native | WebCrypto API |

**Strategy**: Build abstraction layers from day one that compile to different backends.

---

## Phase 0: Zig Hello World & Project Setup

**Goal**: Bootstrap Zig project, compile to native + WASM, verify browser execution

**Zig Concepts to Learn**:
- `build.zig` - Zig's build system (replaces Cargo.toml + build.rs)
- Cross-compilation targets
- Basic syntax: functions, variables, imports
- `std.debug.print` for output

### Tasks

- [ ] Install Zig via mise (add to `.mise.toml`)
- [ ] Create `build.zig` with native + WASM targets
- [ ] Create `src/main.zig` with "Hello, Plasma!"
- [ ] Compile to native: `zig build`
- [ ] Compile to WASM: `zig build -Dtarget=wasm32-freestanding`
- [ ] Create `wasm-test/index.html` to load and run WASM
- [ ] Verify WASM runs in browser (console.log output)
- [ ] Set up GitHub Actions for Zig CI
  - [ ] `zig fmt` check
  - [ ] `zig build` native
  - [ ] `zig build` WASM
  - [ ] `zig build test`

**Example `build.zig`**:
```zig
const std = @import("std");

pub fn build(b: *std.Build) void {
    // Native target (default)
    const native_target = b.standardTargetOptions(.{});
    const native_optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "plasma",
        .root_source_file = b.path("src/main.zig"),
        .target = native_target,
        .optimize = native_optimize,
    });
    b.installArtifact(exe);

    // WASM target for browsers
    const wasm = b.addExecutable(.{
        .name = "plasma",
        .root_source_file = b.path("src/main.zig"),
        .target = b.resolveTargetQuery(.{
            .cpu_arch = .wasm32,
            .os_tag = .freestanding,
        }),
        .optimize = .ReleaseSmall,
    });
    wasm.entry = .disabled;
    wasm.rdynamic = true;

    const wasm_install = b.addInstallArtifact(wasm, .{
        .dest_dir = .{ .custom = "wasm" },
    });

    const wasm_step = b.step("wasm", "Build WASM target");
    wasm_step.dependOn(&wasm_install.step);
}
```

**Deliverable**: Binary runs on native + WASM loads in browser

---

## Phase 1: Error Handling & Result Types

**Goal**: Implement Zig error handling patterns, create foundational error types

**Zig Concepts to Learn**:
- Error sets and error unions (`!T`)
- `try`, `catch`, `errdefer`
- Optional types (`?T`)
- `orelse` and `if (x) |value|` patterns

### Tasks

- [ ] Create `src/errors.zig` with Plasma error set
- [ ] Implement `PlasmaError` error set (ConfigError, StorageError, AuthError, etc.)
- [ ] Create helper functions for error formatting
- [ ] Write tests for error handling patterns
- [ ] Document Zig vs Rust error handling differences

**Example**:
```zig
pub const PlasmaError = error{
    ConfigNotFound,
    ConfigParseError,
    StorageReadError,
    StorageWriteError,
    AuthTokenExpired,
    AuthTokenInvalid,
    NetworkError,
    WasmNotSupported,
};

pub fn loadConfig(path: []const u8) PlasmaError!Config {
    const file = std.fs.cwd().openFile(path, .{}) catch |err| switch (err) {
        error.FileNotFound => return error.ConfigNotFound,
        else => return error.ConfigParseError,
    };
    defer file.close();
    // ...
}
```

**Deliverable**: Error handling foundation that works on native + WASM

---

## Phase 2: Allocators & Memory Management

**Goal**: Master Zig's allocator model - critical for WASM compatibility

**Zig Concepts to Learn**:
- Allocator interface (`std.mem.Allocator`)
- `GeneralPurposeAllocator` for debugging
- `ArenaAllocator` for batch allocations
- `FixedBufferAllocator` for stack-based allocation
- WASM page allocator

### Tasks

- [ ] Create `src/allocators.zig` with platform-specific allocators
- [ ] Implement arena allocator wrapper for request handling
- [ ] Create memory-bounded allocator for cache limits
- [ ] Add memory usage tracking
- [ ] Test with different allocator backends (native vs WASM)
- [ ] Implement `deinit` patterns consistently

**Example**:
```zig
pub const PlatformAllocator = struct {
    backing: std.mem.Allocator,

    pub fn init() PlatformAllocator {
        if (builtin.target.isWasm()) {
            return .{ .backing = std.heap.wasm_allocator };
        } else {
            return .{ .backing = std.heap.page_allocator };
        }
    }
};
```

**Deliverable**: Memory management that works efficiently on native + WASM

---

## Phase 3: Strings, Slices & Hashing

**Goal**: Implement content-addressable storage primitives

**Zig Concepts to Learn**:
- Slices (`[]const u8`, `[]u8`)
- String handling (Zig strings are just slices)
- `std.crypto.hash.sha2.Sha256`
- Hex encoding/decoding
- Compile-time string operations

### Tasks

- [ ] Create `src/hash.zig` with SHA256 utilities
- [ ] Implement `ContentId` type (32-byte hash)
- [ ] Create hex encoding/decoding functions
- [ ] Implement hash verification
- [ ] Add WASM-compatible crypto (use std.crypto, it compiles to WASM)
- [ ] Benchmark hashing performance native vs WASM
- [ ] Write comprehensive tests

**Example**:
```zig
pub const ContentId = struct {
    bytes: [32]u8,

    pub fn fromData(data: []const u8) ContentId {
        var hasher = std.crypto.hash.sha2.Sha256.init(.{});
        hasher.update(data);
        return .{ .bytes = hasher.finalResult() };
    }

    pub fn toHex(self: ContentId) [64]u8 {
        return std.fmt.bytesToHex(self.bytes, .lower);
    }

    pub fn fromHex(hex: []const u8) !ContentId {
        var result: ContentId = undefined;
        _ = try std.fmt.hexToBytes(&result.bytes, hex);
        return result;
    }
};
```

**Deliverable**: SHA256 content addressing works on native + WASM

---

## Phase 4: Structs, Unions & Configuration

**Goal**: Implement configuration system with TOML-like parsing

**Zig Concepts to Learn**:
- Struct definitions and initialization
- Tagged unions (`union(enum)`)
- Default values with `= value`
- Comptime type reflection
- JSON parsing with `std.json`

### Tasks

- [ ] Create `src/config.zig` with all configuration types
- [ ] Port `PlasmaConfig`, `CacheConfig`, `UpstreamConfig`, etc.
- [ ] Implement JSON parsing (TOML parser can come later)
- [ ] Add environment variable expansion
- [ ] Implement config merging (CLI > env > file)
- [ ] Create default configurations
- [ ] Write serialization/deserialization tests

**Example**:
```zig
pub const CacheConfig = struct {
    dir: []const u8 = ".plasma/cache",
    max_size: u64 = 10 * 1024 * 1024 * 1024, // 10GB
    eviction_policy: EvictionPolicy = .lfu,
    default_ttl_seconds: u64 = 7 * 24 * 60 * 60, // 7 days
};

pub const EvictionPolicy = enum {
    lru,
    lfu,
    ttl,

    pub fn asStr(self: EvictionPolicy) []const u8 {
        return switch (self) {
            .lru => "lru",
            .lfu => "lfu",
            .ttl => "ttl",
        };
    }
};
```

**Deliverable**: Configuration system with JSON support

---

## Phase 5: Interfaces & Storage Abstraction

**Goal**: Create storage trait/interface pattern in Zig

**Zig Concepts to Learn**:
- Interface pattern with `*anyopaque` + vtable
- Comptime interface checking
- Generic functions
- Type erasure

### Tasks

- [ ] Create `src/storage/storage.zig` with Storage interface
- [ ] Implement interface pattern (vtable approach)
- [ ] Define methods: `put`, `get`, `exists`, `delete`, `size`, `stats`
- [ ] Create `StorageStats` type
- [ ] Write generic test harness for any Storage implementation
- [ ] Document interface pattern for future implementations

**Example**:
```zig
pub const Storage = struct {
    ptr: *anyopaque,
    vtable: *const VTable,

    pub const VTable = struct {
        put: *const fn (*anyopaque, []const u8, []const u8) anyerror!void,
        get: *const fn (*anyopaque, []const u8) anyerror!?[]const u8,
        exists: *const fn (*anyopaque, []const u8) anyerror!bool,
        delete: *const fn (*anyopaque, []const u8) anyerror!void,
        deinit: *const fn (*anyopaque) void,
    };

    pub fn put(self: Storage, id: []const u8, data: []const u8) !void {
        return self.vtable.put(self.ptr, id, data);
    }

    pub fn get(self: Storage, id: []const u8) !?[]const u8 {
        return self.vtable.get(self.ptr, id);
    }
    // ...
};
```

**Deliverable**: Storage interface that backends can implement

---

## Phase 6: In-Memory Storage Backend

**Goal**: Implement in-memory cache (works on native + WASM)

**Zig Concepts to Learn**:
- `std.HashMap` and `std.AutoHashMap`
- `std.ArrayList`
- Ownership semantics with allocators
- Memory management patterns

### Tasks

- [ ] Create `src/storage/memory.zig`
- [ ] Implement `MemoryStorage` implementing `Storage` interface
- [ ] Add LRU eviction support
- [ ] Add size tracking and limits
- [ ] Implement access time tracking for LRU
- [ ] Write tests with eviction scenarios
- [ ] Benchmark memory usage

**Example**:
```zig
pub const MemoryStorage = struct {
    allocator: std.mem.Allocator,
    data: std.StringHashMap(Entry),
    total_bytes: u64,
    max_bytes: u64,

    const Entry = struct {
        data: []const u8,
        access_count: u64,
        last_access: i64,
    };

    pub fn init(allocator: std.mem.Allocator, max_bytes: u64) MemoryStorage {
        return .{
            .allocator = allocator,
            .data = std.StringHashMap(Entry).init(allocator),
            .total_bytes = 0,
            .max_bytes = max_bytes,
        };
    }

    pub fn storage(self: *MemoryStorage) Storage {
        return .{
            .ptr = self,
            .vtable = &vtable,
        };
    }

    // ... implement VTable functions
};
```

**Deliverable**: Working in-memory cache on native + WASM

---

## Phase 7: Filesystem Storage Backend (Native Only)

**Goal**: Implement filesystem-based storage for native targets

**Zig Concepts to Learn**:
- `std.fs` for file operations
- Directory iteration
- File permissions
- Conditional compilation with `builtin.target`

### Tasks

- [ ] Create `src/storage/filesystem.zig`
- [ ] Implement `FilesystemStorage` implementing `Storage` interface
- [ ] Use content-addressed directory structure (like Git)
- [ ] Add atomic writes (write to temp, then rename)
- [ ] Implement metadata storage (access times, sizes)
- [ ] Add eviction based on directory scanning
- [ ] Compile-time disable for WASM target
- [ ] Write integration tests

**Example directory structure**:
```
.plasma/cache/
  ab/
    cdef1234...  (blob content)
  12/
    3456abcd...
  metadata.json  (access times, stats)
```

**Deliverable**: Filesystem storage for native, graceful fallback for WASM

---

## Phase 8: Platform Abstraction Layer

**Goal**: Create unified platform abstraction for native vs WASM

**Zig Concepts to Learn**:
- `@import("builtin")` for target detection
- Conditional compilation
- Comptime branching
- Platform-specific code organization

### Tasks

- [ ] Create `src/platform/mod.zig`
- [ ] Create `src/platform/native.zig`
- [ ] Create `src/platform/wasm.zig`
- [ ] Abstract: storage, networking, time, random
- [ ] Implement WASM stubs that call JavaScript
- [ ] Define `extern` functions for JS callbacks
- [ ] Create JS glue code for WASM
- [ ] Test platform detection at compile time

**Example**:
```zig
const builtin = @import("builtin");

pub const platform = if (builtin.target.cpu_arch == .wasm32)
    @import("platform/wasm.zig")
else
    @import("platform/native.zig");

// Usage:
const storage = platform.createDefaultStorage(allocator);
```

**Deliverable**: Clean platform abstraction compiling to native + WASM

---

## Phase 9: HTTP Server (Native)

**Goal**: Implement HTTP server for cache endpoints

**Zig Concepts to Learn**:
- `std.http.Server`
- Request/response handling
- Headers and body parsing
- Connection handling
- Async patterns (or threaded model)

### Tasks

- [ ] Create `src/http/server.zig`
- [ ] Implement basic HTTP/1.1 server using `std.http`
- [ ] Add routes: `GET /cache/:hash`, `PUT /cache/:hash`, `HEAD /cache/:hash`
- [ ] Implement streaming for large artifacts
- [ ] Add content-type handling
- [ ] Implement proper HTTP status codes
- [ ] Add request logging
- [ ] Write integration tests with real HTTP requests
- [ ] Benchmark throughput

**Example**:
```zig
pub const HttpServer = struct {
    storage: Storage,
    server: std.http.Server,

    pub fn handleRequest(self: *HttpServer, request: *std.http.Server.Request) !void {
        const path = request.target;

        if (std.mem.startsWith(u8, path, "/cache/")) {
            const hash = path[7..];
            switch (request.method) {
                .GET => try self.handleGet(request, hash),
                .PUT => try self.handlePut(request, hash),
                .HEAD => try self.handleHead(request, hash),
                else => try request.respond(.method_not_allowed, .{}),
            }
        }
    }
};
```

**Deliverable**: Working HTTP server compatible with Gradle/Nx/TurboRepo

---

## Phase 10: HTTP Client & Fetch Abstraction

**Goal**: Implement HTTP client that works on native (sockets) and WASM (fetch)

**Zig Concepts to Learn**:
- `std.http.Client` for native
- WASM extern functions for JavaScript interop
- Async/callback patterns for WASM

### Tasks

- [ ] Create `src/http/client.zig` with platform abstraction
- [ ] Implement native HTTP client using `std.http.Client`
- [ ] Implement WASM HTTP client using extern JS fetch
- [ ] Create JS glue code: `plasma_fetch(url, method, body) -> Promise`
- [ ] Handle streaming responses
- [ ] Add timeout and retry logic
- [ ] Test upstream cache fetching

**WASM JS Interop**:
```javascript
// plasma-wasm.js
const wasmInstance = await WebAssembly.instantiate(wasmBytes, {
    env: {
        plasma_fetch: (urlPtr, urlLen, methodPtr, methodLen, callback) => {
            const url = readString(urlPtr, urlLen);
            const method = readString(methodPtr, methodLen);
            fetch(url, { method }).then(response => {
                // Call back into WASM with result
            });
        }
    }
});
```

**Deliverable**: HTTP client working on native + browser

---

## Phase 11: CLI Argument Parsing

**Goal**: Implement command-line interface for native binary

**Zig Concepts to Learn**:
- `std.process.args()`
- String parsing and matching
- Comptime string handling
- Help text generation

### Tasks

- [ ] Create `src/cli.zig`
- [ ] Implement argument parser (or use zig-clap library)
- [ ] Define commands: `daemon`, `server`, `run`, `cache`, `config`, `health`
- [ ] Add flag parsing: `--config`, `--log-level`, `--port`
- [ ] Generate help text
- [ ] Handle subcommands
- [ ] Write tests for argument parsing

**Example**:
```zig
pub const Command = union(enum) {
    daemon: DaemonArgs,
    server: ServerArgs,
    run: RunArgs,
    config: ConfigCommand,
    health: HealthArgs,
    help,
    version,
};

pub fn parseArgs(allocator: std.mem.Allocator) !Command {
    var args = std.process.args();
    _ = args.skip(); // skip executable name

    const cmd = args.next() orelse return .help;

    if (std.mem.eql(u8, cmd, "daemon")) {
        return .{ .daemon = try parseDaemonArgs(allocator, &args) };
    }
    // ...
}
```

**Deliverable**: Full CLI matching Rust version's capabilities

---

## Phase 12: Logging & Observability

**Goal**: Implement structured logging with platform-specific output

**Zig Concepts to Learn**:
- `std.log` framework
- Custom log functions
- Comptime log level filtering
- Writer abstraction

### Tasks

- [ ] Create `src/logging.zig`
- [ ] Implement structured logger with JSON output option
- [ ] Add log levels: debug, info, warn, error
- [ ] Platform-specific output (stdout for native, console.log for WASM)
- [ ] Add request context (request ID, duration)
- [ ] Create `[plasma]` prefix formatting
- [ ] Add metrics collection hooks

**Example**:
```zig
pub fn log(
    comptime level: std.log.Level,
    comptime format: []const u8,
    args: anytype,
) void {
    const prefix = "[plasma] ";
    const level_str = switch (level) {
        .debug => "DEBUG",
        .info => "INFO ",
        .warn => "WARN ",
        .err => "ERROR",
    };

    if (builtin.target.cpu_arch == .wasm32) {
        // Call JS console.log
        wasmLog(level_str ++ " " ++ prefix ++ format, args);
    } else {
        std.debug.print(level_str ++ " " ++ prefix ++ format ++ "\n", args);
    }
}
```

**Deliverable**: Consistent logging across native + WASM

---

## Phase 13: gRPC Protocol Buffers

**Goal**: Implement Protocol Buffer parsing/serialization for Bazel API

**Zig Concepts to Learn**:
- Binary protocol parsing
- Comptime code generation
- Packed struct layout
- Varint encoding

### Tasks

- [ ] Create `src/proto/mod.zig`
- [ ] Implement varint encoding/decoding
- [ ] Implement protobuf wire format parser
- [ ] Port Bazel proto definitions to Zig structs
- [ ] Create serialization functions
- [ ] Handle streaming messages
- [ ] Write fuzz tests for parsing
- [ ] Consider using protobuf Zig library if mature

**Example**:
```zig
pub const Digest = struct {
    hash: []const u8,
    size_bytes: i64,

    pub fn encode(self: Digest, writer: anytype) !void {
        try writeField(writer, 1, .len_delimited, self.hash);
        try writeField(writer, 2, .varint, self.size_bytes);
    }

    pub fn decode(reader: anytype) !Digest {
        var result: Digest = undefined;
        while (reader.next()) |field| {
            switch (field.number) {
                1 => result.hash = try field.readBytes(),
                2 => result.size_bytes = try field.readVarint(),
                else => try field.skip(),
            }
        }
        return result;
    }
};
```

**Deliverable**: Protobuf codec for Bazel Remote Execution API

---

## Phase 14: gRPC/HTTP2 Transport

**Goal**: Implement gRPC transport for Bazel compatibility

**Zig Concepts to Learn**:
- HTTP/2 framing
- gRPC wire format
- Stream multiplexing
- Header compression (HPACK)

### Tasks

- [ ] Create `src/grpc/transport.zig`
- [ ] Implement HTTP/2 frame parsing (or use library)
- [ ] Implement gRPC framing (length-prefixed messages)
- [ ] Handle unary and streaming RPCs
- [ ] Implement ContentAddressableStorage service
- [ ] Implement ActionCache service
- [ ] For WASM: implement gRPC-Web (HTTP/1.1 based)
- [ ] Write integration tests with Bazel client

**Deliverable**: gRPC server compatible with Bazel

---

## Phase 15: IndexedDB Storage (WASM/Browser)

**Goal**: Implement browser-based persistent storage using IndexedDB

**Zig Concepts to Learn**:
- WASM extern functions
- Async patterns in Zig
- JavaScript Promise interop

### Tasks

- [ ] Create `src/storage/indexeddb.zig`
- [ ] Define extern JS functions for IndexedDB operations
- [ ] Create JS wrapper: `plasma-indexeddb.js`
- [ ] Implement Storage interface using IndexedDB
- [ ] Handle async nature of IndexedDB (callback-based)
- [ ] Add size quota management
- [ ] Test in browser environment

**JS Wrapper**:
```javascript
// plasma-indexeddb.js
class PlasmaIndexedDB {
    constructor(dbName = 'plasma-cache') {
        this.db = null;
        this.dbName = dbName;
    }

    async open() {
        return new Promise((resolve, reject) => {
            const request = indexedDB.open(this.dbName, 1);
            request.onupgradeneeded = (e) => {
                const db = e.target.result;
                db.createObjectStore('blobs', { keyPath: 'id' });
            };
            request.onsuccess = () => {
                this.db = request.result;
                resolve();
            };
        });
    }

    async put(id, data) { /* ... */ }
    async get(id) { /* ... */ }
}
```

**Deliverable**: Persistent browser storage for WASM build

---

## Phase 16: Authentication (JWT)

**Goal**: Implement JWT validation with RS256

**Zig Concepts to Learn**:
- Base64 decoding
- RSA signature verification
- JSON parsing for claims
- Cryptographic operations

### Tasks

- [ ] Create `src/auth/jwt.zig`
- [ ] Implement Base64URL decoding
- [ ] Parse JWT structure (header.payload.signature)
- [ ] Implement RS256 signature verification
- [ ] Extract and validate claims (exp, sub, etc.)
- [ ] Support multiple keys (kid lookup)
- [ ] For WASM: use SubtleCrypto via extern
- [ ] Add key hot-reload support
- [ ] Write tests with real JWT tokens

**Deliverable**: Zero-latency JWT validation on native + WASM

---

## Phase 17: Eviction Policies

**Goal**: Implement cache eviction (LRU, LFU, TTL)

**Zig Concepts to Learn**:
- Priority queues
- Custom sorting
- Time handling
- Concurrent access patterns

### Tasks

- [ ] Create `src/eviction/mod.zig`
- [ ] Implement LRU eviction (access time based)
- [ ] Implement LFU eviction (access count based)
- [ ] Implement TTL eviction (expiration based)
- [ ] Create eviction coordinator
- [ ] Add background eviction task
- [ ] Implement eviction metrics
- [ ] Test eviction under load

**Deliverable**: Complete eviction system

---

## Phase 18: Service Worker Integration (Browser)

**Goal**: Create Service Worker for browser-based cache interception

**Zig Concepts to Learn**:
- WASM module structure for Service Workers
- JavaScript interop for SW events

### Tasks

- [ ] Create `src/wasm/service-worker.zig`
- [ ] Export WASM functions for fetch interception
- [ ] Create `plasma-sw.js` Service Worker wrapper
- [ ] Intercept build tool requests
- [ ] Route to IndexedDB cache
- [ ] Handle upstream fallback
- [ ] Add cache update strategies
- [ ] Test with real build tools in browser

**Deliverable**: Browser-based cache proxy via Service Worker

---

## Phase 19: P2P Discovery (Native)

**Goal**: Implement mDNS-based peer discovery

**Zig Concepts to Learn**:
- UDP sockets
- DNS message format
- Multicast networking
- Concurrent peer management

### Tasks

- [ ] Create `src/p2p/discovery.zig`
- [ ] Implement mDNS client/server
- [ ] Handle service announcement
- [ ] Implement peer tracking
- [ ] Add peer health checking
- [ ] Handle network interface changes
- [ ] Write tests with multiple instances

**Deliverable**: Zero-config P2P discovery for local networks

---

## Phase 20: P2P Cache Sharing

**Goal**: Implement peer-to-peer cache sharing protocol

**Zig Concepts to Learn**:
- Custom binary protocols
- HMAC authentication
- Parallel network requests

### Tasks

- [ ] Create `src/p2p/protocol.zig`
- [ ] Implement P2P cache protocol (Exists, Get, Put)
- [ ] Add HMAC authentication
- [ ] Implement parallel peer querying
- [ ] Add consent system
- [ ] Create P2P metrics
- [ ] Write integration tests

**Deliverable**: Complete P2P cache sharing

---

## Phase 21: Build System Adapters

**Goal**: Implement protocol adapters for specific build systems

**Tasks**:

- [ ] **Gradle Adapter** (`src/adapters/gradle.zig`)
  - HTTP Basic Auth
  - `/cache/{hash}` endpoints

- [ ] **Nx Adapter** (`src/adapters/nx.zig`)
  - Bearer token auth
  - Nx-specific headers

- [ ] **TurboRepo Adapter** (`src/adapters/turborepo.zig`)
  - `/v8/artifacts/{hash}?teamId=` format

- [ ] **Bazel Adapter** (`src/adapters/bazel.zig`)
  - gRPC Remote Execution API
  - CAS + ActionCache services

**Deliverable**: Build system compatibility layer

---

## Phase 22: Recipe Engine

**Goal**: Implement script caching with annotations

**Zig Concepts to Learn**:
- Process spawning
- Script parsing
- Glob pattern matching
- Archive handling (tar + compression)

### Tasks

- [ ] Create `src/recipe/mod.zig`
- [ ] Implement KDL-style annotation parser
- [ ] Implement input file hashing
- [ ] Implement output archiving
- [ ] Add environment variable tracking
- [ ] Create cache key computation
- [ ] Handle script execution
- [ ] Write tests for cache hit/miss scenarios

**Deliverable**: Script caching system

---

## Phase 23: JavaScript Recipe Runtime (WASM)

**Goal**: Implement portable JavaScript recipes

**Zig Concepts to Learn**:
- Embedding JavaScript engines
- WASM-to-WASM interaction
- Async JavaScript execution

### Tasks

- [ ] Evaluate JS engine options for Zig (QuickJS bindings?)
- [ ] Create `src/recipe_portable/mod.zig`
- [ ] Implement Plasma API bindings for JS
- [ ] Handle async recipe execution
- [ ] Create recipe registry client
- [ ] Test with sample recipes

**Note**: This is complex - may use JavaScript directly in browser WASM builds instead of embedded engine.

**Deliverable**: Cross-platform recipe execution

---

## Phase 24: Hot Reload & Configuration

**Goal**: Implement configuration hot-reload

**Zig Concepts to Learn**:
- File watching
- Signal handling
- Atomic configuration updates

### Tasks

- [ ] Create `src/hot_reload.zig`
- [ ] Implement file watcher (inotify/kqueue/ReadDirectoryChanges)
- [ ] Handle SIGHUP for reload trigger
- [ ] Implement atomic config swap
- [ ] Add config validation before reload
- [ ] Test reload under load

**Deliverable**: Zero-downtime configuration updates

---

## Phase 25: Metrics & Prometheus

**Goal**: Implement metrics exposition

**Zig Concepts to Learn**:
- Counter/gauge/histogram patterns
- Text formatting
- Concurrent metric updates

### Tasks

- [ ] Create `src/metrics/mod.zig`
- [ ] Implement Counter, Gauge, Histogram types
- [ ] Create Prometheus text format encoder
- [ ] Add `/metrics` endpoint
- [ ] Instrument storage, HTTP, cache operations
- [ ] Test metric accuracy

**Deliverable**: Prometheus-compatible metrics

---

## Phase 26: Full Integration & Testing

**Goal**: End-to-end testing of complete system

### Tasks

- [ ] Create integration test suite
- [ ] Test native builds with real build systems
- [ ] Test WASM in actual browser
- [ ] Performance benchmarking
- [ ] Memory leak testing
- [ ] Stress testing
- [ ] Documentation

**Deliverable**: Production-ready Zig implementation

---

## Phase 27: Documentation & Polish

**Goal**: Complete documentation and release preparation

### Tasks

- [ ] Write comprehensive README
- [ ] Create API documentation
- [ ] Add inline code comments
- [ ] Create example configurations
- [ ] Write migration guide (from Rust version)
- [ ] Update CLAUDE.md for Zig patterns
- [ ] Create v1.0.0-zig release

**Deliverable**: Well-documented Zig release

---

## Progress Tracking

### Completed Phases
- None yet (Phase 0 starting)

### Current Phase
- Phase 0: Zig Hello World & Project Setup

### Migration Notes

**Key Differences from Rust**:

| Aspect | Rust | Zig |
|--------|------|-----|
| Memory | Ownership/borrowing | Explicit allocators |
| Errors | `Result<T, E>` | Error unions `!T` |
| Async | `async/await` + tokio | Manual or callbacks (WASM) |
| Build | Cargo | build.zig |
| Dependencies | crates.io | Zig packages or C libs |
| Generics | Monomorphization | Comptime functions |
| Strings | `String`/`&str` | `[]const u8` slices |

**WASM Compatibility Decisions**:

1. **No RocksDB**: Use in-memory + IndexedDB for WASM
2. **No native threads**: Use event loop / Web Workers
3. **No TCP sockets**: Use Fetch API via JS interop
4. **No gRPC native**: Use gRPC-Web (HTTP/1.1)
5. **Crypto**: Use std.crypto (compiles to WASM) or WebCrypto

---

## References

- [Zig Documentation](https://ziglang.org/documentation/master/)
- [Zig WASM Guide](https://ziglang.org/documentation/master/#WebAssembly)
- [Building a Web Server in Zig](https://www.openmymind.net/Building-a-Simple-HTTP-Server-in-Zig/)
- [Zig Cookbook](https://ziglearn.org/)
- [WASM and Zig](https://blog.logrocket.com/zig-webassembly-tutorial/)
